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

export function createRun({ task, binary, go2rtc, configProfile = 'default' }) {
  taskId(task);
  if (!['default', 'health'].includes(configProfile)) throw new Error('Unknown configuration profile');
  if (configProfile === 'health') go2rtc = false;
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), `lightnvr-eng-${task}-`));
  const token = crypto.randomUUID(); const artifacts = path.join(root, 'build/eng-artifacts', task, 'integration-' + token);
  fs.mkdirSync(artifacts, { recursive: true });
  const config = path.join(dir, 'lightnvr.ini'); binary = fs.realpathSync(binary);
  let ini = fs.readFileSync(path.join(root, 'config/lightnvr-health-test.ini'), 'utf8')
    .replaceAll('/tmp/lightnvr-test', dir).replace('root = ./web/dist', `root = ${root}/web/dist`);
  if (configProfile !== 'health') ini = ini.replace('[mqtt]\nenabled = true', '[mqtt]\nenabled = false');
  if (go2rtc) ini = ini.replace('[go2rtc]\nenabled = false', `[go2rtc]\nenabled = true\nbinary_path = ${root}/go2rtc/go2rtc\napi_port = 11984\nrtsp_port = 18554\nwebrtc_port = 18555\nauto_start = true`).replace('max_streams = 1', 'max_streams = 8');
  fs.writeFileSync(config, ini);
  fs.writeFileSync(path.join(dir, 'owner.json'), JSON.stringify({ dir, token, config, configHash: fileHash(config), binary, artifacts }, null, 2));
  const env = Object.fromEntries(Object.entries(process.env).filter(([key]) => !key.startsWith('LIGHTNVR_') && !['GO2RTC_URL', 'GO2RTC_BASE_PATH', 'GO2RTC_RTSP_PORT', 'DEBUG', 'PWDEBUG', 'HEADED', 'LD_LIBRARY_PATH'].includes(key)));
  Object.assign(env, { LIGHTNVR_RUN_TOKEN: token, LIGHTNVR_TEST_DIR: dir, LIGHTNVR_TEST_CONFIG: config, LIGHTNVR_TEST_BIN: binary, LIGHTNVR_URL: 'http://127.0.0.1:18080', GO2RTC_URL: 'http://127.0.0.1:11984', GO2RTC_RTSP_PORT: '18554', LIGHTNVR_SKIP_GO2RTC: go2rtc ? '0' : '1', LIGHTNVR_TEST_RESULTS_DIR: path.join(artifacts, 'results'), PLAYWRIGHT_HTML_OUTPUT_DIR: path.join(artifacts, 'html'), PLAYWRIGHT_HTML_OPEN: 'never', PLAYWRIGHT_JSON_OUTPUT_NAME: path.join(artifacts, 'results.json'), LD_LIBRARY_PATH: path.dirname(binary) });
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

const usage = 'Usage: --task ID [--project all|api|ui|go2rtc] [--grep regex] [--shard i/n] [--file tests/integration/path.spec.ts ...] [--binary path] [--config-profile default|health] [--without-go2rtc] [--list]';

export function parseOptions(argv) {
  if (argv.includes('--help')) return { help: true };
  const options = { task: '', project: 'api', grep: '', binary: '', go2rtc: true, files: [], shard: '', list: false, configProfile: 'default' };
  const keys = { '--task': 'task', '--project': 'project', '--grep': 'grep', '--binary': 'binary', '--shard': 'shard', '--config-profile': 'configProfile', '--file': 'file' };
  for (let i = 0; i < argv.length; i++) {
    const argument = argv[i];
    if (argument === '--without-go2rtc') { options.go2rtc = false; continue; }
    if (argument === '--list') { options.list = true; continue; }
    const equal = argument.indexOf('=');
    const flag = equal < 0 ? argument : argument.slice(0, equal);
    const key = keys[flag];
    const value = equal < 0 ? argv[++i] : argument.slice(equal + 1);
    if (!key || value === undefined || value === '' || value.startsWith('--')) throw new Error(usage);
    if (key === 'file') {
      const file = fs.realpathSync(path.resolve(root, value));
      const directory = path.join(root, 'tests/integration') + path.sep;
      if (!file.startsWith(directory) || !/\.spec\.(ts|tsx|js|jsx)$/.test(file) || !fs.statSync(file).isFile()) throw new Error('File filters must name exact integration spec files inside tests/integration');
      options.files.push(file);
    } else options[key] = value;
  }
  taskId(options.task);
  if (!['all', 'api', 'ui', 'go2rtc'].includes(options.project)) throw new Error('Unknown Playwright project');
  if (options.grep) new RegExp(options.grep);
  if (options.shard) {
    const match = /^([1-9][0-9]*)\/([1-9][0-9]*)$/.exec(options.shard);
    if (!match || !Number.isSafeInteger(Number(match[2])) || Number(match[1]) > Number(match[2])) throw new Error('Shard must be i/n with 1 <= i <= n');
  }
  if (!['default', 'health'].includes(options.configProfile)) throw new Error('Unknown configuration profile');
  if (options.configProfile === 'health') {
    options.go2rtc = false;
    if (!options.binary && !options.list) throw new Error('The health profile requires an explicit MQTT-enabled --binary');
  }
  return options;
}

export function playwrightArgs(options) {
  const args = [path.join(root, 'node_modules/@playwright/test/cli.js'), 'test', '--retries=0'];
  if (options.project !== 'all') args.push('--project=' + options.project);
  if (options.grep) args.push('--grep', options.grep);
  if (options.shard) args.push('--shard=' + options.shard);
  for (const file of options.files) args.push('^' + file.replace(/[.*+?^${}()|[\]\\]/g, '\\$&') + '$');
  return args;
}

export function archiveRunLogs(run) {
  const destination = path.join(run.artifacts, 'runtime');
  fs.mkdirSync(destination, { recursive: true });
  for (const name of ['lightnvr.ini', 'lightnvr.log', 'owner.json', 'process.json', 'process-stopped.json', 'go2rtc/go2rtc.log', 'go2rtc/go2rtc.yaml']) {
    const source = path.join(run.dir, name);
    if (!fs.existsSync(source) || fs.lstatSync(source).isSymbolicLink() || !fs.lstatSync(source).isFile() || !fs.realpathSync(source).startsWith(fs.realpathSync(run.dir) + path.sep)) continue;
    const target = path.join(destination, name); fs.mkdirSync(path.dirname(target), { recursive: true }); fs.copyFileSync(source, target);
  }
}

async function discover(options, artifacts, env = process.env) {
  fs.mkdirSync(artifacts, { recursive: true });
  const discovery = path.join(artifacts, 'discovery.json');
  const log = path.join(artifacts, 'discovery.log');
  const status = await execute(process.execPath, [...playwrightArgs(options), '--list', '--reporter=json'], { ...env, PLAYWRIGHT_JSON_OUTPUT_NAME: discovery, PLAYWRIGHT_HTML_OPEN: 'never' }, log);
  console.log(fs.readFileSync(log, 'utf8'));
  if (status) return { status, count: 0 };
  return { status: 0, count: selectedCount(JSON.parse(fs.readFileSync(discovery, 'utf8'))) };
}

export async function main(argv = process.argv.slice(2)) {
  const options = parseOptions(argv);
  if (options.help) { console.log(usage); return 0; }
  for (const key of ['LIGHTNVR_URL', 'GO2RTC_URL']) if (process.env[key]) validateLoopback(process.env[key]);
  if (process.env.LIGHTNVR_TEST_CONFIG) throw new Error('Arbitrary LIGHTNVR_TEST_CONFIG is not accepted; use --config-profile default or health');
  if (options.list) {
    const artifacts = path.join(root, 'build/eng-artifacts', options.task, 'discovery-' + crypto.randomUUID());
    const result = await discover(options, artifacts);
    console.log(`Discovery artifacts: ${artifacts}`);
    return result.status;
  }
  if (!options.binary) {
    const buildTask = options.task + '-server';
    const profile = options.go2rtc ? 'integration' : 'lean';
    const result = spawnSync('python3', [path.join(root, 'scripts/quality/run-native.py'), '--task', buildTask, '--profile', profile, '--sanitizer', 'none', '--build-only'], { cwd: root, stdio: 'inherit' });
    if (result.status !== 0) return result.status || 1;
    options.binary = path.join(root, 'build/eng-native', buildTask, profile + '-none/bin/lightnvr');
  }
  const release = acquireLock(); let run; let status = 1;
  try {
    const ports = options.go2rtc ? [18080, 11984, 18554, 18555] : [18080];
    // The health lane deliberately exercises an unreachable local broker.
    if (options.configProfile === 'health') ports.push(18884);
    for (const port of ports) await assertPortFree(port);
    run = createRun(options);
    console.log(`Integration artifacts: ${run.artifacts}\nOwned runtime: ${run.dir}`);
    const discovery = await discover(options, run.artifacts, run.env);
    status = discovery.status;
    if (status) return status;
    fs.writeFileSync(path.join(run.artifacts, 'run.json'), JSON.stringify({ task: options.task, binary: run.binary, binaryHash: fileHash(run.binary), selectedTests: discovery.count, go2rtc: options.go2rtc, configProfile: options.configProfile, files: options.files, shard: options.shard, serialization: 'shared native/integration runtime lease; fixed legacy ports' }, null, 2));
    status = await execute(process.execPath, [...playwrightArgs(options), '--reporter=list,json,html'], run.env, path.join(run.artifacts, 'playwright.log'));
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
    finally {
      try { if (run) archiveRunLogs(run); }
      finally { release(); }
    }
    if (run) fs.writeFileSync(path.join(run.artifacts, 'exit.json'), JSON.stringify({ exitCode: status, runtimeRetained: run.dir, cleanupError: cleanupError?.message }));
    if (cleanupError) throw cleanupError;
  }
}

if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  main().then(code => { process.exitCode = code; }).catch(error => { console.error(`Runner error: ${error.message}`); process.exitCode = 2; });
}
