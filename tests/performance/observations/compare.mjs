#!/usr/bin/env node
/**
 * Compare observation workload results.
 *
 *   node compare.mjs repeat RUN_A.json RUN_B.json      # C4: same mode, p95 variance < 2%
 *   node compare.mjs gate BASELINE.json CANDIDATE.json # X2: no new gaps, p95 +<=5%
 *     [--max-pct N] [--require-saturation] [--max-submit-seconds S] [--advisory KEY]...
 *   node compare.mjs pool BASE.json... -- CAND.json... [--series db_probe_ms] [--q 95] [--max-pct N]
 *
 * --advisory KEY reports a headline p95 (e.g. latency.db_probe_ms.p95) without
 * gating on it: C4's rule for a series that fails its own 2% repeatability.
 * pool pools one raw sample series across runs per mode (metrics.mjs
 * pooledComparison); it is the steadier view of an advisory series.
 *
 * gate also requires the same binary, and for an enabled candidate applies the
 * A11 evidence checks (metrics.mjs evidenceGate): evidence really happened,
 * queue limits held, drops carry reasons, submit never waited on I/O.
 * Exits non-zero when the check fails or the workloads differ.
 */
import {readFile} from 'node:fs/promises';
import {repeatability, releaseGate, evidenceGate, workloadFingerprint, pooledComparison, HEADLINE_P95} from './metrics.mjs';

const [command, a, b, ...rest] = process.argv.slice(2);
if (command === 'pool') {
  const argv = process.argv.slice(3);
  const named = new Map();
  const files = [];
  for (let i = 0; i < argv.length; i++) {
    if (argv[i].startsWith('--') && argv[i] !== '--') named.set(argv[i], argv[++i]);
    else files.push(argv[i]);
  }
  const split = files.indexOf('--');
  if (split < 1 || split === files.length - 1) { console.error('usage: compare.mjs pool BASE.json... -- CAND.json... [--series S] [--q N] [--max-pct N]'); process.exit(2); }
  const load = list => Promise.all(list.map(async f => JSON.parse(await readFile(f, 'utf8'))));
  const [base, cand] = [await load(files.slice(0, split)), await load(files.slice(split + 1))];
  const all = [...base, ...cand];
  const sameWorkload = all.every(r => r.status === 'completed' && workloadFingerprint(r) === workloadFingerprint(all[0]));
  const sameBinary = all.every(r => r.build?.binary_sha256 === all[0].build?.binary_sha256);
  const result = pooledComparison(base, cand, named.get('--series') || 'db_probe_ms', Number(named.get('--q') || 95),
    named.has('--max-pct') ? Number(named.get('--max-pct')) : 5);
  Object.assign(result, {baseline_files: files.slice(0, split), candidate_files: files.slice(split + 1), same_workload: sameWorkload, same_binary: sameBinary});
  console.log(JSON.stringify(result, null, 2));
  process.exit(result.pass && sameWorkload && sameBinary ? 0 : 1);
}
if (!['repeat', 'gate'].includes(command) || !a || !b) {
  console.error('usage: compare.mjs repeat|gate A.json B.json [--max-pct N] [--require-saturation] [--max-submit-seconds S]');
  process.exit(2);
}
const opt = name => { const i = rest.indexOf(name); return i < 0 ? undefined : Number(rest[i + 1]); };
const limit = opt('--max-pct');
const maxSubmit = opt('--max-submit-seconds');
const [runA, runB] = await Promise.all([a, b].map(async file => JSON.parse(await readFile(file, 'utf8'))));
for (const [file, run] of [[a, runA], [b, runB]]) {
  if (run.status !== 'completed') { console.error(`${file}: status ${run.status}; not comparable`); process.exit(1); }
}
const sameWorkload = workloadFingerprint(runA) === workloadFingerprint(runB);
const advisory = rest.flatMap((x, i) => (x === '--advisory' ? [rest[i + 1]] : []));
const unknown = advisory.filter(k => !HEADLINE_P95.includes(k));
if (unknown.length) { console.error(`--advisory: not a headline series: ${unknown.join(', ')}`); process.exit(2); }
const gateOptions = {...(limit === undefined ? {} : {maxIncreasePct: limit})};
const result = command === 'repeat'
  ? repeatability(runA, runB, limit === undefined ? {} : {maxPct: limit})
  : releaseGate(runA, runB, {...gateOptions, keys: HEADLINE_P95.filter(k => !advisory.includes(k))});
if (command === 'gate' && advisory.length) result.advisory = releaseGate(runA, runB, {...gateOptions, keys: advisory}).checks
  .filter(c => advisory.includes(c.key)).map(c => ({...c, gating: false}));
result.same_workload = sameWorkload;
if (command === 'repeat' && runA.mode !== runB.mode) result.same_workload = false;
let pass = result.pass && result.same_workload;
if (command === 'gate') {
  result.same_binary = Boolean(runA.build?.binary_sha256) && runA.build.binary_sha256 === runB.build?.binary_sha256;
  pass = pass && result.same_binary;
  if (runB.mode === 'enabled') {
    result.evidence = evidenceGate(runA, runB, {requireSaturation: rest.includes('--require-saturation'),
      ...(maxSubmit === undefined ? {} : {maxSubmitSeconds: maxSubmit})});
    pass = pass && result.evidence.pass;
  }
  result.verdict = pass ? 'pass' : 'fail';
}
console.log(JSON.stringify(result, null, 2));
process.exit(pass ? 0 : 1);
