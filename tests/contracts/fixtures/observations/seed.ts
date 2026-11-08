/**
 * INT_01 L11 observation fixtures for the isolated contract server.
 *
 * Rows are written straight into the fixture DB in exactly the shape the L2 DB
 * layer writes them: one observation_changes row per revision (cause created
 * for revision 1, evidence afterwards) plus its observation_publications row
 * keyed by the section 5.4 message ID. Seeded publications default to
 * `relayed` so the live relay leaves them alone; pass `publication: 'pending'`
 * to stage work the relay must pick up (the "committed, not yet relayed"
 * boundary). Only the history, snapshot and relay paths under test read these
 * rows; the encoder path itself is covered by the L3/L6/L7 C tests and the
 * crash-boundary test.
 */
import {createHash, randomUUID} from 'node:crypto';
import {mkdirSync, writeFileSync} from 'node:fs';
import path from 'node:path';
import {DatabaseSync, type SQLInputValue} from 'node:sqlite';
import type {Server} from '../../permissionCases';

export const OBSERVATION_EVENT = 'io.lightnvr.detection.observation_changed.v1';

export function uuidv5(namespace: string, name: string): string {
  const hash = createHash('sha1').update(Buffer.from(namespace.replaceAll('-', ''), 'hex')).update(Buffer.from(name, 'utf8')).digest();
  hash[6] = (hash[6] & 0x0f) | 0x50; hash[8] = (hash[8] & 0x3f) | 0x80;
  const hex = hash.subarray(0, 16).toString('hex');
  return `${hex.slice(0, 8)}-${hex.slice(8, 12)}-${hex.slice(12, 16)}-${hex.slice(16, 20)}-${hex.slice(20)}`;
}
/** Section 5.4 (also tests/fixtures/integrations/observations/message-ids.json). */
const MESSAGE_ID_NAMESPACE = uuidv5('6ba7b811-9dad-11d1-80b4-00c04fd430c8', 'https://lightnvr.io/ns/observation-change/v1');
export const messageId = (source: string, observation: string, revision: number) => uuidv5(MESSAGE_ID_NAMESPACE, `${source}|${observation}|${revision}`);

/** Section 7.4: base64url, no padding, of the compact JSON. */
export const encodeCursor = (seq: number, epoch: string) => Buffer.from(JSON.stringify({v: 1, seq, epoch})).toString('base64url');
export const decodeCursor = (cursor: string): {v: number; seq: number; epoch: string} => JSON.parse(Buffer.from(cursor, 'base64url').toString('utf8'));

/** A tiny, recognisable JPEG-framed payload; the snapshot path never decodes it. */
export function jpegBytes(tag: string): Buffer {
  return Buffer.concat([Buffer.from([0xff, 0xd8, 0xff, 0xe0]), Buffer.from(`lightnvr-contract-${tag}-${randomUUID()}`), Buffer.from([0xff, 0xd9])]);
}

function open(server: Server): DatabaseSync {
  if (!/^lightnvr-contract-/.test(path.basename(server.directory))) throw new Error('Observation seed requires an isolated contract fixture');
  const db = new DatabaseSync(path.join(server.directory, 'lightnvr.db'));
  db.exec('PRAGMA busy_timeout=5000; PRAGMA foreign_keys=ON');
  return db;
}
export function sql<T = Record<string, any>>(server: Server, query: string, values: SQLInputValue[] = []): T[] {
  const db = open(server);
  try { return db.prepare(query).all(...values) as T[]; } finally { db.close(); }
}
export function meta(server: Server): {epoch: string; oldest: number; head: number} {
  const row = sql(server, 'SELECT epoch_uuid AS epoch, oldest_seq AS oldest, (SELECT coalesce(max(seq), 0) FROM observation_changes) AS head FROM observation_meta WHERE id = 1')[0];
  return {epoch: String(row.epoch), oldest: Number(row.oldest), head: Number(row.head)};
}

export type EvidenceState = 'pending' | 'ready' | 'unavailable' | 'expired';
export interface SeedOptions {
  camera: string;             // capture-time camera_uuid
  stream?: string;            // stream_name_at_capture
  state?: EvidenceState;      // final evidence state (default ready)
  reason?: string;            // unavailable/expired reason
  label?: string;
  observedAt?: number;        // ms
  recordingId?: number | null;
  publication?: 'relayed' | 'pending';
  source?: string;            // required when publication is pending (message IDs)
}
export interface SeededObservation {observation: string; asset: string; revision: number; bytes?: Buffer; relativePath: string; observedAt: number}

const day = (ms: number) => new Date(ms).toISOString().slice(0, 10);

/** Append one change + publication (the L2 bump), inside an open transaction. */
function change(db: DatabaseSync, observation: string, revision: number, cause: string, state: EvidenceState, reason: string | null, now: number, publication: string, src: string) {
  const info = db.prepare('INSERT INTO observation_changes (observation_uuid, revision, cause, evidence_state, evidence_reason, created_at_ms) VALUES (?, ?, ?, ?, ?, ?)')
    .run(observation, revision, cause, state, reason, now);
  db.prepare('INSERT INTO observation_publications (change_seq, message_id, state, attempts, updated_at_ms) VALUES (?, ?, ?, ?, ?)')
    .run(Number(info.lastInsertRowid), messageId(src || 'urn:lightnvr:contract-seed', observation, revision), publication, publication === 'pending' ? 0 : 1, now);
}

/** One observation and its asset, with the full revision history up to `state`. */
export function seedObservation(server: Server, options: SeedOptions): SeededObservation {
  const now = Date.now(), observedAt = options.observedAt ?? now, state = options.state ?? 'ready';
  const observation = randomUUID(), asset = randomUUID();
  const relativePath = `observations/${options.camera}/${day(observedAt)}/${asset}.jpg`;
  const publication = options.publication ?? 'relayed', src = options.source ?? '';
  if (publication === 'pending' && !src) throw new Error('pending publications need the installation source');
  let bytes: Buffer | undefined;
  if (state === 'ready') {
    bytes = jpegBytes(observation);
    const file = path.join(server.directory, 'recordings', relativePath);
    mkdirSync(path.dirname(file), {recursive: true, mode: 0o750});
    writeFileSync(file, bytes, {mode: 0o640});
  }
  const reason = state === 'unavailable' || state === 'expired' ? (options.reason ?? (state === 'expired' ? 'retention_expired' : 'queue_full')) : null;
  const hasMedia = state === 'ready' || state === 'expired';
  const db = open(server);
  try {
    db.exec('BEGIN IMMEDIATE');
    db.prepare(`INSERT INTO observation_assets (uuid, camera_uuid, state, reason, origin, media_type, path, bytes, width, height, captured_at_ms, transform_json, expires_at_ms, created_at_ms, updated_at_ms)
      VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?, ?)`).run(asset, options.camera, state, reason,
      hasMedia ? 'inference_input' : null, hasMedia ? 'image/jpeg' : null, hasMedia ? relativePath : null,
      hasMedia ? (bytes?.length ?? 64) : null, hasMedia ? 640 : null, hasMedia ? 360 : null, observedAt,
      hasMedia ? '{"mode":"none","source_width":640,"source_height":360,"inference_width":640,"inference_height":360,"scale_x":1,"scale_y":1,"pad_x":0,"pad_y":0}' : null,
      state === 'ready' ? now + 7 * 86400000 : null, now, now);
    const finalRevision = state === 'pending' ? 1 : state === 'expired' ? 3 : 2;
    db.prepare(`INSERT INTO object_observations (uuid, camera_uuid, stream_name_at_capture, batch_id, object_index, observed_at_ms, created_at_ms, engine_key, engine_type, model, label, confidence, box_x, box_y, box_width, box_height, recording_id, asset_uuid, revision)
      VALUES (?, ?, ?, ?, 0, ?, ?, 'legacy-primary', 'object', 'contract-model', ?, 0.87, 0.1, 0.2, 0.3, 0.4, ?, ?, ?)`)
      .run(observation, options.camera, options.stream ?? String((db.prepare('SELECT name FROM streams WHERE camera_uuid = ?').get(options.camera) as {name?: string} | undefined)?.name ?? 'unknown'), randomUUID(), observedAt, now, options.label ?? 'bird', options.recordingId ?? null, asset, finalRevision);
    change(db, observation, 1, 'created', 'pending', null, now, publication, src);
    if (state !== 'pending') change(db, observation, 2, 'evidence', state === 'expired' ? 'ready' : state, state === 'expired' ? null : reason, now, publication, src);
    if (state === 'expired') change(db, observation, 3, 'evidence', 'expired', reason, now, publication, src);
    db.exec('COMMIT');
    return {observation, asset, revision: finalRevision, bytes, relativePath, observedAt};
  } catch (error) { try { db.exec('ROLLBACK'); } catch {} throw error; } finally { db.close(); }
}

/** The L3 pending -> ready commit (file first, then one transaction bumping every referencing observation). */
export function completeEvidence(server: Server, seeded: SeededObservation, src = ''): Buffer {
  const bytes = jpegBytes(seeded.observation);
  const file = path.join(server.directory, 'recordings', seeded.relativePath);
  mkdirSync(path.dirname(file), {recursive: true, mode: 0o750});
  writeFileSync(file, bytes, {mode: 0o640});
  transition(server, seeded, 'ready', null, src, bytes);
  seeded.bytes = bytes;
  return bytes;
}

/** A compare-and-set evidence transition with its change + publication rows (relayed). */
export function transition(server: Server, seeded: SeededObservation, to: EvidenceState, reason: string | null, src = '', bytes?: Buffer) {
  const now = Date.now(), db = open(server);
  try {
    db.exec('BEGIN IMMEDIATE');
    const updated = db.prepare(`UPDATE observation_assets SET state = ?, reason = ?, origin = coalesce(origin, 'inference_input'), media_type = coalesce(media_type, 'image/jpeg'),
      path = coalesce(path, ?), bytes = coalesce(?, bytes), width = coalesce(width, 640), height = coalesce(height, 360), updated_at_ms = ? WHERE uuid = ? AND state <> ?`)
      .run(to, reason, seeded.relativePath, bytes?.length ?? null, now, seeded.asset, to);
    if (Number(updated.changes) !== 1) throw new Error(`asset ${seeded.asset} already ${to}`);
    seeded.revision += 1;
    db.prepare('UPDATE object_observations SET revision = ? WHERE uuid = ?').run(seeded.revision, seeded.observation);
    change(db, seeded.observation, seeded.revision, 'evidence', to, reason, now, 'relayed', src);
    db.exec('COMMIT');
  } catch (error) { try { db.exec('ROLLBACK'); } catch {} throw error; } finally { db.close(); }
}
