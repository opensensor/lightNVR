"""Independent RED/GREEN contract tests; synthetic limits are not qualification."""
import importlib.util
import json
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest
from unittest.mock import patch

ROOT = Path(__file__).resolve().parents[3]
spec = importlib.util.spec_from_file_location('soak', ROOT / 'scripts/quality/soak.py')
soak = importlib.util.module_from_spec(spec)
spec.loader.exec_module(soak)


def profile():
    return {'repetitions': 2, 'sample_interval_seconds': .1, 'scenario_timeout_seconds': 300,
            'scenarios': ['media-video', 'media-aac', 'lifecycle'], 'growth_limits': {'rss_bytes': None, 'fds': None, 'threads': None}}


def identity(pid=123, parent=100, binary='/bin/lightnvr', args=None):
    return {'pid': pid, 'parent': parent, 'start': '1234', 'binary': binary, 'args': args or ['lightnvr', '-c', '/tmp/owned/config']}


class Contract(unittest.TestCase):
    def test_valid_profile(self):
        self.assertEqual(soak.validate_profile(profile()), profile())

    def test_invalid_profiles_fail_closed(self):
        for field, values in {'repetitions':[0, -1, True, 1000], 'sample_interval_seconds':[0, float('nan'), 100],
                              'scenario_timeout_seconds':[0, 100000], 'scenarios':[[], ['unknown'], ['lifecycle','lifecycle']]}.items():
            for value in values:
                with self.subTest(field=field, value=value):
                    candidate=profile();candidate[field]=value
                    with self.assertRaises(ValueError): soak.validate_profile(candidate)

    def test_negative_or_unknown_growth_limit_rejected(self):
        for limits in [{'rss_bytes':-1}, {'unknown':1}]:
            candidate=profile();candidate['growth_limits']=limits
            with self.assertRaises(ValueError):soak.validate_profile(candidate)

    def test_missing_limit_cannot_pass(self):
        result=soak.qualify([{'id':'nvr.threads','limit':None}], {'nvr.threads': 2})
        self.assertFalse(result['passed']);self.assertEqual(result['metrics'][0]['status'],'unqualified')

    def test_missing_metric_cannot_pass(self):
        result=soak.qualify([{'id':'nvr.threads','limit':10}], {})
        self.assertFalse(result['passed']);self.assertEqual(result['metrics'][0]['status'],'unavailable')

    def test_metric_failure_and_synthetic_success(self):
        self.assertFalse(soak.qualify([{'id':'nvr.threads','limit':1}], {'nvr.threads':2})['passed'])
        self.assertTrue(soak.qualify([{'id':'nvr.threads','limit':2}], {'nvr.threads':2})['passed'])

    def test_empty_budget_not_green(self):
        self.assertFalse(soak.qualify([], {})['passed'])

    def test_nonfinite_observation_not_green(self):
        self.assertFalse(soak.qualify([{'id':'nvr.threads','limit':2}], {'nvr.threads':float('nan')})['passed'])

    def test_resource_growth_is_failure(self):
        samples=[{'nvr': {'rss_bytes':n,'fds':4,'threads':2}} for n in [100,200,300]]
        result=soak.resource_summary(samples, {'rss_bytes':50,'fds':0,'threads':0})
        self.assertFalse(result['passed']);self.assertIn('rss_bytes',str(result['failures']))

    def test_missing_and_null_growth_cannot_qualify(self):
        self.assertFalse(soak.resource_summary([], {'rss_bytes':1})['passed'])
        self.assertFalse(soak.resource_summary([{'nvr':{'rss_bytes':2}}], {'rss_bytes':None})['passed'])

    def test_source_and_browser_classification_separate(self):
        self.assertEqual(soak.role(identity(binary='/nvr'), None, '/nvr'),'nvr')
        self.assertEqual(soak.role(identity(binary='/go2rtc'), 'nvr', '/nvr'),'managed')
        self.assertEqual(soak.role(identity(binary='/ffmpeg',args=['ffmpeg','-f','lavfi','-i','testsrc']), 'managed', '/nvr'),'source')
        self.assertEqual(soak.role(identity(binary='/go2rtc'), None, '/nvr'),'source')
        self.assertEqual(soak.role(identity(binary='/chrome'), None, '/nvr'),'browser')
        self.assertEqual(soak.role(identity(binary='/child'), 'source', '/nvr'),'source')

    def test_pid_reuse_before_read_refused(self):
        saved=identity();current={**saved,'start':'other'}
        with patch.object(soak,'read_identity',return_value=current):
            self.assertEqual(soak.sample_process(saved)['status'],'identity_changed')

    def test_pid_reuse_during_read_refused(self):
        saved=identity();current={**saved,'binary':'/unrelated'}
        with patch.object(soak,'read_identity',side_effect=[saved,current]), patch.object(soak,'read_metrics',return_value={'rss_bytes':10}):
            self.assertEqual(soak.sample_process(saved)['status'],'identity_changed')

    def test_missing_process_is_unavailable(self):
        with patch.object(soak,'read_identity',return_value=None):
            self.assertEqual(soak.sample_process(identity())['status'],'exited')

    def test_live_owned_process_can_be_sampled(self):
        child=subprocess.Popen([sys.executable,'-c','import time;time.sleep(3)'])
        try:
            saved=soak.read_identity(child.pid)
            self.assertEqual(soak.sample_process(saved)['status'],'observed')
        finally:child.terminate();child.wait()

    def test_empty_lifecycle_report_fails(self):
        self.assertFalse(soak.assess_report('lifecycle',[])['passed'])

    def test_lifecycle_failure_propagates(self):
        checks=[{'name':str(i),'passed':True} for i in range(13)]
        self.assertTrue(soak.assess_report('lifecycle',checks)['passed'])
        checks[3]['passed']=False
        self.assertFalse(soak.assess_report('lifecycle',checks)['passed'])

    def test_recovery_deadline_and_gap_fail(self):
        base={'failures':[], 'cleanup':[{'forced':False,'returncode':0}], 'recovery_seconds':2,
              'limits':{'recovery_deadline_seconds':60,'continuous_gap_tolerance_seconds':1.1,'consecutive_segments':3},
              'before':[{'row':{'start_time':n*3,'end_time':n*3+3}} for n in range(3)],
              'after':[{'row':{'start_time':20+n*3,'end_time':23+n*3}} for n in range(3)]}
        self.assertTrue(soak.assess_report('media-video',{'results':[base]})['passed'])
        base['recovery_seconds']=61
        self.assertFalse(soak.assess_report('media-video',{'results':[base]})['passed'])
        base['recovery_seconds']=2;base['after'][1]['row']['start_time']=40
        self.assertFalse(soak.assess_report('media-video',{'results':[base]})['passed'])

    def test_sanitizer_detection(self):
        for text in ['ERROR: AddressSanitizer: heap-use-after-free','runtime error: signed integer overflow','WARNING: ThreadSanitizer: data race','LeakSanitizer: detected memory leaks']:
            self.assertTrue(soak.sanitizer_findings(text))
        self.assertFalse(soak.sanitizer_findings('normal log'))

    def test_child_nonzero_cannot_pass(self):
        with tempfile.TemporaryDirectory() as directory:
            outcome=soak.run_child([sys.executable,'-c','raise SystemExit(7)'],Path(directory),'/nonexistent/nvr',.05,3)
            self.assertEqual(outcome['returncode'],7);self.assertFalse(outcome['passed'])

    def test_child_timeout_cannot_pass(self):
        with tempfile.TemporaryDirectory() as directory:
            outcome=soak.run_child([sys.executable,'-c','import time;time.sleep(5)'],Path(directory),'/nonexistent/nvr',.05,.2)
            self.assertTrue(outcome['timed_out']);self.assertFalse(outcome['passed'])

    def test_green_exit_without_observation_cannot_pass(self):
        with tempfile.TemporaryDirectory() as directory:
            outcome=soak.run_child([sys.executable,'-c','pass'],Path(directory),'/nonexistent/nvr',.05,3)
            self.assertFalse(outcome['passed']);self.assertIn('No NVR',str(outcome['failures']))

    def test_existing_lock_is_preserved(self):
        with tempfile.TemporaryDirectory() as directory:
            lock=Path(directory)/'lock';lock.mkdir();(lock/'owner').write_text('another owner')
            with self.assertRaises(RuntimeError):soak.ensure_runtime_available(lock)
            self.assertEqual((lock/'owner').read_text(),'another owner')

    def test_supervisor_never_nests_child_lock(self):
        with tempfile.TemporaryDirectory() as directory:
            lock=Path(directory)/'child-lock'
            code="from pathlib import Path;p=Path("+repr(str(lock))+");p.mkdir();(p/'owner').write_text('child');(p/'owner').unlink();p.rmdir()"
            outcome=soak.run_child([sys.executable,'-c',code],Path(directory)/'out','/nonexistent/nvr',.05,3)
            self.assertEqual(outcome['returncode'],0);self.assertFalse(lock.exists())

    def test_abandoned_child_lock_is_preserved(self):
        with tempfile.TemporaryDirectory() as directory:
            lock=Path(directory)/'child-lock'
            code="from pathlib import Path;p=Path("+repr(str(lock))+");p.mkdir();(p/'owner').write_text('child');raise SystemExit(9)"
            outcome=soak.run_child([sys.executable,'-c',code],Path(directory)/'out','/nonexistent/nvr',.05,3)
            self.assertEqual(outcome['returncode'],9)
            with self.assertRaises(RuntimeError):soak.ensure_runtime_available(lock)
            self.assertEqual((lock/'owner').read_text(),'child')

    def test_descendants_cannot_adopt_same_name_unrelated_pid(self):
        root=identity(pid=100,parent=1);child=identity(pid=101,parent=100);unrelated=identity(pid=102,parent=1)
        with patch.object(soak,'process_table',return_value={100:root,101:child,102:unrelated}):
            self.assertEqual(set(soak.owned_snapshot(root)),{100,101})

if __name__=='__main__':unittest.main()
