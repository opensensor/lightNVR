"""Ownership and fail-closed tests for the engineering test runners."""
import importlib.util
import json
import os
from pathlib import Path
import subprocess
import sys
import tempfile
import unittest

ROOT = Path(__file__).resolve().parents[2]
SPEC = importlib.util.spec_from_file_location('native_runner', ROOT / 'scripts/quality/run-native.py')
native = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(native)


class NativeRunnerTests(unittest.TestCase):
    def test_task_id_rejects_path_escape(self):
        for value in ('../other', '', '/tmp/task', 'T1/../../other', 'T1;touch x'):
            with self.subTest(value=value), self.assertRaises(ValueError):
                native.validate_task(value)
        self.assertEqual(native.validate_task('T31.3'), 'T31.3')

    def test_empty_selection_fails(self):
        with self.assertRaises(ValueError):
            native.test_names({'tests': []})

    def test_invalid_selection_fails(self):
        for payload in ({}, {'tests': [{'name': '../escape'}]}, {'tests': [{'name': ''}]}, {'tests': 'bad'}):
            with self.subTest(payload=payload), self.assertRaises(ValueError):
                native.test_names(payload)

    def test_selected_names_preserved(self):
        self.assertEqual(native.test_names({'tests': [{'name': 'test_system_health_proc'}]}), ['test_system_health_proc'])

    def test_lock_serializes_distinct_runs(self):
        with tempfile.TemporaryDirectory() as directory:
            lock = Path(directory) / 'lock'
            with native.runtime_lock(lock):
                with self.assertRaises(FileExistsError):
                    with native.runtime_lock(lock):
                        self.fail('second lock acquired')
            self.assertFalse(lock.exists())

    def test_lock_never_removes_replaced_owner(self):
        with tempfile.TemporaryDirectory() as directory:
            lock = Path(directory) / 'lock'
            with self.assertRaises(RuntimeError):
                with native.runtime_lock(lock):
                    (lock / 'owner').write_text('other-owner')
            self.assertEqual((lock / 'owner').read_text(), 'other-owner')

    def test_subprocess_failure_propagates(self):
        with tempfile.TemporaryDirectory() as directory:
            with self.assertRaises(subprocess.CalledProcessError) as raised:
                native.run_command([sys.executable, '-c', 'raise SystemExit(7)'], Path(directory) / 'log')
            self.assertEqual(raised.exception.returncode, 7)

    def test_dependency_env_excludes_stale_local_paths(self):
        env = native.build_env({'PKG_CONFIG_PATH': '/usr/local/lib/pkgconfig', 'CFLAGS': '-I/usr/local/include', 'LIGHTNVR_URL': 'https://production.invalid'})
        self.assertEqual(env['PKG_CONFIG_PATH'], '')
        self.assertNotIn('CFLAGS', env)
        self.assertNotIn('LIGHTNVR_URL', env)
        self.assertNotIn('/usr/local', env['PKG_CONFIG_LIBDIR'])


class IntegrationRunnerTests(unittest.TestCase):
    def node(self, code):
        result = subprocess.run(['node', '--input-type=module', '-e', code], cwd=ROOT, text=True, capture_output=True)
        self.assertEqual(result.returncode, 0, result.stdout + result.stderr)

    def test_loopback_rejects_remote_and_credentials(self):
        self.node("""
          import assert from 'node:assert/strict';
          import { validateLoopback } from './scripts/quality/run-integration.mjs';
          for (const url of ['https://production.invalid', 'http://127.0.0.1.evil', 'http://user:pass@localhost:18080', 'file:///tmp/server']) {
            assert.throws(() => validateLoopback(url));
          }
          for (const url of ['http://localhost:18080', 'http://127.0.0.1:18080', 'http://[::1]:18080']) validateLoopback(url);
        """)

    def test_integration_lock_uses_same_exclusive_lease(self):
        with tempfile.TemporaryDirectory() as directory:
            lock = Path(directory) / 'lock'
            with native.runtime_lock(lock):
                self.node(f"""
                  import assert from 'node:assert/strict';
                  import {{ acquireLock }} from './scripts/quality/run-integration.mjs';
                  assert.throws(() => acquireLock({json.dumps(str(lock))}));
                """)

    def test_selection_zero_and_invalid_are_failure(self):
        self.node("""
          import assert from 'node:assert/strict';
          import { selectedCount } from './scripts/quality/run-integration.mjs';
          assert.throws(() => selectedCount({suites: []}));
          assert.throws(() => selectedCount({}));
          assert.equal(selectedCount({suites: [{suites: [], specs: [{tests: [{projectName: 'api'}]}]}]}), 1);
        """)

    def test_ownership_rejects_missing_and_forged_token_without_deleting(self):
        self.node("""
          import assert from 'node:assert/strict';
          import fs from 'node:fs'; import os from 'node:os'; import path from 'node:path';
          import { readRunContext } from './tests/integration/fixtures/run-ownership.ts';
          const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'lightnvr-eng-test-'));
          try {
            fs.writeFileSync(path.join(dir,'sentinel'), 'keep');
            assert.throws(() => readRunContext({LIGHTNVR_TEST_DIR:dir,LIGHTNVR_RUN_TOKEN:'forged'}));
            assert.equal(fs.readFileSync(path.join(dir,'sentinel'),'utf8'),'keep');
          } finally {fs.rmSync(dir,{recursive:true});}
        """)

    def test_generated_config_is_owned_and_loopback(self):
        self.node("""
          import assert from 'node:assert/strict'; import fs from 'node:fs';
          import { createRun } from './scripts/quality/run-integration.mjs';
          import { readRunContext } from './tests/integration/fixtures/run-ownership.ts';
          const run = createRun({task:'T13-test',binary:process.execPath,go2rtc:false});
          try {
            const ctx = readRunContext(run.env);
            const ini = fs.readFileSync(ctx.config,'utf8');
            assert.ok(ini.includes('bind_ip = 127.0.0.1'));
            assert.ok(ini.includes('path = '+ctx.dir+'/lightnvr.db'));
            assert.ok(!ini.includes('/tmp/lightnvr-test/'));
            assert.equal(ctx.dir,run.dir);
            fs.appendFileSync(ctx.config,'\\n# tampered');
            assert.throws(()=>readRunContext(run.env));
          } finally {fs.rmSync(run.dir,{recursive:true});fs.rmSync(run.artifacts,{recursive:true});}
        """)

    def test_pid_provenance_rejects_unrelated_process(self):
        self.node("""
          import assert from 'node:assert/strict'; import fs from 'node:fs';
          import { createRun } from './scripts/quality/run-integration.mjs';
          import { recordOwnedProcess } from './tests/integration/fixtures/run-ownership.ts';
          const run = createRun({task:'T13-pid-test',binary:process.execPath,go2rtc:false});
          try {assert.throws(()=>recordOwnedProcess(process.pid,run.env)); process.kill(process.pid,0);}
          finally {fs.rmSync(run.dir,{recursive:true});fs.rmSync(run.artifacts,{recursive:true});}
        """)

    def test_owned_process_stop_and_pid_reuse_rejection(self):
        self.node("""
          import assert from 'node:assert/strict'; import fs from 'node:fs'; import path from 'node:path';
          import { spawn } from 'node:child_process'; import { once } from 'node:events';
          import { createRun } from './scripts/quality/run-integration.mjs';
          import { recordOwnedProcess, stopOwnedProcess } from './tests/integration/fixtures/run-ownership.ts';
          const run = createRun({task:'T13-own-test',binary:'/usr/bin/python3',go2rtc:false});
          const child = spawn('/usr/bin/python3',['-c','import time; time.sleep(30)','-c',run.config]);
          try {
            await once(child,'spawn'); recordOwnedProcess(child.pid,run.env);
            const filename=path.join(run.dir,'process.json');
            const saved=JSON.parse(fs.readFileSync(filename,'utf8'));
            fs.writeFileSync(filename,JSON.stringify({...saved,start:'wrong-start-time'}));
            await assert.rejects(stopOwnedProcess(run.env),/identity changed/); process.kill(child.pid,0);
            fs.writeFileSync(filename,JSON.stringify(saved));
            const exited=once(child,'exit'); await stopOwnedProcess(run.env); await exited;
            assert.equal(JSON.parse(fs.readFileSync(path.join(run.artifacts,'shutdown.json'),'utf8')).graceful,true);
          } finally {child.kill();fs.rmSync(run.dir,{recursive:true});fs.rmSync(run.artifacts,{recursive:true});}
        """)

    def test_forced_shutdown_is_failure_and_descendants_are_cleaned(self):
        self.node("""
          import assert from 'node:assert/strict'; import fs from 'node:fs'; import path from 'node:path';
          import { spawn } from 'node:child_process'; import { once } from 'node:events';
          import { createRun } from './scripts/quality/run-integration.mjs';
          import { recordOwnedProcess, stopOwnedProcess } from './tests/integration/fixtures/run-ownership.ts';
          const run=createRun({task:'T13-force-test',binary:'/usr/bin/python3',go2rtc:false});
          const child=spawn('/usr/bin/python3',['-c',"import subprocess,time,signal; signal.signal(signal.SIGTERM,signal.SIG_IGN); p=subprocess.Popen(['/usr/bin/sleep','30']); print(p.pid,flush=True); time.sleep(30)",'-c',run.config]);
          try {
            await once(child,'spawn'); await once(child.stdout,'data'); recordOwnedProcess(child.pid,run.env);
            await assert.rejects(stopOwnedProcess(run.env,10),/graceful shutdown deadline/);
            const report=JSON.parse(fs.readFileSync(path.join(run.artifacts,'shutdown.json'),'utf8'));
            assert.equal(report.graceful,false); assert.equal(report.forcedKill,true);
            assert.ok(report.observedChildren.length>0); assert.deepEqual(report.lingeringChildren,[]);
          } finally {child.kill('SIGKILL');fs.rmSync(run.dir,{recursive:true});fs.rmSync(run.artifacts,{recursive:true});}
        """)

    def test_cli_preserves_project_file_shard_and_grep(self):
        self.node("""
          import assert from 'node:assert/strict';
          import { parseOptions, playwrightArgs } from './scripts/quality/run-integration.mjs';
          const options=parseOptions(['--task','T15.1','--project','all','--shard','2/3','--grep','@api|@ui','--file','tests/integration/specs/system-health.api.spec.ts','--file','tests/integration/specs/system-health.ui.spec.ts','--list']);
          const args=playwrightArgs(options);
          assert.equal(options.list,true);assert.equal(options.files.length,2);
          assert.ok(!args.includes('--project'));assert.ok(args.includes('--shard=2/3'));
          assert.equal(args[args.indexOf('--grep')+1],'@api|@ui');
          const filters=args.filter(arg=>arg.startsWith('^'));assert.equal(filters.length,2);
          assert.ok(new RegExp(filters[0]).test(options.files[0]));
          assert.ok(!new RegExp(filters[0]).test(options.files[0].replace('.api.','XapiY')));
          assert.ok(playwrightArgs(parseOptions(['--task=T15.1','--project=ui','--shard=1/2'])).includes('--project=ui'));
        """)

    def test_cli_rejects_config_and_unsafe_file_or_shard(self):
        self.node("""
          import assert from 'node:assert/strict';
          import { parseOptions } from './scripts/quality/run-integration.mjs';
          for (const extra of [['--config','/tmp/production'],['--file','package.json'],['--file','../outside.spec.ts'],['--shard','0/2'],['--shard','3/2'],['--shard','1/0'],['--project','unknown'],['--config-profile','production'],['--workers','8']]) assert.throws(()=>parseOptions(['--task','T15.1',...extra]));
          assert.throws(()=>parseOptions(['--task','T15.1','--config-profile','health']),/binary/);
        """)

    def test_health_profile_keeps_mqtt_and_omits_go2rtc(self):
        self.node("""
          import assert from 'node:assert/strict';import fs from 'node:fs';
          import { parseOptions, createRun } from './scripts/quality/run-integration.mjs';
          const options=parseOptions(['--task','T15-health-test','--config-profile','health','--binary',process.execPath]);
          const run=createRun(options);
          try {
            const ini=fs.readFileSync(run.config,'utf8');
            assert.ok(ini.includes('[mqtt]\\nenabled = true'));
            assert.ok(ini.includes('broker_host = 127.0.0.1'));
            assert.ok(ini.includes('broker_port = 18884'));
            assert.ok(ini.includes('[go2rtc]\\nenabled = false'));
            assert.equal(run.env.LIGHTNVR_SKIP_GO2RTC,'1');
          }finally{fs.rmSync(run.dir,{recursive:true});fs.rmSync(run.artifacts,{recursive:true});}
        """)

    def test_discovery_does_not_need_binary_or_runtime_directory(self):
        self.node("""
          import assert from 'node:assert/strict';import fs from 'node:fs';
          import { main } from './scripts/quality/run-integration.mjs';
          const before=fs.readdirSync('/tmp').filter(n=>n.startsWith('lightnvr-eng-T15-discovery-test-'));
          assert.equal(await main(['--task','T15-discovery-test','--project','api','--grep','GET /api/system','--binary','/no-binary-required-for-discovery','--list']),0);
          assert.deepEqual(fs.readdirSync('/tmp').filter(n=>n.startsWith('lightnvr-eng-T15-discovery-test-')),before);
          fs.rmSync('build/eng-artifacts/T15-discovery-test',{recursive:true});
        """)

    def test_log_archive_ignores_external_symlinks_and_unrelated_data(self):
        self.node("""
          import assert from 'node:assert/strict';import fs from 'node:fs';import path from 'node:path';
          import { createRun, archiveRunLogs } from './scripts/quality/run-integration.mjs';
          const run=createRun({task:'T15-archive-test',binary:process.execPath,go2rtc:false});
          try {
            fs.writeFileSync(path.join(run.dir,'lightnvr.log'),'owned log');
            fs.writeFileSync(path.join(run.dir,'unrelated-secret'),'do not archive');
            fs.symlinkSync('/etc/passwd',path.join(run.dir,'other.log'));
            archiveRunLogs(run);
            const files=fs.readdirSync(path.join(run.artifacts,'runtime'));
            assert.ok(files.includes('lightnvr.log'));assert.ok(files.includes('lightnvr.ini'));
            assert.ok(!files.includes('other.log'));assert.ok(!files.includes('unrelated-secret'));
            assert.equal(fs.readFileSync(path.join(run.artifacts,'runtime/lightnvr.log'),'utf8'),'owned log');
          }finally{fs.rmSync(run.dir,{recursive:true});fs.rmSync(run.artifacts,{recursive:true});}
        """)

    def test_unselected_cli_fails_before_server_creation(self):
        result = subprocess.run(['node','scripts/quality/run-integration.mjs','--task','../escape'], cwd=ROOT, capture_output=True)
        self.assertNotEqual(result.returncode,0)


if __name__ == '__main__':
    unittest.main()
