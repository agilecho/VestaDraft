#!/usr/bin/env python3
"""Independent oracle: Python Decimal + stdlib JSON, no production imports."""
import json,decimal,hashlib,pathlib,sys
ROOT=pathlib.Path(__file__).resolve().parents[2]
def pairs(items):
 out={}
 for k,v in items:
  if k in out:raise ValueError('duplicate key')
  out[k]=v
 return out
def load(b):return json.loads(b.decode('utf-8'),parse_int=decimal.Decimal,parse_float=decimal.Decimal,object_pairs_hook=pairs,parse_constant=lambda s:(_ for _ in ()).throw(ValueError(s)))
def pointer(v,p):
 for k in p.split('/')[1:]:v=v[int(k)] if isinstance(v,list) else v[k.replace('~1','/').replace('~0','~')]
 return v
def encode(v):
 if v is None:return ['null']
 if isinstance(v,bool):return ['bool',v]
 if isinstance(v,decimal.Decimal):
  t=format(v,'f');t=t.rstrip('0').rstrip('.') if '.' in t else t
  return ['number','0' if not v else t]
 if isinstance(v,str):
  v.encode('utf-8',errors='strict');return ['string',v]
 if isinstance(v,list):return ['array',[encode(a) for a in v]]
 return ['object',[[k,encode(v[k])] for k in sorted(v)]]
def cpj(v):return json.dumps(encode(v),ensure_ascii=False,separators=(',',':')).encode('utf-8')
def main():
 lock=json.loads((ROOT/'contracts/materials.lock.json').read_text());mapping={x['originalPath']:x for x in lock['files']}
 def read(p):
  x=mapping[p];b=(ROOT/x['path']).read_bytes();assert hashlib.sha256(b).hexdigest()==x['sha256'];return load(b)
 c=read(lock['contracts']['profile']);v=read(lock['contracts']['vectors']);roots=sorted(c['logicalRootBindings'],key=lambda a:-len(a['localRoot']))
 def projection(value,schema):
  if isinstance(value,str):
   for r in roots:
    if value.startswith(r['localRoot']):return r['logicalRoot']+value[len(r['localRoot']):]
  if isinstance(value,list):return [projection(a,schema) for a in value]
  if isinstance(value,dict):return {k:projection(a,schema) for k,a in value.items() if not schema or k not in ['$schema','$id','$comment','title','description','default','examples'] and not k.startswith('x-')}
  return value
 e=c['payloadEnvelope'];generation=pointer(c,e['generationSource']['contractPointer']);assert generation==read(lock['contracts']['composition'])['profileGenerationId'];result={}
 for row in c['rows']:
  inputs=[dict(logicalFile=a['logicalFile'],pointer=a['pointer'],projection=a['projection'],value=projection(pointer(read(a['path']),a['pointer']),a['projection']=='schema-validation-keywords-only')) for a in row['inputs']]
  payload=dict(derivation=pointer(c,e['derivationSource']),generation=generation,inputs=inputs);b=cpj(payload);expected=next(x for x in v['vectors'] if x['field']==row['field']);gold=(ROOT/mapping[expected['canonicalPayload']['path']]['path']).read_bytes();assert b==gold,row['field'];h=hashlib.sha256(b'vestacraft.profile-digest/1\0'+row['field'].encode('ascii')+b'\0'+b).hexdigest();assert h==expected['sha256'];generated=ROOT/'contracts/generated/cpj'/f"{row['field']}.cpj";assert b==generated.read_bytes();result[row['field']]=h
 print(json.dumps(dict(oracle='Python Decimal independent full bytes',profiles=result),indent=2))
if __name__=='__main__':main()
