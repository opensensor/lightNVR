"""Exercise the real CMake/Git source preparation in disposable repositories."""
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
HELPER = ROOT / 'cmake/PrepareFaac.cmake'
PATCH = '0001-bound-input-pointer-arithmetic.patch'


def sha(data):
    return hashlib.sha256(data).hexdigest()


def tables(folder, manifest):
    raw = (json.dumps(manifest, indent=2) + '\n').encode()
    (folder / 'manifest.json').write_bytes(raw)
    lines = [f'set(FAAC_MANIFEST_SHA256 "{sha(raw)}")',
             f'set(FAAC_UPSTREAM_COMMIT "{manifest["upstream_commit"]}")',
             f'set(FAAC_PATCH_SHA256 "{manifest["patch_sha256"]}")']
    for name, key in [('FAAC_INPUT_RECORDS', 'input_files'), ('FAAC_OUTPUT_RECORDS', 'output_files')]:
        lines.append(f'set({name}')
        lines += [f'  "{p}|{digest}"' for p, digest in sorted(manifest[key].items())]
        lines.append(')')
    lines.append('set(FAAC_TRANSLATION_UNITS')
    lines += [f'  "{p}"' for p in manifest['translation_units']]
    lines.append(')')
    (folder / 'manifest.cmake').write_text('\n'.join(lines) + '\n')


class PrepareFaacTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix='faac-prepare-')
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.upstream = self.root / 'upstream'
        self.patchdir = self.root / 'patches'
        self.build = self.root / 'build' / 'faac'
        self.upstream.mkdir(); self.patchdir.mkdir(); self.build.parent.mkdir()
        files = {'libfaac/frame.c': b'int frame = 1;\n',
                 'libfaac/faac.c': b'int api = 1;\n',
                 'libfaac/faac_internal.h': b'int internal = 1;\n',
                 'include/faac.h': b'int public_api;\n',
                 'COPYING': b'test license fixture\n'}
        for p, content in files.items():
            f = self.upstream / p; f.parent.mkdir(exist_ok=True); f.write_bytes(content)
        self.git('init', '-q'); self.git('add', '.')
        self.git('-c', 'user.name=Fixture', '-c', 'user.email=fixture@example.invalid',
                 'commit', '-qm', 'disposable upstream')
        self.pin = self.git('rev-parse', 'HEAD').strip()
        subprocess.run(['git', '-C', str(self.root), 'init', '-q'], check=True)
        subprocess.run(['git', '-C', str(self.root), 'update-index', '--add', '--cacheinfo',
                        '160000,' + self.pin + ',upstream'], check=True)
        patch = b'diff --git a/libfaac/frame.c b/libfaac/frame.c\n--- a/libfaac/frame.c\n+++ b/libfaac/frame.c\n@@ -1 +1 @@\n-int frame = 1;\n+int frame = 2;\n'
        (self.patchdir / PATCH).write_bytes(patch)
        self.manifest = {'schema_version': 1, 'translation_units': ['libfaac/faac.c', 'libfaac/frame.c'], 'upstream_commit': self.pin,
                         'patch_sha256': sha(patch),
                         'input_files': {p: sha(v) for p, v in files.items()},
                         'output_files': {p: sha(v) for p, v in files.items()}}
        self.manifest['output_files']['libfaac/frame.c'] = sha(b'int frame = 2;\n')
        self.save()

    def git(self, *args):
        return subprocess.check_output(['git', '-C', str(self.upstream), *args], text=True, stderr=subprocess.DEVNULL)

    def save(self):
        tables(self.patchdir, self.manifest)

    def run_prepare(self, cwd=None):
        return subprocess.run(['cmake', f'-DFAAC_UPSTREAM_DIR={self.upstream}',
            f'-DFAAC_BUILD_DIR={self.build}', f'-DFAAC_PATCH_DIR={self.patchdir}', f'-DFAAC_SUPERPROJECT_DIR={self.root}',
            '-DFAAC_COMPILER=fixture-cc', '-DFAAC_BUILD_PROFILE=fixture-debug',
            '-P', str(HELPER)], text=True, stdout=subprocess.PIPE, stderr=subprocess.STDOUT, cwd=cwd)

    def succeeds(self):
        result = self.run_prepare(); self.assertEqual(result.returncode, 0, result.stdout)
        return json.loads((self.build / 'provenance.json').read_text())

    def fails(self, message):
        result = self.run_prepare(); self.assertNotEqual(result.returncode, 0, result.stdout)
        self.assertIn(message, result.stdout)
        self.assertEqual(self.git('rev-parse', 'HEAD').strip(), self.pin)

    def test_success_is_deterministic_and_leaves_upstream_pristine(self):
        first = self.succeeds()
        self.assertEqual((self.build / 'libfaac/frame.c').read_text(), 'int frame = 2;\n')
        self.assertEqual((self.upstream / 'libfaac/frame.c').read_text(), 'int frame = 1;\n')
        self.assertEqual(self.git('status', '--porcelain'), '')
        self.assertFalse((self.build / '.git').exists())
        self.assertEqual(first, self.succeeds())
        self.assertEqual(first['upstream_commit'], self.pin)
        self.assertEqual(len(first['files']), 5)
        for f in first['files']:
            self.assertEqual(sha(Path(f['compiled_path']).read_bytes()), f['output_sha256'])
        self.assertEqual(first['translation_units'], [str(self.build / 'libfaac/faac.c'), str(self.build / 'libfaac/frame.c')])

    def test_wrong_pin_refused(self):
        self.manifest['upstream_commit'] = '0' * 40; self.save(); self.fails('upstream commit')

    def test_source_drift_refused(self):
        (self.upstream / 'libfaac/frame.c').write_text('changed\n'); self.fails('input hash')

    def test_unlisted_source_refused(self):
        (self.upstream / 'libfaac/unknown.c').write_text('unexpected\n'); self.fails('input inventory')

    def test_json_table_disagreement_refused(self):
        (self.patchdir / 'manifest.json').write_text('{}\n'); self.fails('manifest hash')

    def test_patch_hash_drift_refused(self):
        with (self.patchdir / PATCH).open('ab') as f: f.write(b'changed\n')
        self.fails('patch hash')

    def test_unapplicable_patch_refused(self):
        p = self.patchdir / PATCH
        p.write_bytes(p.read_bytes().replace(b'-int frame = 1;', b'-int frame = 9;'))
        self.manifest['patch_sha256'] = sha(p.read_bytes()); self.save(); self.fails('patch check')
        self.assertFalse(self.build.exists())

    def test_wrong_postimage_refused(self):
        self.manifest['output_files']['libfaac/frame.c'] = '0' * 64; self.save(); self.fails('output hash')
        self.assertFalse(self.build.exists())

    def test_manifest_path_escape_refused(self):
        self.manifest['input_files']['../outside'] = '0' * 64
        self.manifest['output_files']['../outside'] = '0' * 64
        self.save(); self.fails('unsafe path')

    def test_patch_outside_allowed_scope_refused(self):
        p = self.patchdir / PATCH; p.write_bytes(p.read_bytes().replace(b'libfaac/frame.c', b'include/faac.h'))
        self.manifest['patch_sha256'] = sha(p.read_bytes()); self.save(); self.fails('patch path')

    def test_source_symlink_refused(self):
        p = self.upstream / 'libfaac/frame.c'; p.unlink(); p.symlink_to(self.upstream / 'libfaac/faac.c')
        self.fails('symlink')

    def test_output_symlink_refused(self):
        self.build.symlink_to(self.upstream, target_is_directory=True); self.fails('symlink')

    def test_output_ancestor_symlink_refused(self):
        self.build.parent.rmdir(); self.build.parent.symlink_to(self.upstream, target_is_directory=True)
        self.fails('symlink')

    def test_output_tamper_refused_and_not_destroyed(self):
        self.succeeds(); p = self.build / 'libfaac/frame.c'; p.write_text('do not erase\n')
        self.fails('output hash'); self.assertEqual(p.read_text(), 'do not erase\n')

    def test_extra_stale_output_refused(self):
        self.succeeds(); (self.build / 'libfaac/stale.c').write_text('stale\n'); self.fails('output inventory')

    def test_output_manifest_tamper_refused(self):
        self.succeeds(); (self.build / 'provenance.json').write_text('{}\n'); self.fails('provenance')

    def test_missing_output_refused(self):
        self.succeeds(); (self.build / 'include/faac.h').unlink(); self.fails('output inventory')

    def test_build_destination_may_not_overlap_upstream(self):
        self.build = self.upstream / 'copy'; self.fails('overlap')

    def test_incremental_build_cwd_does_not_rewrite_patch_paths(self):
        result = self.run_prepare(cwd=self.build.parent)
        self.assertEqual(result.returncode, 0, result.stdout)
        again = self.run_prepare(cwd=self.build.parent)
        self.assertEqual(again.returncode, 0, again.stdout)

    def test_unused_source_is_copied_but_not_claimed_as_compiled(self):
        path = 'libfaac/optional.c'; data = b'int unused;\n'
        (self.upstream / path).write_bytes(data)
        self.manifest['input_files'][path] = sha(data)
        self.manifest['output_files'][path] = sha(data)
        self.save()
        actual = self.succeeds()
        self.assertTrue((self.build / path).exists())
        self.assertNotIn(str(self.build / path), actual['translation_units'])

    def test_wrong_gitlink_refused(self):
        subprocess.run(['git', '-C', str(self.root), 'update-index', '--add', '--cacheinfo',
                        '160000,' + 'f' * 40 + ',upstream'], check=True)
        self.fails('gitlink')

    def test_literal_manifest_symlink_refused(self):
        p = self.patchdir / 'manifest.cmake'
        real = self.root / 'table'; p.rename(real); p.symlink_to(real)
        self.fails('symlink')

    def test_json_manifest_symlink_refused(self):
        p = self.patchdir / 'manifest.json'
        real = self.root / 'json'; p.rename(real); p.symlink_to(real)
        self.fails('symlink')

    def test_committed_json_and_literal_tables_are_identical(self):
        manifest = json.loads((ROOT / 'patches/faac/manifest.json').read_text())
        expected = self.root / 'expected'; expected.mkdir(); tables(expected, manifest)
        self.assertEqual((expected / 'manifest.cmake').read_bytes(),
                         (ROOT / 'patches/faac/manifest.cmake').read_bytes())


if __name__ == '__main__':
    unittest.main()
