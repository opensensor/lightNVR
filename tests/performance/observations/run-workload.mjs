#!/usr/bin/env node
/**
 * INT_01 observation performance workload (plan tasks C4 and X2).
 *
 *   node tests/performance/observations/run-workload.mjs \
 *     --binary build/int01-c4-release/bin/lightnvr --model /path/to/tiny20.sod \
 *     --output build/perf-observations/disabled-run1 --mode disabled
 *
 * Starts a standalone go2rtc serving N looped RTSP fixture streams, an isolated
 * lightNVR (tests/contracts/fixture-server.mjs) with continuous recording and
 * a local SOD detector on every stream, then samples a fixed window and writes
 * <output>/results.json. See README.md for every measured field.
 */
import {spawn, execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {watch} from 'node:fs';
import {mkdir, readFile, readdir, stat, writeFile, copyFile, open} from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import {reservePort} from '../../contracts/fixture-server.mjs';
import {trackedDiffSha256} from '../git-provenance.mjs';
import {
  summarize, parsePrometheus, sumMetric, metricByLabel, countIntervalGaps,
  catalogGaps, resourceSummary,
} from './metrics.mjs';

const here = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(here, '../../..');
const workload = JSON.parse(await readFile(path.join(here, 'workload.json'), 'utf8'));
const GAP_THRESHOLD_SEC = 5; // RECORDING_GAP_THRESHOLD_SEC in include/telemetry/stream_metrics.h

// ---------- arguments ----------
const args = process.argv.slice(2);
const opt = (name, fallback) => { const i = args.indexOf(name); return i < 0 ? fallback : args[i + 1]; };
const flag = name => args.includes(name);
if (flag('--help')) {
  console.log(`Options (defaults from workload.json):
  --binary PATH          lightNVR binary (default $LIGHTNVR_CONTRACT_BIN or build/int01-c4-release/bin/lightnvr)
  --model PATH           SOD model file (required unless --no-detector)
  --output DIR           new results directory (must not exist)
  --mode NAME            label: disabled | enabled (default disabled)
  --streams N --seconds S --warmup S --width W --height H --fps F --gop G --segment S
  --detection-interval S
  --source FILE          use this H.264 clip instead of the synthetic testsrc2 clip
  --stream-extra FILE    JSON merged into every POST /api/streams body (X2: evidence opt-in)
  --config-extra FILE    INI text appended to the generated lightnvr.ini (X2: evidence budgets)
  --work-dir DIR         where fixture storage lives (default build/perf-observations/work; must be a real disk, not tmpfs)
  --go2rtc PATH          go2rtc binary (default: go2rtc on PATH)
  --no-detector          omit the detector (diagnostic only; not a C4/X2 workload)
  --keep                 keep the fixture directory`);
  process.exit(0);
}
const int = (name, fallback, lo, hi) => {
  const v = Number(opt(name, fallback));
  if (!Number.isInteger(v) || v < lo || v > hi) throw new Error(`${name} must be an integer ${lo}..${hi}`);
  return v;
};
const binary = path.resolve(opt('--binary', process.env.LIGHTNVR_CONTRACT_BIN || 'build/int01-c4-release/bin/lightnvr'));
const output = path.resolve(opt('--output', `build/perf-observations/run-${Date.now()}`));
const mode = opt('--mode', 'disabled');
const streams = int('--streams', workload.streams, 1, 32);
const seconds = int('--seconds', workload.duration_seconds, 10, 7200);
const warmup = int('--warmup', workload.warmup_seconds, 0, 600);
const width = int('--width', workload.width, 64, 3840);
const height = int('--height', workload.height, 64, 2160);
const fps = int('--fps', workload.fps, 1, 60);
const gop = int('--gop', workload.gop, 1, 600);
const segment = int('--segment', workload.segment_seconds, 5, 900);
const detectionInterval = int('--detection-interval', workload.detector.interval_seconds, 1, 3600);
const useDetector = !flag('--no-detector');
const modelPath = opt('--model', process.env.LIGHTNVR_PERF_SOD_MODEL);
if (useDetector && !modelPath) throw new Error('--model PATH (SOD model, e.g. tiny20.sod) is required; see README.md');
const workDir = path.resolve(opt('--work-dir', 'build/perf-observations/work'));
const go2rtcBin = opt('--go2rtc', 'go2rtc');
const streamExtra = opt('--stream-extra') ? JSON.parse(await readFile(opt('--stream-extra'), 'utf8')) : {};
const configExtra = opt('--config-extra') ? await readFile(opt('--config-extra'), 'utf8') : '';

// ---------- helpers ----------
const sha256 = buf => createHash('sha256').update(buf).digest('hex');
const sh = (cmd, argv) => { try { return execFileSync(cmd, argv, {encoding: 'utf8', timeout: 10000, stdio: ['ignore', 'pipe', 'pipe']}).trim(); } catch { return null; } };
const pause = ms => new Promise(r => setTimeout(r, ms));
const now = () => performance.now();
const clkTck = Number(sh('getconf', ['CLK_TCK'])) || 100;

async function procSample(pid) {
  try {
    const [status, statText, io, fds] = await Promise.all([
      readFile(`/proc/${pid}/status`, 'utf8'), readFile(`/proc/${pid}/stat`, 'utf8'),
      readFile(`/proc/${pid}/io`, 'utf8').catch(() => ''), readdir(`/proc/${pid}/fd`).catch(() => []),
    ]);
    const f = statText.slice(statText.lastIndexOf(')') + 2).split(' ');
    return {
      tMs: now(), cpuTicks: Number(f[11]) + Number(f[12]),
      rssKiB: Number(status.match(/^VmRSS:\s+(\d+)/m)?.[1]), peakRssKiB: Number(status.match(/^VmHWM:\s+(\d+)/m)?.[1]),
      threads: Number(status.match(/^Threads:\s+(\d+)/m)?.[1]), fds: fds.length,
      writeBytes: Number(io.match(/^write_bytes:\s+(\d+)/m)?.[1] ?? NaN),
    };
  } catch { return null; }
}

/** Sum CPU ticks of a process and all its descendants. */
async function treeCpuTicks(rootPid) {
  const children = new Map();
  for (const entry of await readdir('/proc')) {
    if (!/^\d+$/.test(entry)) continue;
    try {
      const s = await readFile(`/proc/${entry}/stat`, 'utf8');
      const f = s.slice(s.lastIndexOf(')') + 2).split(' ');
      const ppid = Number(f[1]);
      if (!children.has(ppid)) children.set(ppid, []);
      children.get(ppid).push({pid: Number(entry), ticks: Number(f[11]) + Number(f[12])});
    } catch {}
  }
  let total = 0, count = 0;
  const walk = pid => { for (const c of children.get(pid) || []) { total += c.ticks; count++; walk(c.pid); } };
  try {
    const s = await readFile(`/proc/${rootPid}/stat`, 'utf8');
    const f = s.slice(s.lastIndexOf(')') + 2).split(' ');
    total += Number(f[11]) + Number(f[12]); count++;
  } catch { return null; }
  walk(rootPid);
  return {tMs: now(), ticks: total, processes: count};
}

async function describeHost(storageDir) {
  const cpus = os.cpus();
  const mount = sh('findmnt', ['-n', '-T', storageDir, '-o', 'SOURCE,FSTYPE,OPTIONS'])?.split(/\s+/) || [];
  let deviceModel = null, rotational = null;
  if (mount[0]?.startsWith('/dev/')) {
    const parent = sh('lsblk', ['-no', 'PKNAME', mount[0]])?.split('\n')[0] || path.basename(mount[0]);
    deviceModel = sh('lsblk', ['-dno', 'MODEL', `/dev/${parent}`]);
    rotational = sh('lsblk', ['-dno', 'ROTA', `/dev/${parent}`]) === '1';
  }
  return {
    hostname_sha256: sha256(os.hostname()).slice(0, 16),
    cpu_model: cpus[0]?.model, logical_cores: cpus.length,
    cpu_governor: (await readFile('/sys/devices/system/cpu/cpu0/cpufreq/scaling_governor', 'utf8').catch(() => '')).trim() || null,
    ram_bytes: os.totalmem(), kernel: os.release(), kernel_version: os.version(), arch: os.arch(),
    os_release: (await readFile('/etc/os-release', 'utf8').catch(() => '')).match(/^PRETTY_NAME="?([^"\n]*)/m)?.[1] || null,
    loadavg_at_start: os.loadavg(),
    storage: {path: storageDir, device: mount[0] || null, fstype: mount[1] || null, mount_options: mount[2] || null,
      device_model: deviceModel, rotational},
    ffmpeg: sh('ffmpeg', ['-version'])?.split('\n')[0] || null,
    go2rtc: sh(go2rtcBin, ['--version']) || sh(go2rtcBin, ['-version']),
    node: process.version,
  };
}

// ---------- setup ----------
await mkdir(output, {recursive: false});
await mkdir(workDir, {recursive: true});
const tmpfsLike = sh('findmnt', ['-n', '-T', workDir, '-o', 'FSTYPE']);
if (['tmpfs', 'ramfs'].includes(tmpfsLike)) throw new Error(`--work-dir ${workDir} is ${tmpfsLike}; disk write rate would be meaningless`);
process.env.TMPDIR = workDir; // fixture-server uses os.tmpdir() for its private directory

const git = (...a) => sh('git', ['-C', repoRoot, ...a]);
const report = {
  schema_version: 1, workload_name: workload.name, mode, status: 'running', started_at: new Date().toISOString(),
  host: await describeHost(workDir),
  build: {revision: git('rev-parse', 'HEAD'), branch: git('rev-parse', '--abbrev-ref', 'HEAD'),
    tracked_diff_sha256: await trackedDiffSha256(repoRoot), binary: path.relative(repoRoot, binary),
    binary_sha256: sha256(await readFile(binary)), harness_sha256: sha256(await readFile(fileURLToPath(import.meta.url))),
    metrics_sha256: sha256(await readFile(path.join(here, 'metrics.mjs')))},
  workload: {streams, width, height, fps, gop, codec: workload.codec, segment_seconds: segment, warmup_seconds: warmup,
    duration_seconds: seconds, transport: workload.transport, live_viewers: 0, audio: workload.audio,
    detector: useDetector ? {kind: workload.detector.kind, model_file: path.basename(modelPath), interval_seconds: detectionInterval,
      threshold_percent: workload.detector.threshold_percent} : null},
  retention: workload.retention,
  feature: {stream_extra: streamExtra, config_extra: configExtra || null},
  definitions: {
    'latency.segment_interval_ms': 'Per stream, wall-clock interval between first sight (inotify) of consecutive new segment files; nominal = segment_seconds*1000. A recording-path stall (writer, rotation, DB insert under the global DB mutex) lengthens it.',
    'latency.segment_interval_excess_ms': 'segment_interval_ms minus nominal.',
    'latency.db_probe_ms': `Round trip of GET /api/recordings?limit=1 with a session cookie every ${workload.sampling.db_probe_interval_ms} ms; contends on the same global DB mutex as recording-path catalog writes.`,
    'gaps.metrics_recording_gaps_total': 'Delta of lightnvr_recording_gaps_total over the window (in-process detector, threshold 5 s).',
    'gaps.catalog_gaps_total': 'Consecutive catalog rows per stream with start - previous end > 5 s (whole run, after warmup).',
    'gaps.segment_interval_gaps': 'Segment intervals longer than nominal + 5 s inside the window.',
  },
  limitations: [
    'Workstation measurement; go2rtc and the ffmpeg publishers share the host (their CPU is reported separately under resources.fixture_go2rtc_and_publishers).',
    'No in-process packet-write latency histogram exists in lightNVR today; recording-path latency is the black-box segment-interval proxy plus the DB-mutex probe.',
    'Synthetic testsrc2 content produces no VOC detections: the detector runs inference at its cadence but creates no detections. X2 needs a fixture that yields detections and must rerun disabled mode on that same source and binary.',
  ],
};
if (useDetector) report.workload.detector.model_sha256 = sha256(await readFile(modelPath));
const save = () => writeFile(path.join(output, 'results.json'), JSON.stringify(report, null, 2) + '\n');

// Source clip: deterministic synthetic H.264 (or a user clip).
let source = opt('--source');
if (source) source = path.resolve(source);
else {
  source = path.join(output, 'source.mp4');
  execFileSync('ffmpeg', ['-nostdin', '-hide_banner', '-loglevel', 'error', '-f', 'lavfi', '-i', `testsrc2=size=${width}x${height}:rate=${fps}`,
    '-t', String(workload.source_clip_seconds), '-an', '-c:v', 'libx264', '-threads', '1', '-preset', 'ultrafast', '-pix_fmt', 'yuv420p',
    '-g', String(gop), '-keyint_min', String(gop), '-sc_threshold', '0', '-bf', '0', '-movflags', '+faststart', source], {timeout: 120000});
}
report.workload.source = opt('--source') ? 'user clip (see source_sha256)' : workload.source;
report.workload.source_sha256 = sha256(await readFile(source));
report.workload.source_probe = JSON.parse(execFileSync('ffprobe', ['-v', 'error', '-select_streams', 'v:0', '-show_entries',
  'stream=codec_name,width,height,r_frame_rate,pix_fmt:format=duration', '-of', 'json', source], {encoding: 'utf8'}));

// ---------- go2rtc fixture ----------
const rtspPort = await reservePort(), apiPort = await reservePort();
const ports = {rtsp: rtspPort.port, api: apiPort.port};
await rtspPort.release(); await apiPort.release();
const go2rtcDir = path.join(output, 'go2rtc');
await mkdir(go2rtcDir);
// Streams are declared without a source; one always-on ffmpeg per stream
// publishes the looped clip into go2rtc, so the producer never depends on
// consumer timing (go2rtc exec: sources start lazily and raced the first
// recorder/detector connect in a smoke run).
const yaml = [`api:\n  listen: "127.0.0.1:${ports.api}"`, `rtsp:\n  listen: "127.0.0.1:${ports.rtsp}"`,
  'webrtc:\n  listen: ""', 'srtp:\n  listen: ""', 'log:\n  level: warn', 'streams:',
  ...Array.from({length: streams}, (_, i) => `  cam${i}:`),
].join('\n') + '\n';
await writeFile(path.join(go2rtcDir, 'go2rtc.yaml'), yaml);
const go2rtcLog = await open(path.join(go2rtcDir, 'go2rtc.log'), 'w');
const go2rtc = spawn(go2rtcBin, ['-config', path.join(go2rtcDir, 'go2rtc.yaml')], {stdio: ['ignore', go2rtcLog.fd, go2rtcLog.fd], detached: true});
const go2rtcExit = new Promise(r => go2rtc.once('close', r));
for (let i = 0; ; i++) {
  try { if ((await fetch(`http://127.0.0.1:${ports.api}/api/streams`, {signal: AbortSignal.timeout(500)})).ok) break; } catch {}
  if (i > 50 || go2rtc.exitCode !== null) throw new Error('go2rtc did not start; see go2rtc/go2rtc.log');
  await pause(100);
}
let stopping = false;
const sourceErrors = [];
const publishers = Array.from({length: streams}, (_, i) => {
  const child = spawn('ffmpeg', ['-hide_banner', '-loglevel', 'error', '-nostdin', '-re', '-stream_loop', '-1', '-i', source,
    '-c', 'copy', '-rtsp_transport', 'tcp', '-f', 'rtsp', `rtsp://127.0.0.1:${ports.rtsp}/cam${i}`], {stdio: ['ignore', 'ignore', 'pipe'], detached: true});
  let stderr = '';
  child.stderr.on('data', c => { stderr = (stderr + c).slice(-4096); });
  const exit = new Promise(r => child.once('close', (code, signal) => { if (!stopping) sourceErrors.push({stream: `cam${i}`, code, signal, stderr}); r(); }));
  return {child, exit};
});
// ---------- lightNVR fixture ----------
const {startFixture} = await import('../../contracts/fixture-server.mjs');
let server, watcher, cookie;
const segEvents = [], seenFiles = new Set();
const proc = [], tree = [], probes = [], prom = [];
try {
  for (let i = 0; i < streams; i++) {
    for (let attempt = 0; ; attempt++) {
      const ok = sh('ffprobe', ['-v', 'error', '-rtsp_transport', 'tcp', '-select_streams', 'v:0', '-show_entries', 'stream=codec_name', '-of', 'csv=p=0', `rtsp://127.0.0.1:${ports.rtsp}/cam${i}`]);
      if (ok === 'h264') break;
      if (attempt > 20) throw new Error(`fixture stream cam${i} never became readable`);
      await pause(250);
    }
  }
  server = await startFixture({binary, maxStreams: streams, seed: false, retain: true, extraConfig: configExtra});
  report.fixture_directory = server.directory;
  await server.json('/api/setup/status', {method: 'POST', body: {complete: true}});
  const login = await server.request('/api/auth/login', {principal: 'anonymous', method: 'POST', body: server.credentials.admin});
  cookie = (login.headers.getSetCookie?.() || []).map(c => c.split(';')[0]).join('; ');
  if (!login.ok || !cookie) throw new Error(`session login failed (${login.status})`);
  const sessionFetch = route => fetch(`${server.baseURL}${route}`, {headers: {Cookie: cookie}, signal: AbortSignal.timeout(10000)});

  let modelInFixture = null;
  if (useDetector) {
    modelInFixture = path.join(server.directory, 'models', path.basename(modelPath));
    await copyFile(modelPath, modelInFixture);
  }
  const recordingsDir = path.join(server.directory, 'recordings');
  const streamNames = Array.from({length: streams}, (_, i) => `Load${i}`);
  watcher = watch(recordingsDir, {recursive: true}, (event, file) => {
    if (!file || !/\.mp4$/i.test(file) || seenFiles.has(file)) return;
    const stream = file.split(path.sep).find(part => streamNames.includes(part)) || streamNames.find(n => file.includes(n));
    seenFiles.add(file);
    segEvents.push({stream, file, tMs: now(), wallMs: Date.now()});
  });
  for (const [i, name] of streamNames.entries()) {
    await server.json('/api/streams', {method: 'POST', body: {
      name, url: `rtsp://127.0.0.1:${ports.rtsp}/cam${i}`, enabled: true, streaming_enabled: false, record: true,
      record_audio: false, segment_duration: segment, fps, protocol: 0,
      detection_based_recording: useDetector, detection_model: modelInFixture || '', detection_threshold: workload.detector.threshold_percent,
      detection_interval: detectionInterval, pre_detection_buffer: 0, post_detection_buffer: 0, ...streamExtra,
    }});
  }
  report.phase = 'warmup'; await save();
  await pause(warmup * 1000);

  // ---------- measurement window ----------
  const logPath = path.join(server.directory, 'server.log');
  const logStart = (await stat(logPath)).size;
  const windowStart = now(), windowStartWall = Date.now();
  const promAt = async () => { const r = await sessionFetch('/api/metrics'); return {tMs: now(), parsed: parsePrometheus(await r.text())}; };
  const promStart = await promAt();
  report.phase = 'measuring'; await save();
  const deadline = windowStart + seconds * 1000;
  const loops = [
    (async () => { while (now() < deadline) { const s = await procSample(server.pid); if (s) proc.push(s); const parts = (await Promise.all([go2rtc.pid, ...publishers.map(p => p.child.pid)].map(treeCpuTicks))).filter(Boolean); if (parts.length) tree.push({tMs: now(), ticks: parts.reduce((a, b) => a + b.ticks, 0), processes: parts.reduce((a, b) => a + b.processes, 0)}); await pause(workload.sampling.proc_interval_ms); } })(),
    (async () => { while (now() < deadline) { const t0 = now(); const r = await sessionFetch('/api/recordings?limit=1&page=1'); await r.arrayBuffer(); probes.push({tMs: t0, ms: now() - t0, status: r.status}); await pause(workload.sampling.db_probe_interval_ms); } })(),
    (async () => { while (now() < deadline) { await pause(workload.sampling.prometheus_interval_ms); prom.push(await promAt()); } })(),
  ];
  await Promise.all(loops);
  const promEnd = await promAt();
  const windowEnd = now(), windowEndWall = Date.now();
  const logEnd = (await stat(logPath)).size;

  // ---------- analysis ----------
  const fh = await open(logPath, 'r');
  const buf = Buffer.alloc(logEnd - logStart);
  await fh.read(buf, 0, buf.length, logStart); await fh.close();
  const logWindow = buf.toString('utf8');
  const inferences = (logWindow.match(/sod_cnn_predict returned/g) || []).length;
  report.detector = useDetector ? {
    inferences_in_window: inferences,
    inferences_per_stream_per_second: Math.round(inferences / streams / seconds * 1000) / 1000,
    expected_per_stream_per_second: Math.round(1 / detectionInterval * 1000) / 1000,
    detections_in_window: (logWindow.match(/Valid detection \d+:/g) || []).length,
    source: 'count of "sod_cnn_predict returned" info lines in server.log during the window',
  } : null;
  report.log_errors_in_window = (logWindow.match(/\[ERROR\]/g) || []).length;

  // Intervals whose closing segment appears inside the window (predecessor may be in warmup).
  const endTimes = {};
  for (const e of segEvents) (endTimes[e.stream] ||= []).push(e.tMs);
  const all = [];
  for (const [stream, list] of Object.entries(endTimes)) {
    list.sort((a, b) => a - b);
    for (let i = 1; i < list.length; i++) if (list[i] >= windowStart && list[i] <= windowEnd) all.push({stream, ms: list[i] - list[i - 1]});
  }
  const nominal = segment * 1000;
  const intervalValues = all.map(x => x.ms);
  const okProbes = probes.filter(p => p.status === 200).map(p => p.ms);
  report.latency = {
    segment_interval_ms: summarize(intervalValues),
    segment_interval_excess_ms: summarize(intervalValues.map(v => v - nominal)),
    db_probe_ms: summarize(okProbes),
    db_probe_failures: probes.length - okProbes.length,
    per_stream_segment_interval_ms: Object.fromEntries(Object.keys(endTimes).sort().map(s => [s, summarize(all.filter(x => x.stream === s).map(x => x.ms))])),
  };

  const gapsStart = sumMetric(promStart.parsed, 'lightnvr_recording_gaps_total') ?? NaN;
  const gapsEnd = sumMetric(promEnd.parsed, 'lightnvr_recording_gaps_total') ?? NaN;
  const delta = name => (sumMetric(promEnd.parsed, name) ?? NaN) - (sumMetric(promStart.parsed, name) ?? NaN);

  // Stop recording before reading the catalog so the last segments finalize.
  for (const name of streamNames) await server.json(`/api/streams/${name}/recording`, {method: 'POST', body: {action: 'stop'}}).catch(() => {});
  await pause(3000);
  const rows = [];
  for (let page = 1; ; page++) {
    const r = await server.json(`/api/recordings?limit=1000&page=${page}&sort=id&order=asc`);
    rows.push(...r.recordings);
    if (r.recordings.length < 1000) break;
  }
  const toSec = v => (typeof v === 'number' ? v : Math.floor(Date.parse(v) / 1000));
  const catalogRows = rows.map(r => ({id: r.id, stream: r.stream, start_time: toSec(r.start_time), end_time: toSec(r.end_time)}))
    .filter(r => Number.isFinite(r.start_time) && Number.isFinite(r.end_time) && r.end_time > 0
      && r.start_time * 1000 >= windowStartWall - segment * 1000);
  const catalog = catalogGaps(catalogRows, GAP_THRESHOLD_SEC);
  report.gaps = {
    metrics_recording_gaps_total: gapsEnd - gapsStart,
    metrics_recording_gaps_by_stream_end: metricByLabel(promEnd.parsed, 'lightnvr_recording_gaps_total'),
    catalog_gaps_total: catalog.total, catalog_gaps: catalog.gaps,
    segment_interval_gaps: countIntervalGaps(intervalValues, nominal, GAP_THRESHOLD_SEC * 1000),
  };
  report.recording = {
    segments_created_in_window: segEvents.filter(e => e.tMs >= windowStart && e.tMs <= windowEnd).length,
    expected_segments_in_window: Math.floor(seconds / segment) * streams,
    metrics_segments_delta: delta('lightnvr_recording_segments_total'),
    metrics_bytes_written_delta: delta('lightnvr_recording_bytes_written'),
    frames_dropped_delta: delta('lightnvr_stream_frames_dropped'),
    reconnects_delta: delta('lightnvr_stream_reconnects_total'),
    catalog_rows_after_warmup: catalogRows.length,
    fps_samples: summarize(prom.flatMap(p => Object.values(metricByLabel(p.parsed, 'lightnvr_stream_fps')))),
    streams_up_min: Math.min(...prom.map(p => sumMetric(p.parsed, 'lightnvr_instance_streams_up') ?? NaN)),
  };
  const treeSec = tree.length > 1 ? (tree.at(-1).tMs - tree[0].tMs) / 1000 : NaN;
  report.resources = {
    lightnvr: resourceSummary(proc, clkTck),
    fixture_go2rtc_and_publishers: tree.length > 1 ? {cpu_percent_one_core_avg: Math.round((tree.at(-1).ticks - tree[0].ticks) / clkTck / treeSec * 100 * 1000) / 1000,
      processes: tree.at(-1).processes} : null,
    storage_bytes_end: Number(sh('du', ['-sb', recordingsDir])?.split(/\s+/)[0]),
    loadavg_at_end: os.loadavg(),
  };
  report.window = {start: new Date(windowStartWall).toISOString(), end: new Date(windowEndWall).toISOString(), seconds: Math.round((windowEnd - windowStart) / 10) / 100};
  report.samples = {proc: proc.map(s => ({...s, tMs: Math.round(s.tMs - windowStart)})),
    db_probe_ms: probes.map(p => Math.round(p.ms * 1000) / 1000),
    segment_events: segEvents.map(e => ({stream: e.stream, file: e.file, tMs: Math.round(e.tMs - windowStart)}))};

  const problems = [];
  if (report.recording.streams_up_min < streams) problems.push(`streams_up dropped to ${report.recording.streams_up_min}`);
  if (useDetector && report.detector.inferences_per_stream_per_second < 0.5 / detectionInterval) problems.push('detector ran below half its configured cadence');
  if (intervalValues.length < streams) problems.push('too few segment intervals measured');
  if (report.latency.db_probe_failures > 0) problems.push(`${report.latency.db_probe_failures} DB probe failures`);
  if (sourceErrors.length) problems.push(`fixture publisher exited: ${JSON.stringify(sourceErrors)}`);
  report.problems = problems;
  report.status = problems.length ? 'invalid' : 'completed';
  if (problems.length) process.exitCode = 1;
} catch (error) {
  report.status = 'failed'; report.error = String(error?.stack || error); process.exitCode = 1;
} finally {
  watcher?.close();
  try { await server?.stop({failed: report.status !== 'completed' || flag('--keep')}); } catch (e) { report.cleanup_error = String(e); }
  stopping = true;
  for (const p of publishers) { try { process.kill(-p.child.pid, 'SIGTERM'); } catch {} }
  await Promise.race([Promise.all(publishers.map(p => p.exit)), pause(3000)]);
  for (const p of publishers) { try { process.kill(-p.child.pid, 'SIGKILL'); } catch {} }
  try { process.kill(-go2rtc.pid, 'SIGTERM'); } catch {}
  const t = setTimeout(() => { try { process.kill(-go2rtc.pid, 'SIGKILL'); } catch {} }, 3000);
  await go2rtcExit; clearTimeout(t); await go2rtcLog.close();
  report.finished_at = new Date().toISOString();
  delete report.phase;
  await save();
}
console.log(JSON.stringify({status: report.status, output: path.relative(repoRoot, output), problems: report.problems, error: report.error,
  segment_interval_p95_ms: report.latency?.segment_interval_ms?.p95, db_probe_p95_ms: report.latency?.db_probe_ms?.p95,
  gaps: report.gaps && {metrics: report.gaps.metrics_recording_gaps_total, catalog: report.gaps.catalog_gaps_total, interval: report.gaps.segment_interval_gaps}}));
