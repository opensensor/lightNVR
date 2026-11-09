import test from 'node:test';
import assert from 'node:assert/strict';
import {access, mkdtemp, writeFile, rm} from 'node:fs/promises';
import {tmpdir} from 'node:os';
import path from 'node:path';
import {reservePort, startFixture} from './fixture-server.mjs';

test('occupied port fails without disturbing its owner', async () => {
  const owner = await reservePort();
  try { await assert.rejects(startFixture({port: owner.port}), /EADDRINUSE/); }
  finally { await owner.release(); }
});

test('two real servers own independent ports, files and lifetimes', {timeout: 90000}, async () => {
  const fixtures = [];
  let failed = true;
  try {
    // Sequential initialization still leaves both real processes alive together.
    fixtures.push(await startFixture());
    fixtures.push(await startFixture());
    const [a,b] = fixtures;
    assert.notEqual(a.pid,b.pid); assert.notEqual(a.port,b.port); assert.notEqual(a.directory,b.directory);
    assert.equal((await a.json('/api/streams')).length,2);
    await a.stop();
    await assert.rejects(access(a.directory));
    assert.equal((await b.json('/api/streams')).length,2);
    await access(b.directory);
    failed = false;
  } finally { for (const fixture of fixtures) await fixture.stop({failed}); }
});

test('startup failures retain diagnostic directory', async () => {
  await assert.rejects(startFixture({binary:'/nonexistent/lightnvr-contract-test'}), error => {
    assert.match(error.message, /ENOENT/); assert.match(error.message, /fixture: .*lightnvr-contract-/); return true;
  });
});

test('startup errors include native logs before a fixture can be returned', async () => {
  const directory = await mkdtemp(path.join(tmpdir(), 'lightnvr-startup-test-'));
  const binary = path.join(directory, 'exit-during-startup');
  try {
    await writeFile(binary, '#!/bin/sh\necho "native startup diagnostic" >&2\nexit 0\n', {mode: 0o700});
    await assert.rejects(startFixture({binary}), error => {
      assert.match(error.message, /Server exited during startup \(0\/null\)/);
      assert.match(error.message, /Native startup log:\nnative startup diagnostic/);
      return true;
    });
  } finally { await rm(directory, {recursive: true, force: true}); }
});

test('unexpected native signal is reported as failure and retains logs', {timeout: 45000}, async () => {
  const fixture = await startFixture({seed:false});
  process.kill(fixture.pid,'SIGKILL');
  await assert.rejects(fixture.stop(), /Native fixture failed/);
  await access(fixture.logPath);
});
