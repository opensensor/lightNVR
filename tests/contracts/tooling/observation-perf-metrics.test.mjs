import test from 'node:test';
import assert from 'node:assert/strict';
import {
  percentile, summarize, parsePrometheus, sumMetric, metricByLabel, segmentIntervals,
  countIntervalGaps, catalogGaps, resourceSummary, relativeDifferencePct, repeatability,
  releaseGate, workloadFingerprint,
} from '../../performance/observations/metrics.mjs';

test('percentile interpolates linearly (type 7)', () => {
  assert.equal(percentile([1, 2, 3, 4, 5], 50), 3);
  assert.equal(percentile([1, 2, 3, 4], 50), 2.5);
  assert.equal(percentile([10, 20], 95), 19.5);
  assert.equal(percentile([7], 95), 7);
  assert.equal(percentile([], 95), null);
  assert.equal(percentile([3, 1, 2], 0), 1);
  assert.equal(percentile([3, 1, 2], 100), 3);
  assert.throws(() => percentile([1], 101));
});

test('summarize ignores non-finite values and reports null for empty input', () => {
  const s = summarize([1, 2, NaN, 3, Infinity, 4]);
  assert.equal(s.count, 4);
  assert.equal(s.min, 1);
  assert.equal(s.max, 4);
  assert.equal(s.mean, 2.5);
  assert.equal(summarize([]).p95, null);
});

test('parsePrometheus reads labels, values and skips comments', () => {
  const text = [
    '# HELP lightnvr_recording_gaps_total Recording gaps detected',
    '# TYPE lightnvr_recording_gaps_total counter',
    'lightnvr_recording_gaps_total{stream="Load0"} 0',
    'lightnvr_recording_gaps_total{stream="Load1"} 2',
    'lightnvr_instance_memory_rss_bytes 123456',
    'lightnvr_stream_error_total{stream="a b",type="decode"} 3',
  ].join('\n');
  const parsed = parsePrometheus(text);
  assert.equal(sumMetric(parsed, 'lightnvr_recording_gaps_total'), 2);
  assert.deepEqual(metricByLabel(parsed, 'lightnvr_recording_gaps_total'), {Load0: 0, Load1: 2});
  assert.equal(sumMetric(parsed, 'lightnvr_instance_memory_rss_bytes'), 123456);
  assert.equal(sumMetric(parsed, 'lightnvr_stream_error_total', l => l.type === 'decode'), 3);
  assert.equal(parsed.get('lightnvr_stream_error_total')[0].labels.stream, 'a b');
  assert.equal(sumMetric(parsed, 'missing'), null);
});

test('segmentIntervals orders creation times per stream', () => {
  const {perStream, all} = segmentIntervals([
    {stream: 'A', tMs: 20000}, {stream: 'A', tMs: 0}, {stream: 'A', tMs: 10000},
    {stream: 'B', tMs: 5000}, {stream: 'B', tMs: 15500},
  ]);
  assert.deepEqual(perStream, {A: [10000, 10000], B: [10500]});
  assert.equal(all.length, 3);
  assert.equal(countIntervalGaps(all, 10000, 400), 1);
  assert.equal(countIntervalGaps(all, 10000, 5000), 0);
});

test('catalogGaps finds per-stream discontinuities beyond the threshold', () => {
  const rows = [
    {id: 1, stream: 'A', start_time: 100, end_time: 110},
    {id: 2, stream: 'A', start_time: 110, end_time: 120},
    {id: 3, stream: 'A', start_time: 130, end_time: 140},
    {id: 4, stream: 'B', start_time: 100, end_time: 110},
    {id: 5, stream: 'B', start_time: 114, end_time: 124},
  ];
  const result = catalogGaps(rows, 5);
  assert.equal(result.total, 1);
  assert.deepEqual(result.perStream, {A: 1, B: 0});
  assert.deepEqual(result.gaps[0], {stream: 'A', after_id: 2, gap_seconds: 10});
});

test('resourceSummary computes CPU percent and write rate from deltas', () => {
  const samples = [
    {tMs: 0, cpuTicks: 0, rssKiB: 100, peakRssKiB: 100, writeBytes: 0, threads: 10, fds: 20},
    {tMs: 1000, cpuTicks: 50, rssKiB: 200, peakRssKiB: 210, writeBytes: 1000, threads: 12, fds: 21},
    {tMs: 2000, cpuTicks: 150, rssKiB: 150, peakRssKiB: 210, writeBytes: 3000, threads: 11, fds: 25},
  ];
  const r = resourceSummary(samples, 100);
  assert.equal(r.window_seconds, 2);
  assert.equal(r.cpu_percent_one_core_avg, 75);
  assert.equal(r.cpu_percent_one_core.max, 100);
  assert.equal(r.rss_high_water_kib, 210);
  assert.equal(r.disk_write_bytes, 3000);
  assert.equal(r.disk_write_bytes_per_sec, 1500);
  assert.equal(r.threads_max, 12);
  assert.equal(r.fds_max, 25);
  assert.equal(resourceSummary([samples[0]]), null);
});

test('relativeDifferencePct is symmetric', () => {
  assert.equal(relativeDifferencePct(98, 102), 4);
  assert.equal(relativeDifferencePct(102, 98), 4);
  assert.equal(relativeDifferencePct(0, 0), 0);
  assert.equal(relativeDifferencePct(null, 1), null);
});

const run = (seg, db, gaps = 0) => ({
  latency: {segment_interval_ms: {p95: seg}, db_probe_ms: {p95: db}},
  gaps: {metrics_recording_gaps_total: gaps, catalog_gaps_total: gaps, segment_interval_gaps: gaps},
});

test('repeatability passes only when every headline p95 differs by less than the limit', () => {
  assert.equal(repeatability(run(10000, 2.0), run(10050, 2.02)).pass, true);
  const fail = repeatability(run(10000, 2.0), run(10050, 2.2));
  assert.equal(fail.pass, false);
  assert.equal(fail.checks.find(c => c.key === 'latency.db_probe_ms.p95').pass, false);
  assert.equal(repeatability(run(10000, null), run(10000, null)).pass, false);
});

test('releaseGate enforces +5% p95 and no new gaps', () => {
  assert.equal(releaseGate(run(10000, 2.0), run(10400, 2.1)).pass, true);
  assert.equal(releaseGate(run(10000, 2.0), run(10600, 2.0)).pass, false);
  const gapped = releaseGate(run(10000, 2.0), run(10000, 2.0, 1));
  assert.equal(gapped.pass, false);
  assert.ok(gapped.checks.filter(c => !c.pass).every(c => c.key.startsWith('gaps.')));
});

test('workloadFingerprint ignores timing but not workload shape', () => {
  const base = {workload: {streams: 4, width: 1280, height: 720, fps: 15, gop: 15, segment_seconds: 10,
    duration_seconds: 300, detector: {model_sha256: 'x', interval_seconds: 1}, source_sha256: 's'},
  host: {cpu_model: 'cpu', logical_cores: 32, storage: {device: '/dev/x'}}, latency: {a: 1}};
  const other = structuredClone(base); other.latency = {a: 2};
  assert.equal(workloadFingerprint(base), workloadFingerprint(other));
  other.workload.streams = 8;
  assert.notEqual(workloadFingerprint(base), workloadFingerprint(other));
});
