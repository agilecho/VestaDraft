import fs from 'node:fs';
import path from 'node:path';
import {sha} from './lib/materials.mjs';
const root=process.cwd();
const excludedDirectories=['node_modules/','offline-cache/','target/'];
const excluded=p=>p==='source-package-manifest.json'||excludedDirectories.some(prefix=>p.startsWith(prefix));
function files(dir=root,base=''){
 return fs.readdirSync(dir,{withFileTypes:true}).flatMap(entry=>{
  const name=base+entry.name;if(excluded(name+'/')||excluded(name))return [];
  if(entry.isSymbolicLink())throw Error('source symlink '+name);
  return entry.isDirectory()?files(path.join(dir,entry.name),name+'/'):[name];
 }).sort();
}
function toolchain(rows){
 const inputs=rows.find(row=>row.path==='toolchain.inputs.json'),sources=rows.find(row=>row.path==='toolchain.sources.json');
 if(!inputs||!sources)throw Error('missing toolchain identity documents');
 const values=JSON.parse(fs.readFileSync(path.join(root,inputs.path)));
 return {inputs,sources,versions:{node:values.node,npm:values.npm,rust:values.rust,typescript:values.typescript,python:values.observedTools?.python?.version??null},scope:'Exact source references; current observations and historical execution limits remain distinguished in these documents.'};
}
try{
 if(fs.existsSync(path.join(root,'.git')))throw Error('use a separately staged product source directory');
 const rows=files().map(name=>{const bytes=fs.readFileSync(path.join(root,name));return {path:name,sha256:sha(bytes),bytes:bytes.length};});
 const sourceDigest=sha(JSON.stringify(rows)),identity=toolchain(rows);
 if(process.argv.includes('--seal')){
  const generated=rows.filter(({path:p})=>p.startsWith('contracts/generated/')||/\/(?:generated|profile_digests|resource_roles)\.rs$/.test(p)||p==='crates/model/src/source7/coverage.json'||p.startsWith('apps/vestacraft-web/src/kernel/'));
  const body={schema:'vestacraft.shared-contract-source-package/1',sourceDigest,exclusions:['manifest itself','node_modules/','offline-cache/ (separate dependency-bundle manifest)','target/'],files:rows,generator:rows.find(row=>row.path==='scripts/generate-contracts.mjs'),materialsLock:rows.find(row=>row.path==='contracts/materials.lock.json'),locks:rows.filter(row=>['package-lock.json','Cargo.lock'].includes(row.path)),generatedOutputs:generated,toolchain:identity};
  fs.writeFileSync('source-package-manifest.json',JSON.stringify(body,null,2)+'\n');console.log(JSON.stringify({sourceDigest,files:rows.length,status:'sealed'}));
 }else{
  const expected=JSON.parse(fs.readFileSync('source-package-manifest.json'));
  if(expected.sourceDigest!==sourceDigest||JSON.stringify(rows)!==JSON.stringify(expected.files))throw Error('source identity mismatch');
  if(JSON.stringify(expected.toolchain)!==JSON.stringify(identity))throw Error('toolchain identity binding mismatch');
  console.log(JSON.stringify({sourceDigest,files:rows.length,status:'verified'}));
 }
}catch(error){console.error('source-package:',error.message);process.exitCode=1;}
