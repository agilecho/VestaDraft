//! Raw declarations generate types, fixed serde, full capture and typed role visitors.
use proc_macro::TokenStream;
use proc_macro2::TokenStream as Tokens;
use quote::quote;
use syn::{Attribute, Data, DeriveInput, Fields, Ident};
fn error<T: quote::ToTokens>(t: T, code: &str) -> syn::Error {
    syn::Error::new_spanned(t, code)
}
fn camel(s: &str) -> String {
    let mut up = false;
    s.chars()
        .filter_map(|c| {
            if c == '_' {
                up = true;
                None
            } else {
                let c = if up { c.to_ascii_uppercase() } else { c };
                up = false;
                Some(c)
            }
        })
        .collect()
}
fn root_attrs(attrs: &[Attribute], en: bool) -> syn::Result<Option<(Ident, Ident)>> {
    let mut owner = None;
    let mut tagged = false;
    for a in attrs {
        if a.path().is_ident("doc") {
            continue;
        }
        if a.path().is_ident("source") {
            a.parse_nested_meta(|m| {
                if en && m.path.is_ident("tag") {
                    let v: syn::LitStr = m.value()?.parse()?;
                    if v.value() != "kind" {
                        return Err(m.error("SOURCE_TAG"));
                    }
                    tagged = true;
                    Ok(())
                } else if !en && m.path.is_ident("owner") {
                    let inner;
                    syn::parenthesized!(inner in m.input);
                    let k: Ident = inner.parse()?;
                    inner.parse::<syn::Token![,]>()?;
                    let field: Ident = inner.parse()?;
                    if !inner.is_empty() || owner.is_some() {
                        return Err(m.error("SOURCE_OWNER"));
                    }
                    owner = Some((k, field));
                    Ok(())
                } else {
                    Err(m.error("SOURCE_TYPE_ATTRIBUTE"))
                }
            })?;
        } else {
            return Err(error(a, "SOURCE_SERIALIZATION_BYPASS"));
        }
    }
    if en && !tagged {
        return Err(syn::Error::new(
            proc_macro2::Span::call_site(),
            "SOURCE_UNCLASSIFIED_VARIANT",
        ));
    }
    Ok(owner)
}
struct Member {
    decl: Tokens,
    audit: Tokens,
    remap: Tokens,
    descriptor: Tokens,
    ident: Ident,
}
fn member(f: &syn::Field, structure: bool) -> syn::Result<Member> {
    let id = f
        .ident
        .clone()
        .ok_or_else(|| error(f, "SOURCE_NAMED_FIELDS_REQUIRED"))?;
    let ty = &f.ty;
    let vis = &f.vis;
    let mut wire = camel(&id.to_string());
    let mut optional = false;
    let mut role = None;
    for a in &f.attrs {
        if a.path().is_ident("doc") {
            continue;
        }
        if !a.path().is_ident("source") {
            return Err(error(a, "SOURCE_SERIALIZATION_BYPASS"));
        }
        a.parse_nested_meta(|m| {
            if m.path.is_ident("wire") {
                let name: syn::LitStr = m.value()?.parse()?;
                wire = name.value();
                return Ok(());
            }
            if m.path.is_ident("optional") && structure && !optional {
                optional = true;
                return Ok(());
            }
            let name = m
                .path
                .get_ident()
                .map(ToString::to_string)
                .unwrap_or_default();
            if !["definition", "reference", "alias"].contains(&name.as_str()) || role.is_some() {
                return Err(m.error("SOURCE_UNREGISTERED_ROLE"));
            }
            let inner;
            syn::parenthesized!(inner in m.input);
            let kind: Ident = inner.parse()?;
            if !inner.is_empty() {
                return Err(m.error("SOURCE_KIND"));
            }
            role = Some((name, kind));
            Ok(())
        })?;
    }
    if optional && role.is_some() {
        return Err(error(f, "SOURCE_OPTIONAL_ID"));
    }
    let read = if structure {
        quote!(&self.#id)
    } else {
        quote!(#id)
    };
    let change = if structure {
        quote!(&mut self.#id)
    } else {
        quote!(#id)
    };
    let (audit, remap, role_desc) = if let Some((name, k)) = role {
        let r = match name.as_str() {
            "definition" => quote!(Definition),
            "reference" => quote!(Reference),
            _ => quote!(Alias),
        };
        (
            quote!(::vesta_model::source_audit::identity::<::vesta_model::source_audit::#k,_>(#read,::vesta_model::source_audit::Role::#r)?),
            quote!(::vesta_model::source_audit::rebind_identity::<::vesta_model::source_audit::#k,_>(#change,map)?;),
            quote!(#name),
        )
    } else {
        (
            quote!(::vesta_model::source_audit::value(#read)?),
            quote!(::vesta_model::source_audit::AuditSource::rebind(#change,map)?;),
            quote!("required-persistent"),
        )
    };
    let (decl, audit, descriptor) = if optional {
        (
            quote!(#[serde(rename=#wire, skip_serializing_if="Option::is_none")] #vis #id:#ty),
            quote!(self.#id.as_ref().map(|inner| ::vesta_model::source_audit::value(inner).map(|node| (#wire.to_owned(),node))).transpose()?),
            quote!(
                ::serde_json::json!({"wire":#wire,"role":#role_desc,"presence":"optional","type":<#ty as ::vesta_model::source_audit::AuditSource>::descriptor()})
            ),
        )
    } else {
        let audit = if structure {
            quote!(Some((#wire.to_owned(),#audit)))
        } else {
            quote!((#wire.to_owned(),#audit))
        };
        (
            quote!(#[serde(rename=#wire)] #vis #id:#ty),
            audit,
            quote!(
                ::serde_json::json!({"wire":#wire,"role":#role_desc,"type":<#ty as ::vesta_model::source_audit::AuditSource>::descriptor()})
            ),
        )
    };
    Ok(Member {
        ident: id.clone(),
        decl,
        audit,
        remap,
        descriptor,
    })
}
fn expand(input: DeriveInput) -> syn::Result<Tokens> {
    if !input.generics.params.is_empty() {
        return Err(error(&input.generics, "SOURCE_UNRESOLVED_GENERIC"));
    }
    let name = &input.ident;
    let vis = &input.vis;
    let owner = root_attrs(&input.attrs, matches!(&input.data, Data::Enum(_)))?;
    let (decl, body, rebind, desc) = match &input.data {
        Data::Struct(s) => {
            let Fields::Named(fs) = &s.fields else {
                return Err(error(&s.fields, "SOURCE_NAMED_FIELDS_REQUIRED"));
            };
            let fields = fs
                .named
                .iter()
                .map(|f| member(f, true))
                .collect::<syn::Result<Vec<_>>>()?;
            let ds = fields.iter().map(|m| &m.decl);
            let ns = fields.iter().map(|m| &m.audit);
            let rs = fields.iter().map(|m| &m.remap);
            let dd = fields.iter().map(|m| &m.descriptor);
            (
                quote!(#[derive(Clone,Debug,PartialEq,::serde::Serialize,::serde::Deserialize)] #[serde(deny_unknown_fields)] #vis struct #name{#(#ds,)*}),
                quote!(Ok(::vesta_model::source_audit::Node::Map([#(#ns,)*].into_iter().flatten().collect()))),
                quote!(#(#rs)* Ok(())),
                quote!(::serde_json::json!({"type":stringify!(#name),"fields":[#(#dd,)*]})),
            )
        }
        Data::Enum(e) => {
            let mut decls = vec![];
            let mut arms = vec![];
            let mut remaps = vec![];
            let mut descriptors = vec![];
            for v in &e.variants {
                if !v.attrs.is_empty() || v.discriminant.is_some() {
                    return Err(error(v, "SOURCE_UNCLASSIFIED_VARIANT"));
                }
                let vn = &v.ident;
                let tag = vn.to_string();
                match &v.fields {
                    Fields::Unit => {
                        decls.push(quote!(#vn));
                        arms.push(quote!(Self::#vn=>Ok(::vesta_model::source_audit::Node::Map([("kind".into(),::vesta_model::source_audit::Node::Text(#tag.into()))].into_iter().collect()))));
                        remaps.push(quote!(Self::#vn=>Ok(())));
                        descriptors.push(quote!(::serde_json::json!({"tag":#tag,"fields":[]})));
                    }
                    Fields::Named(f) => {
                        let ms = f
                            .named
                            .iter()
                            .map(|x| member(x, false))
                            .collect::<syn::Result<Vec<_>>>()?;
                        let ds = ms.iter().map(|x| &x.decl);
                        let ids: Vec<_> = ms.iter().map(|x| &x.ident).collect();
                        let ns = ms.iter().map(|x| &x.audit);
                        let rs = ms.iter().map(|x| &x.remap);
                        let dd = ms.iter().map(|x| &x.descriptor);
                        decls.push(quote!(#vn{#(#ds,)*}));
                        arms.push(quote!(Self::#vn{#(#ids,)*}=>Ok(::vesta_model::source_audit::Node::Map([("kind".into(),::vesta_model::source_audit::Node::Text(#tag.into())),("value".into(),::vesta_model::source_audit::Node::Map([#(#ns,)*].into_iter().collect()))].into_iter().collect()))));
                        remaps.push(quote!(Self::#vn{#(#ids,)*}=>{#(#rs)* Ok(())}));
                        descriptors.push(quote!(
                            ::serde_json::json!({"tag":#tag,"fields":[#(#dd,)*]})
                        ));
                    }
                    _ => return Err(error(v, "SOURCE_NAMED_VARIANT_REQUIRED")),
                }
            }
            (
                quote!(#[derive(Clone,Debug,PartialEq,::serde::Serialize,::serde::Deserialize)] #[serde(tag="kind",content="value",deny_unknown_fields)] #vis enum #name{#(#decls,)*}),
                quote!(match self{#(#arms,)*}),
                quote!(match self{#(#remaps,)*}),
                quote!(
                    ::serde_json::json!({"type":stringify!(#name),"variants":[#(#descriptors,)*]})
                ),
            )
        }
        _ => return Err(error(name, "SOURCE_UNRESOLVED_TYPE")),
    };
    let body = if let Some((k, f)) = &owner {
        quote!({let node=(||->::vesta_model::Result<::vesta_model::source_audit::Node>{#body})()?;::vesta_model::source_audit::owned::<::vesta_model::source_audit::#k,_>(&self.#f,node)})
    } else {
        body
    };
    let owner_desc = if let Some((k, f)) = &owner {
        quote!(::serde_json::json!({"kind":stringify!(#k),"field":stringify!(#f)}))
    } else {
        quote!(::serde_json::Value::Null)
    };
    Ok(
        quote!(#decl impl ::vesta_model::source_audit::AuditSource for #name {
        fn audit(&self)->::vesta_model::Result<::vesta_model::source_audit::Node>{#body}
        fn rebind(&mut self,map:&::vesta_model::source_audit::Remap)->::vesta_model::Result<()>{#rebind}
        fn descriptor()->::serde_json::Value{::serde_json::json!({"shape":#desc,"owner":#owner_desc})}
    } impl ::vesta_model::source_audit::ValueSource for #name {}),
    )
}
#[proc_macro]
pub fn source_model(input: TokenStream) -> TokenStream {
    expand(syn::parse_macro_input!(input as DeriveInput))
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
#[cfg(test)]
mod tests {
    #[test]
    fn supported_type_emits_capture_and_role_implementation() {
        let t = super::expand(syn::parse_str("pub struct Scalar {pub value:f64}").unwrap())
            .unwrap()
            .to_string();
        assert!(t.contains("AuditSource"));
        assert!(t.contains("required-persistent"));
    }
    #[test]
    fn skip_is_rejected_before_serde() {
        let e =
            super::expand(syn::parse_str("pub struct Bad {#[serde(skip)] pub value:f64}").unwrap())
                .err()
                .unwrap()
                .to_string();
        assert!(e.contains("SOURCE_SERIALIZATION_BYPASS"));
    }
}
