#!/usr/bin/env node
/** T29 isolated real-process lifecycle checks. Never adopts an existing server. */
import assert from 'node:assert/strict';
import fs from 'node:fs';
import path from 'node:path';
import net from 'node:net';
import { spawn, spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';
import { chromium } from '@playwright/test';
import { acquireLock, createRun, archiveRunLogs } from '../../../scripts/quality/run-integration.mjs';
import { fileHash, recordOwnedProcess, recordOwnedDescendants, stopOwnedProcess } from '../../integration/fixtures/run-ownership.ts';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../..');
const fixtures = path.join(root, 'tests/fixtures/eng/upgrade');
const fixtureTool = path.join(root, 'tests/operational/lifecycle/fixture.py');
const binary = process.argv[2];
if (!binary || process.argv.length !== 3) throw new Error('Usage: node tests/operational/lifecycle/run.mjs /absolute/path/to/lightnvr');
const release = acquireLock();
let run, child, browser;
const checks = [];
const check = async (name, action) => {
  try { const detail = await action(); checks.push({ name, passed: true, detail }); }
  catch (error) { checks.push({ name, passed: false, error: error.message }); }
};
const pause = ms => new Promise(resolve => setTimeout(resolve, ms));
const auth = 'Basic ' + Buffer.from('admin:admin').toString('base64');
const endpoint = 'http://127.0.0.1:18080';
const python = (operation, sql) => {
  const result = spawnSync('python3', [fixtureTool, operation, run.dir, ...(sql ? [sql] : [])], { env: run.env, encoding: 'utf8' });
  if (result.status !== 0) throw new Error(result.stderr || 'Fixture operation failed');
  return JSON.parse(result.stdout);
};
const query = sql => python('query', sql);
async function until(action, label, deadline = 45000) {
  const start = performance.now(); let error;
  while (performance.now() - start < deadline) {
    try { const result = await action(); if (result) return result; } catch (caught) { error = caught; }
    await pause(200);
  }
  throw new Error(`${label} exceeded ${deadline}ms: ${error?.message || 'predicate stayed false'}`);
}
async function api(url, options = {}) {
  const response = await fetch(endpoint + url, { ...options, headers: { Authorization: auth, 'Content-Type': 'application/json' }, signal: AbortSignal.timeout(3000) });
  return response;
}
async function start(phase) {
  const fd = fs.openSync(path.join(run.artifacts, `${phase}.log`), 'w');
  child = spawn(run.binary, ['-c', run.config], { cwd: root, env: run.env, detached: true, stdio: ['ignore', fd, fd] });
  await new Promise((resolve, reject) => { child.once('spawn', resolve); child.once('error', reject); });
  fs.closeSync(fd);
  try { recordOwnedProcess(child.pid, run.env); }
  catch (error) { child.kill('SIGKILL'); throw error; }
  child.unref();
  await until(async () => (await api('/api/health')).status === 200, `${phase} ready`);
  await until(async () => {
    const response = await fetch('http://127.0.0.1:11984/go2rtc/api/streams?name=t29_source&src=' + encodeURIComponent('ffmpeg:virtual?video&size=160x120#video=h264'), { method: 'PUT', headers: { Authorization: auth }, signal: AbortSignal.timeout(3000) });
    return response.ok;
  }, `${phase} go2rtc source`);
  recordOwnedDescendants(run.env);
}
async function stop(phase) {
  recordOwnedDescendants(run.env);
  await stopOwnedProcess(run.env, 60000);
  const result = JSON.parse(fs.readFileSync(path.join(run.artifacts, 'shutdown.json')));
  fs.copyFileSync(path.join(run.artifacts, 'shutdown.json'), path.join(run.artifacts, `${phase}-shutdown.json`));
  assert.equal(result.graceful, true);
  assert.equal(result.forcedKill, false);
  assert.equal(result.forcedChildCleanup, false);
  assert.ok(result.elapsedMs < 60000);
  return result;
}
function probe(file) {
  const result = spawnSync('ffprobe', ['-v', 'error', '-show_entries', 'format=duration:stream=codec_type', '-of', 'json', file], { encoding: 'utf8' });
  assert.equal(result.status, 0, result.stderr);
  const media = JSON.parse(result.stdout);
  assert.ok(media.streams.some(stream => stream.codec_type === 'video'));
  assert.ok(Number(media.format.duration) > 0);
  return media;
}
async function preferences(state) {
  browser = await chromium.launch({ headless: true });
  try {
  const context = await browser.newContext({ storageState: state, httpCredentials: { username: 'admin', password: 'admin' } });
  const page = await context.newPage();
  await page.goto(endpoint + '/settings.html', { waitUntil: 'domcontentloaded' });
  await until(() => page.evaluate(() => document.documentElement.dataset.reduceMotion === 'true' && document.documentElement.classList.contains('dark')), 'browser preference application', 15000);
  await until(() => page.evaluate(() => document.documentElement.style.getPropertyValue('--primary').trim() === '217 25% 55%'), 'saved blue theme and intensity application', 15000);
  const expected = JSON.parse(fs.readFileSync(path.join(fixtures, '0.45.0-preferences.json')));
  assert.deepEqual(await page.evaluate(keys => Object.fromEntries(keys.map(key => [key, localStorage.getItem(key)])), Object.keys(expected)), expected);
  const saved = await context.storageState();
  return saved;
  } finally { await browser.close(); browser = null; }
}
try {
  for (const port of [18080, 11984, 18554, 18555]) {
    for (const host of ['127.0.0.1', '::1']) await new Promise((resolve, reject) => {
      const server = net.createServer(); server.once('error', reject);
      server.listen({ port, host, ipv6Only: true }, () => server.close(resolve));
    });
  }
  run = createRun({ task: 'T29', binary, go2rtc: true });
  let ini = fs.readFileSync(path.join(fixtures, '0.45.0.ini'), 'utf8')
    .replaceAll('/tmp/lightnvr-test', run.dir).replace('root = ./web/dist', `root = ${root}/web/dist`)
    .replace('[mqtt]\nenabled = true', '[mqtt]\nenabled = false')
    .replace('max_streams = 1', 'max_streams = 8')
    .replace('record_mp4_directly = false', 'record_mp4_directly = true')
    .replace('mp4_segment_duration = 60', 'mp4_segment_duration = 2')
    .replace('auto_delete_oldest = true', 'auto_delete_oldest = false')
    .replace('[go2rtc]\nenabled = false', `[go2rtc]\nenabled = true\nbinary_path = ${root}/go2rtc/go2rtc\napi_port = 11984\nrtsp_port = 18554\nwebrtc_port = 18555\nauto_start = true`);
  fs.writeFileSync(run.config, ini);
  const ownerPath = path.join(run.dir, 'owner.json'); const owner = JSON.parse(fs.readFileSync(ownerPath));
  fs.writeFileSync(ownerPath, JSON.stringify({ ...owner, configHash: fileHash(run.config) }, null, 2));
  const seeded = python('seed');
  fs.writeFileSync(path.join(run.artifacts, 'seed.json'), JSON.stringify(seeded, null, 2));
  fs.writeFileSync(path.join(run.artifacts, 'run.json'), JSON.stringify({ binary: run.binary, binaryHash: fileHash(run.binary), fixture: JSON.parse(fs.readFileSync(path.join(fixtures, 'provenance.json'))), deadlineMs: 60000, recordingDeadlineMs: 45000 }, null, 2));
  console.log(`T29 artifacts: ${run.artifacts}`);
  await start('upgrade');
  await check('previous-release database integrity and schema continuity', () => {
    assert.deepEqual(query('PRAGMA integrity_check'), [{ integrity_check: 'ok' }]);
    assert.deepEqual(query('SELECT version FROM schema_migrations ORDER BY version'), seeded.schema_versions);
    assert.equal(query("SELECT value FROM system_settings WHERE key='t29.synthetic.preference'")[0].value, 'preserve-me');
  });
  await check('committed historical footage remains playable and byte-identical', () => {
    const file = path.join(run.dir, 'recordings/mp4/historical-complete.mp4');
    assert.equal(fileHash(file), seeded.complete_sha256); probe(file);
    assert.equal(query("SELECT is_complete FROM recordings WHERE file_path LIKE '%/historical-complete.mp4'")[0].is_complete, 1);
  });
  await check('stale valid media finalized and empty/missing rows pruned', async () => {
    await until(() => query("SELECT is_complete FROM recordings WHERE file_path LIKE '%/stale-valid.mp4'")[0]?.is_complete === 1, 'startup media reconciliation', 15000);
    const rows = query('SELECT file_path,is_complete,size_bytes FROM recordings');
    const valid = rows.find(row => row.file_path.endsWith('/stale-valid.mp4'));
    assert.equal(valid?.is_complete, 1); assert.ok(valid.size_bytes > 0); probe(valid.file_path);
    assert.ok(!rows.some(row => /stale-(zero|missing)\.mp4$/.test(row.file_path)));
    assert.equal(fs.existsSync(path.join(run.dir, 'recordings/mp4/stale-zero.mp4')), false);
    assert.equal(rows.find(row => row.file_path.endsWith('/recent-incomplete.mp4'))?.is_complete, 0);
  });
  await check('truncated stale media is not advertised as complete footage', () => {
    const row = query("SELECT is_complete FROM recordings WHERE file_path LIKE '%/stale-truncated.mp4'")[0];
    assert.ok(!row || row.is_complete === 0, 'Nonempty but unplayable fragment was promoted to complete');
  });
  const legacyPrefs = JSON.parse(fs.readFileSync(path.join(fixtures, '0.45.0-preferences.json')));
  let browserState = { cookies: [], origins: [{ origin: endpoint, localStorage: [...Object.entries(legacyPrefs), ['auth', Buffer.from('admin:admin').toString('base64')]].map(([name, value]) => ({ name, value })) }] };
  await check('previous browser preferences are applied', async () => { browserState = await preferences(browserState); });
  const created = await api('/api/streams', { method: 'POST', body: JSON.stringify({ name: 't29_live', url: 'rtsp://127.0.0.1:18554/t29_source', enabled: true, streaming_enabled: true, record: true, segment_duration: 2, width: 160, height: 120, fps: 5, codec: 'h264' }) });
  assert.equal(created.status, 201, await created.text());
  const committed = await until(() => query("SELECT * FROM recordings WHERE stream_name='t29_live' AND is_complete=1 AND size_bytes>0")[0], 'actual recorder committed segment');
  await check('actual recording is playable before shutdown', () => probe(committed.file_path));
  const recordingHash = fileHash(committed.file_path);
  await check('active recording graceful shutdown meets deadline without fallback', () => stop('graceful'));
  if (fs.existsSync(path.join(run.dir, 'process.json'))) throw new Error('Graceful shutdown left active ownership record');
  await start('restart');
  await check('restart preserves committed recording and stream configuration', () => {
    assert.equal(fileHash(committed.file_path), recordingHash); probe(committed.file_path);
    assert.equal(query("SELECT record FROM streams WHERE name='t29_live'")[0].record, 1);
    assert.deepEqual(query('PRAGMA integrity_check'), [{ integrity_check: 'ok' }]);
  });
  await check('browser preference state survives server restart', async () => { browserState = await preferences(browserState); });
  const newRecording = await until(() => query(`SELECT * FROM recordings WHERE stream_name='t29_live' AND id>${committed.id} AND is_complete=0`)[0], 'active post-restart recording');
  recordOwnedDescendants(run.env);
  const saved = JSON.parse(fs.readFileSync(path.join(run.dir, 'process.json')));
  const injected = spawnSync('python3', ['-c', `import os,signal,sys\npid=int(sys.argv[1]);fd=os.pidfd_open(pid)\nstat=open('/proc/'+str(pid)+'/stat').read().rsplit(') ',1)[1].split()\nassert stat[19]==sys.argv[2]\nassert os.path.realpath('/proc/'+str(pid)+'/exe')==sys.argv[3]\nsignal.pidfd_send_signal(fd,signal.SIGKILL);os.close(fd)`, String(saved.pid), saved.start, run.binary], { encoding: 'utf8' });
  assert.equal(injected.status, 0, injected.stderr);
  await until(() => child.exitCode !== null || child.signalCode !== null, 'injected kill observed', 5000);
  let faultCleanupError;
  try { await stopOwnedProcess(run.env, 1000); } catch (error) { faultCleanupError = error.message; }
  const fault = JSON.parse(fs.readFileSync(path.join(run.artifacts, 'shutdown.json')));
  fs.writeFileSync(path.join(run.artifacts, 'injected-kill.json'), JSON.stringify({ injected: 'SIGKILL', recordingId: newRecording.id, cleanup: fault, faultCleanupError, gracefulQualification: false }, null, 2));
  assert.equal(fault.lingeringChildren.length, 0);
  await start('crash-recovery');
  await check('forced-kill restart recovers database and preserves committed media', () => {
    assert.deepEqual(query('PRAGMA integrity_check'), [{ integrity_check: 'ok' }]);
    assert.equal(fileHash(committed.file_path), recordingHash); probe(committed.file_path);
    assert.equal(query("SELECT value FROM system_settings WHERE key='t29.synthetic.preference'")[0].value, 'preserve-me');
  });
  await check('interrupted fresh segment remains honestly incomplete within one-hour grace', () => {
    const row = query(`SELECT is_complete FROM recordings WHERE id=${newRecording.id}`)[0];
    assert.equal(row?.is_complete, 0);
  });
  await check('recording resumes after forced-kill recovery', async () => {
    const row = await until(() => query(`SELECT * FROM recordings WHERE stream_name='t29_live' AND id>${newRecording.id} AND is_complete=1 AND size_bytes>0`)[0], 'post-crash committed recording');
    probe(row.file_path);
  });
  await check('final graceful shutdown meets deadline', () => stop('final'));
} catch (error) { checks.push({ name: 'lifecycle runner', passed: false, error: error.stack }); }
finally {
  if (browser) await browser.close();
  if (run) {
    try { await stopOwnedProcess(run.env, 60000); } catch (error) { checks.push({ name: 'cleanup', passed: false, error: error.message }); }
    archiveRunLogs(run);
    fs.writeFileSync(path.join(run.artifacts, 'checks.json'), JSON.stringify(checks, null, 2));
  }
  release();
}
console.log(JSON.stringify(checks, null, 2));
process.exitCode = checks.length && checks.every(result => result.passed) ? 0 : 1;
