import test from 'node:test';
import assert from 'node:assert/strict';
import {readFile} from 'node:fs/promises';

// INT_01 C2: the observations contract (docs/integrations/OBSERVATIONS_CONTRACT.md)
// is present in canonical OpenAPI with the frozen authorization mapping (§12.1),
// and every operation maps to a registered native route.
const root = new URL('../../../', import.meta.url);
const read = file => readFile(new URL(file, root), 'utf8');

const camera = action => ({version: 1, kind: 'camera', requirements: [{action, resource: 'raw'}]});
const expected = [
  {id: 'getIntegrationCapabilities', method: 'get', path: '/api/integrations/capabilities',
    route: '/api/integrations/capabilities', handler: 'handle_get_integration_capabilities',
    auth: {version: 1, kind: 'authenticated'}},
  {id: 'listObservations', method: 'get', path: '/api/observations',
    route: '/api/observations', handler: 'handle_get_observations', auth: camera('live.view')},
  {id: 'getObservation', method: 'get', path: '/api/observations/{id}',
    route: '/api/observations/#', handler: 'handle_get_observation', auth: camera('live.view')},
  {id: 'getObservationSnapshot', method: 'get', path: '/api/observations/{id}/snapshot',
    route: '/api/observations/#/snapshot', handler: 'handle_get_observation_snapshot',
    auth: camera('snapshot.create')},
  // P2: observations.annotate is not in the action catalog until L13, so the
  // stub is authenticated-only; L14 switches it to camera observations.annotate.
  {id: 'putObservationClassification', method: 'put',
    path: '/api/observations/{id}/classifications/{namespace}/{name}',
    route: '/api/observations/#/classifications/#/#', handler: 'handle_put_observation_classification',
    auth: {version: 1, kind: 'authenticated'}},
];
const schemas = ['Observation', 'ObservationEvidence', 'ObservationTransform', 'ObservationPage',
  'ObservationChangeEvent', 'ObservationClassification', 'IntegrationCapabilities', 'ObservationError'];

test('observation operations carry the frozen authorization mapping', async () => {
  const doc = JSON.parse(await read('docs/api/openapi.json'));
  for (const op of expected) {
    const operation = doc.paths[op.path]?.[op.method];
    assert.ok(operation, `missing ${op.method.toUpperCase()} ${op.path}`);
    assert.equal(operation.operationId, op.id);
    assert.doesNotMatch(operation.operationId, /Evidence/);
    assert.deepEqual(operation['x-lightnvr-authorization'], op.auth, `authorization: ${op.id}`);
    // L8 implemented the GET operations; only the P2 PUT stub still answers 501.
    if (op.id === 'putObservationClassification') {
      assert.ok(operation.responses['501'], `${op.id} documents the 501 stub response`);
    } else {
      assert.equal(operation.responses['501'], undefined, `${op.id} no longer returns 501`);
    }
    assert.ok(doc['x-lightnvr-coverage'].covered_operations.includes(op.id), `coverage: ${op.id}`);
  }
  for (const name of schemas) assert.ok(doc.components.schemas[name], `missing schema ${name}`);
});

test('observation operations map to registered native routes', async () => {
  const [coverageText, source, stubs] = await Promise.all([read('docs/api/contracts/coverage.json'),
    read('src/web/libuv_api_handlers.c'), read('src/web/api_handlers_observations.c')]);
  const coverage = JSON.parse(coverageText);
  for (const op of expected) {
    const entry = coverage.operations.find(e => e.operation_id === op.id);
    assert.ok(entry, `coverage entry: ${op.id}`);
    assert.equal(entry.method, op.method.toUpperCase());
    assert.equal(entry.route_pattern, op.route);
    assert.equal(entry.canonical_path, op.path);
    assert.equal(entry.handler, op.handler);
    assert.ok(source.includes(`"${op.route}", "${op.method.toUpperCase()}"`), `registration: ${op.id}`);
    assert.match(stubs, new RegExp(`void ${op.handler}\\(`), `handler definition: ${op.handler}`);
  }
});

test('the observation schemas accept the contract examples', async () => {
  const validators = await import(new URL('web/generated/api/validators.js', root));
  const observation = {
    id: '55555555-5555-4555-8555-555555555555',
    source: 'urn:lightnvr:11111111-1111-4111-8111-111111111111',
    camera_uuid: '22222222-2222-4222-8222-222222222222',
    stream_name_at_capture: 'feeder-east',
    batch_id: '77777777-7777-4777-8777-777777777777',
    object_index: 0,
    observed_at: '2026-11-08T14:03:12.481Z',
    created_at: '2026-11-08T14:03:12.530Z',
    detector: {engine_key: 'legacy-primary', engine_type: 'object', model: 'ssd_mobilenet_v2.sod', model_version: null},
    label: 'bird', confidence: 0.87,
    box: {format: 'xywh_normalized', x: 0.412, y: 0.288, width: 0.12, height: 0.165},
    evidence: {
      asset_id: '88888888-8888-4888-8888-888888888888', state: 'ready', reason: null,
      origin: 'inference_input', media_type: 'image/jpeg', width: 640, height: 360, bytes: 48211,
      captured_at: '2026-11-08T14:03:12.481Z', expires_at: '2026-11-15T14:03:12.481Z',
      transform: {mode: 'none', source_width: 640, source_height: 360, inference_width: 640,
        inference_height: 360, scale_x: 1.0, scale_y: 1.0, pad_x: 0, pad_y: 0},
      snapshot_url: '/api/observations/55555555-5555-4555-8555-555555555555/snapshot',
    },
    track: null, recording_reference: {recording_id: 81234}, revision: 2,
    classifications: {automated: [], owner_confirmed: []},
  };
  assert.equal(validators.validateObservation(observation), true);
  assert.equal(validators.validateObservationPage({items: [observation], next_cursor: 'eyJ2IjoxfQ', has_more: false}), true);
  assert.equal(validators.validateObservation({...observation, evidence: {...observation.evidence, state: 'done'}}), false);
  assert.equal(validators.validateObservationChangeEvent({
    observation_id: observation.id, revision: 2, camera_uuid: observation.camera_uuid, label: 'bird',
    observed_at: observation.observed_at, change_cause: 'evidence', evidence_state: 'ready',
    evidence_reason: null, asset_id: observation.evidence.asset_id, snapshot_url: observation.evidence.snapshot_url,
  }), true);
  assert.equal(validators.validateObservationError({error: 'Not implemented', code: 'not_implemented'}), true);
  assert.equal(validators.validateObservationError({
    error: 'Cursor is older than retained observation history', code: 'cursor_expired',
    oldest_cursor: 'eyJ2IjoxfQ', oldest_observed_at: null,
  }), true);
  assert.equal(validators.validateApiError({error: 'Not implemented', code: 'not_implemented'}), true);
});
