// SPDX-License-Identifier: GPL-3.0-or-later
import { test, after } from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs/promises';
import os from 'node:os';
import path from 'node:path';
import { pathToFileURL, fileURLToPath } from 'node:url';
import { build } from 'esbuild';
import { spawnSync } from 'node:child_process';
import { buildArtifacts, writeArtifacts, checkArtifacts, loadContract } from '../../scripts/quality/contracts.mjs';

const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
const scratch = await fs.mkdtemp(path.join(os.tmpdir(), 'lightnvr-contracts-'));
after(() => fs.rm(scratch, { recursive: true, force: true }));
await fs.cp(path.join(root, 'docs/api'), path.join(scratch, 'docs/api'), { recursive: true });
await fs.cp(path.join(root, 'tests/contracts/fixtures'), path.join(scratch, 'tests/contracts/fixtures'), { recursive: true });
const artifacts = await buildArtifacts({ root: scratch });
await writeArtifacts(scratch, artifacts);
const generated = await import(pathToFileURL(path.join(scratch, 'web/js/api/generated/validators.mjs')));
const fixtureRoot = path.join(scratch, 'tests/contracts/fixtures/streams');
const fixture = async name => JSON.parse(await fs.readFile(path.join(fixtureRoot, name), 'utf8'));

test('all reviewed fixtures match their expected validation outcome', async () => {
  const manifest = await fixture('index.json');
  assert.equal(manifest.fixtures.length, 91);
  for (const entry of manifest.fixtures) {
    const data = await fixture(entry.file);
    const validator = generated['validate' + entry.schema.split('/').at(-1)];
    assert.equal(typeof validator, 'function', entry.file);
    assert.equal(validator(data.body), entry.expect === 'valid', entry.file);
  }
});

test('viewer models reject credentials without requiring sensitive fields', async () => {
  const safe = (await fixture('full.live-only.sensitive-keys-absent.valid.json')).body;
  assert.equal(generated.validateStreamFullResponseLiveOnly(safe), true);
  safe.stream.onvif_password = 'synthetic-leak';
  assert.equal(generated.validateStreamFullResponseLiveOnly(safe), false);
});

test('permitted additive response fields work but client request drift fails', async () => {
  const safe = (await fixture('client-config.valid.json')).body;
  assert.equal(generated.validateClientConfig({ ...safe, future_addition: 1 }), true);
  assert.equal(generated.validateStreamUpdateRequest({ record: true, unknown_option: 1 }), false);
});

test('UTF-8 byte bounds are enforced independently of Unicode character count', () => {
  assert.equal(generated.validateStreamName('é'.repeat(127)), true);
  assert.equal(generated.validateStreamName('é'.repeat(128)), false);
  assert.equal(generated.validateStreamName('x'.repeat(255)), true);
});

test('JSON-in-string handles disabled, malformed and structurally wrong values', () => {
  assert.equal(generated.validateEptzConfigString(''), true);
  assert.equal(generated.validateEptzConfigString('{'), false);
  assert.equal(generated.validateEptzConfigString('null'), false);
  assert.equal(generated.validateEptzConfigString('{}'), false);
});

test('fresh generation is deterministic and check leaves checkout bytes untouched', async () => {
  assert.deepEqual(await buildArtifacts({ root: scratch }), artifacts);
  assert.deepEqual(await checkArtifacts(scratch, artifacts), []);
  for (const [name, text] of Object.entries(artifacts)) assert.equal(await fs.readFile(path.join(scratch,name),'utf8'),text);
});

test('check rejects stale, missing and unexpected files without repairing them', async () => {
  const filename='web/js/api/generated/types.ts';
  const target=path.join(scratch,filename);
  await fs.writeFile(target, 'stale\n');
  assert.ok((await checkArtifacts(scratch, artifacts)).some(x => x.includes(filename)));
  assert.equal(await fs.readFile(target,'utf8'),'stale\n');
  await fs.unlink(target);
  assert.ok((await checkArtifacts(scratch,artifacts)).some(x => x.includes(filename)));
  await fs.writeFile(target, artifacts[filename]);
  const extra=path.join(scratch,'web/js/api/generated/forgotten.ts');
  await fs.writeFile(extra,'untracked');
  assert.ok((await checkArtifacts(scratch,artifacts)).some(x => x.includes('forgotten.ts')));
  await fs.unlink(extra);
});

test('operation manifest retains status/header and role/surface selection information', async () => {
  const manifest=JSON.parse(artifacts['tests/contracts/generated/manifest.json']);
  assert.deepEqual(manifest.operations.updateStream.success_statuses,[200,202]);
  assert.ok(manifest.operations.getStreamFull.responses['200'].headers['Cache-Control']);
  assert.equal(manifest.operations.getStreamFull.variant_by_field.field,'stream.can_control_privacy');
  assert.equal(manifest.operations.listStreams.variant_by_query.parameter,'surface');
});

test('generated code has no compiler imports and can bundle for the browser', async () => {
  const runtime=artifacts['web/js/api/generated/validators.mjs'];
  assert.doesNotMatch(runtime, /(?<!["'])\brequire\s*\(|\bfrom\s+["'](?:ajv|openapi-typescript)/);
  assert.ok(runtime.length>1000);
  for (const [name,source] of Object.entries(artifacts)) if(name.endsWith('.mjs')) assert.doesNotMatch(source,/x-lightnvr-source|src\/web\//);
});

test('one imported boundary does not retain all generated validators', async () => {
  async function size(specifier, expression) {
    const result=await build({stdin:{contents:`import ${specifier} from './web/js/api/generated/validators.mjs'; globalThis.boundary=${expression};`,resolveDir:scratch},bundle:true,write:false,format:'esm',platform:'browser',minify:true});
    return result.outputFiles[0].contents.length;
  }
  const one=await size('{validateClientConfig}','validateClientConfig');
  const all=await size('* as validators','validators');
  assert.ok(one < all / 2, `single boundary ${one} vs all ${all}`);
});

test('unresolved, remote and traversal refs fail before generation', async () => {
  const doc=path.join(scratch,'docs/api/contracts/common.yaml');
  const original=await fs.readFile(doc,'utf8');
  for(const ref of ['#/components/schemas/DoesNotExist','https://example.invalid/schema.json','../../../package.json#/version']){
    await fs.writeFile(doc,original+`\n  x-invalid:\n    $ref: '${ref}'\n`);
    await assert.rejects(loadContract(scratch));
  }
  await fs.writeFile(doc,original);
});

test('git-untracked expected generated files cannot pass check', async () => {
  assert.equal(spawnSync('git',['init','-q'],{cwd:scratch}).status,0);
  const errors=await checkArtifacts(scratch,artifacts);
  assert.ok(errors.some(x=>x.includes('untracked')));
  assert.equal(spawnSync('git',['add','web/js/api/generated','tests/contracts/generated'],{cwd:scratch}).status,0);
  assert.deepEqual(await checkArtifacts(scratch,artifacts),[]);
});
