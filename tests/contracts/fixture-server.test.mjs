import test from 'node:test';
import assert from 'node:assert/strict';
import {access} from 'node:fs/promises';
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
    assert.match(error.message, /ENOENT/); assert.match(error.message, /fixture: \/tmp\/lightnvr-contract-/); return true;
  });
});

test('unexpected native signal is reported as failure and retains logs', {timeout: 45000}, async () => {
  const fixture = await startFixture({seed:false});
  process.kill(fixture.pid,'SIGKILL');
  await assert.rejects(fixture.stop(), /Native fixture failed/);
  await access(fixture.logPath);
});
