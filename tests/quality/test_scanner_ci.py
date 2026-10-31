# SPDX-License-Identifier: GPL-3.0-or-later
"""Real subprocess provenance and explicitly partial findings-lane checks."""
import hashlib
import json
from pathlib import Path
import shutil
import subprocess
import sys
import tempfile
import unittest
import zipfile

REPO = Path(__file__).resolve().parents[2]
sys.path.insert(0, str(REPO / 'scripts/quality'))
import scanner_run
import findings_lane
import findings

REVISION = 'a' * 40

class ScannerPipelineTests(unittest.TestCase):
    def setUp(self):
        self.root = Path(tempfile.mkdtemp(prefix='scanner-ci-'))
        self.reports = self.root / 'reports'
        (self.root / 'web/js').mkdir(parents=True)
        (self.root / 'web/js/example.js').write_text('export const answer = 42;\n')
        self.scanners = {'scanners': [
            {'id':'eslint-web','tool':'eslint','report_format':'eslint-json','access_status':'accessible','report_path_convention':'eslint.json'},
            {'id':'codeql-cpp','tool':'CodeQL','report_format':'sarif-2.1.0','access_status':'accessible','report_path_convention':'codeql.sarif'}]}
        self.inventory = {'findings':[],'buckets':[]}
        self.policy = {'dispositions':{'needs-triage':{'exception':False}},'accepted_gaps':[],'accepted_exit_codes':{'default':[0],'eslint-web':[0,1]},'require_run_manifest':True}

    def tearDown(self):
        shutil.rmtree(self.root)

    def spec(self, messages=None, code=0):
        report=[{'filePath':str(self.root/'web/js/example.js'),'messages':messages or []}]
        return {'scanner_id':'eslint-web','version_command':[sys.executable,'-c','print("eslint 9.38.0")'],
            'commands':[{'argv':[sys.executable,'-c',f'import json,sys; print(json.dumps({report!r})); sys.exit({code})'],
                         'stdout_report':'eslint.json','reports':['eslint.json'],'accepted_exit_codes':[0,1]}],
            'scope':{'kind':'eslint-json','report':'eslint.json'}}

    def run_scan(self, spec=None):
        return scanner_run.run_scanner(spec or self.spec(),root=self.root,reports=self.reports,revision=REVISION)

    def lane(self, expected_revision=REVISION):
        return findings_lane.evaluate_lane(reports=self.reports,inventory=self.inventory,scanners=self.scanners,policy=self.policy,
            scanner_ids=['eslint-web'],expected_revision=expected_revision,today='2026-11-01')

    def manifest(self):
        return json.loads((self.reports/'run-manifest.json').read_text())

    def write_manifest(self, value):
        (self.reports/'run-manifest.json').write_text(json.dumps(value))

    def test_clean_subprocess_records_actual_files_versions_hashes_and_partial_pass(self):
        result=self.run_scan()
        self.assertEqual(result['status'],'success',result)
        entry=self.manifest()['scanners']['eslint-web']
        self.assertEqual(entry['scope']['files'],['web/js/example.js'])
        self.assertEqual(entry['files_scanned'],1)
        self.assertEqual(entry['tool_version'],'eslint 9.38.0')
        self.assertEqual(entry['report_sha256']['eslint.json'],hashlib.sha256((self.reports/'eslint.json').read_bytes()).hexdigest())
        verdict=self.lane()
        self.assertEqual(verdict['status'],'partial-pass',verdict)
        self.assertIsNone(verdict['global_verdict'])
        self.assertEqual(verdict['omitted_scanners'],['codeql-cpp'])

    def test_new_finding_fails_even_when_scanner_exit_is_accepted(self):
        self.run_scan(self.spec([{'ruleId':'jsx-a11y/alt-text','severity':2,'line':1,'column':1,'message':'Image needs alternate text.'}],1))
        verdict=self.lane()
        self.assertEqual(verdict['status'],'partial-fail')
        self.assertTrue(verdict['result']['failures'])

    def test_crashed_command_with_report_cannot_pass(self):
        entry=self.run_scan(self.spec(code=139))
        self.assertEqual(entry['status'],'failure')
        self.assertEqual(entry['exit_code'],139)
        self.assertEqual(self.lane()['status'],'partial-fail')

    def test_timeout_is_recorded_and_cannot_pass(self):
        spec=self.spec();spec['commands'][0]['argv']=[sys.executable,'-c','import time; time.sleep(10)'];spec['commands'][0]['timeout_seconds']=0.05
        entry=self.run_scan(spec)
        self.assertEqual(entry['status'],'failure')
        self.assertTrue(entry['commands'][0]['timed_out'])

    def test_stale_preexisting_report_is_rejected_without_overwrite(self):
        self.reports.mkdir();(self.reports/'eslint.json').write_text('old')
        with self.assertRaises(ValueError):self.run_scan()
        self.assertEqual((self.reports/'eslint.json').read_text(),'old')

    def test_report_hash_tampering_fails(self):
        self.run_scan();(self.reports/'eslint.json').write_text('[]')
        self.assertEqual(self.lane()['status'],'partial-fail')
        self.assertTrue(any('hash' in f['detail'] for f in self.lane()['result']['failures']))

    def test_wrong_revision_and_missing_expectation_fail(self):
        self.run_scan()
        self.assertEqual(self.lane('b'*40)['status'],'partial-fail')
        with self.assertRaises(ValueError):self.lane(None)

    def test_required_manifest_metadata_fields_fail_closed(self):
        self.run_scan();original=self.manifest()
        for field in ['status','exit_code','tool_version','reports','report_sha256','scope','files_scanned']:
            with self.subTest(field=field):
                changed=json.loads(json.dumps(original));del changed['scanners']['eslint-web'][field];self.write_manifest(changed)
                self.assertEqual(self.lane()['status'],'partial-fail')
        self.write_manifest(original)

    def test_candidate_inventory_is_not_scan_evidence(self):
        spec=self.spec();spec['scope']={'kind':'candidate-files','files':['web/js/example.js']}
        entry=self.run_scan(spec)
        self.assertEqual(entry['status'],'failure')
        self.assertEqual(self.lane()['status'],'partial-fail')

    def test_report_paths_and_source_symlinks_cannot_escape_roots(self):
        for name in ['../outside.json','/tmp/outside.json']:
            with self.subTest(name=name):
                spec=self.spec();spec['commands'][0]['reports']=[name]
                with self.assertRaises(ValueError):self.run_scan(spec)
        outside=self.root.parent/(self.root.name+'-outside.js');outside.write_text('private')
        try:
            (self.root/'web/js/example.js').unlink();(self.root/'web/js/example.js').symlink_to(outside)
            entry=self.run_scan();self.assertEqual(entry['status'],'failure')
            self.assertNotIn('private',json.dumps(entry))
        finally:outside.unlink()

    def test_report_symlink_is_never_followed_for_hashing(self):
        self.run_scan();p=self.reports/'eslint.json';p.unlink();p.symlink_to(self.root/'web/js/example.js')
        self.assertEqual(self.lane()['status'],'partial-fail')

    def test_global_gate_still_requires_omitted_scanner(self):
        self.run_scan()
        result=findings.evaluate(self.reports,self.inventory,self.scanners,self.policy,'2026-11-01',expected_revision=REVISION)
        self.assertEqual(result['status'],'fail')
        self.assertTrue(any(f['scanner']=='codeql-cpp' for f in result['failures']))

    def test_unknown_or_empty_lane_cannot_drop_scope(self):
        for ids in [[],['unknown']]:
            with self.assertRaises(ValueError):findings_lane.evaluate_lane(reports=self.reports,inventory=self.inventory,scanners=self.scanners,policy=self.policy,scanner_ids=ids,expected_revision=REVISION,today='2026-11-01')

    def test_proposed_exception_is_not_approved_by_lane_projection(self):
        self.run_scan()
        self.policy['accepted_gaps']=[{'scanner':'eslint-web','condition':'not-scanned','owner':'maintainers','reason':'proposed','expires':'2027-01-01'}]
        self.assertEqual(self.lane()['status'],'partial-fail')

    def test_lane_cli_announces_partial_and_does_not_write_baseline(self):
        self.run_scan()
        for name,data in [('inventory',self.inventory),('scanners',self.scanners),('policy',self.policy)]: (self.root/(name+'.json')).write_text(json.dumps(data))
        before=(self.root/'inventory.json').read_bytes()
        process=subprocess.run([sys.executable,str(REPO/'scripts/quality/findings_lane.py'),'--reports',str(self.reports),'--inventory',str(self.root/'inventory.json'),'--scanners',str(self.root/'scanners.json'),'--policy',str(self.root/'policy.json'),'--scanner','eslint-web','--expected-revision',REVISION],capture_output=True,text=True)
        self.assertEqual(process.returncode,0,process.stdout+process.stderr)
        self.assertIn('PARTIAL',process.stdout)
        self.assertIn('codeql-cpp',process.stdout)
        self.assertNotIn('RESULT: PASS',process.stdout)
        self.assertEqual((self.root/'inventory.json').read_bytes(),before)

    def test_missing_tool_version_cannot_pass(self):
        spec=self.spec();spec['version_command']=[sys.executable,'-c','pass']
        entry=self.run_scan(spec)
        self.assertEqual(entry['status'],'failure')
        self.assertEqual(self.lane()['status'],'partial-fail')

    def test_codeql_scope_comes_from_actual_archive_and_matches_checkout(self):
        archive=self.root/'src.zip'
        with zipfile.ZipFile(archive,'w') as zipped:
            zipped.write(self.root/'web/js/example.js',str(self.root/'web/js/example.js').lstrip('/'))
            zipped.writestr('usr/include/external.h','external')
        spec=self.spec();spec['scope']={'kind':'codeql-source-archive','archive':'src.zip'}
        entry=self.run_scan(spec)
        self.assertEqual(entry['status'],'success',entry)
        self.assertEqual(entry['scope']['files'],['web/js/example.js'])
        self.assertEqual(entry['scope']['external_entries_omitted'],1)
        (self.root/'web/js/example.js').write_text('changed source')
        with self.assertRaises(ValueError):scanner_run.scope_evidence(spec,self.root,self.reports)

    def test_second_command_failure_is_not_hidden_by_first_success(self):
        spec=self.spec();second=json.loads(json.dumps(spec['commands'][0]))
        second.update(argv=[sys.executable,'-c','print("[]"); raise SystemExit(5)'],stdout_report='second.json',reports=['second.json'])
        spec['commands'].append(second)
        entry=self.run_scan(spec)
        self.assertEqual(entry['status'],'failure')
        self.assertEqual(entry['exit_code'],5)
        self.assertEqual(self.lane()['status'],'partial-fail')

    def test_malformed_scope_hashes_fail_without_checker_crash(self):
        self.run_scan();manifest=self.manifest()
        manifest['scanners']['eslint-web']['scope']['evidence_sha256']=42
        self.write_manifest(manifest)
        self.assertEqual(self.lane()['status'],'partial-fail')

    def test_existing_scanner_manifest_is_preserved(self):
        self.run_scan();before=(self.reports/'run-manifest.json').read_bytes()
        with self.assertRaises(ValueError):self.run_scan()
        self.assertEqual((self.reports/'run-manifest.json').read_bytes(),before)

    def test_undeclared_or_stale_file_list_is_not_tool_evidence(self):
        self.reports.mkdir();(self.reports/'old-scope.json').write_text('{"files":["web/js/example.js"]}')
        spec=self.spec();spec['scope']={'kind':'file-list-report','report':'old-scope.json'}
        with self.assertRaises(ValueError):self.run_scan(spec)
        spec['commands'][0]['evidence_outputs']=['old-scope.json']
        with self.assertRaises(ValueError):self.run_scan(spec)

    def test_policy_pin_cannot_bypass_required_report_hashes(self):
        self.run_scan()
        shutil.copyfile(self.reports/'eslint.json',self.reports/'unhashed.json')
        self.policy['report_pins']=[{'reports_dir':str(self.reports),'scanner':'eslint-web','paths':['unhashed.json']}]
        self.assertEqual(self.lane()['status'],'partial-fail')
