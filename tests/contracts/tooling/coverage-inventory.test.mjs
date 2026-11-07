import test from 'node:test';
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';

const root = new URL('../../../', import.meta.url);
const read = file => readFile(new URL(file, root), 'utf8');

test('every native route has one current contract or explicit protocol disposition', async () => {
  const sourcePath = 'src/web/libuv_api_handlers.c';
  const [source, coverageText, canonicalText] = await Promise.all([
    read(sourcePath), read('docs/api/contracts/coverage.json'), read('docs/api/openapi.json'),
  ]);
  const coverage = JSON.parse(coverageText), canonical = JSON.parse(canonicalText);
  assert.equal(coverage.source_sha256, createHash('sha256').update(source).digest('hex'));
  // This registration unit intentionally uses literal method/path/handler calls.
  // A new dynamic registration requires an explicit inventory-parser review.
  const registrations = [...source.matchAll(/http_server_register_handler\(\s*server\s*,\s*"([^"]+)"\s*,\s*"([^"]+)"\s*,\s*(\w+)\s*\)/g)];
  assert.equal(registrations.length, [...source.matchAll(/http_server_register_handler\s*\(/g)].length);
  const inventory = new Map();
  for (const entry of coverage.operations) {
    const key = `${entry.method} ${entry.route_pattern}`;
    assert.ok(!inventory.has(key), `duplicate inventory: ${key}`);
    assert.notEqual(entry.coverage, 'later', `missing disposition: ${key}`);
    inventory.set(key, entry);
  }
  assert.equal(inventory.size, registrations.length);
  for (const match of registrations) {
    const [, route, method, handler] = match, key = `${method} ${route}`;
    const entry = inventory.get(key);
    assert.ok(entry, `missing native route: ${key}`);
    assert.equal(entry.handler, handler, `stale handler: ${key}`);
    assert.equal(entry.source, sourcePath);
    assert.equal(entry.line, source.slice(0, match.index).split('\n').length, `stale source line: ${key}`);
  }
  const documented = new Map();
  for (const [path, item] of Object.entries(canonical.paths)) {
    for (const [method, operation] of Object.entries(item)) {
      if (!['get','post','put','patch','delete','head','options','trace'].includes(method)) continue;
      assert.ok(!documented.has(operation.operationId), `duplicate operation: ${operation.operationId}`);
      documented.set(operation.operationId, {path, method: method.toUpperCase()});
    }
  }
  const covered = [];
  for (const entry of inventory.values()) {
    if (entry.operation_id) {
      assert.deepEqual(documented.get(entry.operation_id), {path: entry.canonical_path, method: entry.method});
      covered.push(entry.operation_id);
    } else {
      assert.match(entry.classification, /protocol/);
      assert.ok(typeof entry.reason === 'string' && entry.reason.length > 30, `unexplained exclusion: ${entry.route_pattern}`);
    }
  }
  assert.equal(new Set(covered).size, covered.length);
  assert.deepEqual(covered.sort(), [...documented.keys()].sort());
  assert.deepEqual([...canonical['x-lightnvr-coverage'].covered_operations].sort(), covered);
});
