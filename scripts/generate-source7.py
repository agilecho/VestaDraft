#!/usr/bin/env python3
"""Generate Source7 Rust types from the byte-pinned FC bridge and SG/FS schemas."""
import argparse
from collections import Counter
import hashlib
import json
from pathlib import Path
import re

ROOT = Path(__file__).resolve().parents[1]
CONTRACTS = ROOT / "crates/codecs/contracts"
OUT = ROOT / "crates/model/src/source7"
INPUTS = {
    "FC": (CONTRACTS / "codec-source-document.schema.json", "666ba0fa0135f6956bcbf2a4ca49650ce0a3de82c4d01cb1c07f86d7eb9f9bef"),
    "SG1-r2": (CONTRACTS / "structure-geometry.contract.json", "51e05f021bad4c0a6497f9e32ab70e98bb76f00e3e0b90a57124b5db38d6fad0"),
    "FS1-r1": (ROOT / "contracts/source/adf539aac53ddd41-field-science.contract.json", "adf539aac53ddd41b46651886b00e9d5394a96c36b83f74aeb6e35984898000a"),
    "AUDIT1": (CONTRACTS / "complete-source-audit.schema.json", "f8894b66b54a40495296b5b58353a02bc29d358d231da5683dcac22b0fceab57"),
    "NATIVESECTION1": (CONTRACTS / "native-section-source.schema.json", "177c9c9295937e0881d36d2b5b2c2d6306ceb25a75d6e0714b4951aaa39fd7af"),
    "NATIVESECTION2": (CONTRACTS / "native-legacy-section-source.schema.json", "c465436e6f077077bda4a75b1bb2ce1b84fd009ca976e6ed6349181c6871db97"),
    "NBOND1": (CONTRACTS / "native-bond-source.schema.json", "3abcd56d6dd846cc7b31ba58e7d18baf754cc7845d26c6a8df183ddefdd95dda"),
}

# FC bridge semanticConstraints + typed-source-restore fullVisitorRules. Keys
# are schema pointers, not JSON member names. No runtime name discovers a role.
FC_DEFINITIONS = {
    "Resource": "Resource", "Atom": "Atom", "Bond": "Bond",
    "MoleculeFrame": "Frame", "Molecule": "Molecule", "FieldBinding": "Field",
    "SceneNode": "SceneNode", "Scene": "Scene", "Dependency": "Dependency",
    "OpaqueBlock": "OpaqueBlock", "SourcedSGShape": "Shape",
    "SourcedSGVector": "Vector", "SourcedSGMaterial": "Material",
    "SourcedSGDisplay": "Display", "SourcedSGCamera": "Camera",
    "SourcedSGRenderMesh": "Mesh", "BasisCenter": "BasisCenter",
    "SourceEntity": "SourceEntity", "SectionSource": "Section",
    "SourcedSGBondRule": "BondRule", "SourcedSGBoundary": "Boundary",
    "SourcedSGPlane": "Plane", "NativeBondSource": "BondRule",
}
SG_DEFINITIONS = {
    "SGPhaseNode": "Phase", "SGSite": "Site", "SGSymmetryOperator3": "SymmetryOp",
    "SGHalfspace": "ShapeHalfspace",
}
SG_ALIASES = {
    "SGShape": ("id", "Shape"), "SGVector": ("id", "Vector"),
    "SGBondRule": ("id", "BondRule"), "SGPlane": ("id", "Plane"),
    # SG1 design: each render mesh carries its identity and phase (independent
    # of resource digest); FC wrapper holds that same single mesh definition.
    "SGRenderMesh": ("objectId", "Mesh"),
}
FC_REFERENCES = {
    "RawSpan": {"sourceResourceId": "Resource"},
    "Bond": {"atomIds": "Atom"},
    "SceneNode": {"parent": "SceneNode"},
    "Scene": {"roots": "SceneNode", "cameraId": "Camera"},
    "Dependency": {"resolvedResourceId": "Resource"},
    "SourceEntity": {"dependencyIds": "Dependency"},
    "SectionSource": {"fieldIds": "Field"},
    "NativeBondSource": {"sgProjectionRuleIds": "BondRule"},
}
SG_REFERENCES = {
    "Image": {"phase": "Phase", "site": "Site", "operation": "SymmetryOp"},
    "SGPhaseNode": {"parent": "Phase", "fieldIds": "Field", "shapeId": "Shape"},
    "SGPhaseGraph": {"root": "Phase"},
    "SGBoundary": {"phaseId": "Phase", "ruleIds": "BondRule"},
    "SGShape": {"phaseId": "Phase"},
    "SGRenderContext": {"documentId": "Document", "epoch": "Epoch"},
    "SGRenderMesh": {"phaseId": "Phase"},
    "SGSiteSelector/oneOf/0": {"siteId": "Site"},
    "SGPlaneSpec/oneOf/0": {"phaseId": "Phase"},
    "SGParameter/oneOf/0": {"phaseId": "Phase", "siteId": "Site"},
    # SG1 design §身份 and its Cartesian coverage witness. The semantic
    # visitor resolves this to the snapshot's phase-owned selected point.
    "SGParameter/oneOf/1": {"pointId": "MeasurementPoint"},
    "SGParameter/oneOf/2": {"phaseId": "Phase"},
    "SGParameter/oneOf/3": {"phaseId": "Phase"},
    "SGParameter/oneOf/4": {"phaseId": "Phase", "siteId": "Site"},
    "SGParameter/oneOf/5": {"phaseId": "Phase", "siteId": "Site"},
}
OWNER_KINDS = {
    "document": "Document", "phase": "Phase", "molecule": "Molecule",
    "frame": "Frame", "atom": "Atom", "bond": "Bond", "scene": "Scene",
    "field": "Field", "basis-center": "BasisCenter", "shape": "Shape",
    "vector": "Vector", "material": "Material", "display": "Display",
    "camera": "Camera", "mesh": "Mesh", "scene-node": "SceneNode",
    "source-entity": "SourceEntity", "section": "Section", "bond-rule": "BondRule",
    "boundary": "Boundary", "plane": "Plane",
}
SCENE_OBJECTS = {
    "group": None, "phase-instance": "Phase", "molecule-instance": "Molecule",
    "field-instance": "Field", "explicit-mesh": "Mesh",
    "native-opaque": "SourceEntity", "section-instance": "Section", "plane-instance": "Plane",
}


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def quoted(s):
    return json.dumps(s, ensure_ascii=False)


def upper(s):
    return "".join(x[:1].upper() + x[1:] for x in re.split("[^A-Za-z0-9]+", s))


def snake(s):
    return re.sub(r"([A-Z])", r"_\1", s).lower().lstrip("_")


def pointer_value(node, pointer):
    for part in pointer.split("/")[1:]:
        part = part.replace("~1", "/").replace("~0", "~")
        node = node[int(part)] if isinstance(node, list) else node[part]
    return node


def property_pointer(pointer, field):
    return f"{pointer}/properties/{field}"


class Generator:
    def __init__(self):
        self.sources = {}
        self.pins = {}
        for ns, (path, expected) in INPUTS.items():
            raw = path.read_bytes()
            pin = sha(raw)
            if pin != expected:
                raise ValueError(f"{path}: expected {expected}, got {pin}")
            self.pins[ns] = pin
            self.sources[ns] = json.loads(raw)
        self.types = {}
        self.reached = set()
        self.roles = {}
        self.edges = set()
        self.targets = set()
        self.emitted = {}
        self.annotations = {
            ("FC", "/properties/identity/properties/documentId"): ("definition", "Document"),
            ("FC", "/properties/identity/properties/epoch"): ("definition", "Epoch"),
            ("SG1-r2", "/$defs/SGSelectedImage/properties/pointId"): ("definition", "MeasurementPoint"),
        }
        for namespace, definitions in (("FC", FC_DEFINITIONS), ("SG1-r2", SG_DEFINITIONS)):
            for obj, kind in definitions.items():
                self.annotations[namespace, f"/$defs/{obj}/properties/id"] = ("definition", kind)
        for obj, (slot, kind) in SG_ALIASES.items():
            self.annotations["SG1-r2", f"/$defs/{obj}/properties/{slot}"] = ("alias", kind)
        for namespace, references in (("FC", FC_REFERENCES), ("SG1-r2", SG_REFERENCES)):
            for obj, fields in references.items():
                for slot, kind in fields.items():
                    self.annotations[namespace, f"/$defs/{obj}/properties/{slot}"] = ("reference", kind)

    def resolve(self, ns, link):
        external, marker, pointer = link.partition("#")
        if not marker:
            raise ValueError(f"expected a fragment {link}")
        if external:
            if not external.startswith("urn:draft:input:"):
                raise ValueError(f"unknown namespace {link}")
            ns = external.split(":")[-1]
        return ns, pointer

    def visit(self, ns, ptr):
        key = ns, ptr
        if key in self.reached:
            return
        self.reached.add(key)
        schema = pointer_value(self.sources[ns], ptr)
        if not isinstance(schema, dict):
            return
        if "$ref" in schema:
            target = self.resolve(ns, schema["$ref"])
            self.edges.add((ns, schema["$ref"]))
            self.targets.add(target)
            self.visit(*target)
        for field in schema.get("properties", {}):
            self.visit(ns, property_pointer(ptr, field))
        for field in ("oneOf", "anyOf", "allOf", "prefixItems"):
            for i in range(len(schema.get(field, []))):
                self.visit(ns, f"{ptr}/{field}/{i}")
        if "items" in schema:
            self.visit(ns, f"{ptr}/items")
        if isinstance(schema.get("additionalProperties"), dict):
            self.visit(ns, f"{ptr}/additionalProperties")

    def checked_schema(self, ns, ptr):
        return pointer_value(self.sources[ns], ptr)

    def type_name(self, ns, ptr):
        name = upper(ptr.split("/")[-1])
        if ptr == "":
            return "SourceDocument"
        if ptr == "/properties/identity":
            return "SnapshotIdentity"
        prefix = {"FC": "", "SG1-r2": "", "FS1-r1": "Fs", "NATIVESECTION1": "Modern",
                  "NATIVESECTION2": "Legacy", "NBOND1": "Bond"}.get(ns, ns)
        return prefix + name

    def named(self, ns, ptr):
        if (ns, ptr) == ("AUDIT1", "/$defs/AuditNode"):
            return "AuditTree"
        if (ns, ptr) == ("SG1-r2", "/$defs/Id"):
            raise ValueError("SG Id needs an exact per-use type role")
        key = ns, ptr
        if key not in self.types:
            name = self.type_name(ns, ptr)
            self.types[key] = name
            ty = self.shape(ns, ptr, self.checked_schema(ns, ptr), name)
            if ty != name:
                self.types[key] = ty
        return self.types[key]

    def shape(self, ns, ptr, schema, hint, annotation=None):
        if annotation:
            role, kind = annotation
            if schema.get("type") == "array":
                item = self.shape(ns, f"{ptr}/items", schema["items"], hint, annotation)
                return self.array(schema, item)
            if self.nullable(schema):
                member, suffix = self.non_null(schema)
                return f"Option<{self.shape(ns, ptr + suffix, member, hint, annotation)}>"
            return f"sa::Id<sa::{kind}>"
        if "$ref" in schema:
            return self.named(*self.resolve(ns, schema["$ref"]))
        if self.nullable(schema):
            inner, suffix = self.non_null(schema)
            return f"Option<{self.shape(ns, ptr + suffix, inner, hint)}>"
        if "oneOf" in schema:
            branches = [(ns, f"{ptr}/oneOf/{i}", branch) for i, branch in enumerate(schema["oneOf"])]
            self.union(hint, branches)
            return hint
        kind = schema.get("type")
        if kind == "object":
            if ns == "FC" and ptr in ("/$defs/Owner", "/$defs/SourceEntryRef", "/$defs/SceneNode"):
                self.discriminated(ptr, schema, hint)
                return hint
            self.structure(ns, ptr, schema, hint)
            return hint
        if kind == "array":
            return self.array(schema, self.shape(ns, f"{ptr}/items", schema["items"], hint + "Item"))
        if kind in ("number", "integer"):
            return "Number"
        if kind == "null":
            return "()"
        if kind == "boolean" or isinstance(schema.get("const"), bool):
            return "bool"
        vals = schema.get("enum", [schema.get("const")])
        if kind == "string" or all(isinstance(v, str) for v in vals):
            return "String"
        if vals and all(type(v) is int for v in vals):
            return "Number"
        raise ValueError(f"{ns}#{ptr} unsupported shape {schema}")

    @staticmethod
    def nullable(schema):
        return isinstance(schema.get("type"), list) or any(
            branch.get("type") == "null" for branch in schema.get("oneOf", [])
        )

    @staticmethod
    def non_null(schema):
        if "type" in schema:
            alternatives = schema["type"]
            assert len(alternatives) == 2 and "null" in alternatives
            return dict(schema, type=next(t for t in alternatives if t != "null")), ""
        assert len(schema["oneOf"]) == 2
        return next((b, f"/oneOf/{i}") for i, b in enumerate(schema["oneOf"]) if b.get("type") != "null")

    @staticmethod
    def array(schema, inner):
        if schema.get("minItems") is not None and schema.get("minItems") == schema.get("maxItems"):
            return f"[{inner}; {schema['minItems']}]"
        return f"Vec<{inner}>"

    def structure(self, ns, ptr, schema, name, overrides=None):
        assert schema.get("additionalProperties") is False, (ns, ptr)
        overrides = overrides or {}
        lines = ["source_model! {"]
        owner = None
        for field, member in schema["properties"].items():
            p = property_pointer(ptr, field)
            annotation = overrides.get(field, self.annotations.get((ns, p)))
            if annotation and annotation[0] == "definition" and field in ("id", "documentId"):
                owner = (annotation[1], snake(field))
        if owner:
            lines.append(f"    #[source(owner({owner[0]}, {owner[1]}))]")
        lines.append(f"    pub struct {name} {{")
        for field, member in schema["properties"].items():
            p = property_pointer(ptr, field)
            annotation = overrides.get(field, self.annotations.get((ns, p)))
            optional = field not in schema["required"]
            kind = "NullValue" if field in overrides and overrides[field] is None else self.shape(
                ns, p, member, name + upper(field), annotation
            )
            attributes = [f"wire = {quoted(field)}"]
            if optional:
                attributes.append("optional")
                kind = f"Option<{kind}>"
            if annotation:
                attributes.append(f"{annotation[0]}({annotation[1]})")
                self.roles.setdefault((ns, p), set()).add(annotation)
            ident = snake(field)
            if ident in ("type", "ref", "match", "use"):
                ident = f"r#{ident}"
            lines.extend((f"        #[source({', '.join(attributes)})]", f"        pub {ident}: {kind},"))
        lines.extend(("    }", "}"))
        self.register(name, "\n".join(lines))

    def variant_guard(self, schema, others):
        properties = schema["properties"]
        consts = [(k, v["const"]) for k, v in properties.items() if "const" in v]
        if consts:
            for k, value in consts:
                if all(o["properties"].get(k, {}).get("const") != value for o in others):
                    return f"value.get({quoted(k)}).and_then(Value::as_str) == Some({quoted(value)})"
        exclusive = set(schema["required"]) - set().union(*(set(o["properties"]) for o in others))
        if exclusive:
            return f"value.get({quoted(sorted(exclusive)[0])}).is_some()"
        raise ValueError(f"unqualified union branches: {schema}")

    def union(self, name, branches):
        expanded = [
            (self.checked_schema(*self.resolve(ns, branch["$ref"])) if "$ref" in branch else branch)
            for ns, _, branch in branches
        ]
        variants = []
        for i, (ns, ptr, branch) in enumerate(branches):
            variant = f"Branch{i}"
            ty = self.shape(ns, ptr, branch, name + variant)
            variants.append((variant, ty, self.variant_guard(expanded[i], expanded[:i] + expanded[i + 1:])))
        self.emit_union(name, variants)

    def discriminated(self, ptr, schema, name):
        values = schema["properties"]["kind"]["enum"]
        variants = []
        for discriminator in values:
            obj = name + upper(discriminator)
            field, kind = (
                ("sourceObjectId", SCENE_OBJECTS[discriminator]) if name == "SceneNode"
                else ("id", OWNER_KINDS[discriminator])
            )
            self.structure("FC", ptr, schema, obj,
                           {field: ("reference", kind) if kind else None})
            guard = f'value.get("kind").and_then(Value::as_str) == Some({quoted(discriminator)})'
            variants.append((upper(discriminator), obj, guard))
        self.emit_union(name, variants)

    def emit_union(self, name, variants):
        lines = [f"closed_union!({name} {{"]
        lines.extend(f"    {variant}({ty}) |value| {guard}," for variant, ty, guard in variants)
        lines.append("});")
        self.register(name, "\n".join(lines))

    def register(self, name, code):
        if name in self.emitted:
            raise ValueError(f"generated name collision {name}")
        self.emitted[name] = code

    def generate(self):
        self.visit("FC", "")
        self.named("FC", "")
        absent = set(self.annotations) - set(self.roles)
        if absent:
            raise ValueError(f"unused roles {sorted(absent)}")
        counts = Counter(segment for _, p in self.reached for segment in [p.split("/")[-2] if "/" in p else ""])
        root_keys = self.sources["FC"]["required"]
        assert (len(root_keys), len(self.edges), len(self.targets), counts["properties"], counts["oneOf"]) == (
            22, 89, 86, 501, 79,
        ), (len(root_keys), len(self.edges), len(self.targets), counts)
        coverage = {
            "pins": self.pins,
            "requiredRoots": root_keys,
            "referenceEdges": sorted(self.edges),
            "targetPointers": sorted(f"{n}#{p}" for n, p in self.targets),
            "properties": sorted(f"{n}#{p}" for n, p in self.reached if p.rsplit("/", 2)[-2:-1] == ["properties"]),
            "combinationBranches": sorted(f"{n}#{p}" for n, p in self.reached if p.rsplit("/", 2)[-2:-1] == ["oneOf"]),
            "roles": {f"{n}#{p}": sorted(r) for (n, p), r in sorted(self.roles.items())},
            "scopeRules": {
                "SG1-r2#/$defs/SGParameter/oneOf/1/properties/pointId":
                    "one actual SGSelectedImage of the covariance's owning phase; missing or incompatible scope refuses qualification",
            },
        }
        code = """// Generated by scripts/generate-source7.py; edit the schema and exact roles there.
use super::{AuditTree, NullValue};
use crate::source_audit::{self as sa, AuditSource};
use serde::{Deserialize, Serialize};
use serde_json::{Number, Value};
use vesta_source_derive::source_model;

"""
        return {
            OUT / "generated.rs": code + "\n\n".join(self.emitted.values()) + "\n",
            OUT / "coverage.json": json.dumps(coverage, indent=2, sort_keys=True) + "\n",
        }


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument("--check", action="store_true")
    args = parser.parse_args()
    files = Generator().generate()
    for path, content in files.items():
        if args.check:
            if not path.exists() or path.read_text() != content:
                raise SystemExit(f"stale generated Source7 {path}")
        else:
            path.write_text(content)


if __name__ == "__main__":
    main()
