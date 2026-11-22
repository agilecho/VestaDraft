import path from 'node:path';
import {run} from 'node:test';

// Expected leaf names distinguish real assertions from Node's empty-file pass.
export async function runNamedTests({root, files, expected, report=()=>{}}) {
  if(!Array.isArray(files)||files.length===0||!Array.isArray(expected)||expected.length===0)throw Error('empty test inventory');
  const absolute=files.map(file=>path.resolve(root,file));
  if(new Set(absolute).size!==absolute.length)throw Error('duplicate test file');
  const key=(file,name)=>JSON.stringify([path.resolve(root,file),name]);
  const required=new Set();
  for(const row of expected){
    if(!absolute.includes(path.resolve(root,row.file))||typeof row.name!=='string'||!row.name)throw Error('invalid test inventory');
    const id=key(row.file,row.name);if(required.has(id))throw Error('duplicate expected test');required.add(id);
  }
  const seen=new Set(),problems=[];let summary;
  for await(const event of run({files:absolute,concurrency:1})){
    const data=event.data;
    if(event.type==='test:stdout'||event.type==='test:stderr')report(data.message);
    if(event.type==='test:fail'){report(JSON.stringify({testEvent:'fail',file:data.file,name:data.name})+'\n');problems.push('failed test: '+data.name);}
    if(event.type==='test:pass'){
      report(JSON.stringify({testEvent:'pass',file:data.file,name:data.name,skip:!!data.skip,todo:!!data.todo})+'\n');
      const id=key(data.file,data.name);
      if(data.skip||data.todo)problems.push('skipped/todo test: '+data.name);
      else if(!required.has(id))problems.push('unexpected test: '+data.name);
      else if(seen.has(id))problems.push('duplicate test: '+data.name);
      else seen.add(id);
    }
    if(event.type==='test:summary')summary=data;
  }
  const counts=summary?.counts;
  if(!summary?.success||!counts||counts.tests!==required.size||counts.passed!==required.size||counts.failed||counts.cancelled||counts.skipped||counts.todo)problems.push('incomplete or unsuccessful test counts');
  for(const id of required)if(!seen.has(id))problems.push('missing required test: '+JSON.parse(id)[1]);
  if(problems.length)throw Error('test gate: '+problems.join('; '));
  return {tests:counts.tests,pass:counts.passed,fail:counts.failed,cancelled:counts.cancelled,skipped:counts.skipped,todo:counts.todo,files:absolute.length};
}
