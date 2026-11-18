import {spawnSync} from 'node:child_process';
import {scanStaticClosure} from '../helpers/static-import-closure.mjs';
import {createHash} from 'node:crypto';
import {test} from 'node:test';import assert from 'node:assert/strict';import fs from 'node:fs';import os from 'node:os';import path from 'node:path';import {loadMaterials} from '../../scripts/lib/materials.mjs';import {generate} from '../../scripts/generate-contracts.mjs';
test('source pins, refs, escapes and symlinks reject before reading',()=>{const m=loadMaterials();assert.ok(m.lock.refs.length>1000);const temp=fs.mkdtempSync(path.join(os.tmpdir(),'p201-input-'));try{fs.cpSync('contracts',path.join(temp,'contracts'),{recursive:true});const file=path.join(temp,'contracts/materials.lock.json');const edit=f=>{const x=structuredClone(m.lock);f(x);fs.writeFileSync(file,JSON.stringify(x));};edit(x=>x.files[0].sha256='0'.repeat(64));assert.throws(()=>loadMaterials(file));edit(x=>x.files[0].path='../outside');assert.throws(()=>loadMaterials(file));edit(x=>x.files=x.files.filter(a=>!a.originalPath.endsWith('history-effects-wire.schema.json')));assert.throws(()=>loadMaterials(file));edit(()=>{});const p=path.join(temp,m.lock.files[0].path);fs.unlinkSync(p);fs.symlinkSync(path.resolve(m.lock.files[0].path),p);assert.throws(()=>loadMaterials(file));}finally{fs.rmSync(temp,{recursive:true,force:true});}});
test('check compares generated file bytes and never repairs them',()=>{const temp=fs.mkdtempSync(path.join(os.tmpdir(),'p201-output-'));try{const out=generate('contracts/materials.lock.json',temp);const name=Object.keys(out)[0];fs.appendFileSync(path.join(temp,name),'corrupt');assert.throws(()=>generate('contracts/materials.lock.json',temp,true));assert.ok(fs.readFileSync(path.join(temp,name),'utf8').endsWith('corrupt'));}finally{fs.rmSync(temp,{recursive:true,force:true});}});
test('Web consumer import closure contains no Node builtins or globals',()=>{
 const visited=scanStaticClosure('web/src/kernel/resource-roles.generated.ts');
 assert.ok(visited.size>=6);
 for(const file of ['web/src/kernel/generated.ts','contracts/generated/private-host/resource-codecs.generated.mjs','scripts/lib/resource-runtime.mjs','scripts/lib/exact-json.mjs','scripts/lib/schema.mjs','scripts/lib/pointer.mjs'])assert.ok(visited.has(path.resolve(file)),'missing actual dependency '+file);
});
test('frozen current handler and envelope regressions refuse on resealed copies',()=>{const m=loadMaterials();const temp=fs.mkdtempSync(path.join(os.tmpdir(),'p201-regress-'));try{fs.cpSync('contracts',path.join(temp,'contracts'),{recursive:true});const file=path.join(temp,'contracts/materials.lock.json');for(const which of ['formats','profile']){const lock=structuredClone(m.lock);const row=lock.files.find(x=>x.originalPath===lock.contracts[which]);const source=JSON.parse(fs.readFileSync(row.path));if(which==='formats'){source.profiles[0].handlerPlanned=source.profiles[0].handlerPlanned.replace('native_vesta_v2.rs','native_vesta.rs');}else delete source.payloadEnvelope;const bytes=Buffer.from(JSON.stringify(source));fs.writeFileSync(path.join(temp,row.path),bytes);row.sha256=createHash('sha256').update(bytes).digest('hex');row.bytes=bytes.length;fs.writeFileSync(file,JSON.stringify(lock));assert.throws(()=>generate(file,path.join(temp,'out')));fs.writeFileSync(path.join(temp,row.path),fs.readFileSync(row.path));}}finally{fs.rmSync(temp,{recursive:true,force:true});}});
test('AST traverses all same-line and multiline static imports and exports',()=>{
 const root=fs.mkdtempSync(path.join(os.tmpdir(),'p201-ast-positive-'));
 try {
  const files={
   'main.ts':`import type {T} from './types.ts'; import './side.mjs'; import {a} from './a.mjs';
import {
 b
} from './b.mjs'; export {c} from './c.mjs'; export * from './d.mjs';
// import 'node:fs';
const text = "import 'node:path'";
`,
   'types.ts':'export type T = string;', 'side.mjs':'export const side=1;',
   'a.mjs':'export const a=1;', 'b.mjs':'export const b=1;',
   'c.mjs':'export const c=1;', 'd.mjs':`export * from './a.mjs'; export const d=1;`
  };
  for(const [name,text]of Object.entries(files))fs.writeFileSync(path.join(root,name),text);
  const visited=scanStaticClosure(path.join(root,'main.ts'),{root});
  assert.deepEqual([...visited].sort(),Object.keys(files).map(f=>path.join(root,f)).sort());
 }finally{fs.rmSync(root,{recursive:true,force:true});}
});
test('AST cannot hide Node dependencies after first import or in nested re-exports',()=>{
 const root=fs.mkdtempSync(path.join(os.tmpdir(),'p201-ast-negative-'));
 try {
  fs.writeFileSync(path.join(root,'safe.mjs'),'export const safe=1;');
  const direct=[`import './safe.mjs'; import 'node:fs';`,`import './safe.mjs'; import './safe.mjs'; export * from 'node:path';`,`import './safe.mjs'; import type {T} from 'node:crypto';`];
  for(const text of direct){fs.writeFileSync(path.join(root,'main.ts'),text);assert.throws(()=>scanStaticClosure(path.join(root,'main.ts'),{root}),/external\/Node dependency/);}
  fs.writeFileSync(path.join(root,'main.ts'),`import './level1.mjs';`);
  fs.writeFileSync(path.join(root,'level1.mjs'),`export * from './level2.mjs';`);
  for(const spec of ['node:fs','node:path','node:crypto']){fs.writeFileSync(path.join(root,'level2.mjs'),`export * from '${spec}';`);assert.throws(()=>scanStaticClosure(path.join(root,'main.ts'),{root}),/external\/Node dependency/);}
  for(const text of ['Buffer.from("x");','process.cwd();','require("node:fs");']){fs.writeFileSync(path.join(root,'main.ts'),text);assert.throws(()=>scanStaticClosure(path.join(root,'main.ts'),{root}),/Node global/);}
 }finally{fs.rmSync(root,{recursive:true,force:true});}
});
test('top-level commands propagate missing lock, package, tool and bad oracle expected',()=>{
 const root=fs.mkdtempSync(path.join(os.tmpdir(),'p201-top-level-'));
 try {
  for(const d of ['scripts','contracts','tests/reference'])fs.cpSync(d,path.join(root,d),{recursive:true});
  const run=(argv)=>{const r=spawnSync(argv[0],argv.slice(1),{cwd:root,encoding:'utf8',env:{...process.env,npm_config_offline:'true'}});assert.notEqual(r.status,0,JSON.stringify(argv));assert.ok(!r.stdout?.includes('"status":"passed"'));return r;};
  fs.renameSync(path.join(root,'contracts/materials.lock.json'),path.join(root,'contracts/materials.lock.saved'));
  const missingLock=run([process.execPath,'scripts/verify-inputs.mjs','--manifest','contracts/materials.lock.json']);assert.match(missingLock.stderr,/ENOENT/);
  fs.renameSync(path.join(root,'contracts/materials.lock.saved'),path.join(root,'contracts/materials.lock.json'));
  const missingPackage=run([process.execPath,'--input-type=module','-e',"await import('typescript/unstable/sync')"]);assert.match(missingPackage.stderr,/ERR_MODULE_NOT_FOUND/);
  const absentTool=run([process.execPath,'--input-type=module','-e',`import {spawnSync} from 'node:child_process';const x=spawnSync('./not-an-installed-tool',[]);if(x.error)process.exit(1);process.exit(x.status??1);`]);assert.equal(absentTool.status,1);
  // Change only the independent expected vector in an owned copy; repin that file so
  // the independent oracle reaches its hash assertion rather than just a source SHA check.
  const lockPath=path.join(root,'contracts/materials.lock.json');const lock=JSON.parse(fs.readFileSync(lockPath));const row=lock.files.find(x=>x.originalPath===lock.contracts.vectors);const vectorPath=path.join(root,row.path);const vectors=JSON.parse(fs.readFileSync(vectorPath));vectors.vectors[0].sha256='0'.repeat(64);const bytes=Buffer.from(JSON.stringify(vectors));fs.writeFileSync(vectorPath,bytes);row.sha256=createHash('sha256').update(bytes).digest('hex');row.bytes=bytes.length;fs.writeFileSync(lockPath,JSON.stringify(lock));
  const oracle=run(['python3','tests/reference/profile_oracle.py']);assert.match(oracle.stderr,/AssertionError/);
  console.log(JSON.stringify({faultControls:{missingLock:missingLock.status,missingPackage:missingPackage.status,missingTool:absentTool.status,wrongOracle:oracle.status}}));
 }finally{fs.rmSync(root,{recursive:true,force:true});}
});
