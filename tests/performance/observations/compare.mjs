#!/usr/bin/env node
/**
 * Compare observation workload results.
 *
 *   node compare.mjs repeat RUN_A.json RUN_B.json      # C4: same mode, p95 variance < 2%
 *   node compare.mjs gate BASELINE.json CANDIDATE.json # X2: no new gaps, p95 +<=5%
 *     [--max-pct N] [--require-saturation] [--max-submit-seconds S]
 *
 * gate also requires the same binary, and for an enabled candidate applies the
 * A11 evidence checks (metrics.mjs evidenceGate): evidence really happened,
 * queue limits held, drops carry reasons, submit never waited on I/O.
 * Exits non-zero when the check fails or the workloads differ.
 */
import {readFile} from 'node:fs/promises';
import {repeatability, releaseGate, evidenceGate, workloadFingerprint} from './metrics.mjs';

const [command, a, b, ...rest] = process.argv.slice(2);
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
const result = command === 'repeat'
  ? repeatability(runA, runB, limit === undefined ? {} : {maxPct: limit})
  : releaseGate(runA, runB, limit === undefined ? {} : {maxIncreasePct: limit});
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
