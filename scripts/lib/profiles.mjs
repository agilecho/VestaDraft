import {createHash} from 'node:crypto';import {loadMaterials,pointer} from './materials.mjs';import {cpj,Decimal,plain} from './exact-json.mjs';
export function deriveProfiles(manifest='contracts/materials.lock.json',{contractOverride,compositionOverride,verifyGolden=true}={}){
 const m=loadMaterials(manifest);const c=contractOverride??m.read(m.lock.contracts.profile);const composition=compositionOverride??m.read(m.lock.contracts.composition);
 const e=c.payloadEnvelope;if(!e||!c.interfaceGeneration||!e.generationSource?.contractPointer||!e.derivationSource)throw Error('missing envelope/generation');
 const generation=pointer(c,e.generationSource.contractPointer);if(!/^I[0-9]+$/.test(generation)||generation!==composition.profileGenerationId)throw Error('generation disagreement');
 const derivation=pointer(c,e.derivationSource);const fields=e.inputRecords.fields;if(JSON.stringify(fields)!==JSON.stringify(['logicalFile','pointer','projection','value'])||JSON.stringify(e.payloadSchema.required)!==JSON.stringify(['derivation','generation','inputs'])||e.payloadSchema.additionalProperties!==false)throw Error('unsupported envelope declaration');
 const roots=[...c.logicalRootBindings].sort((a,b)=>b.localRoot.length-a.localRoot.length);
 function project(v,schema){if(v instanceof Decimal)return v;if(typeof v==='string'){for(const r of roots)if(v.startsWith(r.localRoot))return r.logicalRoot+v.slice(r.localRoot.length);return v;}if(Array.isArray(v))return v.map(a=>project(a,schema));if(v&&typeof v==='object')return Object.fromEntries(Object.entries(v).filter(([k])=>!schema||!['$schema','$id','$comment','title','description','default','examples'].includes(k)&&!k.startsWith('x-')).map(([k,a])=>[k,project(a,schema)]));return v;}
 const digests={},payloads={},inputs={};const vectors=plain(m.read(m.lock.contracts.vectors));
 for(const row of c.rows){if(Object.hasOwn(digests,row.field))throw Error('duplicate profile row');const records=row.inputs.map(input=>{const pin=m.lock.files.find(r=>r.originalPath===input.path);if(!pin||pin.sha256!==input.sha256||pin.logicalFile!==input.logicalFile)throw Error('selected source binding drift '+input.logicalFile);if(!['schema-validation-keywords-only','complete-selected-JSON-value'].includes(input.projection))throw Error('unknown projection');return {logicalFile:input.logicalFile,pointer:input.pointer,projection:input.projection,value:project(pointer(m.read(input.path),input.pointer),input.projection==='schema-validation-keywords-only')};});
 const bytes=Buffer.from(cpj({derivation,generation,inputs:records}));const hash=createHash('sha256').update('vestacraft.profile-digest/1\0'+row.field+'\0','ascii').update(bytes).digest('hex');
 if(verifyGolden){const v=vectors.vectors.find(v=>v.field===row.field);if(!v||hash!==v.sha256||!bytes.equals(m.bytes(v.canonicalPayload.path)))throw Error('CPJ/golden disagreement '+row.field);}
 digests[row.field]=hash;payloads[row.field]=bytes;inputs[row.field]=row.inputs;
 }
 if(verifyGolden&&JSON.stringify(Object.keys(digests).sort())!==JSON.stringify(vectors.vectors.map(x=>x.field).sort()))throw Error('profile set mismatch');
 return {digests,payloads,inputs,generation,materials:m};
}
