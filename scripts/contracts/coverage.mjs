#!/usr/bin/env node
/**
 * Refresh docs/api/contracts/coverage.json from src/web/libuv_api_handlers.c.
 *
 * tests/contracts/tooling/coverage-inventory.test.mjs pins the registration
 * source by sha256 and every entry by handler name and line number. This
 * script recomputes those facts after a registration edit while preserving
 * every reviewed field (coverage, classification, eligibility, reason).
 *
 *   node scripts/contracts/coverage.mjs                  # refresh lines/handlers/sha256
 *   node scripts/contracts/coverage.mjs --add-coverage observations
 *                                                        # also add new routes
 *   node scripts/contracts/coverage.mjs --check          # fail on drift, write nothing
 *
 * A registration with no inventory entry is an error unless --add-coverage is
 * given. A new entry is then matched to exactly one canonical OpenAPI
 * operation with the same method and route ('{param}' segments become '#');
 * a route with no operation is still an error, because protocol exclusions
 * need a written reason. An inventory entry whose route is no longer
 * registered is always an error: removals are reviewed by hand.
 */
import {readFile, writeFile} from 'node:fs/promises';
import path from 'node:path';
import {fileURLToPath} from 'node:url';
import {createHash} from 'node:crypto';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const sourcePath = 'src/web/libuv_api_handlers.c';
const coveragePath = 'docs/api/contracts/coverage.json';
const METHODS = ['get', 'post', 'put', 'patch', 'delete', 'head', 'options', 'trace'];

export function registrations(source) {
  // Same literal form the inventory test accepts.
  const matches = [...source.matchAll(/http_server_register_handler\(\s*server\s*,\s*"([^"]+)"\s*,\s*"([^"]+)"\s*,\s*(\w+)\s*\)/g)];
  if (matches.length !== [...source.matchAll(/http_server_register_handler\s*\(/g)].length) {
    throw new Error(`${sourcePath} has a non-literal registration; review the inventory parser`);
  }
  return matches.map(m => ({route: m[1], method: m[2], handler: m[3], line: source.slice(0, m.index).split('\n').length}));
}

export function refresh(coverage, source, canonical, {addCoverage = null, classification = 'protected-review-per-route'} = {}) {
  const regs = registrations(source);
  const byKey = new Map(coverage.operations.map(entry => [`${entry.method} ${entry.route_pattern}`, entry]));
  const registered = new Set(regs.map(r => `${r.method} ${r.route}`));
  const stale = [...byKey.keys()].filter(key => !registered.has(key));
  if (stale.length) throw new Error(`Inventory entries no longer registered (remove by hand): ${stale.join(', ')}`);
  const operations = [];
  for (const [path, item] of Object.entries(canonical.paths)) {
    for (const [method, operation] of Object.entries(item)) {
      if (METHODS.includes(method)) operations.push({path, method: method.toUpperCase(), operation});
    }
  }
  const added = [];
  let previous = null;
  for (const reg of regs) {
    const key = `${reg.method} ${reg.route}`;
    let entry = byKey.get(key);
    if (!entry) {
      if (!addCoverage) throw new Error(`Registration without inventory entry: ${key} (pass --add-coverage <group>)`);
      const matches = operations.filter(op => op.method === reg.method && op.path.replace(/\{[^}]+\}/g, '#') === reg.route);
      if (matches.length !== 1) throw new Error(`New registration ${key} matches ${matches.length} OpenAPI operations; expected 1`);
      const [{path, operation}] = matches;
      entry = {method: reg.method, route_pattern: reg.route, handler: reg.handler, source: sourcePath, line: reg.line,
        coverage: addCoverage, classification, operation_id: operation.operationId,
        eligibility: operation['x-lightnvr-authorization']?.kind || 'See canonical principal predicate.',
        canonical_path: path};
      // Keep the inventory in its existing (roughly source) order.
      const index = previous ? coverage.operations.indexOf(previous) + 1 : coverage.operations.length;
      coverage.operations.splice(index, 0, entry);
      byKey.set(key, entry);
      added.push(key);
    }
    entry.handler = reg.handler;
    entry.source = sourcePath;
    entry.line = reg.line;
    previous = entry;
  }
  coverage.source_sha256 = createHash('sha256').update(source).digest('hex');
  return added;
}

async function main() {
  const args = process.argv.slice(2);
  let check = false, addCoverage = null, classification;
  for (let i = 0; i < args.length; i++) {
    if (args[i] === '--check') check = true;
    else if (args[i] === '--add-coverage') addCoverage = args[++i];
    else if (args[i] === '--classification') classification = args[++i];
    else throw new Error(`Unknown argument: ${args[i]}`);
  }
  const read = file => readFile(path.join(root, file), 'utf8');
  const [source, coverageText, canonicalText] = await Promise.all([read(sourcePath), read(coveragePath), read('docs/api/openapi.json')]);
  const coverage = JSON.parse(coverageText);
  const added = refresh(coverage, source, JSON.parse(canonicalText), {addCoverage, ...(classification ? {classification} : {})});
  const output = JSON.stringify(coverage, null, 2) + '\n';
  if (check) {
    if (output !== coverageText) throw new Error(`${coveragePath} is stale; run node scripts/contracts/coverage.mjs`);
    console.log(`Checked ${coveragePath}`);
    return;
  }
  await writeFile(path.join(root, coveragePath), output);
  console.log(`Refreshed ${coveragePath} (${coverage.operations.length} routes${added.length ? `; added ${added.join(', ')}` : ''})`);
}
if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  main().catch(error => { console.error(error.message); process.exitCode = 1; });
}
