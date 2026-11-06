"""Qualification preparation, cancellation and failure handling regressions."""
from contextlib import ExitStack, contextmanager, redirect_stderr, redirect_stdout
import importlib.util
import io
import json
from pathlib import Path
import subprocess
import tempfile
from types import SimpleNamespace
import unittest
from unittest.mock import Mock, patch


def load(name):
    spec = importlib.util.spec_from_file_location(name, Path(__file__).with_name(name + '.py'))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


RECOVERY = load('interrupted-recording')
LIFECYCLE = load('lifecycle')
UPGRADE = load('upgrade')
PREPARE = load('prepare-upgrade')


class UpgradePreparationAncestryTest(unittest.TestCase):
    def test_reused_checkout_need_not_contain_execution_head(self):
        self.check_ancestry(unrelated=False)

    def test_unrelated_execution_head_fails_without_success_manifest(self):
        self.check_ancestry(unrelated=True)

    def check_ancestry(self, unrelated):
        # Exercise prepare() with real independent repositories. Mock only build
        # provenance/tool probing: no native or Go compilation is needed here.
        with tempfile.TemporaryDirectory() as temp, ExitStack() as stack:
            directory = Path(temp)
            current = directory / 'current'
            prior = directory / 'prior'
            PREPARE.run(['git', 'init', current])
            PREPARE.git(current, 'config', 'user.name', 'Fixture')
            PREPARE.git(current, 'config', 'user.email', 'fixture@example.invalid')
            PREPARE.git(current, '-c', 'commit.gpgsign=false', 'commit', '--allow-empty', '-m', 'prior')
            revision = PREPARE.git(current, 'rev-parse', 'HEAD')
            PREPARE.run(['git', 'clone', '--no-local', current, prior])
            if unrelated:
                PREPARE.git(current, 'checkout', '--orphan', 'unrelated')
            PREPARE.git(current, '-c', 'commit.gpgsign=false', 'commit', '--allow-empty', '-m', 'execution')
            head = PREPARE.git(current, 'rev-parse', 'HEAD')
            with self.assertRaises(subprocess.CalledProcessError):
                PREPARE.git(prior, 'cat-file', '-e', head)

            binary = directory / 'lightnvr'
            go = directory / 'go2rtc'
            binary.write_bytes(b'fixture native binary')
            go.write_bytes(b'fixture go binary')
            current_manifest = directory / 'current.json'
            PREPARE.write_json(current_manifest, {
                'binary_sha256': PREPARE.sha(binary), 'go2rtc_sha256': PREPARE.sha(go),
                'build_flags': {'ENABLE_GO2RTC': 'ON'}})
            reuse_manifest = directory / 'reuse.json'
            PREPARE.write_json(reuse_manifest, {'prior': {
                key: [] for key in ('configure_command', 'build_command', 'go_build_command')}})
            args = SimpleNamespace(output=directory / 'output', current_binary=binary,
                current_go2rtc=go, current_build_manifest=current_manifest,
                reuse_checkout=prior, reuse_manifest=reuse_manifest, current_last_migration='0093')
            real_run = PREPARE.run
            stack.enter_context(patch.object(PREPARE, 'ROOT', current))
            stack.enter_context(patch.object(PREPARE, 'REVISION', revision))
            stack.enter_context(patch.object(PREPARE, 'verify_old_manifest', return_value=(binary, go)))
            stack.enter_context(patch.object(PREPARE, 'source_manifest', return_value={'source_sha256': 'fixture'}))
            stack.enter_context(patch.object(PREPARE, 'cache_flags', return_value={'ENABLE_GO2RTC': 'ON'}))
            stack.enter_context(patch.object(PREPARE, 'run', side_effect=lambda command, **kw:
                real_run(command, **kw) if command[0] == 'git' else 'fixture tool version'))
            stack.enter_context(redirect_stdout(io.StringIO()))
            stack.enter_context(redirect_stderr(io.StringIO()))
            self.assertEqual(PREPARE.prepare(args), 1 if unrelated else 0)
            status = json.loads((args.output / 'preparation-status.json').read_text())
            self.assertEqual(status['status'], 'failed' if unrelated else 'passed')
            manifest = args.output / 'upgrade-manifest.json'
            if unrelated:
                self.assertFalse(manifest.exists())
                self.assertEqual(status['failure_kind'], 'CalledProcessError')
            else:
                evidence = json.loads(manifest.read_text())['prior']
                self.assertEqual(evidence['revision'], revision)
                self.assertEqual(evidence['execution_head'], head)
                self.assertTrue(evidence['ancestor_of_execution_head'])


class QualificationCleanupTest(unittest.TestCase):
    @contextmanager
    def scenario(self, module, directory, failure, cleanup_failure=None):
        """Fail at startup, after resource ownership, without launching a process."""
        recorder = Mock()
        recorder.start.side_effect = failure
        recorder.stop.side_effect = cleanup_failure
        recorder.reservations = [Mock(), Mock()]
        source = Mock()
        fixture = getattr(module, 'fixture', module)
        evidence = {'binary_sha256': 'digest', 'go2rtc_sha256': 'digest'}
        with ExitStack() as stack:
            stack.enter_context(patch.object(fixture, 'sha256', return_value='digest'))
            stack.enter_context(patch.object(fixture, 'source_evidence', return_value=evidence))
            stack.enter_context(patch.object(fixture, 'SyntheticSource', return_value=source))
            if module is UPGRADE:
                manifest_path = directory / 'build.json'
                manifest_path.write_text(json.dumps(evidence))
                manifest = {'prior': {'checkout': '.', 'binary': 'prior', 'go2rtc': 'go',
                    'revision': 'revision', 'binary_sha256': 'digest',
                    'go2rtc_sha256': 'digest', 'submodules': {}},
                    'current': {'go2rtc': 'go', 'build_manifest': str(manifest_path)}}
                stack.enter_context(patch.object(module, 'git', side_effect=['revision', '']))
                stack.enter_context(patch.object(module, 'UpgradeRecorder', return_value=recorder))
                run = lambda: module.qualify(Path('current'), manifest, directory)
            else:
                stack.enter_context(patch.object(fixture, 'Recorder', return_value=recorder))
                if module is LIFECYCLE:
                    stack.enter_context(patch.object(module, 'go_source_evidence', return_value={}))
                    run = lambda: module.qualify(Path('current'), Path('go'), directory, 3)
                else:
                    run = lambda: module.qualify(Path('current'), Path('go'), directory)
            yield run, recorder, source

    def test_operational_failures_are_reported_and_resources_are_stopped(self):
        for module in (RECOVERY, LIFECYCLE, UPGRADE):
            with self.subTest(scenario=module.__name__), tempfile.TemporaryDirectory() as temp:
                directory = Path(temp)
                with self.scenario(module, directory, RuntimeError('startup failed')) as (run, recorder, source):
                    result = run()
                    self.assertEqual(result['status'], 'failed')
                    self.assertEqual(result['error'], 'startup failed')
                    recorder.stop.assert_called_once()
                    source.stop.assert_called_once()
                    self.assertEqual(json.loads((directory / 'result.json').read_text()), result)

    def test_interrupts_propagate_after_cleanup_and_result_write(self):
        for module in (RECOVERY, LIFECYCLE, UPGRADE):
            for interrupt in (KeyboardInterrupt, SystemExit):
                with self.subTest(scenario=module.__name__, interrupt=interrupt.__name__), tempfile.TemporaryDirectory() as temp:
                    directory = Path(temp)
                    with self.scenario(module, directory, interrupt('cancelled')) as (run, recorder, source):
                        with self.assertRaises(interrupt):
                            run()
                        recorder.stop.assert_called_once()
                        source.stop.assert_called_once()
                        self.assertEqual(json.loads((directory / 'result.json').read_text())['status'], 'failed')

    def test_cleanup_interrupts_do_not_skip_remaining_cleanup_or_report(self):
        for module in (RECOVERY, LIFECYCLE, UPGRADE):
            for interrupt in (KeyboardInterrupt, SystemExit):
                with self.subTest(scenario=module.__name__, interrupt=interrupt.__name__), tempfile.TemporaryDirectory() as temp:
                    directory = Path(temp)
                    with self.scenario(module, directory, RuntimeError('startup failed'), interrupt('cancelled')) as (run, recorder, source):
                        with self.assertRaises(interrupt):
                            run()
                        source.stop.assert_called_once()
                        if module is UPGRADE:
                            for reservation in recorder.reservations:
                                reservation.close.assert_called_once()
                        self.assertEqual(json.loads((directory / 'result.json').read_text())['status'], 'failed')

    def test_operational_cleanup_error_is_reported_without_skipping_source(self):
        for module in (RECOVERY, LIFECYCLE, UPGRADE):
            with self.subTest(scenario=module.__name__), tempfile.TemporaryDirectory() as temp:
                directory = Path(temp)
                with self.scenario(module, directory, RuntimeError('startup failed'), OSError('stop failed')) as (run, recorder, source):
                    result = run()
                    self.assertEqual(result['status'], 'failed')
                    key = 'prior_cleanup_error' if module is UPGRADE else 'cleanup_error'
                    self.assertEqual(result[key], 'stop failed')
                    source.stop.assert_called_once()

    def test_stopping_an_unstarted_recorder_returns_zero_seconds(self):
        recorder = RECOVERY.Recorder.__new__(RECOVERY.Recorder)
        recorder.process = None
        self.assertEqual(recorder.stop(), 0.0)

    def test_cleanup_failure_revokes_a_pass_and_attempts_final_preservation_check(self):
        for error in (OSError('stop failed'), KeyboardInterrupt('cancelled'), SystemExit(2)):
            with self.subTest(error=type(error).__name__), tempfile.TemporaryDirectory() as temp:
                directory = Path(temp)
                report = {'status': 'passed'}
                source_stop = Mock()
                preservation_check = Mock()
                cleanup = [('cleanup_error', Mock(side_effect=error)),
                           ('source_cleanup_error', source_stop),
                           ('original_preservation_error', preservation_check)]
                if isinstance(error, Exception):
                    RECOVERY.finalize_report(report, directory, cleanup)
                    self.assertEqual(report['cleanup_error'], 'stop failed')
                else:
                    with self.assertRaises(type(error)):
                        RECOVERY.finalize_report(report, directory, cleanup)
                source_stop.assert_called_once()
                preservation_check.assert_called_once()
                self.assertEqual(report['status'], 'failed')
                self.assertEqual(json.loads((directory / 'result.json').read_text()), report)


if __name__ == '__main__':
    unittest.main()
