import {parseExact,plain} from './exact-json.mjs';import {validate} from './schema.mjs';import {pointer} from './pointer.mjs';
export function codecRuntime(bindings,catalog,nativeFile){
 const roleMap=new Map(bindings.map(x=>[x.role,x]));
 async function digest(bytes){if(!globalThis.crypto?.subtle)throw Error('SHA256 provider unavailable');return [...new Uint8Array(await crypto.subtle.digest('SHA-256',bytes))].map(x=>x.toString(16).padStart(2,'0')).join('');}
 async function decodeResource(role,descriptor,bytes,budget={maxBytes:16*1024*1024,maxDepth:128,maxNodes:2000000}){
  const binding=roleMap.get(role);if(!binding)throw Error('unknown role');const d=parseExact(new TextEncoder().encode(JSON.stringify(descriptor)));validate(d,pointer(catalog[nativeFile],'/$defs/'+binding.descriptorDefinition),nativeFile,catalog);
  if(descriptor.mediaType!==binding.mediaType||!/^I[0-9]+$/.test(binding.schemaGeneration))throw Error('role/media/schema');
  if(!(bytes instanceof Uint8Array))throw Error('owned Uint8Array required');if(!/^(0|[1-9][0-9]*)$/.test(descriptor.byteLength)||BigInt(descriptor.byteLength)>18446744073709551615n||BigInt(descriptor.byteLength)!==BigInt(bytes.byteLength)||bytes.byteLength>budget.maxBytes)throw Error('byte length/budget');
  if(await digest(bytes)!==descriptor.sha256)throw Error('resource SHA256');const value=parseExact(bytes,budget);validate(value,binding.payloadSchema,nativeFile,catalog);return plain(value);
 }
 async function encodeResource(role,payload,budget){const binding=roleMap.get(role);if(!binding)throw Error('unknown role');const bytes=new TextEncoder().encode(JSON.stringify(payload));const descriptor={byteLength:String(bytes.length),sha256:await digest(bytes),mediaType:binding.mediaType};await decodeResource(role,descriptor,bytes,budget);return {descriptor,bytes};}
 return {decodeResource,encodeResource};
}
