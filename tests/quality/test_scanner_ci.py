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

    def test_trivy_json_version_requires_and_records_actual_database_metadata(self):
        spec=self.spec()
        spec['version_format']='trivy-json'
        metadata={'Version':'1.2.3','VulnerabilityDB':{'Version':2,'UpdatedAt':'2026-10-31T00:00:00Z'}}
        spec['version_command']=[sys.executable,'-c',f'import json; print(json.dumps({metadata!r}))']
        entry=self.run_scan(spec)
        self.assertEqual(entry.get('db_version'),metadata['VulnerabilityDB'])
        self.assertEqual(entry['tool_version'],'1.2.3')

    def test_cppcheck_scope_uses_actual_progress_not_candidate_list(self):
        spec=self.spec()
        spec['scanner_id']='cppcheck'
        spec['commands']=[{'argv':[sys.executable,'-c',
            'import pathlib; print("Checking web/js/example.js ..."); pathlib.Path("reports/cppcheck.xml").write_text("<results/>")'],
            'reports':['cppcheck.xml']}]
        spec['scope']={'kind':'cppcheck-progress','report':'logs/cppcheck-0.stdout.log'}
        entry=self.run_scan(spec)
        self.assertEqual(entry['status'],'success',entry)
        self.assertEqual(entry['scope']['files'],['web/js/example.js'])
        self.assertIn('logs/cppcheck-0.stdout.log',entry['scope']['evidence_sha256'])

    def test_merged_diagnostic_stream_preserves_stdout_and_stderr(self):
        spec=self.spec()
        spec['commands'][0]['argv']=[sys.executable,'-c','import sys; print("out"); print("err",file=sys.stderr)']
        spec['commands'][0]['stderr_to_stdout']=True
        spec['scope']={'kind':'invocation-files','files':['web/js/example.js']}
        spec['commands'][0]['append_scope_files']=True
        entry=self.run_scan(spec)
        self.assertEqual(entry['status'],'success',entry)
        self.assertEqual(set((self.reports/'eslint.json').read_text().splitlines()),{'out','err'})

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


class FaacCompiledProvenanceTests(unittest.TestCase):
    """Synthetic scanner output tests validation, not real scanner qualification."""
    def setUp(self):
        self.tmp = tempfile.TemporaryDirectory(prefix='faac-scan-')
        self.addCleanup(self.tmp.cleanup)
        self.root = Path(self.tmp.name)
        self.reports = self.root / 'reports'
        self.build = self.root / 'build/native'
        self.copy = self.build / '_deps/lightnvr-faac'
        self.patchdir = self.root / 'patches/faac'
        self.patchdir.mkdir(parents=True)
        self.patch = self.patchdir / '0001-bound-input-pointer-arithmetic.patch'
        self.patch.write_text('synthetic fixture patch\n')
        self.source = self.root / 'third_party/faac'
        names = ['libfaac/frame.c', 'libfaac/faac.c', 'libfaac/faac_internal.h']
        inputs = {}; outputs = {}; rows = []
        for name in names:
            original = self.source / name; original.parent.mkdir(parents=True, exist_ok=True)
            original.write_text('original ' + name + '\n')
            copied = self.copy / name; copied.parent.mkdir(parents=True, exist_ok=True)
            copied.write_text('patched ' + name + '\n')
            inputs[name] = scanner_run.sha(original); outputs[name] = scanner_run.sha(copied)
            rows.append({'upstream_path':name, 'input_sha256':inputs[name],
                         'output_sha256':outputs[name], 'compiled_path':str(copied)})
        (self.copy / 'include').mkdir()
        self.units = sorted(str(self.copy / p) for p in names if p.endswith('.c'))
        subprocess.run(['git','-C',str(self.source),'init','-q'],check=True)
        subprocess.run(['git','-C',str(self.source),'add','.'],check=True)
        subprocess.run(['git','-C',str(self.source),'-c','user.name=Fixture',
                        '-c','user.email=fixture@example.invalid','commit','-qm','fixture'],check=True)
        self.pin=subprocess.check_output(['git','-C',str(self.source),'rev-parse','HEAD'],text=True).strip()
        subprocess.run(['git','-C',str(self.root),'init','-q'],check=True)
        subprocess.run(['git','-C',str(self.root),'update-index','--add','--cacheinfo',
                        '160000,'+self.pin+',third_party/faac'],check=True)
        self.static = {'schema_version':1, 'upstream_commit':self.pin,
                       'patch_sha256':scanner_run.sha(self.patch), 'input_files':inputs,
                       'output_files':outputs, 'translation_units':sorted(p for p in names if p.endswith('.c'))}
        (self.patchdir / 'manifest.json').write_text(json.dumps(self.static))
        (self.patchdir / 'manifest.cmake').write_text('# fixture table\n')
        self.prov = {'schema_version':1, 'upstream_commit':self.pin,
                     'patch_sha256':self.static['patch_sha256'],
                     'manifest_sha256':scanner_run.sha(self.patchdir / 'manifest.json'),
                     'source_root':str(self.source), 'build_root':str(self.copy),
                     'compiler':'synthetic cc', 'build_profile':'test',
                     'files':rows, 'translation_units':self.units}
        self.provpath = self.copy / 'provenance.json'; self.save()
        self.commands = [{'directory':str(self.build), 'file':p,
                          'arguments':['cc','-I'+str(self.copy/'include'),'-I'+str(self.copy/'libfaac'),'-c',p]}
                         for p in self.units]
        self.compdb = self.build / 'compile_commands.json'; self.save_commands()

    def save(self): self.provpath.write_text(json.dumps(self.prov))
    def save_commands(self): self.compdb.write_text(json.dumps(self.commands))
    def rel(self, p): return Path(p).relative_to(self.root).as_posix()
    def spec(self, kind='invocation-files', files=None):
        files = files if files is not None else [self.rel(p) for p in self.units]
        sid = 'codeql-cpp' if kind == 'codeql-source-archive' else 'clang-tidy'
        scope = {'kind':kind, 'files':files}
        command = {'argv':[sys.executable,'-c','print("[]")'],
                   'reports':['scan.json'], 'stdout_report':'scan.json'}
        if kind == 'invocation-files': command['append_scope_files'] = True
        else:
            archive = self.build / 'src.zip'
            with zipfile.ZipFile(archive,'w') as z:
                for f in files: z.write(self.root / f, f)
            scope = {'kind':kind, 'archive':self.rel(archive)}
        return {'scanner_id':sid, 'version_command':[sys.executable,'-c','print("synthetic scanner")'],
                'commands':[command], 'scope':scope,
                'faac':{'provenance':self.rel(self.provpath),'compile_commands':self.rel(self.compdb)}}
    def run_scan(self, spec=None):
        return scanner_run.run_scanner(spec or self.spec(), root=self.root,
                                      reports=self.reports, revision=REVISION)
    def fails(self, spec=None):
        result = self.run_scan(spec); self.assertEqual(result['status'],'failure',result)
        self.assertTrue(result['errors'])

    def test_valid_mapping_retains_actual_paths_and_archives_inputs(self):
        result=self.run_scan(); self.assertEqual(result['status'],'success',result)
        mapping=result['scope']['faac_provenance']
        self.assertEqual(mapping['upstream_commit'],self.pin)
        self.assertEqual(result['scope']['files'],[self.rel(p) for p in self.units])
        self.assertEqual(len(mapping['files']),3)
        for path,digest in mapping['evidence_sha256'].items():
            self.assertEqual(scanner_run.sha(self.reports/path),digest)

    def test_missing_required_faac_spec_cannot_skip_validation(self):
        spec=self.spec(); del spec['faac']; self.fails(spec)
    def test_missing_provenance_refused(self):
        self.provpath.unlink(); self.fails()
    def test_changed_generated_source_refused(self):
        Path(self.units[0]).write_text('tampered\n'); self.fails()
    def test_changed_original_source_refused(self):
        (self.source/'libfaac/frame.c').write_text('tampered\n'); self.fails()
    def test_stale_pin_refused(self):
        self.prov['upstream_commit']='d'*40; self.save(); self.fails()
    def test_changed_patch_refused(self):
        self.patch.write_text('another patch\n'); self.fails()
    def test_mixed_original_compile_root_refused(self):
        self.commands[0]['file']=str(self.source/'libfaac/faac.c'); self.save_commands(); self.fails()
    def test_original_private_include_root_refused(self):
        self.commands[0]['arguments'].insert(1,'-I'+str(self.source/'libfaac'))
        self.save_commands(); self.fails()
    def test_unlisted_translation_unit_refused(self):
        self.prov['translation_units']=self.units[:1]; self.save(); self.fails()
    def test_omitted_actual_invocation_cannot_pass(self):
        self.fails(self.spec(files=[self.rel(self.units[0])]))
    def test_codeql_requires_generated_private_header_and_units(self):
        self.fails(self.spec('codeql-source-archive'))
    def test_codeql_archive_preserves_generated_paths(self):
        names=[self.rel(self.copy/p) for p in self.static['input_files']]
        result=self.run_scan(self.spec('codeql-source-archive',names))
        self.assertEqual(result['status'],'success',result)
        self.assertEqual(set(result['scope']['files']),set(names))
        self.assertEqual(len(result['scope']['faac_provenance']['files']),3)
    def test_codeql_changed_archive_source_fails(self):
        names=[self.rel(self.copy/p) for p in self.static['input_files']]
        spec=self.spec('codeql-source-archive',names)
        with zipfile.ZipFile(self.build/'src.zip','w') as z:
            for name in names:z.writestr(name,b'not compiled source')
        self.fails(spec)
    def test_actual_cppcheck_progress_must_include_all_units(self):
        spec=self.spec(); spec['scanner_id']='cppcheck'
        spec['scope']={'kind':'cppcheck-progress','report':'logs/cppcheck-0.stdout.log'}
        spec['commands']=[{'argv':[sys.executable,'-c',
            'import pathlib; print('+repr('Checking '+self.rel(self.units[0])+' ...')+'); pathlib.Path("reports/scan.json").write_text("[]")'],
            'reports':['scan.json']}]
        self.fails(spec)
    def test_native_selection_preserves_prior_sources_and_adds_compiled_faac(self):
        for path in ['src/example.c','src/sod/excluded.c','external/tool/src/example.c']:
            file=self.root/path;file.parent.mkdir(parents=True,exist_ok=True);file.write_text('int example;')
            self.commands.append({'directory':str(self.root),'file':str(file),'arguments':['cc','-c',str(file)]})
        self.save_commands()
        options={'provenance':self.rel(self.provpath),'compile_commands':self.rel(self.compdb)}
        clang=scanner_run.native_analysis_inputs(self.root,options,'clang-tidy')
        cpp=scanner_run.native_analysis_inputs(self.root,options,'cppcheck')
        self.assertEqual(set(clang),{'src/example.c'}|{self.rel(x) for x in self.units})
        self.assertEqual(set(cpp),{'src/example.c','src/sod/excluded.c','external/tool/src/example.c'}|{self.rel(x) for x in self.units})

    def test_actual_gitlink_drift_refused(self):
        subprocess.run(['git','-C',str(self.root),'update-index','--add','--cacheinfo',
                        '160000,'+'f'*40+',third_party/faac'],check=True)
        self.fails()

    def test_filtered_compile_database_cannot_invent_flags(self):
        filtered=self.build/'analysis.json'
        changed=json.loads(json.dumps(self.commands));changed[0]['arguments'].append('-DCHANGED')
        filtered.write_text(json.dumps(changed))
        spec=self.spec();spec['faac']['analysis_compile_commands']=self.rel(filtered)
        self.fails(spec)
