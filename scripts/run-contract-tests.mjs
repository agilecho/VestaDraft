import fs from 'node:fs';
import {runNamedTests} from './lib/test-gate.mjs';

try {
  const inventory=JSON.parse(fs.readFileSync('tests/contracts/suite.json','utf8'));
  const result=await runNamedTests({root:process.cwd(),...inventory,report:message=>process.stdout.write(message)});
  for(const name of ['tests','pass','fail','cancelled','skipped','todo'])console.log('# '+name+' '+result[name]);
  console.log(JSON.stringify({testGate:'complete-required-leaf-tests',...result}));
}catch(error){console.error(error.stack);process.exitCode=1;}
