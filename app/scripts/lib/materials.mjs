import fs from 'node:fs';import path from 'node:path';import {createHash} from 'node:crypto';import {parseExact,plain} from './exact-json.mjs';
export const sha=bytes=>createHash('sha256').update(bytes).digest('hex');
export {pointer} from './pointer.mjs';
import {pointer} from './pointer.mjs';
export function loadMaterials(file='contracts/materials.lock.json'){
 const root=path.resolve(path.dirname(file),'..');const lock=plain(parseExact(fs.readFileSync(file)));const map=new Map(),cache=new Map();
 for(const row of lock.files){if(map.has(row.originalPath))throw Error('duplicate source mapping');if(path.isAbsolute(row.path)||row.path.split('/').includes('..'))throw Error('package path escape');const p=path.resolve(root,row.path);let cur=root;for(const segment of row.path.split('/')){cur=path.join(cur,segment);if(fs.lstatSync(cur).isSymbolicLink())throw Error('symlink input');}const b=fs.readFileSync(p);if(b.length!==row.bytes||sha(b)!==row.sha256)throw Error('input SHA/length '+row.logicalFile);map.set(row.originalPath,row);cache.set(row.originalPath,b);}
 const read=p=>{if(!cache.has(p))throw Error('missing ref/input '+p);return parseExact(cache.get(p));};
 // A checked file/ref graph must also contain the documents selected for each role.
 const contractNames={profile:'profile-derivation.contract.json',generation:'generator-plan.contract.json',resources:'resource-bindings.contract.json',vectors:'profile-digest-vectors.json',composition:'interface-composition.json',formats:'format-implementation-bindings.json'};
 if(!lock.contracts||Array.isArray(lock.contracts)||typeof lock.contracts!=='object'||JSON.stringify(Object.keys(lock.contracts).sort())!==JSON.stringify(Object.keys(contractNames).sort()))throw Error('contract selection set');
 for(const [role,name]of Object.entries(contractNames)){
  const selected=lock.contracts[role],row=map.get(selected);
  if(typeof selected!=='string'||!row||path.posix.basename(selected)!==name||!row.logicalFile?.endsWith('/'+name)||!Array.isArray(row.pointers)||!row.pointers.includes(''))throw Error('contract selection binding '+role);
  const document=pointer(read(selected),'');
  if(!document||typeof document!=='object'||Array.isArray(document))throw Error('contract selection document '+role);
 }
 function resolve(from,ref){const [f,ptr='']=ref.split('#');const p=f?path.posix.resolve(path.posix.dirname(from),f):from;return {file:p,pointer:ptr,value:pointer(read(p),ptr)};}
 const found=[];for(const row of lock.files){if(!row.originalPath.endsWith('.json'))continue;const x=read(row.originalPath);for(const p of row.pointers)pointer(x,p);function walk(v){if(Array.isArray(v))v.forEach(walk);else if(v&&typeof v==='object')for(const [k,a]of Object.entries(v)){if(k==='$ref'){const r=resolve(row.originalPath,a);found.push({fromPath:row.originalPath,ref:a,targetPath:r.file,pointer:r.pointer});}else walk(a);}}walk(x);}
 const normalized=a=>a.map(x=>JSON.stringify([x.fromPath,x.ref,x.targetPath,x.pointer])).sort();
 if(JSON.stringify(normalized(found))!==JSON.stringify(normalized(lock.refs)))throw Error('ref index drift');
 return {lock,read,resolve,bytes:p=>cache.get(p),root};
}
