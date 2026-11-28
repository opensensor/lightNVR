/**
 * Start a lightnvr-rs binary against an owned, disposable fixture and check
 * startup, migrations, auth, stream inventory, OpenAPI and clean shutdown.
 *
 *   node tools/rust-transpile/smoke.mjs [path/to/lightnvr-rs]
 */
import assert from 'node:assert/strict';
import {mkdir, readFile, stat, writeFile} from 'node:fs/promises';
import path from 'node:path';
import {fileURLToPath} from 'node:url';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
process.env.LIGHTNVR_CONTRACT_ROOT ??= root;
const {startFixture} = await import('../../tests/contracts/fixture-server.mjs');
const binary = path.resolve(process.argv[2] || path.join(root, 'rust/target/debug/lightnvr-rs'));

async function processMemory(pid) {
  const status = await readFile(`/proc/${pid}/status`, 'utf8');
  const rollup = await readFile(`/proc/${pid}/smaps_rollup`, 'utf8');
  const kib = (text, key) => {
    const match = text.match(new RegExp(`^${key}:\\s+(\\d+) kB$`, 'm'));
    assert.ok(match, `${key} missing from process memory report`);
    return Number(match[1]);
  };
  return {
    rssKiB: kib(status, 'VmRSS'),
    peakRssKiB: kib(status, 'VmHWM'),
    pssKiB: kib(rollup, 'Pss'),
  };
}

let fixture;
let passed = false;
try {
  fixture = await startFixture({binary, seed: true});
  for (const route of ['/api/streams', '/api/openapi.json']) {
    const response = await fixture.request(route, {principal: 'anonymous'});
    assert.equal(response.status, 401, `anonymous ${route}`);
    await response.arrayBuffer();
  }
  const streams = await fixture.json('/api/streams');
  assert.deepEqual(streams.map(s => s.name).sort(), ['A', 'B']);
  const document = await fixture.json('/api/openapi.json');
  assert.equal(document.openapi, '3.1.1');
  assert.ok(document.paths['/api/streams']);
  const login = await fixture.request('/api/auth/login', {
    principal: 'anonymous', method: 'POST', body: fixture.credentials.admin,
  });
  assert.equal(login.status, 200);
  const cookie = login.headers.getSetCookie().find(value => value.startsWith('session='));
  assert.ok(cookie, 'session cookie');
  await login.arrayBuffer();
  const verified = await fixture.request('/api/auth/verify', {
    principal: 'anonymous', headers: {Cookie: cookie.split(';')[0]},
  });
  assert.equal(verified.status, 200);
  await verified.arrayBuffer();
  const memory = await processMemory(fixture.pid);
  console.log(`Memory after seeded API smoke: RSS ${memory.rssKiB} KiB, ` +
    `peak RSS ${memory.peakRssKiB} KiB, PSS ${memory.pssKiB} KiB`);
  if (process.env.LIGHTNVR_MEMORY_REPORT) {
    const report = path.resolve(process.env.LIGHTNVR_MEMORY_REPORT);
    await mkdir(path.dirname(report), {recursive: true});
    await writeFile(report, JSON.stringify({
      scenario: 'seeded-api-smoke-no-live-streams',
      binary: path.basename(binary),
      binaryBytes: (await stat(binary)).size,
      ...memory,
    }, null, 2) + '\n');
  }
  if (process.env.LIGHTNVR_MAX_PEAK_RSS_KIB) {
    const limit = Number(process.env.LIGHTNVR_MAX_PEAK_RSS_KIB);
    assert.ok(Number.isSafeInteger(limit) && limit > 0, 'invalid peak RSS limit');
    assert.ok(memory.peakRssKiB <= limit,
      `peak RSS ${memory.peakRssKiB} KiB exceeds ${limit} KiB`);
  }
  await fixture.stop();
  fixture = undefined;
  passed = true;
  console.log(`lightnvr-rs smoke passed (${Object.keys(document.paths).length} OpenAPI paths)`);
} catch (error) {
  console.error(error);
  process.exitCode = 1;
} finally {
  if (fixture) await fixture.stop({failed: !passed}).catch(error => {
    console.error(error);
    process.exitCode = 1;
  });
}
