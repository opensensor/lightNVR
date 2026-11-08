import test from 'node:test';
import assert from 'node:assert/strict';
import {
  percentile, summarize, parsePrometheus, sumMetric, metricByLabel, segmentIntervals,
  countIntervalGaps, catalogGaps, resourceSummary, relativeDifferencePct, repeatability,
  releaseGate, workloadFingerprint, histogram, histogramDelta, histogramUpperQuantile,
  observationFeatureSummary, assetReasonSummary, evidenceGate, pooledComparison,
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

// ---------- X2: evidence-enabled saturation gate ----------

const featureText = ({accepted = 0, queueFull = 0, untracked = 0, ready = 0, writeFailed = 0, hwItems = 0, hwBytes = 0,
  camHw = 0, backlog = 0, buckets = [0, 0, 0, 0, 0, 0, 0, 0, 0, 0], sum = 0, max = 0, allocated = 0} = {}) => {
  const les = ['0.000001', '0.000005', '0.00001', '0.00005', '0.0001', '0.00025', '0.0005', '0.001', '0.01', '+Inf'];
  let cumulative = 0;
  return [
    `lightnvr_observation_accepted_total ${accepted}`,
    `lightnvr_observation_dropped_total{reason="queue_full"} ${queueFull}`,
    'lightnvr_observation_dropped_total{reason="budget_exhausted"} 0',
    'lightnvr_observation_dropped_total{reason="disabled"} 0',
    `lightnvr_observation_dropped_total{reason="untracked"} ${untracked}`,
    `lightnvr_observation_evidence_ready_total ${ready}`,
    'lightnvr_observation_evidence_failures_total{reason="encode_failed"} 0',
    `lightnvr_observation_evidence_failures_total{reason="write_failed"} ${writeFailed}`,
    'lightnvr_observation_evidence_failures_total{reason="db_error"} 0',
    'lightnvr_observation_evidence_failures_total{reason="interrupted"} 0',
    'lightnvr_observation_evidence_bytes_written_total 1000',
    `lightnvr_observation_evidence_backlog_items ${backlog}`,
    `lightnvr_observation_evidence_backlog_bytes ${backlog * 100}`,
    'lightnvr_observation_evidence_drop_ring_depth 0',
    `lightnvr_observation_evidence_high_water_items ${hwItems}`,
    `lightnvr_observation_evidence_high_water_bytes ${hwBytes}`,
    `lightnvr_observation_evidence_camera_high_water_items ${camHw}`,
    'lightnvr_observation_evidence_limit_items{scope="global"} 64',
    'lightnvr_observation_evidence_limit_items{scope="camera"} 8',
    'lightnvr_observation_evidence_limit_bytes{scope="global"} 4194304',
    'lightnvr_observation_evidence_limit_bytes{scope="camera"} 4194304',
    `lightnvr_observation_evidence_discards_total{mode="deferred"} ${queueFull}`,
    'lightnvr_observation_evidence_discards_total{mode="inline"} 0',
    'lightnvr_observation_evidence_discard_pending_bytes 0',
    'lightnvr_observation_evidence_limit_bytes{scope="discard"} 33554432',
    `lightnvr_observation_evidence_queue_allocated ${allocated}`,
    `lightnvr_observation_evidence_worker_running ${allocated}`,
    ...les.map((le, i) => `lightnvr_observation_submit_latency_seconds_bucket{le="${le}"} ${(cumulative += buckets[i])}`),
    `lightnvr_observation_submit_latency_seconds_sum ${sum}`,
    `lightnvr_observation_submit_latency_seconds_count ${cumulative}`,
    `lightnvr_observation_submit_latency_max_seconds ${max}`,
  ].join('\n');
};

test('histogram helpers read cumulative buckets, subtract windows and bound quantiles', () => {
  const start = histogram(parsePrometheus(featureText({buckets: [1, 1, 0, 0, 0, 0, 0, 0, 0, 0], sum: 0.000004})),
    'lightnvr_observation_submit_latency_seconds');
  const end = histogram(parsePrometheus(featureText({buckets: [5, 90, 4, 1, 0, 0, 0, 0, 0, 0], sum: 0.0004})),
    'lightnvr_observation_submit_latency_seconds');
  assert.equal(end.count, 100);
  assert.equal(end.buckets.at(-1).le, Infinity);
  const delta = histogramDelta(start, end);
  assert.equal(delta.count, 98);
  assert.equal(delta.buckets[0].count, 4);
  assert.ok(Math.abs(delta.sum - 0.000396) < 1e-12);
  assert.equal(histogramUpperQuantile(end, 0.5), 0.000005);
  assert.equal(histogramUpperQuantile(end, 0.99), 0.00001);
  assert.equal(histogramUpperQuantile(end, 1), 0.00005);
  assert.equal(histogramUpperQuantile(null, 0.5), null);
  assert.equal(histogram(parsePrometheus('x 1'), 'lightnvr_observation_submit_latency_seconds'), null);
});

test('observationFeatureSummary reports window deltas, run high-water marks and sample maxima', () => {
  const start = parsePrometheus(featureText({accepted: 2, queueFull: 1, ready: 2, allocated: 1}));
  const mid = parsePrometheus(featureText({accepted: 5, queueFull: 3, ready: 4, backlog: 3, allocated: 1}));
  const end = parsePrometheus(featureText({accepted: 12, queueFull: 9, ready: 12, hwItems: 3, hwBytes: 300, camHw: 2,
    backlog: 1, buckets: [0, 30, 0, 0, 0, 0, 0, 0, 0, 0], sum: 0.0001, max: 0.00004, allocated: 1}));
  const s = observationFeatureSummary(start, end, [mid]);
  assert.equal(s.accepted, 10);
  assert.equal(s.ready, 10);
  assert.deepEqual(s.dropped, {queue_full: 8, budget_exhausted: 0, disabled: 0, untracked: 0});
  assert.equal(s.high_water_items, 3);
  assert.equal(s.high_water_bytes, 300);
  assert.equal(s.camera_high_water_items, 2);
  assert.equal(s.backlog_items_max, 3);
  assert.deepEqual(s.limits, {items: 64, camera_items: 8, bytes: 4194304, camera_bytes: 4194304, discard_bytes: 33554432});
  assert.deepEqual(s.discards, {deferred: 8, inline: 0});
  assert.equal(s.discard_pending_bytes_max, 0);
  assert.equal(s.submit_latency.count, 30);
  assert.equal(s.submit_latency.max_seconds, 0.00004);
  assert.equal(s.submit_latency.p99_upper_seconds, 0.000005);
  assert.equal(s.queue_allocated_end, 1);
  assert.equal(observationFeatureSummary(parsePrometheus('x 1'), parsePrometheus('x 1'), []), null);
});

test('assetReasonSummary flags unavailable assets without a bounded reason', () => {
  const ok = assetReasonSummary([{state: 'ready', reason: null, n: 5}, {state: 'unavailable', reason: 'queue_full', n: 3},
    {state: 'expired', reason: 'retention_expired', n: 2}]);
  assert.deepEqual(ok.by_state_reason, {ready: 5, 'unavailable:queue_full': 3, 'expired:retention_expired': 2});
  assert.equal(ok.unavailable_or_expired_without_reason, 0);
  assert.deepEqual(ok.unknown_reasons, []);
  const bad = assetReasonSummary([{state: 'unavailable', reason: '', n: 1}, {state: 'unavailable', reason: 'mystery', n: 1}]);
  assert.equal(bad.unavailable_or_expired_without_reason, 1);
  assert.deepEqual(bad.unknown_reasons, ['mystery']);
});

const enabledRun = (over = {}) => ({
  mode: 'enabled',
  detector: {detections_in_window: 40},
  feature_db: {observations: 50, assets: assetReasonSummary([{state: 'ready', reason: null, n: 30},
    {state: 'unavailable', reason: 'queue_full', n: 9}])},
  feature_metrics: observationFeatureSummary(parsePrometheus(featureText({allocated: 1})), parsePrometheus(featureText({
    accepted: 30, queueFull: 9, ready: 30, hwItems: 1, hwBytes: 2800000, camHw: 1,
    buckets: [0, 35, 4, 0, 0, 0, 0, 0, 0, 0], sum: 0.0001, max: 0.00002, allocated: 1})), []),
  ...over,
});
const disabledRun = () => ({mode: 'disabled', feature_metrics: observationFeatureSummary(parsePrometheus(featureText()), parsePrometheus(featureText()), [])});

test('evidenceGate passes a saturated run whose limits held and whose drops were recorded', () => {
  const result = evidenceGate(disabledRun(), enabledRun(), {requireSaturation: true});
  assert.equal(result.pass, true, JSON.stringify(result.checks.filter(c => !c.pass)));
});

test('evidenceGate fails on each violated A11 property', () => {
  const failing = (run, opts, key) => {
    const result = evidenceGate(disabledRun(), run, opts);
    assert.equal(result.pass, false, key);
    assert.ok(result.checks.some(c => c.key === key && !c.pass), `${key}: ${JSON.stringify(result.checks)}`);
  };
  const fm = over => observationFeatureSummary(parsePrometheus(featureText({allocated: 1})),
    parsePrometheus(featureText({accepted: 30, queueFull: 9, ready: 30, hwItems: 1, hwBytes: 2800000, camHw: 1,
      buckets: [0, 39, 0, 0, 0, 0, 0, 0, 0, 0], max: 0.00002, allocated: 1, ...over})), []);
  failing(enabledRun({detector: {detections_in_window: 0}}), {}, 'evidence.detections');
  failing(enabledRun({feature_metrics: fm({accepted: 0, ready: 0})}), {}, 'evidence.ready');
  failing(enabledRun({feature_db: {observations: 0, assets: assetReasonSummary([])}}), {}, 'evidence.db_observations');
  failing(enabledRun({feature_metrics: fm({hwItems: 65})}), {}, 'limits.items');
  failing(enabledRun({feature_metrics: fm({hwBytes: 4194305})}), {}, 'limits.bytes');
  failing(enabledRun({feature_metrics: fm({camHw: 9})}), {}, 'limits.camera_items');
  failing(enabledRun({feature_metrics: fm({untracked: 1})}), {}, 'drops.untracked');
  failing(enabledRun({feature_metrics: fm({writeFailed: 1})}), {}, 'evidence.failures');
  failing(enabledRun({feature_metrics: fm({max: 0.0011})}), {}, 'submit.max_seconds');
  failing(enabledRun({feature_metrics: fm({buckets: [0, 0, 0, 0, 0, 0, 0, 0, 0, 0]})}), {}, 'submit.count');
  failing(enabledRun({feature_db: {observations: 50, assets: assetReasonSummary([{state: 'unavailable', reason: 'queue_full', n: 2}])}}),
    {}, 'drops.reasons_recorded');
  failing(enabledRun({feature_db: {observations: 50, assets: assetReasonSummary([{state: 'unavailable', reason: 'queue_full', n: 9},
    {state: 'unavailable', reason: null, n: 1}])}}), {}, 'drops.reasons_recorded');
  failing(enabledRun({feature_metrics: fm({queueFull: 0})}), {requireSaturation: true}, 'saturation.queue_full_drops');
  failing(enabledRun(), {maxSubmitSeconds: 0.00001}, 'submit.max_seconds');
  const active = disabledRun();
  active.feature_metrics.accepted = 3;
  assert.equal(evidenceGate(active, enabledRun()).checks.find(c => c.key === 'baseline.feature_inactive').pass, false);
});

test('pooledComparison pools raw samples per mode before taking percentiles', () => {
  const runs = values => values.map(v => ({samples: {db_probe_ms: v}}));
  const result = pooledComparison(runs([[1, 2, 3, 4], [5, 6, 7, 8]]), runs([[2, 3, 4, 5], [6, 7, 8, 9]]), 'db_probe_ms', 50);
  assert.equal(result.baseline.count, 8);
  assert.equal(result.baseline.p50, 4.5);
  assert.equal(result.candidate.p50, 5.5);
  assert.equal(result.increase_pct, 22.222);
  assert.equal(result.pass, false);
  assert.equal(pooledComparison(runs([[10, 10]]), runs([[10.4, 10.4]]), 'db_probe_ms', 95).pass, true);
});
