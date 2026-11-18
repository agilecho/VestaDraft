// Original bounded JSON token parser. Decimal tokens never pass through Number.
export class Decimal { constructor(token) {this.text=decimal(token);} }
export function decimal(token) {
 const m=/^(-?)(0|[1-9][0-9]*)(?:\.([0-9]+))?(?:[eE]([+-]?[0-9]+))?$/.exec(token);
 if(!m)throw Error('number lexeme');
 let digits=m[2]+(m[3]??''); if(digits.length>4096)throw Error('number digits budget');
 const exponent=BigInt(m[4]??'0')-BigInt((m[3]??'').length);
 const significant=digits.replace(/^0+/,'');
 const adjusted=exponent+BigInt(significant.length||1)-1n;
 if(adjusted>4096n||adjusted< -4096n||exponent>8192n||exponent< -8192n)throw Error('number exponent budget');
 if(!significant)return '0';
 const point=digits.length+Number(exponent);
 let out=point<=0?'0.'+'0'.repeat(-point)+digits:point>=digits.length?digits+'0'.repeat(point-digits.length):digits.slice(0,point)+'.'+digits.slice(point);
 out=out.replace(/^0+(?=\d)/,'');if(out.includes('.'))out=out.replace(/0+$/,'').replace(/\.$/,'');return m[1]+out;
}
export function scalarString(s){for(const c of s){const n=c.codePointAt(0);if(n>=0xd800&&n<=0xdfff)throw Error('lone surrogate');}return s;}
export function parseExact(bytes,{maxBytes=16*1024*1024,maxDepth=128,maxNodes=2000000}={}) {
 if(bytes.length>maxBytes)throw Error('JSON byte budget');
 const text=typeof bytes==='string'?bytes:new TextDecoder('utf-8',{fatal:true,ignoreBOM:true}).decode(bytes);
 let i=0,nodes=0;const ws=()=>{while(/[\x20\t\r\n]/.test(text[i]??'!'))i++;};
 function string(){const start=i++;while(i<text.length){const c=text[i++];if(c==='"')return scalarString(JSON.parse(text.slice(start,i)));if(c==='\\')i++;if(c.charCodeAt(0)<32)throw Error('control character');}throw Error('unclosed string');}
 function value(depth){if(depth>maxDepth||++nodes>maxNodes)throw Error('JSON tree budget');ws();const c=text[i];
  if(c==='"')return string();
  if(c==='{'||c==='['){i++;const object=c==='{',end=object?'}':']',out=object?Object.create(null):[];ws();if(text[i]===end){i++;return out;}
   for(;;){ws();if(object){if(text[i]!=='"')throw Error('object key');const key=string();if(Object.hasOwn(out,key))throw Error('duplicate key');ws();if(text[i++]!==':')throw Error('colon');out[key]=value(depth+1);}else out.push(value(depth+1));ws();const e=text[i++];if(e===end)return out;if(e!==',')throw Error('separator');}
  }
  for(const [word,v]of [['null',null],['true',true],['false',false]])if(text.startsWith(word,i)){i+=word.length;return v;}
  const m=/^-?(?:0|[1-9][0-9]*)(?:\.[0-9]+)?(?:[eE][+-]?[0-9]+)?/.exec(text.slice(i));if(m){i+=m[0].length;return new Decimal(m[0]);}throw Error('invalid JSON token');
 }
 const out=value(0);ws();if(i!==text.length)throw Error('trailing JSON');return out;
}
export function scalarCompare(a,b){const x=[...a].map(c=>c.codePointAt(0)),y=[...b].map(c=>c.codePointAt(0));for(let i=0;i<Math.min(x.length,y.length);i++)if(x[i]!==y[i])return x[i]-y[i];return x.length-y.length;}
export function cpj(value){function tree(v){if(v===null)return ['null'];if(v instanceof Decimal)return ['number',v.text];if(typeof v==='number')throw Error('binary64 forbidden in CPJ');if(typeof v==='boolean')return ['bool',v];if(typeof v==='string')return ['string',scalarString(v)];if(Array.isArray(v))return ['array',v.map(tree)];return ['object',Object.keys(v).sort(scalarCompare).map(k=>[scalarString(k),tree(v[k])])];}return new TextEncoder().encode(JSON.stringify(tree(value)));}
export function plain(v){if(v instanceof Decimal){const n=Number(v.text);if(!Number.isFinite(n))throw Error('nonfinite consumer number');return n;}if(Array.isArray(v))return v.map(plain);if(v&&typeof v==='object')return Object.fromEntries(Object.entries(v).map(([k,a])=>[k,plain(a)]));return v;}
