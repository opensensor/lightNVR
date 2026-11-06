"""Cancellation and failure handling must not leak owned qualification resources."""
from contextlib import ExitStack, contextmanager
import importlib.util
import json
from pathlib import Path
import tempfile
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
