/**
 * INT_01 P2 classification PUT against the real native server (L14):
 * A12 (retry no-op, stale revision 409, owner overwrite 409, evidence
 * mismatch 422), A7-annotate (another camera's observation is 404 or 403 and
 * nothing is written) and the classification change event on a real MQTT
 * sink with its stable message ID. Contract:
 * docs/integrations/OBSERVATIONS_CONTRACT.md sections 5.4, 10, 11 and 12.
 */
import {createHash, randomUUID} from 'node:crypto';
import path from 'node:path';
import {DatabaseSync} from 'node:sqlite';
import {startMqttSink} from './mqtt-sink';
import {test, expect, ok, admin, viewer, wire, valid, createUser, role, policy, token, cameraScope, database, type Server, type Identity, type Json} from './permissionCases';

const EVENT = 'io.lightnvr.detection.observation_changed.v1';

function uuidv5(namespace: string, name: string): string {
  const hash = createHash('sha1').update(Buffer.from(namespace.replaceAll('-', ''), 'hex')).update(Buffer.from(name, 'utf8')).digest();
  hash[6] = (hash[6] & 0x0f) | 0x50; hash[8] = (hash[8] & 0x3f) | 0x80;
  const hex = hash.subarray(0, 16).toString('hex');
  return `${hex.slice(0, 8)}-${hex.slice(8, 12)}-${hex.slice(12, 16)}-${hex.slice(16, 20)}-${hex.slice(20)}`;
}
// Section 5.4 (also tests/fixtures/integrations/observations/message-ids.json).
const MESSAGE_ID_NAMESPACE = uuidv5('6ba7b811-9dad-11d1-80b4-00c04fd430c8', 'https://lightnvr.io/ns/observation-change/v1');
const messageId = (source: string, observation: string, revision: number) => uuidv5(MESSAGE_ID_NAMESPACE, `${source}|${observation}|${revision}`);

type Seeded = {observation: string; asset: string};
/** A ready observation at revision 2, written straight into the isolated fixture DB. */
function seedReady(server: Server, camera: 'A' | 'B'): Seeded {
  const now = Date.now(), observation = randomUUID(), asset = randomUUID();
  const uuid = server.cameras[camera].camera_uuid;
  database(server, `INSERT INTO observation_assets (uuid, camera_uuid, state, origin, media_type, path, bytes, width, height, captured_at_ms, expires_at_ms, created_at_ms, updated_at_ms)
    VALUES (?, ?, 'ready', 'inference_input', 'image/jpeg', ?, 10, 640, 360, ?, ?, ?, ?)`,
  [asset, uuid, `observations/${uuid}/2026-11-08/${asset}.jpg`, now, now + 86400000, now, now]);
  database(server, `INSERT INTO object_observations (uuid, camera_uuid, stream_name_at_capture, batch_id, object_index, observed_at_ms, created_at_ms, engine_key, engine_type, label, confidence, asset_uuid, revision)
    VALUES (?, ?, ?, ?, 0, ?, ?, 'legacy-primary', 'object', 'bird', 0.87, ?, 2)`,
  [observation, uuid, camera, randomUUID(), now, now, asset]);
  return {observation, asset};
}
function rows(server: Server, sql: string, value: string): number {
  const db = new DatabaseSync(path.join(server.directory, 'lightnvr.db'));
  try { db.exec('PRAGMA busy_timeout=3000'); return Number((db.prepare(sql).get(value) as Json).n); } finally { db.close(); }
}
const annotations = (server: Server, observation: string) => rows(server, 'SELECT count(*) AS n FROM observation_classifications WHERE observation_uuid = ?', observation);
const revision = (server: Server, observation: string) => rows(server, 'SELECT revision AS n FROM object_observations WHERE uuid = ?', observation);

const route = (seeded: Seeded, namespace: string, name = 'species') => `/api/observations/${seeded.observation}/classifications/${namespace}/${name}`;
function body(seeded: Seeded, extra: Json = {}): Json {
  return {classifier: 'yawamf', model_version: 'convnext-v2-2026.10', label: 'Black-capped Chickadee', taxonomy_id: 'inat:144815',
    confidence: 0.93, classified_at: '2026-11-08T14:03:15.002Z', input_asset_id: seeded.asset, input_revision: 2, kind: 'automated',
    expected_revision: 0, ...extra};
}
async function put(server: Server, path: string, payload: Json, identity: Identity = admin) {
  const result = await wire(server, path, identity, 'PUT', payload);
  valid(result.response.ok ? 'ObservationClassificationResult' : 'ObservationError', result.body);
  return result;
}

test('A12: retries are no-ops, stale and automated-over-owner writes conflict, mismatched evidence is 422', async ({server}) => {
  const capabilities = await ok(server, '/api/integrations/capabilities');
  valid('IntegrationCapabilities', capabilities);
  expect(capabilities.operations['observations.annotate']).toEqual({supported: true, authorized: true});
  const namespace = capabilities.principal.writer_namespace;
  expect(namespace).toBe('admin');
  const cameraA = capabilities.cameras.find((camera: Json) => camera.camera_uuid === server.cameras.A.camera_uuid);
  expect(cameraA.actions).toContain('observations.annotate');

  const seeded = seedReady(server, 'A');
  const created = await put(server, route(seeded, namespace), body(seeded));
  expect(created.response.status).toBe(201);
  expect(created.body).toMatchObject({changed: true, observation_revision: 3, annotation: {key: `${namespace}/species`, writer_namespace: namespace, annotation_revision: 1, kind: 'automated'}});

  const retried = await put(server, route(seeded, namespace), body(seeded));
  expect(retried.response.status).toBe(200);
  expect(retried.body).toEqual({...created.body, changed: false});
  expect(revision(server, seeded.observation)).toBe(3);

  const stale = await put(server, route(seeded, namespace), body(seeded, {confidence: 0.95, expected_revision: 0}));
  expect(stale.response.status).toBe(409);
  expect(stale.body).toMatchObject({code: 'revision_conflict', current: created.body.annotation});

  const owner = await put(server, route(seeded, namespace), body(seeded, {label: 'Carolina Chickadee', confidence: 1, model_version: null, taxonomy_id: null, kind: 'owner_confirmed', input_revision: 3, expected_revision: 1}));
  expect(owner.response.status).toBe(200);
  expect(owner.body).toMatchObject({changed: true, observation_revision: 4, annotation: {annotation_revision: 2, kind: 'owner_confirmed', model_version: null}});

  const locked = await put(server, route(seeded, namespace), body(seeded, {confidence: 0.95, input_revision: 4, expected_revision: 2}));
  expect(locked.response.status).toBe(409);
  expect(locked.body).toMatchObject({code: 'owner_decision_locked', current: owner.body.annotation});

  const mismatch = await put(server, route(seeded, namespace, 'other'), body(seeded, {input_asset_id: randomUUID()}));
  expect(mismatch.response.status).toBe(422);
  expect(mismatch.body).toMatchObject({code: 'evidence_mismatch', observation_asset_id: seeded.asset, evidence_state: 'ready'});

  const unknown = await put(server, route(seeded, namespace, 'other'), body(seeded, {surprise: true}));
  expect(unknown.response.status).toBe(400);
  expect(unknown.body).toMatchObject({code: 'invalid_request', field: 'surprise'});

  // Reads separate the owner decision from automated results and keep the detector label.
  const observation = await ok(server, `/api/observations/${seeded.observation}`);
  valid('Observation', observation);
  expect(observation.label).toBe('bird');
  expect(observation.revision).toBe(4);
  expect(observation.classifications).toEqual({automated: [], owner_confirmed: [owner.body.annotation]});
  const page = await ok(server, `/api/observations?camera_uuid=${server.cameras.A.camera_uuid}`);
  valid('ObservationPage', page);
  expect(page.items.find((item: Json) => item.id === seeded.observation).classifications).toEqual(observation.classifications);
  expect(annotations(server, seeded.observation)).toBe(1);
});

test('A7-annotate: another camera is 404, view without annotate is 403, and denials write nothing', async ({server}) => {
  const onA = seedReady(server, 'A'), onB = seedReady(server, 'B');
  const annotator = await role(server, 'Contract annotator', ['live.view', 'observations.annotate']);
  const yawamf = await createUser(server, 'yawamf');
  await policy(server, yawamf, [{role_uuid: annotator, scope: cameraScope(server.cameras.A.camera_uuid)}]);
  const capabilities = await ok(server, '/api/integrations/capabilities', yawamf);
  expect(capabilities.principal.writer_namespace).toBe('yawamf');
  expect(capabilities.cameras.map((camera: Json) => camera.camera_uuid)).toEqual([server.cameras.A.camera_uuid]);

  const hidden = await put(server, route(onB, 'yawamf'), body(onB), yawamf);
  expect(hidden.response.status).toBe(404);
  expect(hidden.body.code).toBe('observation_not_found');
  const missing = await put(server, `/api/observations/${randomUUID()}/classifications/yawamf/species`, body(onB), yawamf);
  expect(missing.body).toEqual(hidden.body);
  // Authorization runs before the namespace and body checks.
  const hiddenBad = await wire(server, route(onB, 'someone-else'), yawamf, 'PUT', {nonsense: 1});
  expect(hiddenBad.response.status).toBe(404);
  expect(hiddenBad.body).toEqual(hidden.body);

  // The fixture viewer holds live.view on A, not observations.annotate.
  const forbidden = await put(server, route(onA, 'contract_viewer'), body(onA), viewer);
  expect(forbidden.response.status).toBe(403);
  expect(forbidden.body.code).toBe('forbidden');
  const otherNamespace = await put(server, route(onA, 'admin'), body(onA), yawamf);
  expect(otherNamespace.response.status).toBe(403);
  expect(otherNamespace.body.code).toBe('namespace_forbidden');
  expect(annotations(server, onA.observation)).toBe(0);
  expect(annotations(server, onB.observation)).toBe(0);
  expect(revision(server, onA.observation)).toBe(2);
  expect(revision(server, onB.observation)).toBe(2);

  const created = await put(server, route(onA, 'yawamf'), body(onA), yawamf);
  expect(created.response.status).toBe(201);
  // Scoped tokens carry the action explicitly; a read-only token cannot annotate.
  const writeToken = await token(server, yawamf, cameraScope(server.cameras.A.camera_uuid), ['live.view', 'observations.annotate']);
  const viaToken = await put(server, route(onA, 'yawamf', 'species-token'), body(onA, {input_revision: 3}), writeToken);
  expect(viaToken.response.status).toBe(201);
  const readToken = await token(server, yawamf, cameraScope(server.cameras.A.camera_uuid), ['live.view']);
  const denied = await put(server, route(onA, 'yawamf', 'read-only'), body(onA, {input_revision: 3}), readToken);
  expect(denied.response.status).toBe(403);
  expect(denied.body.code).toBe('forbidden');
  expect(annotations(server, onA.observation)).toBe(2);
});

test('a stored classification reaches a real MQTT sink once, with its stable message ID', async ({server}) => {
  test.skip(process.env.LIGHTNVR_CONTRACT_MQTT !== '1', 'Requires the explicitly MQTT-enabled contract build (enabled in CI).');
  const sink = await startMqttSink();
  try {
    const destination = await ok(server, '/api/event-destinations', admin, 'POST', {name: 'Observation sink', broker: {host: '127.0.0.1', port: sink.port, client_id: 'observation-contract-sink', topic_template: 'fixture/{type}/{subject_id}', qos: 1, keepalive_seconds: 5}, tls: {mode: 'disabled'}});
    await ok(server, '/api/event-routes', admin, 'POST', {name: 'Observation route', destination: destination.key, event_types: [EVENT], camera_scope: {type: 'selector', selector: {version: 1, expression: {op: 'camera_uuid', values: [server.cameras.A.camera_uuid]}}}});
    const source = (await ok(server, '/api/integrations/capabilities')).source.id;
    const seeded = seedReady(server, 'A');
    const changes = () => sink.publications.map(p => JSON.parse(p.payload.toString())).filter(e => e.type === EVENT && e.data?.observation_id === seeded.observation);

    const created = await put(server, route(seeded, 'admin'), body(seeded));
    expect(created.response.status).toBe(201);
    await expect.poll(() => changes().length, {timeout: 20000}).toBeGreaterThan(0);
    const [event] = changes();
    expect(event.id).toBe(messageId(source, seeded.observation, 3));
    expect(event.source).toBe(source);
    expect(event.data).toMatchObject({revision: 3, change_cause: 'classification', evidence_state: 'ready', asset_id: seeded.asset, label: 'bird'});
    valid('ObservationChangeEvent', event.data);

    // An identical retry stages nothing, so no new message appears.
    expect((await put(server, route(seeded, 'admin'), body(seeded))).response.status).toBe(200);
    await new Promise(resolve => setTimeout(resolve, 2500));
    expect(new Set(changes().map(e => e.id))).toEqual(new Set([event.id]));
  } finally { await sink.stop(); }
});
