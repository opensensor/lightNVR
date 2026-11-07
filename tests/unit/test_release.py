#!/usr/bin/env python3
"""Release coordination regressions; no GitHub writes or real release tags."""
import importlib.util
import json
from pathlib import Path
import shutil
import subprocess
import tempfile
import unittest
from unittest.mock import patch

SPEC = importlib.util.spec_from_file_location(
    'release', Path(__file__).resolve().parents[2] / 'scripts/release.py')
release = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(release)


def run(workflow, sha='candidate', status='completed', conclusion='success', ident=1, **extra):
    return dict(id=ident, path=f'.github/workflows/{workflow}', head_sha=sha,
                head_branch='main', event='push', status=status,
                conclusion=conclusion, html_url=f'https://example.test/runs/{ident}', **extra)


def green(sha='candidate'):
    return [run(w, sha, ident=i) for i, w in enumerate(sorted(release.REQUIRED_WORKFLOWS), 1)]


class CiTests(unittest.TestCase):
    def test_missing_workflows_wait(self):
        waiting, failed = release.ci_state([], 'candidate')
        self.assertEqual(set(waiting), release.REQUIRED_WORKFLOWS)
        self.assertFalse(failed)

    def test_other_sha_pr_and_dispatch_cannot_satisfy_ci(self):
        runs = green('old')
        runs += [dict(r, event='pull_request') for r in green()]
        runs += [dict(r, event='workflow_dispatch') for r in green()]
        self.assertEqual(set(release.ci_state(runs, 'candidate')[0]), release.REQUIRED_WORKFLOWS)

    def test_latest_run_overrides_previous_success(self):
        runs = green() + [run('lightnvr-rs.yml', ident=99, status='in_progress', conclusion=None)]
        waiting, failed = release.ci_state(runs, 'candidate')
        self.assertEqual(waiting, ['lightnvr-rs.yml (in_progress)'])
        self.assertFalse(failed)

    def test_failures_and_cancellations_block(self):
        for conclusion in ('failure', 'cancelled', 'timed_out', 'skipped', 'neutral'):
            with self.subTest(conclusion=conclusion):
                runs = green() + [run('lightnvr-rs.yml', ident=99, conclusion=conclusion)]
                self.assertTrue(release.ci_state(runs, 'candidate')[1])

    def test_optional_triggered_workflow_also_blocks(self):
        runs = green() + [run('docs-link-check.yml', conclusion='failure')]
        self.assertTrue(release.ci_state(runs, 'candidate')[1])

    def wait(self, pages, heads, **kwargs):
        with patch.object(release, 'fetch_main', side_effect=heads), \
             patch.object(release, 'check_descendants'), \
             patch.object(release, 'check_versions'), \
             patch.object(release, 'api', side_effect=pages):
            return release.wait_for_ci('owner/repo', 'candidate', '1.2.3', 10, interval=0)

    def test_follows_regeneration_instead_of_failing_cancelled_old_run(self):
        old = green() + [run('lightnvr-rs.yml', ident=99, conclusion='cancelled')]
        jobs = [{'jobs': [{'steps': [{'name': release.FRESHNESS_STEP, 'conclusion': 'success'}]}]}]
        result = self.wait([[{'workflow_runs': old}], [{'workflow_runs': green('bot')}], jobs],
                           ['candidate', 'bot', 'bot', 'bot', 'bot'])
        self.assertEqual(result, 'bot')

    def test_green_regeneration_run_without_freshness_proof_is_rejected(self):
        for conclusion in ('skipped', 'failure', None):
            jobs = [{'jobs': [{'steps': [{'name': release.FRESHNESS_STEP, 'conclusion': conclusion}]}]}]
            with self.subTest(conclusion=conclusion), self.assertRaisesRegex(release.ReleaseError, 'did not verify'):
                self.wait([[{'workflow_runs': green()}], jobs], ['candidate'] * 3)

    def test_timeout_does_not_return_candidate(self):
        with patch.object(release.time, 'monotonic', side_effect=[0, 11]):
            with self.assertRaisesRegex(release.ReleaseError, 'Timed out'):
                release.wait_for_ci('owner/repo', 'candidate', '1.2.3', 10)

    def test_existing_tag_never_follows_main(self):
        jobs = [{'jobs': [{'steps': [{'name': release.FRESHNESS_STEP, 'conclusion': 'success'}]}]}]
        with patch.object(release, 'fetch_main') as fetch, \
             patch.object(release, 'check_versions'), \
             patch.object(release, 'api', side_effect=[[{'workflow_runs': green()}], jobs]):
            self.assertEqual(release.wait_for_ci('owner/repo', 'candidate', '1.2.3', 10,
                                                follow_main=False), 'candidate')
        fetch.assert_not_called()


class RepositoryTests(unittest.TestCase):
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix='lightnvr-release-test-')
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name) / 'repo'
        self.root.mkdir()
        self.remote = Path(self.tmp.name) / 'remote.git'
        subprocess.run(['git', 'init', '--bare', '-q', str(self.remote)], check=True)
        subprocess.run(['git', 'init', '-q', '-b', 'main', str(self.root)], check=True)
        self.root_patch = patch.object(release, 'ROOT', self.root)
        self.root_patch.start()
        self.addCleanup(self.root_patch.stop)
        release.git('config', 'user.name', 'Release Test')
        release.git('config', 'user.email', 'test@example.test')
        release.git('remote', 'add', 'origin', str(self.remote))
        (self.root / 'web').mkdir()
        (self.root / 'rust').mkdir()
        (self.root / 'CMakeLists.txt').write_text('project(LightNVR VERSION 1.2.3 LANGUAGES C)\n')
        (self.root / 'web/package.json').write_text(json.dumps({'version': '1.2.3'}))
        (self.root / 'web/package-lock.json').write_text(json.dumps({
            'version': '1.2.3', 'lockfileVersion': 3,
            'packages': {'': {'version': '1.2.3'}, 'node_modules/example': {'version': '9.8.7'}}}))
        (self.root / 'rust/Cargo.toml').write_text('[package]\nversion = "1.2.3"\n')
        self.anchor = self.commit('initial')
        release.git('push', '-q', 'origin', 'main')

    def commit(self, message):
        release.git('add', '.')
        release.git('commit', '-q', '-m', message)
        return release.git('rev-parse', 'HEAD')

    def test_generated_descendant_allowed(self):
        (self.root / 'rust/generated.rs').write_text('// generated\n')
        head = self.commit('regenerated')
        release.check_descendants(self.anchor, head)

    def test_source_change_then_revert_still_rejected(self):
        (self.root / 'source.c').write_text('change\n')
        self.commit('new feature')
        (self.root / 'source.c').unlink()
        head = self.commit('revert feature')
        with self.assertRaisesRegex(release.ReleaseError, 'outside generated'):
            release.check_descendants(self.anchor, head)

    def test_stale_cargo_version_rejected(self):
        (self.root / 'rust/Cargo.toml').write_text('[package]\nversion = "1.2.2"\n')
        self.commit('stale')
        with self.assertRaisesRegex(release.ReleaseError, 'stale'):
            release.check_versions('HEAD', '1.2.3', rust=True)

    def test_no_push_creates_no_tag(self):
        (self.root / 'local-notes.md').write_text('Keep these notes local.\n')
        with patch.object(release, 'publish') as publish:
            release.main(['1.2.3', '--no-push', '--yes'])
        publish.assert_not_called()
        self.assertEqual(release.git('tag', '--list'), '')
        self.assertEqual(release.git('ls-remote', '--tags', 'origin'), '')
        self.assertEqual((self.root / 'local-notes.md').read_text(), 'Keep these notes local.\n')

    def test_tracked_changes_block_preparation(self):
        (self.root / 'CMakeLists.txt').write_text('uncommitted changes\n')
        for staged in (False, True):
            if staged:
                release.git('add', 'CMakeLists.txt')
            with self.subTest(staged=staged), self.assertRaisesRegex(release.ReleaseError, 'Tracked files'):
                release.main(['1.2.4', '--no-push', '--yes'])
        self.assertEqual(release.git('rev-parse', 'HEAD'), self.anchor)
        self.assertEqual(release.git('tag', '--list'), '')

    def install_bump_script(self):
        (self.root / 'scripts').mkdir()
        shutil.copy2(Path(SPEC.origin).parent / 'bump-version.sh', self.root / 'scripts/bump-version.sh')
        (self.root / '.gitignore').write_text('web/js/version.js\ninclude/core/version.h\n')
        self.commit('prepare scripts')

    def assert_web_lock_version(self, expected):
        lock = json.loads((self.root / 'web/package-lock.json').read_text())
        self.assertEqual(lock['version'], expected)
        self.assertEqual(lock['packages']['']['version'], expected)
        self.assertEqual(lock['packages']['node_modules/example']['version'], '9.8.7')

    def test_version_bump_preparation_creates_commit_but_no_tag(self):
        self.install_bump_script()
        with patch.object(release, 'publish') as publish:
            release.main(['1.2.4', '--no-push', '--yes'])
        publish.assert_not_called()
        self.assertEqual(release.versions('HEAD'), ('1.2.4', '1.2.4'))
        self.assert_web_lock_version('1.2.4')
        self.assertEqual(release.git('status', '--porcelain'), '')
        self.assertEqual(release.git('tag', '--list'), '')
        self.assertEqual(release.git('rev-parse', 'origin/main'), self.anchor)

    def test_preparation_repairs_legacy_web_version(self):
        self.install_bump_script()
        (self.root / 'web/package.json').write_text(json.dumps({'version': '1.0.0'}))
        self.commit('legacy web package version')
        release.main(['1.2.4', '--no-push', '--yes'])
        self.assertEqual(release.versions('HEAD'), ('1.2.4', '1.2.4'))
        self.assert_web_lock_version('1.2.4')
        self.assertEqual(release.git('status', '--porcelain'), '')
        self.assertEqual(release.git('tag', '--list'), '')

    def test_preparation_repairs_web_when_cmake_already_has_target_version(self):
        self.install_bump_script()
        (self.root / 'web/package.json').write_text(json.dumps({'version': '1.0.0'}))
        self.commit('partial preparation')
        release.main(['1.2.3', '--no-push', '--yes'])
        self.assertEqual(release.versions('HEAD'), ('1.2.3', '1.2.3'))
        self.assert_web_lock_version('1.2.3')
        self.assertEqual(release.git('status', '--porcelain'), '')

    def test_publication_still_rejects_mismatched_web_version(self):
        (self.root / 'web/package.json').write_text(json.dumps({'version': '1.0.0'}))
        self.commit('legacy web package version')
        with self.assertRaisesRegex(release.ReleaseError, 'versions'):
            release.check_versions('HEAD', '1.2.3', rust=True)

    def test_prepare_and_resume_generated_commit(self):
        real_command = release.command
        def command(*args, **kwargs):
            if args[:3] == ('gh', 'repo', 'view'):
                return json.dumps({'nameWithOwner': 'owner/repo'})
            if args[:2] == ('gh', 'api'):
                return '{}'
            return real_command(*args, **kwargs)
        with patch.object(release, 'command', side_effect=command), \
             patch.object(release, 'publish') as publish:
            release.main(['1.2.3', '--prepare-only', '--yes'])
            publish.assert_not_called()
            self.assertEqual(release.git('tag', '--list'), '')
            (self.root / 'rust/generated.rs').write_text('// generated\n')
            head = self.commit('regenerated')
            release.git('push', '-q', 'origin', 'main')
            release.git('reset', '--hard', self.anchor)
            release.main(['1.2.3', '--publish-only', '--yes'])
            publish.assert_called_once_with('owner/repo', head, '1.2.3', 14400)

    def test_publish_only_pushes_requested_tag_at_verified_commit(self):
        (self.root / 'local-notes.md').write_text('Keep these notes local.\n')
        release.git('tag', 'unrelated-local-tag')
        with patch.object(release, 'wait_for_ci', return_value=self.anchor):
            release.publish('owner/repo', self.anchor, '1.2.3', 10)
        remote_tags = release.git('ls-remote', '--tags', 'origin')
        self.assertIn('refs/tags/v1.2.3', remote_tags)
        self.assertNotIn('unrelated-local-tag', remote_tags)
        self.assertEqual(release.git('rev-parse', 'v1.2.3^{}'), self.anchor)
        self.assertEqual((self.root / 'local-notes.md').read_text(), 'Keep these notes local.\n')

    def test_tracked_changes_during_ci_block_publication(self):
        def finish_ci(*args):
            (self.root / 'CMakeLists.txt').write_text('changed while waiting\n')
            return self.anchor
        with patch.object(release, 'wait_for_ci', side_effect=finish_ci):
            with self.assertRaisesRegex(release.ReleaseError, 'Local checkout changed'):
                release.publish('owner/repo', self.anchor, '1.2.3', 10)
        self.assertEqual(release.git('tag', '--list'), '')
        self.assertEqual(release.git('ls-remote', '--tags', 'origin'), '')

    def test_failed_ci_never_creates_tag(self):
        with patch.object(release, 'wait_for_ci', side_effect=release.ReleaseError('CI failure')):
            with self.assertRaisesRegex(release.ReleaseError, 'CI failure'):
                release.publish('owner/repo', self.anchor, '1.2.3', 10)
        self.assertEqual(release.git('tag', '--list'), '')

    def test_main_moves_before_tagging(self):
        with patch.object(release, 'wait_for_ci', return_value=self.anchor), \
             patch.object(release, 'fetch_main', return_value='new-commit'):
            with self.assertRaisesRegex(release.ReleaseError, 'main moved'):
                release.publish('owner/repo', self.anchor, '1.2.3', 10)
        self.assertEqual(release.git('tag', '--list'), '')

    def test_concurrent_main_push_rejects_atomic_tag_push(self):
        real_command = release.command
        moved = release.command('git', 'commit-tree', 'HEAD^{tree}', '-p', self.anchor,
                                input='Concurrent change\n')
        def command(*args, **kwargs):
            if args[:3] == ('git', 'push', '--atomic'):
                real_command('git', 'push', '-q', 'origin', f'{moved}:refs/heads/concurrent')
                real_command('git', '--git-dir', str(self.remote), 'update-ref',
                             'refs/heads/main', moved)
            return real_command(*args, **kwargs)
        with patch.object(release, 'wait_for_ci', return_value=self.anchor), \
             patch.object(release, 'command', side_effect=command):
            with self.assertRaisesRegex(release.ReleaseError, 'Push failed'):
                release.publish('owner/repo', self.anchor, '1.2.3', 10)
        self.assertEqual(release.git('ls-remote', '--tags', 'origin'), '')
        self.assertIn(moved, release.git('ls-remote', 'origin', 'refs/heads/main'))

    def test_existing_unprefixed_version_blocks_duplicate_release(self):
        release.git('tag', '1.2.3')
        with self.assertRaisesRegex(release.ReleaseError, 'already exists'):
            release.check_tag_absent('1.2.3')


if __name__ == '__main__':
    unittest.main()
