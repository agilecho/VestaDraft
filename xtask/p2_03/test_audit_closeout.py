"""Audit regressions: omitted paths and count-only success must be observable."""
import copy
import importlib.util
import json
from pathlib import Path
import subprocess
import tempfile
import unittest

HERE = Path(__file__).resolve().parent


def load(name):
    path = HERE / (name + '.py')
    spec = importlib.util.spec_from_file_location(name, path)
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


class FullDeltaTests(unittest.TestCase):
    def delta(self, before, after):
        self.assertTrue((HERE / 'input_delta.py').is_file(), 'complete-set delta tool is missing')
        return load('input_delta').compare(before, after)

    def test_disjoint_paths_are_addition_and_deletion(self):
        result = self.delta({'old': 'a' * 64}, {'new': 'a' * 64})
        self.assertEqual(result['added'], ['new'])
        self.assertEqual(result['removed'], ['old'])
        self.assertEqual(result['modified'], [])

    def test_changed_and_unchanged_are_reported_with_new_paths(self):
        result = self.delta({'change': 'a' * 64, 'same': 'b' * 64},
                            {'change': 'c' * 64, 'same': 'b' * 64, 'extra': 'd' * 64})
        self.assertEqual(result['modified'], ['change'])
        self.assertEqual(result['unchanged'], ['same'])
        self.assertEqual(result['added'], ['extra'])

    def test_file_to_symlink_is_a_change_even_with_same_digest(self):
        result = self.delta({'x': 'a' * 64}, {'x': {'sha256': 'a' * 64, 'kind': 'symlink'}})
        self.assertEqual(result['modified'], ['x'])

    def test_malformed_hash_is_rejected(self):
        with self.assertRaises(ValueError):
            self.delta({'x': 'not-a-digest'}, {})

    def test_snapshot_covers_tracked_new_deleted_and_ignored_files(self):
        self.assertTrue((HERE / 'input_delta.py').is_file(), 'complete-set delta tool is missing')
        with tempfile.TemporaryDirectory() as tmp:
            root = Path(tmp)
            subprocess.run(['git', 'init', '-q', str(root)], check=True)
            (root / 'tracked').write_text('old')
            (root / 'deleted').write_text('gone')
            subprocess.run(['git', '-C', str(root), 'add', '--', 'tracked', 'deleted'], check=True)
            (root / 'deleted').unlink()
            (root / 'new').write_text('new')
            (root / '.gitignore').write_text('ignored\n')
            (root / 'ignored').write_text('private')
            result = load('input_delta').snapshot(root)
            self.assertEqual(set(result), {'.gitignore', 'tracked', 'new'})
            self.assertEqual(result['new']['bytes'], 3)


class DeepReadbackTests(unittest.TestCase):
    def source(self):
        return {'sourceEntities': [
            {'id': 'graph', 'nativeKind': 'vics-current-coordination-population',
             'sourceTree': {'nativeState': {'atoms': [{'x': 1}, {'x': 2}], 'bonds': [{'a': 0, 'b': 1}],
                                          'affine': [[1, 0]], 'bounds6': [0, 1], 'sites': []}}},
            {'id': 'poly', 'nativeKind': 'vics-native-display/polyhedron',
             'sourceTree': {'nativeState': {'center': 0, 'neighbors': [1]}}}]}

    def module(self):
        self.assertTrue((HERE / 'audit_readback.py').is_file(), 'deep-summary tool is missing')
        return load('audit_readback')

    def test_summary_serializes_actual_population_and_resource_bytes(self):
        summary = self.module().population_summary(self.source(), {'r': b'abc'}, lambda x: x)
        self.assertEqual(summary['population'], {'atoms': 2, 'bonds': 1, 'polyhedra': 1})
        self.assertEqual(summary['definitions'], 2)
        self.assertEqual(summary['resources'][0]['sha256'],
                         'ba7816bf8f01cfea414140de5dae2223b00361a396177a9cb410ff61f20015ad')

    def test_same_counts_with_changed_coordinates_change_deep_digest(self):
        module = self.module()
        left = self.source()
        right = copy.deepcopy(left)
        right['sourceEntities'][0]['sourceTree']['nativeState']['atoms'][0]['x'] = 9
        a = module.population_summary(left, {}, lambda x: x)
        b = module.population_summary(right, {}, lambda x: x)
        self.assertEqual(a['population'], b['population'])
        self.assertNotEqual(a['actualDeepStateSHA256'], b['actualDeepStateSHA256'])

    def test_missing_or_duplicate_population_graph_refuses(self):
        module = self.module()
        for source in ({'sourceEntities': []},
                       {'sourceEntities': [self.source()['sourceEntities'][0]] * 2}):
            with self.subTest(source=source), self.assertRaises(ValueError):
                module.population_summary(source, {}, lambda x: x)

    def test_incomplete_graph_is_not_a_zero_population_success(self):
        source = self.source()
        del source['sourceEntities'][0]['sourceTree']['nativeState']['atoms']
        with self.assertRaises(ValueError):
            self.module().population_summary(source, {}, lambda x: x)

    def test_report_publication_preserves_an_existing_receipt(self):
        module = self.module()
        with tempfile.TemporaryDirectory() as tmp:
            path = Path(tmp) / 'readback.json'
            module.publish(path, {'deepCheckPassed': True, 'value': 1})
            with self.assertRaises(FileExistsError):
                module.publish(path, {'deepCheckPassed': True, 'value': 2})
            self.assertEqual(json.loads(path.read_text())['value'], 1)


if __name__ == '__main__':
    unittest.main()
