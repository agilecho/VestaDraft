"""Read recorded VESTA arm64 memory only; never starts a producer or product.

Decodes the frozen VICS Scene/light/BondSpec fields and HKLPlane prefix. It
intentionally does not declare native atom/polyhedron geometry qualified.
"""
from pathlib import Path
import hashlib
import json
import struct
import sys


def sha(raw):
    return hashlib.sha256(raw).hexdigest()


def blob(root, record, size=None):
    path = (root / record['file']).resolve()
    if path.parent != root.resolve():
        raise ValueError('dump escapes case directory')
    raw = path.read_bytes()
    if sha(raw) != record['sha256'] or 'bytes' in record and len(raw) != record['bytes']:
        raise ValueError('dump digest or size mismatch')
    if size is not None and len(raw) != size:
        raise ValueError('unexpected native record width')
    return raw


def word(raw, offset, width=4):
    return {'kind': f'F{width * 8}Bits', 'ieeeHexBE': raw[offset:offset + width][::-1].hex()}


def scene(raw):
    slots = {hex(o): struct.unpack_from('<i', raw, o)[0] for o in (
        0x234, 0x268, 0x26c, 0x270, 0x274, 0x284, 0x288, 0x28c,
        0x2a4, 0x2a8, 0x2b8, 0x5b4)}
    slots['0x228'] = struct.unpack_from('<h', raw, 0x228)[0]
    slots['0x5b0'] = struct.unpack_from('<I', raw, 0x5b0)[0]
    for o in (0x22c, 0x278, 0x298, 0x29c, 0x2ac, 0x2b4, 0x3d0, 0x3d8, 0x3e0, 0x3f8, 0x40c):
        slots[hex(o)] = word(raw, o)
    for o, n in ((0x230,3),(0x2a0,3),(0x2b0,4),(0x3cc,4),(0x3d4,4),
                 (0x3dc,4),(0x3f4,4),(0x3fc,3)):
        slots[f'{hex(o)}:{n}'] = list(raw[o:o+n])
    lights = []
    for index in range(4):
        start = 0x410 + index * 0x68
        lights.append({'index':index,
            'matrix16':[word(raw,start+i*4) for i in range(16)],
            'direction3':[word(raw,start+0x40+i*4) for i in range(3)],
            'position4':[word(raw,start+0x4c+i*4) for i in range(4)],
            'rgba':[list(raw[start+o:start+o+4]) for o in (0x5c,0x60,0x64)]})
    return {'sceneSlots':slots,'lights':lights,
        'view':{'matrix16':[word(raw,0x88+i*8,8) for i in range(16)],
                'settings4':[word(raw,o) for o in (0x108,0x10c,0x110,0x120)]}}


def rule(raw):
    # BondSpec allocation is 0x48. Old collector read 0x58; trailing allocator
    # bytes are retained in the raw dump but are never interpreted as this object.
    if len(raw) < 0x48:
        raise ValueError('short BondSpec')
    def label(offset):
        token = raw[offset:offset+11].split(b'\0',1)
        if len(token)!=2 or len(token[0])>10:
            raise ValueError('unterminated native selector')
        return token[0].hex()
    return {'selectorAHex':label(0x1c),'selectorBHex':label(0x27),
        'minimum':word(raw,0x18),'maximum':word(raw,0x14),
        'searchMode':struct.unpack_from('<i',raw,0xc)[0],
        'boundaryMode':struct.unpack_from('<i',raw,0x10)[0],
        'radius':word(raw,0x34),'lineWidth':word(raw,0x38),
        'rgba':list(raw[0x3c:0x40]),'style':raw[0x40],
        'fullFlags':struct.unpack_from('<H',raw,0x42)[0]}


def decode(root):
    root = Path(root)
    rows = [json.loads(line) for line in (root/'trace.jsonl').read_text().splitlines()]
    if [r['seq'] for r in rows] != list(range(1,len(rows)+1)):
        raise ValueError('trace sequence differs')
    result=json.loads((root/'target-result.json').read_bytes())
    if not result['launchSucceeded'] or result['callbackErrors'] or result['state']!=10:
        raise ValueError('not a complete captured run')
    snapshots=[]
    for r in rows:
        if isinstance(r.get('scene'),dict):
            item={'event':r['event'],'sequence':r['seq'],'sceneAddress':r['scene']['address'],
                'module':r['module'],**scene(blob(root,r['scene'],0x618)),'phases':[]}
            for phase in r.get('phases',[]):
                current={'ordinal':phase['scenePhaseOrdinal'],'address':phase['pointer'],
                    'populations':{},'bondRules':[]}
                for key,c in phase['collections'].items():
                    if len({p for p,f in c['entries']}) != len(c['entries']):
                        raise ValueError('duplicate native population address')
                    entry={'count':len(c['entries']),'addresses':[p for p,f in c['entries']],
                           'fullFlags':[f for p,f in c['entries']]}
                    if 'file' in c:
                        raw=blob(root,c)
                        if len(raw)!=len(c['entries'])*c['recordBytes']:
                            raise ValueError('incomplete population dump')
                        offset,width={'atoms':(0x70,2),'bonds':(0x28,1),
                            'polyhedra':(0x38,1),'faces':(0x84,2)}[key]
                        for i,(_,flag) in enumerate(c['entries']):
                            start=i*c['recordBytes']+offset
                            if int.from_bytes(raw[start:start+width],'little') != flag:
                                raise ValueError('native flag disagrees with its memory record')
                        if key=='faces':
                            entry['geometry']=[{'hkl':list(struct.unpack_from('<3d',raw,i*0x88+0x10)),
                                'hklDistanceWord':word(raw,i*0x88+0x28,8),
                                'normal3':[word(raw,i*0x88+0x48+j*8,8) for j in range(3)],
                                'offsetA':word(raw,i*0x88+0x60,8)} for i in range(len(c['entries']))]
                    current['populations'][key]=entry
                for record in phase['bondRuleRecords']:
                    current['bondRules'].append({'address':record['address'],**rule(blob(root,record,0x58))})
                item['phases'].append(current)
            snapshots.append(item)
        if 'scratch' in r:
            blob(root,r['scratch'],0x8c)
            blob(root,r['sourceLine'],0x100)
        if 'face' in r:
            blob(root,r['face'],0x88)
    return {'schema':'P2-03-recorded-native-words-1','case':str(root.resolve()),
        'traceSHA256':sha((root/'trace.jsonl').read_bytes()),'targetResult':result,'snapshots':snapshots,
        'qualification':{'allSceneSlotsAndFourInlineLightsDecoded':True,
            'fullCurrentBondRuleWordsDecoded':True,'historicalProducerFullyQualified':False,
            'limitations':['Atom-type heap definitions were not captured.',
                'Phase polyhedron 0x40 prefix is not complete geometry or Scene shape alpha.',
                'No native boundary edit/re-expansion was executed.',
                'HKLPlane geometry does not alone identify the native form/operation heap definitions.']}}


if __name__=='__main__':
    value=decode(sys.argv[1])
    Path(sys.argv[2]).write_text(json.dumps(value,indent=2)+'\n')
    print(json.dumps({'snapshots':len(value['snapshots']),'completeQualification':False}))
