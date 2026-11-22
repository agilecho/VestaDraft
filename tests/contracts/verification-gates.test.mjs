import {test} from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import {spawnSync} from 'node:child_process';

test('input entry rejects missing and misbound contract selections',()=>{
  const root=fs.mkdtempSync(path.join(os.tmpdir(),'p201-selection-'));
  try {
    fs.cpSync('contracts',path.join(root,'contracts'),{recursive:true});
    const file=path.join(root,'contracts/materials.lock.json');
    const original=JSON.parse(fs.readFileSync(file));
    const invoke=()=>spawnSync(process.execPath,[path.resolve('scripts/verify-inputs.mjs'),'--manifest',file],{encoding:'utf8'});
    assert.equal(invoke().status,0);
    const mutations=[
      x=>{x.contracts.profile='vesta-material:no-such/profile.json';},
      x=>{x.contracts.profile=x.contracts.vectors;},
      x=>{delete x.contracts.profile;},
      x=>{x.contracts.profile=null;},
      x=>{x.contracts.extra=x.contracts.profile;},
      x=>{delete x.contracts;},
      x=>{x.files.find(r=>r.originalPath===x.contracts.profile).pointers=['/payloadEnvelope'];},
    ];
    for(const mutate of mutations){
      const lock=structuredClone(original);mutate(lock);fs.writeFileSync(file,JSON.stringify(lock));
      const result=invoke();assert.notEqual(result.status,0);assert.match(result.stderr,/contract selection/);assert.ok(!result.stdout.includes('"status":"verified"'));
    }
  } finally {fs.rmSync(root,{recursive:true,force:true});}
});

function probe(sources,names=['first','second']){
  const root=fs.mkdtempSync(path.join(os.tmpdir(),'p201-test-gate-'));
  try {
    const file=path.join(root,'cases.mjs');fs.writeFileSync(file,sources);
    const helper=new URL('../../scripts/lib/test-gate.mjs',import.meta.url).href;
    const runner=path.join(root,'runner.mjs');
    fs.writeFileSync(runner,`import {runNamedTests} from ${JSON.stringify(helper)};try{const result=await runNamedTests(${JSON.stringify({root,files:['cases.mjs'],expected:names.map(name=>({file:'cases.mjs',name}))})});console.log(JSON.stringify(result));}catch(e){console.error(e.message);process.exitCode=1;}`);
    // The child is a fresh runner, rather than a recursive node --test worker.
    const env={...process.env};delete env.NODE_TEST_CONTEXT;
    return spawnSync(process.execPath,[runner],{encoding:'utf8',timeout:10000,env});
  } finally {fs.rmSync(root,{recursive:true,force:true});}
}

test('test gate rejects empty skipped and failed required tests',()=>{
  for(const source of [
    'export {};',
    "import {test} from 'node:test';test('first',{skip:true},()=>{});test('second',{skip:true},()=>{});",
    "import {test} from 'node:test';test('first',()=>{throw Error('fixture failure')});test('second',()=>{});",
  ]){const result=probe(source);assert.equal(result.status,1,result.stderr);assert.match(result.stderr,/test gate:/);}
});

test('test gate requires the complete unique named test inventory',()=>{
  const base="import {test} from 'node:test';test('first',()=>{});test('second',()=>{});";
  const passed=probe(base);assert.equal(passed.status,0,passed.stderr);assert.equal(JSON.parse(passed.stdout).pass,2);
  for(const source of [base.replace("test('second',()=>{});",''),base+"test('extra',()=>{});",base.replace("'second'","'first'")]){
    const result=probe(source);assert.equal(result.status,1,result.stderr);assert.match(result.stderr,/test gate:/);
  }
  assert.equal(probe(base,['first','first']).status,1);
});

test('source manifest verifies explicit toolchain identity references',()=>{
  const root=fs.mkdtempSync(path.join(os.tmpdir(),'p201-toolchain-manifest-'));
  try {
    const productPaths=['Cargo.toml','Cargo.lock','rust-toolchain.toml','package.json','package-lock.json',
      'toolchain.inputs.json','toolchain.sources.json','tsconfig.contracts.json','.cargo','apps','assets',
      'contracts','crates','scripts','tests','xtask'];
    for(const source of productPaths)fs.cpSync(source,path.join(root,source),{recursive:true});
    const invoke=args=>spawnSync(process.execPath,['scripts/verify-source-package.mjs',...args],{cwd:root,encoding:'utf8'});
    fs.writeFileSync(path.join(root,'.git'),'worktree marker');
    assert.match(invoke(['--seal']).stderr,/separately staged product source directory/);
    fs.unlinkSync(path.join(root,'.git'));
    fs.mkdirSync(path.join(root,'target'));
    fs.writeFileSync(path.join(root,'target','local-build'),'excluded output');
    const sealed=invoke(['--seal']);assert.equal(sealed.status,0,sealed.stderr);
    const good=invoke([]);assert.equal(good.status,0,good.stderr);
    const file=path.join(root,'source-package-manifest.json'),manifest=JSON.parse(fs.readFileSync(file));
    assert.equal(manifest.toolchain.inputs.path,'toolchain.inputs.json');
    assert.equal(manifest.toolchain.sources.path,'toolchain.sources.json');
    assert.deepEqual(manifest.locks.map(row=>row.path),['Cargo.lock','package-lock.json']);
    assert.ok(!manifest.files.some(row=>row.path.startsWith('target/')));
    for(const generated of ['crates/bindings/src/generated.rs','crates/model/src/source7/generated.rs',
      'crates/model/src/source7/coverage.json','apps/vestacraft-web/src/kernel/generated.ts']){
      assert.ok(manifest.generatedOutputs.some(row=>row.path===generated),generated);
    }
    manifest.toolchain.inputs.sha256='0'.repeat(64);fs.writeFileSync(file,JSON.stringify(manifest));
    const rejected=invoke([]);assert.equal(rejected.status,1);assert.match(rejected.stderr,/toolchain identity binding mismatch/);
  } finally {fs.rmSync(root,{recursive:true,force:true});}
});
