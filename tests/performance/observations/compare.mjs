#!/usr/bin/env node
/**
 * Compare observation workload results.
 *
 *   node compare.mjs repeat RUN_A.json RUN_B.json      # C4: same mode, p95 variance < 2%
 *   node compare.mjs gate BASELINE.json CANDIDATE.json # X2: no new gaps, p95 +<=5%
 *
 * Exits non-zero when the check fails or the workloads differ.
 */
import {readFile} from 'node:fs/promises';
import {repeatability, releaseGate, workloadFingerprint} from './metrics.mjs';

const [command, a, b, ...rest] = process.argv.slice(2);
if (!['repeat', 'gate'].includes(command) || !a || !b) {
  console.error('usage: compare.mjs repeat|gate A.json B.json [--max-pct N]');
  process.exit(2);
}
const at = rest.indexOf('--max-pct');
const limit = at < 0 ? undefined : Number(rest[at + 1]);
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
console.log(JSON.stringify(result, null, 2));
process.exit(result.pass && result.same_workload ? 0 : 1);
