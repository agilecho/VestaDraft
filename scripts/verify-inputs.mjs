import {loadMaterials} from './lib/materials.mjs';
try{const i=process.argv.indexOf('--manifest');const m=loadMaterials(i<0?'contracts/materials.lock.json':process.argv[i+1]);console.log(JSON.stringify({files:m.lock.files.length,refs:m.lock.refs.length,status:'verified'}));}catch(e){console.error('inputs:',e.message);process.exitCode=1;}
