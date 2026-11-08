#!/usr/bin/env node
// INT_01 C3: build the canonical observation contract fixtures in
// tests/fixtures/integrations/observations/ from one coherent scenario, and
// vendor them into YA-WAMF (backend/tests/fixtures/lightnvr/).
//
//   node scripts/contracts/observation-fixtures.mjs            write the JSON fixtures
//   node scripts/contracts/observation-fixtures.mjs --check    fail if any JSON fixture is stale
//   node scripts/contracts/observation-fixtures.mjs --jpeg     also re-encode the evidence JPEG
//   node scripts/contracts/observation-fixtures.mjs --vendor <ya-wamf checkout> [--allow-dirty]
//   node scripts/contracts/observation-fixtures.mjs --check-vendor <ya-wamf checkout>
//
// The JPEG is encoded only when missing or with --jpeg, because libjpeg output
// can differ between sharp releases; the committed bytes are canonical and the
// JSON (bytes, sha256) is derived from them. Contract:
// docs/integrations/OBSERVATIONS_CONTRACT.md.
import {createHash} from 'node:crypto';
import {execFileSync} from 'node:child_process';
import {existsSync} from 'node:fs';
import {mkdir, readFile, readdir, rm, writeFile, copyFile} from 'node:fs/promises';
import path from 'node:path';
import {fileURLToPath} from 'node:url';

export const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
export const fixtureRel = 'tests/fixtures/integrations/observations';
export const fixtureDir = path.join(root, fixtureRel);
export const vendorRel = 'backend/tests/fixtures/lightnvr';

// ---- contract helpers (§5.4, §7.4) -------------------------------------------------
const NAMESPACE_URL = '6ba7b811-9dad-11d1-80b4-00c04fd430c8';
export function uuidv5(namespace, name) {
  const hash = createHash('sha1').update(Buffer.from(namespace.replaceAll('-', ''), 'hex'))
    .update(Buffer.from(name, 'utf8')).digest();
  hash[6] = (hash[6] & 0x0f) | 0x50;
  hash[8] = (hash[8] & 0x3f) | 0x80;
  const hex = hash.subarray(0, 16).toString('hex');
  return `${hex.slice(0, 8)}-${hex.slice(8, 12)}-${hex.slice(12, 16)}-${hex.slice(16, 20)}-${hex.slice(20)}`;
}
const NS_NAME = 'https://lightnvr.io/ns/observation-change/v1';
export const MESSAGE_ID_NAMESPACE = uuidv5(NAMESPACE_URL, NS_NAME);
export const messageId = (source, observationId, revision) =>
  uuidv5(MESSAGE_ID_NAMESPACE, `${source}|${observationId}|${revision}`);
export const encodeCursor = (seq, epoch) => Buffer.from(JSON.stringify({v: 1, seq, epoch})).toString('base64url');
const secondPrecision = time => time.replace(/\.\d{3}Z$/, 'Z');
const round6 = value => Number(value.toFixed(6));

// ---- scenario constants ------------------------------------------------------------
const INSTALLATION = '11111111-1111-4111-8111-111111111111';
const SOURCE = `urn:lightnvr:${INSTALLATION}`;
const CAMERA = '22222222-2222-4222-8222-222222222222';
const EPOCH = '66666666-6666-4666-8666-666666666666';
const TOPIC_PREFIX = 'lightnvr';
const EVENT_TYPE = 'io.lightnvr.detection.observation_changed.v1';
const OLDEST_SEQ = 1001;
const PAGE_WATERMARK = 1010;
const WRITER = {username: 'yawamf', writer_namespace: 'yawamf'};
const TRACK_SESSION = '9b0c7d1e-5f43-4b8e-9d0e-0b6c1f2a3d4e';

const IDS = {
  A: {id: '55555555-5555-4555-8555-555555555555', batch: '77777777-7777-4777-8777-777777777777', asset: '88888888-8888-4888-8888-888888888888'},
  B: {id: 'bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb', batch: 'b0b0b0b0-b0b0-4b0b-8b0b-b0b0b0b0b0b0', asset: 'b1b1b1b1-b1b1-4b1b-8b1b-b1b1b1b1b1b1'},
  C0: {id: 'cccccccc-cccc-4ccc-8ccc-cccccccccc00', batch: 'aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa', asset: '99999999-9999-4999-8999-999999999999'},
  C1: {id: 'cccccccc-cccc-4ccc-8ccc-cccccccccc01', batch: 'aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa', asset: '99999999-9999-4999-8999-999999999999'},
  D: {id: 'dddddddd-dddd-4ddd-8ddd-dddddddddddd', batch: 'ffffffff-ffff-4fff-8fff-ffffffffffff', asset: 'eeeeeeee-eeee-4eee-8eee-eeeeeeeeeeee'},
  E: {id: 'e0e0e0e0-e0e0-4e0e-8e0e-e0e0e0e0e0e0', batch: 'e1e1e1e1-e1e1-4e1e-8e1e-e1e1e1e1e1e1', asset: 'e2e2e2e2-e2e2-4e2e-8e2e-e2e2e2e2e2e2'},
};

// Evidence JPEG: 160x80, boxes on 8-pixel block boundaries so the edges are
// exact after 4:4:4 JPEG coding.
const JPEG_FILE = 'media/two-birds-160x80.jpg';
const JPEG = {
  width: 160, height: 80, background_rgb: [198, 204, 188],
  boxes: [
    {key: 'C0', rgb: [196, 52, 40], pixels: {x: 16, y: 16, width: 32, height: 32}},
    {key: 'C1', rgb: [40, 84, 196], pixels: {x: 96, y: 24, width: 48, height: 40}},
  ],
};
const normalizedBox = ({x, y, width, height}, w, h) =>
  ({format: 'xywh_normalized', x: round6(x / w), y: round6(y / h), width: round6(width / w), height: round6(height / h)});

// Letterboxed HTTP detector: 1280x720 submitted JPEG, server letterboxed to
// 640x640 (scale 0.5, 140 px top/bottom padding).
const LETTERBOX = {stored: {width: 1280, height: 720}, inference: {width: 640, height: 640}, scale: 0.5, pad_x: 0, pad_y: 140,
  inference_box: {u: 256, v: 185, width: 128, height: 72}};
const letterboxBox = (() => {
  const {u, v, width, height} = LETTERBOX.inference_box;
  const {stored, scale, pad_x: padX, pad_y: padY} = LETTERBOX;
  return {format: 'xywh_normalized', x: round6((u - padX) / scale / stored.width), y: round6((v - padY) / scale / stored.height),
    width: round6(width / scale / stored.width), height: round6(height / scale / stored.height)};
})();

// ---- annotations (§10) -------------------------------------------------------------
const automatedWrite = {
  classifier: 'yawamf', model_version: 'convnext-v2-2026.10', label: 'Black-capped Chickadee', taxonomy_id: 'inat:144815',
  confidence: 0.93, classified_at: '2026-11-08T14:03:15.002Z', input_asset_id: IDS.A.asset, input_revision: 2,
  kind: 'automated', expected_revision: 0,
};
const ownerWrite = {
  classifier: 'yawamf', model_version: null, label: 'Carolina Chickadee', taxonomy_id: null, confidence: 1,
  classified_at: '2026-11-08T18:20:41.100Z', input_asset_id: IDS.A.asset, input_revision: 3,
  kind: 'owner_confirmed', expected_revision: 1,
};
const annotation = (write, annotationRevision, createdAt, updatedAt) => {
  const {expected_revision: _, ...fields} = write;
  return {key: 'yawamf/species', writer_namespace: 'yawamf', ...fields, annotation_revision: annotationRevision,
    created_at: createdAt, updated_at: updatedAt};
};
const automatedAnnotation = annotation(automatedWrite, 1, '2026-11-08T14:03:15.120Z', '2026-11-08T14:03:15.120Z');
const ownerAnnotation = annotation(ownerWrite, 2, '2026-11-08T14:03:15.120Z', '2026-11-08T18:20:41.300Z');

// ---- change log (§5): the single persistence order of the scenario ------------------
const changes = [
  {seq: 1001, key: 'A', revision: 1, cause: 'created', state: 'pending', reason: null, at: '2026-11-08T14:03:12.530Z'},
  {seq: 1002, key: 'B', revision: 1, cause: 'created', state: 'unavailable', reason: 'queue_full', at: '2026-11-08T14:03:12.611Z'},
  {seq: 1003, key: 'A', revision: 2, cause: 'evidence', state: 'ready', reason: null, at: '2026-11-08T14:03:12.712Z'},
  {seq: 1004, key: 'C0', revision: 1, cause: 'created', state: 'pending', reason: null, at: '2026-11-08T14:03:13.291Z'},
  {seq: 1005, key: 'C1', revision: 1, cause: 'created', state: 'pending', reason: null, at: '2026-11-08T14:03:13.291Z'},
  {seq: 1006, key: 'C0', revision: 2, cause: 'evidence', state: 'ready', reason: null, at: '2026-11-08T14:03:13.402Z'},
  {seq: 1007, key: 'C1', revision: 2, cause: 'evidence', state: 'ready', reason: null, at: '2026-11-08T14:03:13.402Z'},
  {seq: 1008, key: 'D', revision: 1, cause: 'created', state: 'pending', reason: null, at: '2026-11-08T14:03:14.085Z'},
  {seq: 1009, key: 'D', revision: 2, cause: 'evidence', state: 'ready', reason: null, at: '2026-11-08T14:03:14.140Z'},
  {seq: 1010, key: 'A', revision: 3, cause: 'classification', state: 'ready', reason: null, at: '2026-11-08T14:03:15.120Z'},
  {seq: 1011, key: 'E', revision: 1, cause: 'created', state: 'unavailable', reason: 'no_matching_media', at: '2026-11-08T15:30:02.018Z'},
  {seq: 1012, key: 'A', revision: 4, cause: 'classification', state: 'ready', reason: null, at: '2026-11-08T18:20:41.300Z'},
  {seq: 1013, key: 'A', revision: 5, cause: 'evidence', state: 'expired', reason: 'retention_expired', at: '2026-11-15T14:03:13.004Z'},
];
// seq is persistence order (§7): B is dropped (queue_full) while A is still
// encoding, so its created row commits before A's ready transition.

// ---- observations (§3, §4) ---------------------------------------------------------
const detectorLocal = {engine_key: 'legacy-primary', engine_type: 'object', model: 'ssd_mobilenet_v2.sod', model_version: null};
const none = (w, h) => ({mode: 'none', source_width: w, source_height: h, inference_width: w, inference_height: h,
  scale_x: 1, scale_y: 1, pad_x: 0, pad_y: 0});
const noMedia = asset => ({asset_id: asset, state: 'unavailable', reason: null, origin: null, media_type: null, width: null,
  height: null, bytes: null, captured_at: null, expires_at: null, transform: null});

function observationA(revision) {
  const base = {
    id: IDS.A.id, source: SOURCE, camera_uuid: CAMERA, stream_name_at_capture: 'feeder-east', batch_id: IDS.A.batch,
    object_index: 0, observed_at: '2026-11-08T14:03:12.481Z', created_at: '2026-11-08T14:03:12.530Z',
    detector: detectorLocal, label: 'bird', confidence: 0.87,
    box: {format: 'xywh_normalized', x: 0.412, y: 0.288, width: 0.12, height: 0.165},
  };
  const media = {asset_id: IDS.A.asset, origin: 'inference_input', media_type: 'image/jpeg', width: 640, height: 360, bytes: 48211,
    captured_at: '2026-11-08T14:03:12.481Z', expires_at: '2026-11-15T14:03:12.481Z', transform: none(640, 360)};
  const evidence = revision === 1
    ? {...media, state: 'pending', reason: null, width: null, height: null, bytes: null, expires_at: null}
    : revision === 5
      ? {...media, state: 'expired', reason: 'retention_expired'}
      : {...media, state: 'ready', reason: null, snapshot_url: `/api/observations/${IDS.A.id}/snapshot`};
  const classifications = revision <= 2 ? {automated: [], owner_confirmed: []}
    : revision === 3 ? {automated: [automatedAnnotation], owner_confirmed: []}
      : {automated: [], owner_confirmed: [ownerAnnotation]};
  return {...base, evidence: order(evidence), track: null, recording_reference: {recording_id: 81234}, revision, classifications};
}
// Evidence key order as in contract §3.6.
const EVIDENCE_KEYS = ['asset_id', 'state', 'reason', 'origin', 'media_type', 'width', 'height', 'bytes', 'captured_at',
  'expires_at', 'transform', 'snapshot_url'];
const order = evidence => Object.fromEntries(EVIDENCE_KEYS.filter(k => k in evidence).map(k => [k, evidence[k]]));

function observationB() {
  return {
    id: IDS.B.id, source: SOURCE, camera_uuid: CAMERA, stream_name_at_capture: 'feeder-east', batch_id: IDS.B.batch,
    object_index: 0, observed_at: '2026-11-08T14:03:12.598Z', created_at: '2026-11-08T14:03:12.611Z',
    detector: detectorLocal, label: 'bird', confidence: 0.71,
    box: {format: 'xywh_normalized', x: 0.05, y: 0.4, width: 0.1, height: 0.14},
    evidence: {...noMedia(IDS.B.asset), reason: 'queue_full'},
    track: null, recording_reference: {recording_id: 81234}, revision: 1, classifications: {automated: [], owner_confirmed: []},
  };
}

function observationC(index, jpegBytes) {
  const key = index === 0 ? 'C0' : 'C1';
  const box = JPEG.boxes[index];
  return {
    id: IDS[key].id, source: SOURCE, camera_uuid: CAMERA, stream_name_at_capture: 'feeder-east', batch_id: IDS[key].batch,
    object_index: index, observed_at: '2026-11-08T14:03:13.250Z', created_at: '2026-11-08T14:03:13.291Z',
    detector: detectorLocal, label: 'bird', confidence: index === 0 ? 0.91 : 0.78,
    box: normalizedBox(box.pixels, JPEG.width, JPEG.height),
    evidence: {
      asset_id: IDS[key].asset, state: 'ready', reason: null, origin: 'inference_input', media_type: 'image/jpeg',
      width: JPEG.width, height: JPEG.height, bytes: jpegBytes.length,
      captured_at: '2026-11-08T14:03:13.250Z', expires_at: '2026-11-15T14:03:13.250Z',
      // SOD pre-resize to a 320x320 tensor: independent x/y scale (§4.4).
      transform: {mode: 'resize', source_width: JPEG.width, source_height: JPEG.height, inference_width: 320,
        inference_height: 320, scale_x: 320 / JPEG.width, scale_y: 320 / JPEG.height, pad_x: 0, pad_y: 0},
      snapshot_url: `/api/observations/${IDS[key].id}/snapshot`,
    },
    track: null, recording_reference: {recording_id: 81235}, revision: 2, classifications: {automated: [], owner_confirmed: []},
  };
}

function observationD() {
  const {stored, inference, scale} = LETTERBOX;
  return {
    id: IDS.D.id, source: SOURCE, camera_uuid: CAMERA, stream_name_at_capture: 'feeder-east', batch_id: IDS.D.batch,
    object_index: 0, observed_at: '2026-11-08T14:03:14.000Z', created_at: '2026-11-08T14:03:14.085Z',
    detector: {engine_key: 'http-yolo', engine_type: 'api', model: 'yolov8n', model_version: '8.3.0'}, label: 'bird',
    confidence: 0.84, box: letterboxBox,
    evidence: {
      asset_id: IDS.D.asset, state: 'ready', reason: null, origin: 'submitted_image', media_type: 'image/jpeg',
      width: stored.width, height: stored.height, bytes: 182340,
      captured_at: '2026-11-08T14:03:14.000Z', expires_at: '2026-11-15T14:03:14.000Z',
      transform: {mode: 'letterbox', source_width: stored.width, source_height: stored.height, inference_width: inference.width,
        inference_height: inference.height, scale_x: scale, scale_y: scale, pad_x: LETTERBOX.pad_x, pad_y: LETTERBOX.pad_y},
      snapshot_url: `/api/observations/${IDS.D.id}/snapshot`,
    },
    track: {id: '17', namespace: 'http-yolo', session: TRACK_SESSION},
    recording_reference: {recording_id: 81236}, revision: 2, classifications: {automated: [], owner_confirmed: []},
  };
}

function observationE() {
  return {
    id: IDS.E.id, source: SOURCE, camera_uuid: CAMERA, stream_name_at_capture: 'feeder-east', batch_id: IDS.E.batch,
    object_index: 0, observed_at: '2026-11-08T15:30:01.900Z', created_at: '2026-11-08T15:30:02.018Z',
    detector: {engine_key: 'onvif-events', engine_type: 'onvif', model: null, model_version: null}, label: 'animal',
    confidence: 1, box: null, evidence: {...noMedia(IDS.E.asset), reason: 'no_matching_media'},
    track: null, recording_reference: null, revision: 1, classifications: {automated: [], owner_confirmed: []},
  };
}

// ---- builders ----------------------------------------------------------------------
const json = value => `${JSON.stringify(value, null, 2)}\n`;
const errorBody = (code, error, extra = {}) => ({error, code, ...extra});

async function encodeJpeg() {
  const {default: sharp} = await import('sharp');
  const {width, height} = JPEG;
  const raw = Buffer.alloc(width * height * 3);
  for (let y = 0; y < height; y++) {
    for (let x = 0; x < width; x++) {
      const box = JPEG.boxes.find(({pixels: p}) => x >= p.x && x < p.x + p.width && y >= p.y && y < p.y + p.height);
      raw.set(box ? box.rgb : JPEG.background_rgb, (y * width + x) * 3);
    }
  }
  return sharp(raw, {raw: {width, height, channels: 3}})
    .jpeg({quality: 95, chromaSubsampling: '4:4:4', progressive: false, mozjpeg: false, optimiseCoding: false})
    .toBuffer();
}

export async function loadJpeg({regenerate = false} = {}) {
  const file = path.join(fixtureDir, JPEG_FILE);
  if (!regenerate && existsSync(file)) return readFile(file);
  return encodeJpeg();
}

// Returns Map<relative file, string content> for every generated JSON fixture.
export async function buildFixtures({jpeg} = {}) {
  const jpegBytes = jpeg ?? await loadJpeg();
  const files = new Map();
  const fixtures = [];
  const add = (file, value, meta) => { files.set(file, json(value)); fixtures.push({file, ...meta}); };

  // Observations.
  const obs = {
    A1: observationA(1), A2: observationA(2), A3: observationA(3), A4: observationA(4), A5: observationA(5),
    B1: observationB(), C0: observationC(0, jpegBytes), C1: observationC(1, jpegBytes), D2: observationD(), E1: observationE(),
  };
  const obsFile = {
    A1: 'observations/single-pending.json', A2: 'observations/single-ready.json',
    A3: 'observations/single-ready-classified.json', A4: 'observations/single-owner-confirmed.json',
    A5: 'observations/single-expired.json', B1: 'observations/single-unavailable.json',
    C0: 'observations/two-birds-object-0.json', C1: 'observations/two-birds-object-1.json',
    D2: 'observations/http-letterbox.json', E1: 'observations/onvif-no-matching-media.json',
  };
  const obsDescription = {
    A1: 'Single bird, revision 1 (created): evidence pending; dimensions and bytes are set once ready.',
    A2: 'Single bird, revision 2 (evidence): evidence ready. Identical to contract §3.6.',
    A3: 'Single bird, revision 3 (classification): automated yawamf/species annotation stored.',
    A4: 'Single bird, revision 4 (classification): owner_confirmed correction replaced the automated annotation.',
    A5: 'Single bird, revision 5 (evidence): media expired by retention; metadata, dimensions and transform kept.',
    B1: 'Single bird created directly unavailable (queue_full): no media fields, no transform.',
    C0: 'Two birds in one frame, object 0: shares asset and batch with object 1. Box matches media/two-birds-160x80.json.',
    C1: 'Two birds in one frame, object 1: shares asset and batch with object 0. Box matches media/two-birds-160x80.json.',
    D2: 'HTTP API detector, submitted 1280x720 JPEG letterboxed server-side to 640x640; box already mapped to the stored image.',
    E1: 'ONVIF object-class event: no box, evidence unavailable (no_matching_media).',
  };
  for (const key of Object.keys(obs)) {
    add(obsFile[key], obs[key], {kind: 'observation', schema: 'Observation', observation_id: obs[key].id,
      revision: obs[key].revision, description: obsDescription[key],
      ...(key === 'D2' ? {inference_box: LETTERBOX.inference_box} : {})});
  }
  const obsAt = (key, revision) => ({A: {1: 'A1', 2: 'A2', 3: 'A3', 4: 'A4', 5: 'A5'}, B: {1: 'B1'}, C0: {2: 'C0'},
    C1: {2: 'C1'}, D: {2: 'D2'}, E: {1: 'E1'}})[key]?.[revision];

  // Change events: complete MQTT envelopes (event_envelope.c key order).
  const eventFiles = {1001: 'a-r1-created-pending', 1002: 'b-r1-created-unavailable', 1003: 'a-r2-evidence-ready',
    1006: 'c0-r2-evidence-ready', 1007: 'c1-r2-evidence-ready', 1010: 'a-r3-classification',
    1011: 'e-r1-created-no-matching-media', 1013: 'a-r5-evidence-expired'};
  const vectors = [];
  for (const change of changes) {
    const name = eventFiles[change.seq];
    if (!name) continue;
    const observation = obs[obsAt(change.key, change.revision)];
    const id = messageId(SOURCE, observation.id, change.revision);
    const data = {
      observation_id: observation.id, revision: change.revision, camera_uuid: observation.camera_uuid,
      label: observation.label, observed_at: observation.observed_at, change_cause: change.cause,
      evidence_state: change.state, evidence_reason: change.reason, asset_id: observation.evidence.asset_id,
      ...(change.state === 'ready' ? {snapshot_url: `/api/observations/${observation.id}/snapshot`} : {}),
    };
    const envelope = {specversion: '1.0', id, type: EVENT_TYPE, source: SOURCE, subject: `camera/${observation.camera_uuid}`,
      time: secondPrecision(change.at), datacontenttype: 'application/json', severity: 'info', sensitivity: 'operational', data};
    add(`events/${name}.json`, envelope, {kind: 'event', seq: change.seq, change_cause: change.cause,
      evidence_state: change.state, topic: `${TOPIC_PREFIX}/v1/events/${EVENT_TYPE}/${observation.camera_uuid}`, retain: false,
      observation: obsFile[obsAt(change.key, change.revision)],
      description: `MQTT payload for change seq ${change.seq} (${change.cause}, evidence ${change.state}).`});
    vectors.push({source: SOURCE, observation_id: observation.id, revision: change.revision,
      name: `${SOURCE}|${observation.id}|${change.revision}`, message_id: id});
  }

  // History pages (§7.3) at watermark PAGE_WATERMARK, limit 2.
  const current = (key, watermark) => {
    const latest = changes.filter(c => c.key === key && c.seq <= watermark).at(-1);
    return obs[obsAt(key, latest.revision)];
  };
  const page = (start, limit, watermark) => {
    const keys = [];
    let last = start;
    for (const change of changes.filter(c => c.seq > start && c.seq <= watermark)) {
      if (!keys.includes(change.key)) {
        if (keys.length === limit) break;
        keys.push(change.key);
      }
      last = change.seq;
    }
    return {items: keys.map(k => current(k, watermark)), next_cursor: encodeCursor(last, EPOCH),
      has_more: changes.some(c => c.seq > last && c.seq <= watermark), last};
  };
  let start = OLDEST_SEQ - 1;
  for (let chain = 1; chain <= 3; chain++) {
    const {last, ...body} = page(start, 2, PAGE_WATERMARK);
    const query = {...(chain > 1 ? {cursor: encodeCursor(start, EPOCH)} : {}), limit: '2'};
    add(`pages/page-${chain}.json`, body, {kind: 'page', schema: 'ObservationPage', chain, status: 200,
      request: {method: 'GET', path: '/api/observations', query}, watermark: PAGE_WATERMARK, next_seq: last,
      description: `History page ${chain} of 3 (limit 2, watermark seq ${PAGE_WATERMARK}). Items hold current state; ` +
        'next_cursor is the last change scanned.'});
    start = last;
  }
  add('pages/head.json', {items: [], next_cursor: encodeCursor(PAGE_WATERMARK, EPOCH), has_more: false},
    {kind: 'page', schema: 'ObservationPage', status: 200, request: {method: 'GET', path: '/api/observations', query: {cursor: 'head'}},
      watermark: PAGE_WATERMARK, next_seq: PAGE_WATERMARK, description: 'cursor=head (D10): no items, cursor at the watermark.'});

  // Error bodies (§8.2, §11).
  const oldestCursor = encodeCursor(OLDEST_SEQ - 1, EPOCH);
  const errors = [
    ['errors/cursor-expired.json', 410, {}, errorBody('cursor_expired', 'Cursor is older than retained observation history',
      {oldest_cursor: oldestCursor, oldest_observed_at: '2026-11-08T14:03:12.481Z'}),
    {method: 'GET', path: '/api/observations', query: {cursor: encodeCursor(412, EPOCH)}},
    'A cursor before the prune boundary. An epoch mismatch (DB restore) or a seq ahead of head returns the same body, e.g. ' +
      `cursor ${encodeCursor(1010, '0f0f0f0f-0f0f-4f0f-8f0f-0f0f0f0f0f0f')}.`],
    ['errors/invalid-cursor.json', 400, {}, errorBody('invalid_cursor', 'Malformed cursor'),
      {method: 'GET', path: '/api/observations', query: {cursor: 'not-a-cursor!'}}, 'Malformed cursor; restart without one.'],
    ['errors/evidence-pending.json', 202, {'Retry-After': '2'}, errorBody('evidence_pending', 'Evidence is not ready yet',
      {retry_after_seconds: 2}), {method: 'GET', path: `/api/observations/${IDS.A.id}/snapshot`},
    'Snapshot of single-pending.json. Retry-After header equals retry_after_seconds.'],
    ['errors/evidence-unavailable.json', 404, {}, errorBody('evidence_unavailable', 'Evidence is unavailable',
      {reason: 'queue_full'}), {method: 'GET', path: `/api/observations/${IDS.B.id}/snapshot`},
    'Snapshot of single-unavailable.json.'],
    ['errors/evidence-expired.json', 410, {}, errorBody('evidence_expired', 'Evidence media has expired',
      {reason: 'retention_expired', expired_at: '2026-11-15T14:03:13.004Z'}),
    {method: 'GET', path: `/api/observations/${IDS.A.id}/snapshot`}, 'Snapshot of single-expired.json; metadata stays readable.'],
    ['errors/observation-not-found.json', 404, {}, errorBody('observation_not_found', 'Observation not found'),
      {method: 'GET', path: '/api/observations/00000000-0000-4000-8000-000000000000'},
      'Unknown, unauthorized and deleted-camera observations are indistinguishable.'],
    ['errors/snapshot-forbidden.json', 403, {}, errorBody('forbidden', 'Forbidden'),
      {method: 'GET', path: `/api/observations/${IDS.A.id}/snapshot`}, 'live.view held without snapshot.create (D4).'],
    ['errors/not-implemented.json', 501, {}, errorBody('not_implemented', 'Not implemented'),
      {method: 'GET', path: '/api/observations'}, 'C2 stub response, until the operation lands (L8/L14).'],
    ['errors/service-unavailable.json', 503, {'Retry-After': '5'}, errorBody('service_unavailable', 'Service temporarily unavailable'),
      {method: 'GET', path: '/api/observations'}, 'TemporarilyUnreachable outcome; honour Retry-After.'],
  ];
  for (const [file, status, headers, body, request, description] of errors) {
    add(file, body, {kind: 'error', schema: 'ObservationError', status, code: body.code, headers, request, description});
  }

  // Capability documents (§9).
  const operation = (supported, authorized, reason) => ({supported, authorized, ...(reason ? {reason} : {})});
  const capabilities = (p2) => ({
    contract_versions: {observations: 1, classifications: 1, observation_event: EVENT_TYPE},
    source: {id: SOURCE, kind: 'lightnvr', installation_uuid: INSTALLATION, product_version: p2 ? '0.49.0' : '0.48.0'},
    principal: {writer_namespace: WRITER.writer_namespace},
    operations: {
      'observations.history': operation(true, true),
      'observations.snapshot': operation(true, true),
      'observations.annotate': p2 ? operation(true, true) : operation(false, false, 'not_available_in_this_version'),
      track_lifecycle: operation(false, false, 'not_implemented'),
      recorded_frame: operation(false, false, 'not_implemented'),
      event_clip: operation(false, false, 'not_implemented'),
      range_clip: operation(false, false, 'not_implemented'),
    },
    limits: {history_page_default: 100, history_page_max: 500, history_filter_cameras_max: 10, history_filter_labels_max: 8,
      max_objects_per_batch: 20, label_max_bytes: 31, snapshot_retry_after_max_seconds: 30, annotations_per_observation_max: 8},
    history: {retention_days: 30, oldest_cursor: oldestCursor, oldest_observed_at: '2026-11-08T14:03:12.481Z'},
    delivery: {mqtt_enabled: true, topic: `${TOPIC_PREFIX}/v1/events/${EVENT_TYPE}/{camera_uuid}`},
    cameras: [{camera_uuid: CAMERA, evidence_enabled: true,
      actions: p2 ? ['live.view', 'snapshot.create', 'observations.annotate'] : ['live.view', 'snapshot.create']}],
  });
  add('capabilities/p1.json', capabilities(false), {kind: 'capabilities', schema: 'IntegrationCapabilities', status: 200,
    request: {method: 'GET', path: '/api/integrations/capabilities'},
    description: 'P1 installation: annotate not available. writer_namespace is derived from the user (D3), here "yawamf".'});
  add('capabilities/p2.json', capabilities(true), {kind: 'capabilities', schema: 'IntegrationCapabilities', status: 200,
    request: {method: 'GET', path: '/api/integrations/capabilities'},
    description: 'P2 installation: annotate supported and authorized on the camera.'});

  // Classification PUT exchanges (§10.4). observation_file is the state before the request.
  const putPath = (id, namespace = 'yawamf', name = 'species') => `/api/observations/${id}/classifications/${namespace}/${name}`;
  const exchange = (description, observationFile, body, status, responseBody, path = putPath(IDS.A.id)) =>
    ({description, observation_file: observationFile, request: {method: 'PUT', path, principal: WRITER, body},
      response: {status, body: responseBody}});
  const conflictingAutomated = {...automatedWrite, model_version: 'convnext-v2-2026.11', confidence: 0.95,
    classified_at: '2026-11-08T14:04:02.550Z'};
  const rerunAfterOwner = {...automatedWrite, model_version: 'convnext-v2-2026.11', confidence: 0.95,
    classified_at: '2026-11-09T02:00:00.000Z', input_revision: 4, expected_revision: 2};
  const exchanges = [
    ['classifications/01-create-automated.json', exchange('Create an automated annotation: 201, observation revision 2 -> 3.',
      obsFile.A2, automatedWrite, 201, {changed: true, observation_revision: 3, annotation: automatedAnnotation})],
    ['classifications/02-retry-noop.json', exchange('Identical retry after a lost response: 200 changed=false regardless of ' +
      'expected_revision; nothing written, revision not bumped.', obsFile.A3, automatedWrite, 200,
    {changed: false, observation_revision: 3, annotation: automatedAnnotation})],
    ['classifications/03-revision-conflict.json', exchange('Different automated result sent with a stale expected_revision ' +
      '(0, key exists at 1): 409 revision_conflict with current. Refetch and reconcile.', obsFile.A3, conflictingAutomated, 409,
    errorBody('revision_conflict', 'Annotation revision does not match', {current: automatedAnnotation}))],
    ['classifications/04-owner-confirm.json', exchange('Owner correction replaces the automated annotation: 200, ' +
      'annotation_revision 1 -> 2, observation revision 3 -> 4.', obsFile.A3, ownerWrite, 200,
    {changed: true, observation_revision: 4, annotation: ownerAnnotation})],
    ['classifications/05-owner-decision-locked.json', exchange('Automated re-run after an owner decision: 409 ' +
      'owner_decision_locked with current. Drop the automated write.', obsFile.A4, rerunAfterOwner, 409,
    errorBody('owner_decision_locked', 'An owner-confirmed annotation cannot be replaced by an automated one',
      {current: ownerAnnotation}))],
    ['classifications/06-evidence-mismatch-asset.json', exchange('input_asset_id names another observation\'s asset: 422.',
      obsFile.A2, {...automatedWrite, input_asset_id: IDS.C0.asset}, 422,
      errorBody('evidence_mismatch', 'Annotation input does not match the observation evidence',
        {observation_asset_id: IDS.A.asset, evidence_state: 'ready'}))],
    ['classifications/07-evidence-mismatch-pending.json', exchange('Evidence still pending (not ready or expired): 422.',
      obsFile.A1, {...automatedWrite, input_revision: 1}, 422,
      errorBody('evidence_mismatch', 'Annotation input does not match the observation evidence',
        {observation_asset_id: IDS.A.asset, evidence_state: 'pending'}))],
    ['classifications/08-namespace-forbidden.json', exchange('Key namespace is not the principal\'s derived namespace: 403.',
      obsFile.A2, automatedWrite, 403, errorBody('namespace_forbidden', 'Writer namespace not permitted'),
      putPath(IDS.A.id, 'frigate-bridge'))],
  ];
  for (const [file, value] of exchanges) {
    add(file, value, {kind: 'classification_exchange', status: value.response.status,
      code: value.response.body.code ?? null, description: value.description});
  }

  // Evidence JPEG and its box manifest.
  const media = {
    description: 'Unannotated evidence image shared by two-birds-object-0/1. Solid-color boxes on 8-pixel block ' +
      'boundaries; pixel coordinates are exact, colors are within JPEG tolerance (±12 per channel).',
    asset_id: IDS.C0.asset, file: JPEG_FILE, media_type: 'image/jpeg', width: JPEG.width, height: JPEG.height,
    bytes: jpegBytes.length, sha256: createHash('sha256').update(jpegBytes).digest('hex'),
    background_rgb: JPEG.background_rgb,
    boxes: JPEG.boxes.map(box => {
      const {format: _, ...normalized} = normalizedBox(box.pixels, JPEG.width, JPEG.height);
      return {observation_file: obsFile[box.key], observation_id: IDS[box.key].id, rgb: box.rgb, pixels: box.pixels, normalized};
    }),
  };
  fixtures.push({file: JPEG_FILE, kind: 'jpeg', manifest: 'media/two-birds-160x80.json', description: media.description});
  add('media/two-birds-160x80.json', media, {kind: 'jpeg_manifest', description: 'Box pixel coordinates for the JPEG.'});

  // Message-ID vectors (§5.4).
  add('message-ids.json', {
    rule: 'message_id = UUIDv5(namespace.uuid, source + "|" + observation_id + "|" + revision), revision as a decimal string',
    namespace: {name: NS_NAME, parent: `NAMESPACE_URL ${NAMESPACE_URL}`, uuid: MESSAGE_ID_NAMESPACE},
    contract_test_vector: messageId(SOURCE, IDS.A.id, 2),
    vectors,
  }, {kind: 'message_id_vectors', description: 'Every change-event message ID, including the contract §5.4 test vector.'});

  const manifest = {
    description: 'INT_01 canonical observation contract fixtures (task C3). Generated by ' +
      'scripts/contracts/observation-fixtures.mjs; validated by tests/contracts/tooling/observation-fixtures.test.mjs.',
    contract: 'docs/integrations/OBSERVATIONS_CONTRACT.md',
    constants: {source: SOURCE, installation_uuid: INSTALLATION, camera_uuid: CAMERA, stream_name: 'feeder-east',
      epoch_uuid: EPOCH, oldest_seq: OLDEST_SEQ, topic_prefix: TOPIC_PREFIX, event_type: EVENT_TYPE,
      message_id_namespace: MESSAGE_ID_NAMESPACE, writer: WRITER},
    changes: changes.map(c => ({seq: c.seq, observation_id: IDS[c.key].id, revision: c.revision, cause: c.cause,
      evidence_state: c.state, evidence_reason: c.reason, created_at: c.at})),
    fixtures: fixtures.sort((a, b) => a.file.localeCompare(b.file)),
  };
  files.set('manifest.json', json(manifest));
  return files;
}

// ---- vendoring ---------------------------------------------------------------------
async function listFiles(dir, prefix = '') {
  const out = [];
  for (const entry of await readdir(dir, {withFileTypes: true})) {
    const rel = prefix ? `${prefix}/${entry.name}` : entry.name;
    if (entry.isDirectory()) out.push(...await listFiles(path.join(dir, entry.name), rel));
    else out.push(rel);
  }
  return out.sort();
}
const sha256 = bytes => createHash('sha256').update(bytes).digest('hex');
const git = (...args) => execFileSync('git', ['-C', root, ...args], {encoding: 'utf8'}).trim();

async function vendor(target, {allowDirty}) {
  const dirty = git('status', '--porcelain', '--', fixtureRel, 'scripts/contracts/observation-fixtures.mjs');
  if (dirty && !allowDirty) throw new Error(`Commit the fixtures before vendoring (dirty:\n${dirty})`);
  const commit = git('rev-parse', 'HEAD');
  const dest = path.join(target, vendorRel);
  await rm(dest, {recursive: true, force: true});
  const files = await listFiles(fixtureDir);
  const sums = [];
  for (const file of files) {
    await mkdir(path.dirname(path.join(dest, file)), {recursive: true});
    await copyFile(path.join(fixtureDir, file), path.join(dest, file));
    sums.push(`${sha256(await readFile(path.join(fixtureDir, file)))}  ${file}`);
  }
  await writeFile(path.join(dest, 'SOURCE.md'), `# lightNVR observation contract fixtures (vendored)

These files are an identical copy of \`${fixtureRel}/\` from
lightNVR commit \`${commit}\`${dirty ? ' (plus uncommitted changes)' : ''}.
Do not edit them here. The contract is lightNVR
\`docs/integrations/OBSERVATIONS_CONTRACT.md\`; \`manifest.json\` describes every file.

Re-sync from a lightNVR checkout:

\`\`\`sh
node scripts/contracts/observation-fixtures.mjs --vendor <path to YetAnother-WhosAtMyFeeder>
node scripts/contracts/observation-fixtures.mjs --check-vendor <path to YetAnother-WhosAtMyFeeder>
\`\`\`

lightNVR's \`tests/contracts/tooling/observation-fixtures.test.mjs\` also compares
this copy when the YA-WAMF checkout is a sibling directory (or
\`LIGHTNVR_YAWAMF_DIR\` points at it).

## SHA-256

\`\`\`text
${sums.join('\n')}
\`\`\`
`);
  return {commit, count: files.length, dest};
}

async function checkVendor(target) {
  const dest = path.join(target, vendorRel);
  const expected = await listFiles(fixtureDir);
  const actual = (await listFiles(dest)).filter(f => f !== 'SOURCE.md');
  const problems = [];
  for (const file of new Set([...expected, ...actual])) {
    if (!expected.includes(file)) problems.push(`extra: ${file}`);
    else if (!actual.includes(file)) problems.push(`missing: ${file}`);
    else if (!(await readFile(path.join(dest, file))).equals(await readFile(path.join(fixtureDir, file)))) problems.push(`differs: ${file}`);
  }
  return problems;
}

// ---- CLI ---------------------------------------------------------------------------
if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  const args = process.argv.slice(2);
  const valueOf = flag => args[args.indexOf(flag) + 1];
  if (args.includes('--vendor')) {
    const {commit, count, dest} = await vendor(path.resolve(valueOf('--vendor')), {allowDirty: args.includes('--allow-dirty')});
    console.log(`Vendored ${count} files from ${commit} into ${dest}`);
  } else if (args.includes('--check-vendor')) {
    const problems = await checkVendor(path.resolve(valueOf('--check-vendor')));
    if (problems.length) { console.error(problems.join('\n')); process.exit(1); }
    console.log('Vendored observation fixtures match.');
  } else {
    const regenerate = args.includes('--jpeg');
    const jpeg = await loadJpeg({regenerate});
    const files = await buildFixtures({jpeg});
    if (args.includes('--check')) {
      const stale = [];
      for (const [file, content] of files) {
        const target = path.join(fixtureDir, file);
        if (!existsSync(target) || await readFile(target, 'utf8') !== content) stale.push(file);
      }
      if (stale.length) { console.error(`Stale observation fixtures:\n${stale.join('\n')}`); process.exit(1); }
      console.log(`Observation fixtures up to date (${files.size} JSON files).`);
    } else {
      await mkdir(path.join(fixtureDir, 'media'), {recursive: true});
      if (regenerate || !existsSync(path.join(fixtureDir, JPEG_FILE))) await writeFile(path.join(fixtureDir, JPEG_FILE), jpeg);
      for (const [file, content] of files) {
        await mkdir(path.dirname(path.join(fixtureDir, file)), {recursive: true});
        await writeFile(path.join(fixtureDir, file), content);
      }
      console.log(`Wrote ${files.size} JSON fixtures and ${JPEG_FILE} to ${fixtureRel}/`);
    }
  }
}
