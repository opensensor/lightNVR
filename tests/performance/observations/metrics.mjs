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
