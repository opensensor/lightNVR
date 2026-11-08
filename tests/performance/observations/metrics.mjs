/**
 * Pure aggregation helpers for the INT_01 observation performance workload.
 * No I/O here: everything is unit-tested by
 * tests/contracts/tooling/observation-perf-metrics.test.mjs.
 */

/** Linear-interpolated percentile (Hyndman–Fan type 7, same as numpy default). */
export function percentile(values, p) {
  if (!Array.isArray(values) || values.length === 0) return null;
  if (!(p >= 0 && p <= 100)) throw new Error(`percentile out of range: ${p}`);
  const sorted = values.filter(Number.isFinite).sort((a, b) => a - b);
  if (sorted.length === 0) return null;
  const rank = (p / 100) * (sorted.length - 1);
  const lo = Math.floor(rank), hi = Math.ceil(rank);
  return sorted[lo] + (sorted[hi] - sorted[lo]) * (rank - lo);
}

/** Distribution summary used for every latency/interval series in results.json. */
export function summarize(values) {
  const finite = (values || []).filter(Number.isFinite);
  if (finite.length === 0) return {count: 0, min: null, p50: null, p95: null, p99: null, max: null, mean: null};
  const round = v => Math.round(v * 1000) / 1000;
  return {
    count: finite.length,
    min: round(Math.min(...finite)),
    p50: round(percentile(finite, 50)),
    p95: round(percentile(finite, 95)),
    p99: round(percentile(finite, 99)),
    max: round(Math.max(...finite)),
    mean: round(finite.reduce((a, b) => a + b, 0) / finite.length),
  };
}

/** Parse Prometheus text exposition (the format served by GET /api/metrics). */
export function parsePrometheus(text) {
  const out = new Map();
  for (const raw of String(text).split('\n')) {
    const line = raw.trim();
    if (!line || line.startsWith('#')) continue;
    const m = line.match(/^([a-zA-Z_:][a-zA-Z0-9_:]*)(\{([^}]*)\})?\s+(\S+)/);
    if (!m) continue;
    const labels = {};
    if (m[3]) for (const pair of m[3].matchAll(/([a-zA-Z_][a-zA-Z0-9_]*)="((?:[^"\\]|\\.)*)"/g)) labels[pair[1]] = pair[2];
    const value = Number(m[4]);
    if (!out.has(m[1])) out.set(m[1], []);
    out.get(m[1]).push({labels, value});
  }
  return out;
}

/** Sum a metric across label sets, optionally filtered by a label predicate. */
export function sumMetric(parsed, name, filter = () => true) {
  const rows = parsed.get(name);
  if (!rows) return null;
  return rows.filter(r => filter(r.labels)).reduce((a, r) => a + r.value, 0);
}

/** Per-label-value map of one metric, e.g. {Load0: 0, Load1: 2} for recording gaps by stream. */
export function metricByLabel(parsed, name, label = 'stream') {
  const out = {};
  for (const r of parsed.get(name) || []) out[r.labels[label]] = r.value;
  return out;
}

/**
 * Interval between consecutive segment-file creations for each stream.
 * events: [{stream, tMs}] (tMs = monotonic first-seen time of a new segment file).
 * The first segment of each stream has no predecessor and yields no interval.
 */
export function segmentIntervals(events) {
  const byStream = new Map();
  for (const e of events) {
    if (!byStream.has(e.stream)) byStream.set(e.stream, []);
    byStream.get(e.stream).push(e.tMs);
  }
  const perStream = {}, all = [];
  for (const [stream, times] of byStream) {
    times.sort((a, b) => a - b);
    const intervals = [];
    for (let i = 1; i < times.length; i++) intervals.push(times[i] - times[i - 1]);
    perStream[stream] = intervals;
    all.push(...intervals);
  }
  return {perStream, all};
}

/** Count intervals that exceed the nominal segment length by more than thresholdMs. */
export function countIntervalGaps(intervalsMs, nominalMs, thresholdMs) {
  return intervalsMs.filter(v => v > nominalMs + thresholdMs).length;
}

/**
 * Gaps in the persisted recording catalog: consecutive rows of one stream whose
 * start_time is more than thresholdSec after the previous row's end_time.
 * rows: [{stream, start_time, end_time}] with unix-second timestamps.
 */
export function catalogGaps(rows, thresholdSec) {
  const byStream = new Map();
  for (const r of rows) {
    if (!byStream.has(r.stream)) byStream.set(r.stream, []);
    byStream.get(r.stream).push(r);
  }
  const perStream = {}, gaps = [];
  for (const [stream, list] of byStream) {
    list.sort((a, b) => a.start_time - b.start_time);
    let count = 0;
    for (let i = 1; i < list.length; i++) {
      const gap = list[i].start_time - list[i - 1].end_time;
      if (gap > thresholdSec) { count++; gaps.push({stream, after_id: list[i - 1].id, gap_seconds: gap}); }
    }
    perStream[stream] = count;
  }
  return {total: gaps.length, perStream, gaps};
}

/**
 * Process resource rates from /proc samples ordered by time.
 * sample: {tMs, cpuTicks, rssKiB, peakRssKiB, writeBytes, threads, fds}
 */
export function resourceSummary(samples, clockTicksPerSec = 100) {
  if (samples.length < 2) return null;
  const first = samples[0], last = samples[samples.length - 1];
  const seconds = (last.tMs - first.tMs) / 1000;
  const cpuPct = [];
  for (let i = 1; i < samples.length; i++) {
    const dt = (samples[i].tMs - samples[i - 1].tMs) / 1000;
    if (dt > 0) cpuPct.push(((samples[i].cpuTicks - samples[i - 1].cpuTicks) / clockTicksPerSec) / dt * 100);
  }
  const round = v => (v == null ? null : Math.round(v * 1000) / 1000);
  return {
    window_seconds: round(seconds),
    cpu_percent_one_core: summarize(cpuPct),
    cpu_percent_one_core_avg: round(((last.cpuTicks - first.cpuTicks) / clockTicksPerSec) / seconds * 100),
    rss_kib: summarize(samples.map(s => s.rssKiB)),
    rss_high_water_kib: Math.max(...samples.map(s => s.peakRssKiB).filter(Number.isFinite)),
    disk_write_bytes: last.writeBytes - first.writeBytes,
    disk_write_bytes_per_sec: round((last.writeBytes - first.writeBytes) / seconds),
    threads_max: Math.max(...samples.map(s => s.threads).filter(Number.isFinite)),
    fds_max: Math.max(...samples.map(s => s.fds).filter(Number.isFinite)),
  };
}

/** Relative difference |a-b| / mean(a,b) in percent; null if undefined. */
export function relativeDifferencePct(a, b) {
  if (!Number.isFinite(a) || !Number.isFinite(b)) return null;
  const mean = (a + b) / 2;
  if (mean === 0) return a === b ? 0 : null;
  return Math.abs(a - b) / Math.abs(mean) * 100;
}

/** Fetch a dotted path such as "latency.segment_interval_ms.p95". */
export function pick(obj, dotted) {
  return dotted.split('.').reduce((o, k) => (o == null ? undefined : o[k]), obj);
}

/** Headline p95 series that C4 repeatability and the X2 gate are evaluated on. */
export const HEADLINE_P95 = Object.freeze([
  'latency.segment_interval_ms.p95',
  'latency.db_probe_ms.p95',
]);

/**
 * C4 repeatability: two runs on the same host and mode must have p95 variance
 * under maxPct on every headline series.
 */
export function repeatability(runA, runB, {maxPct = 2, keys = HEADLINE_P95} = {}) {
  const checks = keys.map(key => {
    const a = pick(runA, key), b = pick(runB, key);
    const variancePct = relativeDifferencePct(a, b);
    return {key, a, b, variance_pct: variancePct == null ? null : Math.round(variancePct * 1000) / 1000,
      pass: variancePct != null && variancePct < maxPct};
  });
  return {max_pct: maxPct, pass: checks.every(c => c.pass), checks};
}

/**
 * X2 gate (PRD §8): candidate (evidence enabled) vs baseline (disabled), same
 * host/workload. No additional recording gaps; each headline p95 rises by at
 * most maxIncreasePct.
 */
export function releaseGate(baseline, candidate, {maxIncreasePct = 5, keys = HEADLINE_P95} = {}) {
  const checks = keys.map(key => {
    const b = pick(baseline, key), c = pick(candidate, key);
    const raw = Number.isFinite(b) && Number.isFinite(c) && b > 0 ? (c - b) / b * 100 : null;
    const increasePct = raw == null ? null : Math.round(raw * 1000) / 1000; // avoid 5.000000001 > 5
    return {key, baseline: b, candidate: c, increase_pct: increasePct, pass: increasePct != null && increasePct <= maxIncreasePct};
  });
  for (const key of ['gaps.metrics_recording_gaps_total', 'gaps.catalog_gaps_total', 'gaps.segment_interval_gaps']) {
    const b = pick(baseline, key), c = pick(candidate, key);
    checks.push({key, baseline: b, candidate: c, pass: Number.isFinite(b) && Number.isFinite(c) && c <= b});
  }
  return {max_increase_pct: maxIncreasePct, pass: checks.every(c => c.pass), checks};
}

/** Workloads must match for a comparison to be meaningful. */
export function workloadFingerprint(result) {
  const w = result.workload || {};
  return JSON.stringify({
    streams: w.streams, width: w.width, height: w.height, fps: w.fps, gop: w.gop,
    segment_seconds: w.segment_seconds, duration_seconds: w.duration_seconds,
    detector: w.detector && {model_sha256: w.detector.model_sha256, interval_seconds: w.detector.interval_seconds},
    source_sha256: w.source_sha256, cpu: result.host?.cpu_model, cores: result.host?.logical_cores,
    storage_device: result.host?.storage?.device,
  });
}

// ---------- X2: evidence-enabled saturation gate (A11) ----------

/** Prometheus histogram {buckets: [{le, count}] cumulative and ascending, count, sum}; null when absent. */
export function histogram(parsed, name) {
  const rows = (parsed.get(`${name}_bucket`) || [])
    .map(r => ({le: r.labels.le === '+Inf' ? Infinity : Number(r.labels.le), count: r.value}))
    .filter(b => !Number.isNaN(b.le)).sort((a, b) => a.le - b.le);
  if (rows.length === 0) return null;
  return {buckets: rows, count: sumMetric(parsed, `${name}_count`) ?? rows.at(-1).count, sum: sumMetric(parsed, `${name}_sum`)};
}

/** end - start, bucket by bucket (a measurement window of a cumulative histogram). */
export function histogramDelta(start, end) {
  if (!end) return null;
  if (!start) return end;
  const before = new Map(start.buckets.map(b => [b.le, b.count]));
  return {
    buckets: end.buckets.map(b => ({le: b.le, count: b.count - (before.get(b.le) ?? 0)})),
    count: end.count - start.count,
    sum: Number.isFinite(end.sum) && Number.isFinite(start.sum) ? end.sum - start.sum : null,
  };
}

/** Upper bound of the first bucket holding quantile q (0..1): a conservative estimate. */
export function histogramUpperQuantile(h, q) {
  if (!h || !(h.count > 0)) return null;
  const target = q * h.count;
  for (const b of h.buckets) if (b.count >= target) return b.le;
  return Infinity;
}

const DROP_REASONS = ['queue_full', 'budget_exhausted', 'disabled', 'untracked'];
const FAILURE_REASONS = ['encode_failed', 'write_failed', 'db_error', 'disabled', 'interrupted'];
const SUBMIT_HISTOGRAM = 'lightnvr_observation_submit_latency_seconds';

/**
 * L10 observation families over a measurement window: counter deltas
 * (start..end), per-process-run high-water gauges and configured limits (end),
 * and gauge maxima over every scrape. Null when the binary has no L10 metrics.
 */
export function observationFeatureSummary(start, end, samples = []) {
  if (!end.get('lightnvr_observation_accepted_total')) return null;
  const at = (p, name, filter) => sumMetric(p, name, filter);
  const delta = (name, filter) => (at(end, name, filter) ?? NaN) - (at(start, name, filter) ?? 0);
  const all = [start, ...samples, end];
  const maxOf = name => { const v = all.map(p => at(p, name)).filter(Number.isFinite); return v.length ? Math.max(...v) : null; };
  const scoped = (name, scope) => at(end, name, l => l.scope === scope);
  const latency = histogramDelta(histogram(start, SUBMIT_HISTOGRAM), histogram(end, SUBMIT_HISTOGRAM));
  const round = v => (Number.isFinite(v) ? Math.round(v * 1e9) / 1e9 : v ?? null);
  const leText = le => (Number.isFinite(le) ? le : '+Inf');
  return {
    accepted: delta('lightnvr_observation_accepted_total'),
    dropped: Object.fromEntries(DROP_REASONS.map(r => [r, delta('lightnvr_observation_dropped_total', l => l.reason === r)])),
    ready: delta('lightnvr_observation_evidence_ready_total'),
    failures: Object.fromEntries(FAILURE_REASONS.map(r => [r, delta('lightnvr_observation_evidence_failures_total', l => l.reason === r)])),
    unavailable_submitted: delta('lightnvr_observation_evidence_unavailable_total'),
    bytes_written: delta('lightnvr_observation_evidence_bytes_written_total'),
    high_water_items: at(end, 'lightnvr_observation_evidence_high_water_items'),
    high_water_bytes: at(end, 'lightnvr_observation_evidence_high_water_bytes'),
    camera_high_water_items: at(end, 'lightnvr_observation_evidence_camera_high_water_items'),
    limits: {
      items: scoped('lightnvr_observation_evidence_limit_items', 'global'),
      camera_items: scoped('lightnvr_observation_evidence_limit_items', 'camera'),
      bytes: scoped('lightnvr_observation_evidence_limit_bytes', 'global'),
      camera_bytes: scoped('lightnvr_observation_evidence_limit_bytes', 'camera'),
      discard_bytes: scoped('lightnvr_observation_evidence_limit_bytes', 'discard'),
    },
    discards: {
      deferred: delta('lightnvr_observation_evidence_discards_total', l => l.mode === 'deferred'),
      inline: delta('lightnvr_observation_evidence_discards_total', l => l.mode === 'inline'),
    },
    discard_pending_bytes_max: maxOf('lightnvr_observation_evidence_discard_pending_bytes'),
    backlog_items_max: maxOf('lightnvr_observation_evidence_backlog_items'),
    backlog_bytes_max: maxOf('lightnvr_observation_evidence_backlog_bytes'),
    drop_ring_depth_max: maxOf('lightnvr_observation_evidence_drop_ring_depth'),
    budget_exhausted_cameras_max: maxOf('lightnvr_observation_evidence_budget_exhausted_cameras'),
    relay_lag_seconds_max: maxOf('lightnvr_observation_relay_lag_seconds'),
    queue_allocated_end: at(end, 'lightnvr_observation_evidence_queue_allocated'),
    worker_running_end: at(end, 'lightnvr_observation_evidence_worker_running'),
    retained_bytes_by_camera_end: metricByLabel(end, 'lightnvr_observation_evidence_retained_bytes', 'camera_uuid'),
    budget_bytes_by_camera_end: metricByLabel(end, 'lightnvr_observation_evidence_budget_bytes', 'camera_uuid'),
    history_observations_end: at(end, 'lightnvr_observation_history_observations'),
    publications_end: metricByLabel(end, 'lightnvr_observation_publications', 'state'),
    submit_latency: latency && {
      count: latency.count,
      sum_seconds: round(latency.sum),
      mean_seconds: latency.count > 0 && Number.isFinite(latency.sum) ? round(latency.sum / latency.count) : null,
      max_seconds: at(end, 'lightnvr_observation_submit_latency_max_seconds'),
      p50_upper_seconds: leText(histogramUpperQuantile(latency, 0.5)),
      p99_upper_seconds: leText(histogramUpperQuantile(latency, 0.99)),
      p999_upper_seconds: leText(histogramUpperQuantile(latency, 0.999)),
      buckets: latency.buckets.map(b => ({le: leText(b.le), count: b.count})),
      max_note: 'max_seconds is the per-process-run maximum (includes warmup); buckets are the window delta',
    },
  };
}

/** Contract §4.3 bounded reasons, by asset state. */
export const ASSET_REASONS = Object.freeze({
  unavailable: ['queue_full', 'budget_exhausted', 'encode_failed', 'write_failed', 'interrupted', 'no_matching_media', 'disabled'],
  expired: ['retention_expired', 'deleted'],
});

/** rows: [{state, reason, n}] from observation_assets GROUP BY state, reason. */
export function assetReasonSummary(rows) {
  const byStateReason = {};
  let withoutReason = 0;
  const unknown = new Set();
  for (const {state, reason, n} of rows) {
    byStateReason[reason ? `${state}:${reason}` : state] = (byStateReason[reason ? `${state}:${reason}` : state] || 0) + n;
    if (ASSET_REASONS[state]) {
      if (!reason) withoutReason += n;
      else if (!ASSET_REASONS[state].includes(reason)) unknown.add(reason);
    }
  }
  return {by_state_reason: byStateReason, unavailable_or_expired_without_reason: withoutReason, unknown_reasons: [...unknown].sort()};
}

/**
 * X2 A11 evidence checks on an enabled run (the recording-path checks are
 * releaseGate): detections and evidence really happened, queue limits held,
 * every drop carries a bounded reason (none untracked, the DB holds at least
 * the window's drops by reason), no post-admission failures, and the producer
 * submit never exceeded maxSubmitSeconds (it never waits on I/O).
 * requireSaturation additionally demands that admission refused work.
 */
export function evidenceGate(baseline, candidate, {requireSaturation = false, maxSubmitSeconds = 0.001} = {}) {
  const f = candidate.feature_metrics || {};
  const lim = f.limits || {};
  const assets = candidate.feature_db?.assets || {by_state_reason: {}};
  const le = (v, limit) => Number.isFinite(v) && Number.isFinite(limit) && v <= limit;
  const checks = [];
  const add = (key, pass, value, limit) => checks.push({key, value: value ?? null, limit: limit ?? null, pass: Boolean(pass)});
  const b = baseline.feature_metrics;
  add('baseline.feature_inactive', b && b.accepted === 0 && !b.queue_allocated_end, b && {accepted: b.accepted, queue_allocated: b.queue_allocated_end});
  add('evidence.detections', candidate.detector?.detections_in_window > 0, candidate.detector?.detections_in_window);
  add('evidence.ready', f.accepted > 0 && f.ready > 0, {accepted: f.accepted, ready: f.ready});
  add('evidence.db_observations', candidate.feature_db?.observations > 0, candidate.feature_db?.observations);
  add('limits.items', le(f.high_water_items, lim.items) && le(f.backlog_items_max, lim.items), {high_water: f.high_water_items, backlog_max: f.backlog_items_max}, lim.items);
  add('limits.bytes', le(f.high_water_bytes, lim.bytes) && le(f.backlog_bytes_max ?? 0, lim.bytes), {high_water: f.high_water_bytes, backlog_max: f.backlog_bytes_max}, lim.bytes);
  add('limits.camera_items', le(f.camera_high_water_items, lim.camera_items), f.camera_high_water_items, lim.camera_items);
  add('drops.untracked', f.dropped?.untracked === 0, f.dropped?.untracked, 0);
  const recorded = ['queue_full', 'budget_exhausted'].every(r => (assets.by_state_reason[`unavailable:${r}`] || 0) >= (f.dropped?.[r] ?? Infinity));
  add('drops.reasons_recorded', recorded && assets.unavailable_or_expired_without_reason === 0 && (assets.unknown_reasons || []).length === 0,
    {window_drops: f.dropped, db: assets.by_state_reason, without_reason: assets.unavailable_or_expired_without_reason, unknown: assets.unknown_reasons});
  const failed = f.failures ? f.failures.encode_failed + f.failures.write_failed + f.failures.db_error : NaN;
  add('evidence.failures', failed === 0, f.failures, 0);
  add('submit.count', f.submit_latency?.count > 0, f.submit_latency?.count);
  add('submit.max_seconds', Number.isFinite(f.submit_latency?.max_seconds) && f.submit_latency.max_seconds < maxSubmitSeconds,
    f.submit_latency?.max_seconds, maxSubmitSeconds);
  if (requireSaturation) add('saturation.queue_full_drops', f.dropped?.queue_full > 0, f.dropped?.queue_full);
  return {require_saturation: requireSaturation, max_submit_seconds: maxSubmitSeconds, pass: checks.every(c => c.pass), checks};
}
