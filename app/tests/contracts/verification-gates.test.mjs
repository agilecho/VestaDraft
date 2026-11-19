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
    fs.cpSync(process.cwd(),root,{recursive:true,filter:source=>!path.relative(process.cwd(),source).split(path.sep).includes('node_modules')});
    const invoke=args=>spawnSync(process.execPath,['scripts/verify-source-package.mjs',...args],{cwd:root,encoding:'utf8'});
    const sealed=invoke(['--seal']);assert.equal(sealed.status,0,sealed.stderr);
    const good=invoke([]);assert.equal(good.status,0,good.stderr);
    const file=path.join(root,'source-package-manifest.json'),manifest=JSON.parse(fs.readFileSync(file));
    assert.equal(manifest.toolchain.inputs.path,'toolchain.inputs.json');
    assert.equal(manifest.toolchain.sources.path,'toolchain.sources.json');
    manifest.toolchain.inputs.sha256='0'.repeat(64);fs.writeFileSync(file,JSON.stringify(manifest));
    const rejected=invoke([]);assert.equal(rejected.status,1);assert.match(rejected.stderr,/toolchain identity binding mismatch/);
  } finally {fs.rmSync(root,{recursive:true,force:true});}
});
