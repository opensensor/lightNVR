#!/usr/bin/env node
/** Isolated, serialized Playwright runs. Never adopt existing servers or PID files. */
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import crypto from 'node:crypto';
import net from 'node:net';
import { spawn, spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { fileHash, stopOwnedProcess } from '../../tests/integration/fixtures/run-ownership.ts';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const lockPath = path.join(root, 'build/eng-artifacts/.runtime-lock');
const taskId = value => { if (!/^[A-Za-z0-9][A-Za-z0-9_.-]{0,79}$/.test(value) || value.includes('..')) throw new Error('Invalid task ID'); return value; };

export function validateLoopback(value) {
  const url = new URL(value);
  if (url.protocol !== 'http:' || !['localhost', '127.0.0.1', '[::1]'].includes(url.hostname) || url.username || url.password) throw new Error('Only credential-free loopback HTTP test targets are allowed');
}

export function acquireLock(lock = lockPath) {
  fs.mkdirSync(path.dirname(lock), { recursive: true }); fs.mkdirSync(lock);
  const token = crypto.randomUUID(); fs.writeFileSync(path.join(lock, 'owner'), token);
  return () => { if (fs.readFileSync(path.join(lock, 'owner'), 'utf8') !== token) throw new Error('Runtime lock ownership changed'); fs.unlinkSync(path.join(lock, 'owner')); fs.rmdirSync(lock); };
}

export function selectedCount(report) {
  if (!report || !Array.isArray(report.suites) || report.errors?.length) throw new Error('Invalid or failed Playwright discovery');
  const count = suites => suites.reduce((sum, suite) => sum + (suite.specs || []).reduce((n, spec) => n + (spec.tests || []).length, 0) + count(suite.suites || []), 0);
  const total = count(report.suites); if (!total) throw new Error('No tests selected; refusing empty success'); return total;
}

export function createRun({ task, binary, go2rtc }) {
  taskId(task);
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), `lightnvr-eng-${task}-`));
  const token = crypto.randomUUID(); const artifacts = path.join(root, 'build/eng-artifacts', task, 'integration-' + token);
  fs.mkdirSync(artifacts, { recursive: true });
  const config = path.join(dir, 'lightnvr.ini'); binary = fs.realpathSync(binary);
  let ini = fs.readFileSync(path.join(root, 'config/lightnvr-health-test.ini'), 'utf8')
    .replaceAll('/tmp/lightnvr-test', dir).replace('root = ./web/dist', `root = ${root}/web/dist`)
    .replace('[mqtt]\nenabled = true', '[mqtt]\nenabled = false');
  if (go2rtc) ini = ini.replace('[go2rtc]\nenabled = false', `[go2rtc]\nenabled = true\nbinary_path = ${root}/go2rtc/go2rtc\napi_port = 11984\nrtsp_port = 18554\nwebrtc_port = 18555\nauto_start = true`).replace('max_streams = 1', 'max_streams = 8');
  fs.writeFileSync(config, ini);
  fs.writeFileSync(path.join(dir, 'owner.json'), JSON.stringify({ dir, token, config, configHash: fileHash(config), binary, artifacts }, null, 2));
  const env = Object.fromEntries(Object.entries(process.env).filter(([key]) => !key.startsWith('LIGHTNVR_') && !['GO2RTC_URL', 'GO2RTC_BASE_PATH', 'GO2RTC_RTSP_PORT', 'DEBUG', 'PWDEBUG', 'HEADED', 'LD_LIBRARY_PATH'].includes(key)));
  Object.assign(env, { LIGHTNVR_RUN_TOKEN: token, LIGHTNVR_TEST_DIR: dir, LIGHTNVR_TEST_CONFIG: config, LIGHTNVR_TEST_BIN: binary, LIGHTNVR_URL: 'http://127.0.0.1:18080', GO2RTC_URL: 'http://127.0.0.1:11984', GO2RTC_RTSP_PORT: '18554', LIGHTNVR_SKIP_GO2RTC: go2rtc ? '0' : '1', LIGHTNVR_TEST_RESULTS_DIR: path.join(artifacts, 'results'), PLAYWRIGHT_HTML_OUTPUT_DIR: path.join(artifacts, 'html'), PLAYWRIGHT_JSON_OUTPUT_NAME: path.join(artifacts, 'results.json'), LD_LIBRARY_PATH: path.dirname(binary) });
  return { dir, artifacts, config, binary, env };
}

async function assertPortFree(port) {
  for (const host of ['127.0.0.1', '::1']) {
    await new Promise((resolve, reject) => {
      const server = net.createServer();
      server.once('error', error => {
        if (host === '::1' && error.code === 'EADDRNOTAVAIL') resolve();
        else reject(new Error(`Port ${host}:${port} is occupied/unavailable; refusing an existing service`));
      });
      server.listen({ port, host, ipv6Only: true }, () => server.close(resolve));
    });
  }
}

function execute(command, args, env, log) {
  return new Promise((resolve, reject) => {
    const fd = fs.openSync(log, 'w');
    const child = spawn(command, args, { cwd: root, env, stdio: ['ignore', fd, fd] });
    child.once('error', error => { fs.closeSync(fd); reject(error); });
    child.once('exit', (code, signal) => { fs.closeSync(fd); resolve(code ?? (signal ? 1 : 0)); });
  });
}

export async function main(argv = process.argv.slice(2)) {
  const options = { task: '', project: 'api', grep: '', binary: '', go2rtc: true };
  for (let i = 0; i < argv.length; i++) {
    const arg = argv[i];
    if (arg === '--without-go2rtc') { options.go2rtc = false; continue; }
    const key = { '--task': 'task', '--project': 'project', '--grep': 'grep', '--binary': 'binary' }[arg];
    if (!key || argv[i + 1] === undefined) throw new Error('Usage: --task ID [--project api|ui|go2rtc] [--grep regex] [--binary path] [--without-go2rtc]');
    options[key] = argv[++i];
  }
  taskId(options.task);
  if (!['api', 'ui', 'go2rtc'].includes(options.project)) throw new Error('Unknown Playwright project');
  for (const key of ['LIGHTNVR_URL', 'GO2RTC_URL']) if (process.env[key]) validateLoopback(process.env[key]);
  if (!options.binary) {
    const buildTask = options.task + '-server';
    const profile = options.go2rtc ? 'integration' : 'lean';
    const result = spawnSync('python3', [path.join(root, 'scripts/quality/run-native.py'), '--task', buildTask, '--profile', profile, '--sanitizer', 'none', '--build-only'], { cwd: root, stdio: 'inherit' });
    if (result.status !== 0) return result.status || 1;
    options.binary = path.join(root, 'build/eng-native', buildTask, profile + '-none/bin/lightnvr');
  }
  const release = acquireLock(); let run; let status = 1;
  try {
    for (const port of options.go2rtc ? [18080, 11984, 18554, 18555] : [18080]) await assertPortFree(port);
    run = createRun(options);
    console.log(`Integration artifacts: ${run.artifacts}\nOwned runtime: ${run.dir}`);
    const args = [path.join(root, 'node_modules/@playwright/test/cli.js'), 'test', '--project', options.project, '--retries=0'];
    if (options.grep) args.push('--grep', options.grep);
    const discovery = path.join(run.artifacts, 'discovery.json');
    status = await execute(process.execPath, [...args, '--list', '--reporter=json'], { ...run.env, PLAYWRIGHT_JSON_OUTPUT_NAME: discovery }, path.join(run.artifacts, 'discovery.log'));
    if (status) return status;
    const count = selectedCount(JSON.parse(fs.readFileSync(discovery, 'utf8')));
    fs.writeFileSync(path.join(run.artifacts, 'run.json'), JSON.stringify({ task: options.task, binary: run.binary, binaryHash: fileHash(run.binary), selectedTests: count, go2rtc: options.go2rtc, serialization: 'shared native/integration runtime lease; fixed legacy ports' }, null, 2));
    status = await execute(process.execPath, [...args, '--reporter=list,json'], run.env, path.join(run.artifacts, 'playwright.log'));
    console.log(fs.readFileSync(path.join(run.artifacts, 'playwright.log'), 'utf8').slice(-12000));
    if (status === 0) {
      const results = JSON.parse(fs.readFileSync(path.join(run.artifacts, 'results.json'), 'utf8'));
      if (!results.stats || results.stats.expected < 1) throw new Error('No executed passing tests; skipped-only runs are not qualification');
    }
    return status;
  } finally {
    let cleanupError;
    try { if (run) await stopOwnedProcess(run.env); }
    catch (error) { cleanupError = error; status = 2; }
    finally { release(); }
    // Keep owned runtime data/logs for diagnosis. Never recursively delete external paths.
    if (run) fs.writeFileSync(path.join(run.artifacts, 'exit.json'), JSON.stringify({ exitCode: status, runtimeRetained: run.dir, cleanupError: cleanupError?.message }));
    if (cleanupError) throw cleanupError;
  }
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  main().then(code => { process.exitCode = code; }).catch(error => { console.error(`Runner error: ${error.message}`); process.exitCode = 2; });
}
