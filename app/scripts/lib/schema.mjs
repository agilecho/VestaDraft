import {Decimal,plain} from './exact-json.mjs';import {pointer} from './pointer.mjs';
export function validate(value,schema,file,catalog,depth=0){
 if(depth>256)throw Error('schema recursion budget');if(schema===true)return;if(schema===false)throw Error('false schema');
 const recur=(v,s,f=file)=>validate(v,s,f,catalog,depth+1);
 if(schema.$ref){const [name,p='']=schema.$ref.split('#');const target=name?new URL(name,'file://'+file).pathname:file;if(!catalog[target])throw Error('unresolved schema');recur(value,pointer(catalog[target],p),target);}
 if(schema.allOf)schema.allOf.forEach(s=>recur(value,s));
 for(const [keyword,count]of [['oneOf',1],['anyOf',-1]])if(schema[keyword]){let passed=0;for(const s of schema[keyword])try{recur(value,s);passed++;}catch{}if(count===1?passed!==1:passed===0)throw Error(keyword);}
 if(schema.not){let valid=true;try{recur(value,schema.not);}catch{valid=false;}if(valid)throw Error('not');}
 const equal=(a,b)=>JSON.stringify(plain(a))===JSON.stringify(plain(b));
 if(Object.hasOwn(schema,'const')&&!equal(value,schema.const))throw Error('const');if(schema.enum&&!schema.enum.some(v=>equal(value,v)))throw Error('enum');
 const type=value===null?'null':value instanceof Decimal?'number':Array.isArray(value)?'array':typeof value==='object'?'object':typeof value;
 if(schema.type){const types=Array.isArray(schema.type)?schema.type:[schema.type];if(!types.includes(type)&&!(type==='number'&&types.includes('integer')&&!value.text.includes('.')))throw Error('type '+schema.type);}
 if(type==='object'){for(const k of schema.required??[])if(!Object.hasOwn(value,k))throw Error('required '+k);for(const [k,v]of Object.entries(value)){if(Object.hasOwn(schema.properties??{},k))recur(v,schema.properties[k]);else if(schema.additionalProperties===false)throw Error('unknown field '+k);else if(typeof schema.additionalProperties==='object')recur(v,schema.additionalProperties);}}
 if(type==='array'){if(schema.minItems!==undefined&&value.length<Number(schema.minItems.text??schema.minItems)||schema.maxItems!==undefined&&value.length>Number(schema.maxItems.text??schema.maxItems))throw Error('array size');value.forEach((v,i)=>{const s=schema.prefixItems?.[i]??schema.items;if(s!==undefined)recur(v,s);});}
 if(type==='string'){const n=[...value].length;if(schema.minLength!==undefined&&n<Number(schema.minLength.text??schema.minLength)||schema.maxLength!==undefined&&n>Number(schema.maxLength.text??schema.maxLength))throw Error('string size');if(schema.pattern&&!new RegExp(schema.pattern,'u').test(value))throw Error('pattern');if(schema.pattern==='^(0|[1-9][0-9]*)$'&&Number(schema.maxLength?.text??schema.maxLength)===20&&BigInt(value)>18446744073709551615n)throw Error('u64 range');}
 if(type==='number'){const n=Number(value.text);if(!Number.isFinite(n))throw Error('finite number');for(const [k,cmp]of [['minimum',(a,b)=>a>=b],['maximum',(a,b)=>a<=b],['exclusiveMinimum',(a,b)=>a>b],['exclusiveMaximum',(a,b)=>a<b]])if(schema[k]!==undefined&&!cmp(n,Number(schema[k].text??schema[k])))throw Error(k);}
}
