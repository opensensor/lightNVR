import test from 'node:test';
import assert from 'node:assert/strict';
import {mkdtemp, writeFile, rm} from 'node:fs/promises';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import path from 'node:path';
import os from 'node:os';
import {trackedDiffSha256} from '../../performance/git-provenance.mjs';

test('diff provenance handles clean trees and diffs larger than the sync buffer', async () => {
  const directory = await mkdtemp(path.join(os.tmpdir(), 'lightnvr-git-provenance-'));
  const git = (...args) => execFileSync('git', args, {cwd: directory, maxBuffer: 8 * 1024 * 1024});
  const hash = bytes => createHash('sha256').update(bytes).digest('hex');
  try {
    git('init', '--quiet');
    await writeFile(path.join(directory, 'generated.rs'), '// initial\n');
    git('add', 'generated.rs');
    git('-c', 'user.name=Fixture', '-c', 'user.email=fixture@example.invalid',
      '-c', 'commit.gpgsign=false', 'commit', '--quiet', '-m', 'fixture');
    assert.equal(await trackedDiffSha256(directory), hash(''));
    await writeFile(path.join(directory, 'generated.rs'), '// regenerated café 🐱\n'.repeat(100000));
    const diff = git('diff', '--no-ext-diff', '--no-textconv', 'HEAD');
    assert.ok(diff.length > 1024 * 1024);
    assert.equal(await trackedDiffSha256(directory), hash(diff));
  } finally {
    await rm(directory, {recursive: true, force: true});
  }
});

test('diff provenance rejects Git failures instead of reporting a partial checksum', async () => {
  const directory = await mkdtemp(path.join(os.tmpdir(), 'lightnvr-git-provenance-error-'));
  try {
    execFileSync('git', ['init', '--quiet'], {cwd: directory});
    // A new repository has no HEAD to compare against.
    await assert.rejects(trackedDiffSha256(directory), /git diff failed/);
  } finally {
    await rm(directory, {recursive: true, force: true});
  }
});
