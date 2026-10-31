import { test } from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import YAML from 'yaml';
import os from 'node:os';
import path from 'node:path';
import { spawnSync } from 'node:child_process';

const workflowPath = '.github/workflows/engineering-foundation.yml';
const workflow = () => YAML.parse(fs.readFileSync(workflowPath, 'utf8'));
const settings = () => JSON.parse(fs.readFileSync('quality/required-checks.json', 'utf8'));
const commands = job => job.steps.map(step => step.run || '').join('\n');

test('foundation jobs run on every main PR without path or success bypasses', () => {
  const w = workflow();
  assert.deepEqual(w.on.pull_request.branches, ['main']);
  for (const event of [w.on.pull_request, w.on.push]) {
    assert.equal(event.paths, undefined); assert.equal(event['paths-ignore'], undefined);
  }
  for (const job of Object.values(w.jobs)) {
    assert.equal(job.if, undefined); assert.equal(job['continue-on-error'], undefined);
    assert.ok(job['timeout-minutes'] > 0);
    for (const step of job.steps) {
      assert.equal(step['continue-on-error'], undefined);
      if (step.run) assert.equal(step.if, undefined);
    }
    assert.doesNotMatch(commands(job), /\|\|\s*(true|:)|--update-baseline|--generate|--capture|--propose/);
  }
});

test('required identities map to actual jobs and remain proposals', () => {
  const w = workflow(), s = settings();
  assert.equal(s.status, 'proposed-not-applied');
  assert.equal(s.remote_changes_applied, false);
  assert.equal(s.future_contract_lane.status, 'pending-T39');
  assert.equal(s.future_contract_lane.required_check, null);
  const ours = s.checks.filter(check => check.workflow === workflowPath);
  assert.equal(ours.length, Object.keys(w.jobs).length);
  for (const check of ours) {
    assert.ok(w.jobs[check.job_id]);
    assert.equal(check.context, w.jobs[check.job_id].name);
    assert.equal(check.enforcement_verified, false);
  }
  assert.equal(new Set(s.checks.map(check => check.context)).size, s.checks.length);
});

test('type, lint, unit and schema jobs execute real existing commands', () => {
  const jobs = workflow().jobs;
  assert.match(commands(jobs.types), /npm --prefix web run typecheck/);
  assert.match(commands(jobs.types), /npm run typecheck:tests/);
  assert.match(commands(jobs.lint), /npm --prefix web run lint/);
  assert.match(commands(jobs.unit), /npm --prefix web test -- --runInBand/);
  assert.match(commands(jobs.schema), /node scripts\/quality\/contracts.mjs --check/);
  assert.doesNotMatch(commands(jobs.unit), /passWithNoTests/);
  assert.match(commands(jobs.tooling), /node --test tests\/quality\/\*\.test\.mjs/);
  assert.match(commands(jobs.tooling), /python3 -m unittest discover -s tests\/quality/);
});

test('default and modern asset jobs build the matching profile before checking', () => {
  const jobs = workflow().jobs;
  for (const profile of ['default', 'modern']) {
    const job = jobs['assets-' + profile], c = commands(job);
    assert.match(c, /npm --prefix web run build/);
    assert.match(c, new RegExp('assets.mjs --check --profile ' + profile));
    assert.ok(c.indexOf('npm --prefix web run build') < c.indexOf('assets.mjs --check'));
    assert.equal(String(job.env.LIGHTNVR_WEB_LEGACY), profile === 'modern' ? 'false' : 'true');
    assert.ok(job.steps.some(step => step.uses?.startsWith('actions/upload-artifact@') && step.if === 'always()'));
  }
});

test('every job uses locked Node and installs both existing dependency trees', () => {
  for (const job of Object.values(workflow().jobs)) {
    assert.ok(job.steps.some(step => step.uses?.startsWith('actions/setup-node@') && step.with?.['node-version-file'] === '.nvmrc'));
    assert.match(commands(job), /npm ci\s*\n/);
    assert.match(commands(job), /npm ci --prefix web/);
    assert.equal(job.defaults.run.shell, 'bash');
  }
});

test('analysis proposal points to the full coverage job, never an individual lane', () => {
  const s = settings();
  const analysis = s.checks.filter(check => check.workflow === '.github/workflows/static-analysis.yml');
  assert.equal(analysis.length, 1);
  const w = YAML.parse(fs.readFileSync('.github/workflows/static-analysis.yml', 'utf8'));
  const job = w.jobs[analysis[0].job_id];
  assert.ok(job);
  assert.equal(analysis[0].context, job.name);
  assert.match(commands(job), /scripts\/quality\/findings.py --check/);
  assert.doesNotMatch(commands(job), /findings_lane.py/);
});


test('a failing check exits nonzero through diagnostic tee and retains its log', () => {
  const dir = fs.mkdtempSync(path.join(os.tmpdir(), 'foundation-failure-'));
  try {
    const bin = path.join(dir, 'bin'); fs.mkdirSync(bin);
    for (const tool of ['npm', 'node', 'python3']) {
      fs.writeFileSync(path.join(bin, tool), '#!/bin/sh\necho injected-check-failure\nexit 7\n', { mode: 0o755 });
    }
    for (const [id, job] of Object.entries(workflow().jobs)) {
      const step = job.steps.find(step => step.name === 'Run checks');
      const result = spawnSync('bash', ['-eo', 'pipefail', '-c', step.run], { cwd: dir, env: { ...process.env, PATH: bin + ':' + process.env.PATH }, encoding: 'utf8' });
      assert.equal(result.status, 7, id + ': ' + result.stdout + result.stderr);
      assert.match(fs.readFileSync(path.join(dir, 'build/eng-artifacts/foundation-' + id, 'check.log'), 'utf8'), /injected-check-failure/);
    }
  } finally { fs.rmSync(dir, { recursive: true, force: true }); }
});
