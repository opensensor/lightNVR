import test from 'node:test';
import assert from 'node:assert/strict';
import {createHash} from 'node:crypto';
import {existsSync} from 'node:fs';
import {readFile, readdir} from 'node:fs/promises';
import path from 'node:path';
import {pathToFileURL} from 'node:url';
import Ajv2020 from 'ajv/dist/2020.js';
import addFormats from 'ajv-formats';
import {root, schemaDocument} from '../../../scripts/contracts/generate.mjs';

// INT_01 C3: canonical observation fixtures shared by lightNVR and YA-WAMF.
// Every fixture is validated against the C2 schemas in docs/api/openapi.json,
// and the cross-fixture invariants of docs/integrations/OBSERVATIONS_CONTRACT.md
// (message IDs, cursors, envelope shape, box math) are recomputed here
// independently of the generator.
const fixtureDir = path.join(root, 'tests/fixtures/integrations/observations');
const fixturePath = file => path.join(fixtureDir, file);
const readJson = async file => JSON.parse(await readFile(fixturePath(file), 'utf8'));

const doc = JSON.parse(await readFile(path.join(root, 'docs/api/openapi.json'), 'utf8'));
const ajv = new Ajv2020({strict: true, allErrors: true, allowUnionTypes: true, validateFormats: true});
addFormats(ajv);
ajv.addSchema(schemaDocument(doc));
const schema = name => {
  const validate = ajv.getSchema(`urn:lightnvr:contracts#/$defs/${name}`);
  assert.ok(validate, `OpenAPI schema ${name} exists`);
  return validate;
};
const generated = await import(pathToFileURL(path.join(root, 'web/generated/api/validators.js')));

// Validate with the unmodified-Ajv oracle and, where exported, the generated
// web validator; both must agree.
function check(name, value, label) {
  const validate = schema(name);
  const ok = validate(value);
  assert.ok(ok, `${label}: ${name}: ${ajv.errorsText(validate.errors)}`);
  const projected = generated[`validate${name}`];
  if (projected) assert.equal(projected(value), true, `${label}: generated validate${name}`);
}

function uuidv5(namespace, name) {
  const hash = createHash('sha1').update(Buffer.from(namespace.replaceAll('-', ''), 'hex'))
    .update(Buffer.from(name, 'utf8')).digest();
  hash[6] = (hash[6] & 0x0f) | 0x50;
  hash[8] = (hash[8] & 0x3f) | 0x80;
  const hex = hash.subarray(0, 16).toString('hex');
  return `${hex.slice(0, 8)}-${hex.slice(8, 12)}-${hex.slice(12, 16)}-${hex.slice(16, 20)}-${hex.slice(20)}`;
}
const NAMESPACE_URL = '6ba7b811-9dad-11d1-80b4-00c04fd430c8';
const decodeCursor = cursor => JSON.parse(Buffer.from(cursor, 'base64url').toString('utf8'));
const encodeCursor = (seq, epoch) => Buffer.from(JSON.stringify({v: 1, seq, epoch})).toString('base64url');

async function listFiles(dir, prefix = '') {
  const out = [];
  for (const entry of await readdir(dir, {withFileTypes: true})) {
    const rel = prefix ? `${prefix}/${entry.name}` : entry.name;
    if (entry.isDirectory()) out.push(...await listFiles(path.join(dir, entry.name), rel));
    else out.push(rel);
  }
  return out.sort();
}

const manifest = existsSync(fixturePath('manifest.json')) ? await readJson('manifest.json') : null;
const entries = kind => manifest.fixtures.filter(entry => entry.kind === kind);
const requireManifest = () => assert.ok(manifest, 'tests/fixtures/integrations/observations/manifest.json exists');

test('the manifest lists every fixture file and the required inventory', async () => {
  requireManifest();
  const listed = manifest.fixtures.map(entry => entry.file).sort();
  const onDisk = (await listFiles(fixtureDir)).filter(file => !['manifest.json', 'README.md'].includes(file));
  assert.deepEqual(onDisk, listed, 'manifest matches the fixture directory');
  assert.equal(new Set(listed).size, listed.length, 'no duplicate manifest entries');

  const observations = await Promise.all(entries('observation').map(e => readJson(e.file)));
  for (const state of ['pending', 'ready', 'unavailable', 'expired']) {
    assert.ok(observations.some(o => o.evidence.state === state && o.label === 'bird'), `single-bird ${state}`);
  }
  const byAsset = Object.groupBy(observations.filter(o => o.revision === 2 && o.evidence.state === 'ready'),
    o => o.evidence.asset_id);
  assert.ok(Object.values(byAsset).some(group => new Set(group.map(o => o.id)).size >= 2), 'two birds share one asset');
  assert.ok(observations.some(o => o.evidence.transform?.mode === 'letterbox' && o.detector.engine_type === 'api'
    && o.evidence.origin === 'submitted_image'), 'letterboxed HTTP transform');

  const events = await Promise.all(entries('event').map(e => readJson(e.file)));
  for (const cause of ['created', 'evidence', 'classification']) {
    assert.ok(events.some(e => e.data.change_cause === cause), `change event ${cause}`);
  }
  const pages = await Promise.all(entries('page').map(e => readJson(e.file)));
  assert.ok(pages.some(p => p.has_more) && pages.some(p => !p.has_more && p.items.length), 'paginated pages');

  const errors = await Promise.all(entries('error').map(async e => ({...e, body: await readJson(e.file)})));
  for (const [status, code] of [[410, 'cursor_expired'], [410, 'evidence_expired'], [202, 'evidence_pending']]) {
    assert.ok(errors.some(e => e.status === status && e.body.code === code), `${status} ${code} body`);
  }
  assert.ok(entries('capabilities').length >= 1, 'capability document');

  const exchanges = await Promise.all(entries('classification_exchange').map(e => readJson(e.file)));
  const outcomes = exchanges.map(x => `${x.response.status}:${x.response.body.code ?? (x.response.body.changed ? 'changed' : 'noop')}`);
  for (const outcome of ['201:changed', '200:noop', '200:changed', '409:revision_conflict',
    '409:owner_decision_locked', '422:evidence_mismatch']) {
    assert.ok(outcomes.includes(outcome), `classification exchange ${outcome}`);
  }
  assert.ok(entries('jpeg').length >= 1 && entries('message_id_vectors').length === 1, 'JPEG and message-ID vectors');
});

test('every JSON fixture validates against the OpenAPI schemas', async () => {
  requireManifest();
  let validated = 0;
  for (const entry of manifest.fixtures) {
    if (entry.schema) {
      check(entry.schema, await readJson(entry.file), entry.file);
      validated++;
    }
    if (entry.kind === 'event') {
      check('ObservationChangeEvent', (await readJson(entry.file)).data, entry.file);
      validated++;
    }
    if (entry.kind === 'classification_exchange') {
      const exchange = await readJson(entry.file);
      assert.equal(exchange.request.method, 'PUT', entry.file);
      check('ObservationClassificationWrite', exchange.request.body, `${entry.file} request`);
      const status = exchange.response.status;
      check([200, 201].includes(status) ? 'ObservationClassificationResult' : 'ObservationError',
        exchange.response.body, `${entry.file} response`);
      if (status >= 400) check('ApiError', exchange.response.body, `${entry.file} response`);
      validated += 2;
    }
    if (entry.kind === 'error') check('ApiError', await readJson(entry.file), entry.file);
  }
  assert.ok(validated >= 30, `validated ${validated} fixtures`);
});

test('the validator rejects malformed fixtures', async () => {
  requireManifest();
  const ready = await readJson(entries('observation').find(e => e.file.endsWith('single-ready.json')).file);
  const pending = await readJson(entries('observation').find(e => e.file.endsWith('single-pending.json')).file);
  const observation = schema('Observation');
  const rejects = (validate, value, why) => assert.equal(validate(value), false, why);
  rejects(observation, {...ready, evidence: {...ready.evidence, reason: 'queue_full'}}, 'reason on a ready state');
  rejects(observation, {...pending, evidence: {...pending.evidence, snapshot_url: ready.evidence.snapshot_url}},
    'snapshot_url on a pending state');
  const {snapshot_url: _, ...noUrl} = ready.evidence;
  rejects(observation, {...ready, evidence: noUrl}, 'ready without snapshot_url');
  rejects(observation, {...ready, evidence: {...ready.evidence, state: 'expired', reason: null}}, 'expired without reason');
  rejects(observation, {...ready, label: 'x'.repeat(32)}, 'label over 31 bytes');
  rejects(observation, {...ready, object_index: 20}, 'object_index over MAX_DETECTIONS');
  assert.equal(generated.validateObservation({...ready, evidence: {...ready.evidence, reason: 'queue_full'}}), false,
    'generated validator rejects reason on a ready state');
  const exchange = await readJson(entries('classification_exchange')[0].file);
  rejects(schema('ObservationClassificationWrite'), {...exchange.request.body, unexpected: true}, 'closed PUT body');
  const event = await readJson(entries('event').find(e => e.change_cause === 'evidence' && e.evidence_state === 'ready').file);
  rejects(schema('ObservationChangeEvent'), {...event.data, evidence_reason: 'disabled'}, 'event reason on ready');
});

test('change events are full MQTT envelopes with contract §5.4 message IDs', async () => {
  requireManifest();
  const vectors = await readJson('message-ids.json');
  const derived = uuidv5(NAMESPACE_URL, vectors.namespace.name);
  assert.equal(derived, '0bc01005-a431-522b-9124-a114f67d187a', 'frozen namespace');
  assert.equal(vectors.namespace.uuid, derived);
  const contractVector = uuidv5(derived, 'urn:lightnvr:11111111-1111-4111-8111-111111111111|55555555-5555-4555-8555-555555555555|2');
  assert.equal(contractVector, '43ebdf02-b2d5-5403-a0ef-2a4e7d58ada9', 'contract test vector');
  assert.ok(vectors.vectors.some(v => v.message_id === contractVector), 'vectors include the contract test vector');
  for (const v of vectors.vectors) {
    assert.equal(v.name, `${v.source}|${v.observation_id}|${v.revision}`);
    assert.equal(uuidv5(derived, v.name), v.message_id, `vector ${v.name}`);
  }

  const {constants} = manifest;
  const keys = ['specversion', 'id', 'type', 'source', 'subject', 'time', 'datacontenttype', 'severity', 'sensitivity', 'data'];
  const seen = new Set();
  for (const entry of entries('event')) {
    const envelope = await readJson(entry.file);
    assert.deepEqual(Object.keys(envelope), keys, `${entry.file}: envelope keys in event_envelope.c order`);
    assert.equal(envelope.specversion, '1.0');
    assert.equal(envelope.type, 'io.lightnvr.detection.observation_changed.v1');
    assert.equal(envelope.source, constants.source);
    assert.equal(envelope.datacontenttype, 'application/json');
    assert.equal(envelope.severity, 'info');
    assert.equal(envelope.sensitivity, 'operational');
    assert.equal(envelope.subject, `camera/${envelope.data.camera_uuid}`, `${entry.file}: subject`);
    assert.match(envelope.time, /^\d{4}-\d\d-\d\dT\d\d:\d\d:\d\dZ$/, `${entry.file}: second-precision time`);
    assert.equal(envelope.id, uuidv5(derived, `${envelope.source}|${envelope.data.observation_id}|${envelope.data.revision}`),
      `${entry.file}: UUIDv5 message id`);
    assert.ok(vectors.vectors.some(v => v.message_id === envelope.id), `${entry.file}: listed in message-ids.json`);
    assert.equal(entry.topic, `${constants.topic_prefix}/v1/events/${envelope.type}/${envelope.data.camera_uuid}`);
    assert.equal(entry.retain, false);
    assert.equal(entry.change_cause, envelope.data.change_cause);
    assert.equal(entry.evidence_state, envelope.data.evidence_state);
    if (envelope.data.evidence_state === 'ready') {
      assert.equal(envelope.data.snapshot_url, `/api/observations/${envelope.data.observation_id}/snapshot`);
    }
    // Privacy rules of event_envelope.c: no path-like or token keys.
    for (const key of Object.keys(envelope.data)) assert.doesNotMatch(key, /(^path$|_path|token)/, key);
    assert.ok(Buffer.byteLength(JSON.stringify(envelope)) < 96 * 1024);
    const identity = `${envelope.source}|${envelope.id}`;
    assert.ok(!seen.has(identity), `${entry.file}: unique source + id`);
    seen.add(identity);
    // The change event agrees with the observation fixture at that revision.
    if (entry.observation) {
      const observation = await readJson(entry.observation);
      assert.equal(observation.id, envelope.data.observation_id);
      assert.equal(observation.revision, envelope.data.revision);
      assert.equal(observation.evidence.state, envelope.data.evidence_state);
      assert.equal(observation.evidence.reason, envelope.data.evidence_reason);
      assert.equal(observation.evidence.asset_id, envelope.data.asset_id);
      assert.equal(observation.observed_at, envelope.data.observed_at);
      assert.equal(observation.label, envelope.data.label);
    }
  }
});

test('cursors, pages and recovery bodies follow the §7 algorithm', async () => {
  requireManifest();
  const {epoch_uuid: epoch, oldest_seq: oldestSeq} = manifest.constants;
  const decoded = cursor => {
    assert.ok(cursor.length <= 256 && !cursor.includes('='), 'cursor is unpadded base64url <= 256 chars');
    const value = decodeCursor(cursor);
    assert.deepEqual(Object.keys(value), ['v', 'seq', 'epoch']);
    assert.equal(encodeCursor(value.seq, value.epoch), cursor, 'compact JSON round trip');
    return value;
  };
  // Contract §7.4 example cursor.
  assert.equal(encodeCursor(1042, '66666666-6666-4666-8666-666666666666'),
    'eyJ2IjoxLCJzZXEiOjEwNDIsImVwb2NoIjoiNjY2NjY2NjYtNjY2Ni00NjY2LTg2NjYtNjY2NjY2NjY2NjY2In0');

  const changes = manifest.changes;
  for (const entry of entries('page')) {
    const page = await readJson(entry.file);
    const next = decoded(page.next_cursor);
    assert.equal(next.epoch, epoch);
    assert.equal(next.seq, entry.next_seq, `${entry.file}: next_cursor seq`);
    const start = entry.request.query.cursor === undefined ? oldestSeq - 1
      : entry.request.query.cursor === 'head' ? entry.watermark : decoded(entry.request.query.cursor).seq;
    // Recompute the page from the change log (§7.3 steps 3-6).
    const limit = Number(entry.request.query.limit ?? 100);
    const ids = [];
    let last = start;
    for (const change of changes.filter(c => c.seq > start && c.seq <= entry.watermark)) {
      if (!ids.includes(change.observation_id)) {
        if (ids.length === limit) break;
        ids.push(change.observation_id);
      }
      last = change.seq;
    }
    if (entry.request.query.cursor === 'head') ids.length = 0;
    assert.deepEqual(page.items.map(item => item.id), ids, `${entry.file}: items`);
    assert.equal(next.seq, last, `${entry.file}: next_cursor is the last scanned change`);
    assert.equal(page.has_more, changes.some(c => c.seq > last && c.seq <= entry.watermark), `${entry.file}: has_more`);
    // Items hold current state at read time (the latest revision <= watermark).
    for (const item of page.items) {
      const latest = changes.filter(c => c.observation_id === item.id && c.seq <= entry.watermark).at(-1);
      assert.equal(item.revision, latest.revision, `${entry.file}: ${item.id} current revision`);
    }
  }
  // Page chaining: each page's request cursor is the previous page's next_cursor.
  const chain = entries('page').filter(e => e.chain).sort((a, b) => a.chain - b.chain);
  for (let i = 1; i < chain.length; i++) {
    assert.equal(chain[i].request.query.cursor, (await readJson(chain[i - 1].file)).next_cursor, `chain ${i}`);
  }

  const expired = await readJson(entries('error').find(e => e.file.endsWith('cursor-expired.json')).file);
  assert.equal(decoded(expired.oldest_cursor).seq, oldestSeq - 1, 'oldest_cursor encodes oldest_seq - 1');
  assert.equal(decoded(expired.oldest_cursor).epoch, epoch, 'oldest_cursor is in the current epoch');
  const capabilities = await Promise.all(entries('capabilities').map(e => readJson(e.file)));
  for (const doc of capabilities) assert.equal(doc.history.oldest_cursor, expired.oldest_cursor);

  for (const entry of entries('error')) {
    const body = await readJson(entry.file);
    assert.equal(body.code, entry.code, entry.file);
    if (entry.status === 202) {
      const retry = Number(entry.headers['Retry-After']);
      assert.ok(Number.isInteger(retry) && retry >= 1 && retry <= 30);
      assert.equal(body.retry_after_seconds, retry, 'Retry-After header matches the body');
    }
  }
});

test('the evidence JPEG has the boxes its manifest and observations describe', async () => {
  requireManifest();
  for (const entry of entries('jpeg')) {
    const bytes = await readFile(fixturePath(entry.file));
    const media = await readJson(entry.manifest);
    assert.equal(media.sha256, createHash('sha256').update(bytes).digest('hex'), `${entry.file}: sha256`);
    assert.equal(media.bytes, bytes.length);
    // Parse the SOF0 marker for dimensions without a decoder.
    assert.equal(bytes.readUInt16BE(0), 0xffd8, 'SOI');
    let offset = 2, size = null;
    while (offset < bytes.length) {
      const marker = bytes.readUInt16BE(offset);
      if (marker === 0xffc0) { size = {height: bytes.readUInt16BE(offset + 5), width: bytes.readUInt16BE(offset + 7)}; break; }
      offset += 2 + bytes.readUInt16BE(offset + 2);
    }
    assert.deepEqual(size, {width: media.width, height: media.height}, 'baseline JPEG dimensions');

    const {default: sharp} = await import('sharp');
    const {data, info} = await sharp(bytes).raw().toBuffer({resolveWithObject: true});
    const pixel = (x, y) => Array.from(data.subarray((y * info.width + x) * info.channels, (y * info.width + x) * info.channels + 3));
    const near = (actual, expected, label) => assert.ok(actual.every((c, i) => Math.abs(c - expected[i]) <= 12),
      `${label}: ${actual} vs ${expected}`);
    near(pixel(2, 2), media.background_rgb, 'background');
    for (const box of media.boxes) {
      const {x, y, width, height} = box.pixels;
      assert.equal(box.normalized.x, Number((x / media.width).toFixed(6)));
      assert.equal(box.normalized.y, Number((y / media.height).toFixed(6)));
      assert.equal(box.normalized.width, Number((width / media.width).toFixed(6)));
      assert.equal(box.normalized.height, Number((height / media.height).toFixed(6)));
      for (const [px, py] of [[x + 1, y + 1], [x + width - 2, y + height - 2], [x + (width >> 1), y + (height >> 1)]]) {
        near(pixel(px, py), box.rgb, `${box.observation_file} inside`);
      }
      near(pixel(x - 2, y + (height >> 1)), media.background_rgb, `${box.observation_file} left of box`);
      near(pixel(x + width + 1, y + (height >> 1)), media.background_rgb, `${box.observation_file} right of box`);
      const observation = await readJson(box.observation_file);
      assert.deepEqual({...observation.box}, {format: 'xywh_normalized', ...box.normalized});
      assert.equal(observation.evidence.asset_id, media.asset_id);
      assert.equal(observation.evidence.width, media.width);
      assert.equal(observation.evidence.height, media.height);
      assert.equal(observation.evidence.bytes, bytes.length);
      // Consumer crop math (§3.2): px = x * evidence.width, never re-applying transform.
      assert.equal(Math.round(observation.box.x * observation.evidence.width), x);
      assert.equal(Math.round(observation.box.height * observation.evidence.height), height);
    }
  }
});

test('the letterbox transform maps the inference box onto the stored box', async () => {
  requireManifest();
  const entry = entries('observation').find(e => e.inference_box);
  assert.ok(entry, 'letterbox fixture declares its inference-space box');
  const observation = await readJson(entry.file);
  const t = observation.evidence.transform;
  assert.equal(t.mode, 'letterbox');
  assert.equal(t.scale_x, t.scale_y, 'letterbox scale is uniform');
  assert.equal(t.inference_width, Math.round(observation.evidence.width * t.scale_x + 2 * t.pad_x));
  assert.equal(t.inference_height, Math.round(observation.evidence.height * t.scale_y + 2 * t.pad_y));
  const {u, v, width, height} = entry.inference_box;
  const round6 = value => Number(value.toFixed(6));
  assert.deepEqual({...observation.box}, {
    format: 'xywh_normalized',
    x: round6((u - t.pad_x) / t.scale_x / observation.evidence.width),
    y: round6((v - t.pad_y) / t.scale_y / observation.evidence.height),
    width: round6(width / t.scale_x / observation.evidence.width),
    height: round6(height / t.scale_y / observation.evidence.height),
  });
});

test('classification exchanges follow the §10.4 rules', async () => {
  requireManifest();
  const capabilities = await Promise.all(entries('capabilities').map(e => readJson(e.file)));
  for (const entry of entries('classification_exchange')) {
    const {request, response, observation_file: observationFile} = await readJson(entry.file);
    const match = request.path.match(/^\/api\/observations\/([0-9a-f-]{36})\/classifications\/([^/]+)\/([^/]+)$/);
    assert.ok(match, `${entry.file}: two-segment PUT path`);
    const [, id, namespace, name] = match;
    const observation = await readJson(observationFile);
    assert.equal(observation.id, id, `${entry.file}: observation`);
    // The writer namespace is user-derived (D3) and advertised in capabilities.
    assert.ok(capabilities.some(c => c.principal.writer_namespace === request.principal.writer_namespace));
    const body = response.body;
    if (response.status === 200 || response.status === 201) {
      assert.equal(namespace, request.principal.writer_namespace);
      assert.equal(body.annotation.key, `${namespace}/${name}`);
      assert.equal(body.annotation.writer_namespace, namespace);
      for (const field of ['classifier', 'label', 'confidence', 'classified_at', 'input_asset_id', 'input_revision', 'kind']) {
        assert.deepEqual(body.annotation[field], request.body[field], `${entry.file}: ${field}`);
      }
      assert.equal(body.annotation.input_asset_id, observation.evidence.asset_id);
      if (response.status === 201) assert.equal(body.annotation.annotation_revision, 1);
      assert.equal(body.observation_revision, observation.revision + (body.changed ? 1 : 0),
        `${entry.file}: observation revision bump`);
    } else if (body.code === 'revision_conflict' || body.code === 'owner_decision_locked') {
      assert.equal(response.status, 409);
      assert.ok(body.current, `${entry.file}: 409 carries current`);
      if (body.code === 'owner_decision_locked') {
        assert.equal(body.current.kind, 'owner_confirmed');
        assert.equal(request.body.kind, 'automated');
      } else {
        assert.notEqual(request.body.expected_revision ?? 0, body.current.annotation_revision);
      }
    } else if (body.code === 'evidence_mismatch') {
      assert.equal(response.status, 422);
      assert.equal(body.observation_asset_id, observation.evidence.asset_id);
      assert.equal(body.evidence_state, observation.evidence.state);
      assert.ok(request.body.input_asset_id !== observation.evidence.asset_id
        || !['ready', 'expired'].includes(observation.evidence.state));
    } else if (body.code === 'namespace_forbidden') {
      assert.equal(response.status, 403);
      assert.notEqual(namespace, request.principal.writer_namespace);
    }
  }
});

// Vendored copy in YA-WAMF (backend/tests/fixtures/lightnvr/). Sync with
// `node scripts/contracts/observation-fixtures.mjs --vendor <ya-wamf checkout>`.
const yawamf = process.env.LIGHTNVR_YAWAMF_DIR ?? path.resolve(root, '../YetAnother-WhosAtMyFeeder');
const vendored = path.join(yawamf, 'backend/tests/fixtures/lightnvr');
test('the YA-WAMF vendored copy matches', {skip: !existsSync(vendored) && `no vendored copy at ${vendored}`}, async () => {
  requireManifest();
  const files = [...manifest.fixtures.map(e => e.file), 'manifest.json', 'README.md'].sort();
  assert.deepEqual((await listFiles(vendored)).filter(f => f !== 'SOURCE.md'), files, 'vendored file list');
  for (const file of files) {
    assert.ok((await readFile(path.join(vendored, file))).equals(await readFile(fixturePath(file))), `vendored ${file}`);
  }
  const source = await readFile(path.join(vendored, 'SOURCE.md'), 'utf8');
  assert.match(source, /lightNVR commit `[0-9a-f]{40}`/, 'SOURCE.md names the lightNVR commit');
});

test('the generator reproduces the committed fixtures', async () => {
  requireManifest();
  const {buildFixtures} = await import(pathToFileURL(path.join(root, 'scripts/contracts/observation-fixtures.mjs')));
  const built = await buildFixtures();
  for (const [file, content] of built) {
    assert.equal(content, await readFile(fixturePath(file), 'utf8'), `${file} is up to date`);
  }
});
