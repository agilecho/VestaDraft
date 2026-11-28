"""Serialize a checked YBa native-population readback using the frozen checker."""
import argparse
from hashlib import sha256
import importlib.util
import json
import os
from pathlib import Path
import sys

GRAPH_KIND = 'vics-current-coordination-population'


def canonical_digest(value):
    return sha256(json.dumps(value, sort_keys=True, separators=(',', ':'),
                             ensure_ascii=False, allow_nan=False).encode()).hexdigest()


def population_summary(source, blobs, decode):
    definitions = source['sourceEntities']
    graphs = [r for r in definitions if r['nativeKind'] == GRAPH_KIND]
    if len(graphs) != 1 or len({r['id'] for r in definitions}) != len(definitions):
        raise ValueError('one uniquely identified native population graph is required')
    graph = decode(graphs[0]['sourceTree'])['nativeState']
    if any(not isinstance(graph.get(key), list) for key in ('atoms', 'bonds', 'affine', 'bounds6', 'sites')):
        raise ValueError('native population graph is incomplete')
    polyhedra = [decode(r['sourceTree'])['nativeState'] for r in definitions
                 if r['nativeKind'] == 'vics-native-display/polyhedron']
    return {'population': {'atoms': len(graph['atoms']), 'bonds': len(graph['bonds']),
                           'polyhedra': len(polyhedra)}, 'definitions': len(definitions),
            'resources': [{'id': name, 'bytes': len(raw), 'sha256': sha256(raw).hexdigest()}
                          for name, raw in sorted(blobs.items())],
            'actualDeepStateSHA256': canonical_digest({'graph': graph, 'polyhedra': polyhedra})}


def publish(path, result):
    data = json.dumps(result, ensure_ascii=False, indent=2, allow_nan=False) + '\n'
    with Path(path).open('x') as stream:
        stream.write(data)
        stream.flush()
        os.fsync(stream.fileno())


def checked_yba(checker, artifact, zoom):
    checker, artifact = Path(checker).resolve(strict=True), Path(artifact).resolve(strict=True)
    sys.path.insert(0, str(checker.parent))
    spec = importlib.util.spec_from_file_location('frozen_yba_population_checker', checker)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    capture_path = module.fixture / 'yba-display-capture-compact.json'
    capture = module.read(capture_path)
    expected_path = module.HERE / 'verified-display-expectations.json'
    expected = module.read(expected_path)['expectedProductWords']
    original_path = Path(module.read(module.NATIVE / 'yba-proposed-matrix.json')['source'])
    original = original_path.read_bytes()
    if sha256(original).hexdigest() != capture['originalSHA256']:
        raise ValueError('original bytes do not match the frozen capture')
    # This compares all frozen display values, ownership, geometry and raw spans.
    # An exception propagates before any success receipt is published.
    identity = module.check(artifact, capture, expected, original, zoom)
    source, blobs = module.coverage.reader.published(artifact)
    summary = population_summary(source, blobs, module.audit)
    return {'schema': 'vesta.p2-03.checked-population-readback/1', 'deepCheckPassed': True,
            'identity': identity, **summary,
            'referenceGraphSHA256': canonical_digest(capture['graph']),
            'inputs': [{'path': str(p), 'bytes': p.stat().st_size,
                        'sha256': sha256(p.read_bytes()).hexdigest()}
                       for p in (checker, artifact, capture_path, expected_path, original_path)],
            'comparisonScope': 'Frozen checker: display/view, full affine/atoms/bonds/sites/polyhedron states, ownership, source spans and resource bytes.',
            'newProductRuns': 0}


def main():
    parser = argparse.ArgumentParser(description=__doc__)
    parser.add_argument('--checker', type=Path, required=True)
    parser.add_argument('--artifact', type=Path, required=True)
    parser.add_argument('--zoom', type=float, required=True)
    parser.add_argument('--output', type=Path, required=True)
    args = parser.parse_args()
    result = checked_yba(args.checker, args.artifact, args.zoom)
    publish(args.output, result)
    print(json.dumps({'deepCheckPassed': result['deepCheckPassed'], 'population': result['population']}))


if __name__ == '__main__':
    main()
