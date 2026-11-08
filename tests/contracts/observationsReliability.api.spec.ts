/**
 * INT_01 L11 reliability against the real native binary (A5 lightNVR half,
 * A6): process kills and restarts on the same database and storage tree, a
 * DB restore, and a broker outage. Each case drives the shipped startup order
 * (L4 reconcile -> L3 evidence -> L5 relay drain) and the real router, outbox
 * and MQTT delivery worker against an owned MQTT sink.
 *
 * Kills inside the evidence worker and relay (after batch insert, after the
 * tmp write, after rename before commit, after commit before relay, after the
 * outbox step) need in-process stage hooks and are covered by
 * tests/unit/test_observation_crash_boundaries.c. Here the binary is killed at
 * the boundaries an external process can hit deterministically: committed but
 * not yet relayed, and relayed but not yet delivered (no PUBACK).
 * Contract: docs/integrations/OBSERVATIONS_CONTRACT.md sections 5.5, 7.5, 7.6, 14.
 */
import {copyFileSync, existsSync, mkdirSync, readFileSync, rmSync, writeFileSync} from 'node:fs';
import path from 'node:path';
import {startMqttSink} from './mqtt-sink';
import {reservePort} from './fixture-server.mjs';
import {test, expect, ok, admin, wire, valid, type Server, type Json} from './permissionCases';
import {OBSERVATION_EVENT, seedObservation, messageId, decodeCursor, encodeCursor, meta, sql} from './fixtures/observations/seed';

test.skip(process.env.LIGHTNVR_CONTRACT_MQTT !== '1', 'Requires the explicitly MQTT-enabled contract build (enabled in CI).');

async function route(server: Server, port: number, name: string) {
  const destination = await ok(server, '/api/event-destinations', admin, 'POST', {name: `${name} sink`, broker: {host: '127.0.0.1', port, client_id: `${name}-sink`, topic_template: 'fixture/{type}/{subject_id}', qos: 1, keepalive_seconds: 5}, tls: {mode: 'disabled'}});
  valid('EventDestination', destination);
  // Observation events need a route without a label predicate (label predicates only match detection.object.v1).
  await ok(server, '/api/event-routes', admin, 'POST', {name: `${name} route`, destination: destination.key, event_types: [OBSERVATION_EVENT], camera_scope: {type: 'selector', selector: {version: 1, expression: {op: 'camera_uuid', values: [server.cameras.A.camera_uuid]}}}});
}
const installation = async (server: Server) => (await ok(server, '/api/integrations/capabilities')).source.id as string;
const events = (sink: Awaited<ReturnType<typeof startMqttSink>>) => sink.publications.map(p => JSON.parse(p.payload.toString())).filter(e => e.type === OBSERVATION_EVENT);
const outbox = (server: Server) => sql(server, 'SELECT event_id, state, attempt_count FROM event_outbox WHERE event_type = ? ORDER BY id', [OBSERVATION_EVENT]);
const publications = (server: Server, state: string) => Number(sql(server, 'SELECT count(*) AS n FROM observation_publications WHERE state = ?', [state])[0].n);
const serverLog = (server: Server) => readFileSync(path.join(server.directory, 'server.log'), 'utf8');
async function walk(server: Server, cursor = ''): Promise<{items: Json[]; cursor: string}> {
  const items: Json[] = [];
  for (let pages = 0; pages < 100; pages++) {
    const body = await ok(server, `/api/observations?limit=30${cursor ? `&cursor=${cursor}` : ''}`);
    valid('ObservationPage', body);
    items.push(...body.items);
    cursor = body.next_cursor;
    if (!body.has_more) break;
  }
  return {items, cursor};
}

test('A6: after SIGKILL the next start reconciles, then starts evidence, then drains committed changes to MQTT', async ({server}) => {
  const sink = await startMqttSink();
  try {
    await route(server, sink.port, 'restart-drain');
    const src = await installation(server), A = server.cameras.A.camera_uuid;
    await server.kill('SIGKILL');

    // Committed before the crash, never relayed (pending publications).
    const ready = seedObservation(server, {camera: A, publication: 'pending', source: src});
    // Killed mid-write: a pending asset with only a .jpg.tmp, and one whose final file was renamed but never committed.
    const midWrite = seedObservation(server, {camera: A, state: 'pending', publication: 'pending', source: src});
    const tmp = path.join(server.directory, 'recordings', `${midWrite.relativePath}.tmp`);
    mkdirSync(path.dirname(tmp), {recursive: true});
    writeFileSync(tmp, 'partial jpeg');
    const renamed = seedObservation(server, {camera: A, state: 'pending', publication: 'pending', source: src});
    const uncommitted = path.join(server.directory, 'recordings', renamed.relativePath);
    writeFileSync(uncommitted, Buffer.from([0xff, 0xd8, 0xff, 0xd9]));

    const offset = serverLog(server).length;
    await server.relaunch();
    const log = serverLog(server).slice(offset);
    const at = (needle: string) => { const i = log.indexOf(needle); expect(i, `startup log contains "${needle}"`).toBeGreaterThanOrEqual(0); return i; };
    const reconcile = at('Observation reconciliation: 1 temp and 1 stale files deleted, 2 pending interrupted');
    const evidence = at('Observation evidence initialized');
    const drained = at('Observation relay drained');
    const relay = at('Observation publication relay started');
    expect(reconcile).toBeLessThan(evidence);
    expect(evidence).toBeLessThan(drained);
    expect(drained).toBeLessThan(relay);

    // No ready asset points at an unfinished file; nothing unfinished is left on disk.
    expect(existsSync(tmp)).toBe(false);
    expect(existsSync(uncommitted)).toBe(false);
    for (const seeded of [midWrite, renamed]) {
      const body = await ok(server, `/api/observations/${seeded.observation}`);
      expect(body).toMatchObject({revision: 2, evidence: {state: 'unavailable', reason: 'interrupted'}});
      const response = await server.request(`/api/observations/${seeded.observation}/snapshot`);
      expect(response.status).toBe(404);
      expect((await response.json()).reason).toBe('interrupted');
    }
    const image = await server.request(`/api/observations/${ready.observation}/snapshot`);
    expect(Buffer.from(await image.arrayBuffer()).equals(ready.bytes!)).toBe(true);
    expect(sql(server, "SELECT count(*) AS n FROM observation_assets WHERE state = 'pending'")[0].n).toBe(0);

    // Every committed change (and the reconciliation's own) reaches the broker under its stable ID.
    const expected = new Set([messageId(src, ready.observation, 1), messageId(src, ready.observation, 2),
      messageId(src, midWrite.observation, 1), messageId(src, midWrite.observation, 2),
      messageId(src, renamed.observation, 1), messageId(src, renamed.observation, 2)]);
    await expect.poll(() => new Set(events(sink).map(e => e.id)).size, {timeout: 30000}).toBe(expected.size);
    expect(new Set(events(sink).map(e => e.id))).toEqual(expected);
    for (const event of events(sink)) valid('ObservationChangeEvent', event.data);
    const interrupted = events(sink).find(e => e.id === messageId(src, midWrite.observation, 2));
    expect(interrupted.data).toMatchObject({revision: 2, change_cause: 'evidence', evidence_state: 'unavailable', evidence_reason: 'interrupted'});
    expect(publications(server, 'pending')).toBe(0);
  } finally { await sink.stop(); }
});

test('A6: relayed but unacknowledged changes survive SIGKILL and are redelivered once acknowledged, with the same message IDs', async ({server}) => {
  const sink = await startMqttSink();
  try {
    await route(server, sink.port, 'redelivery');
    const src = await installation(server);
    sink.acknowledge(false);
    const seeded = seedObservation(server, {camera: server.cameras.A.camera_uuid, publication: 'pending', source: src});
    const ids = [messageId(src, seeded.observation, 1), messageId(src, seeded.observation, 2)];
    // The live relay (1 s poll) moves both revisions into the outbox; the broker sees them but never acknowledges.
    await expect.poll(() => new Set(events(sink).map(e => e.id)).size, {timeout: 20000}).toBe(2);
    expect(publications(server, 'relayed')).toBe(2);
    expect(outbox(server).map(row => row.state)).not.toContain('delivered');

    await server.kill('SIGKILL');
    const before = outbox(server);
    expect(before.map(row => row.event_id)).toEqual(ids);
    expect(before.every(row => row.state !== 'delivered')).toBe(true);

    sink.acknowledge(true);
    sink.disconnect();
    await server.relaunch();
    await expect.poll(() => outbox(server).every(row => row.state === 'delivered'), {timeout: 45000}).toBe(true);
    // At least once, never a new identity: each revision keeps one message ID across redelivery,
    // and the restart's relay drain staged nothing new (no duplicate outbox rows).
    const delivered = events(sink);
    expect(new Set(delivered.map(e => e.id))).toEqual(new Set(ids));
    for (const id of ids) expect(delivered.filter(e => e.id === id).length).toBeGreaterThanOrEqual(2);
    expect(outbox(server).map(row => row.event_id)).toEqual(ids);
    expect(delivered.filter(e => e.id === ids[1]).every(e => e.data.revision === 2 && e.data.evidence_state === 'ready')).toBe(true);
    expect(new Set(delivered.filter(e => e.id === ids[1]).map(e => JSON.stringify(e.data))).size).toBe(1);
  } finally { await sink.stop(); }
});

test('D9: after a DB restore, cursors from before the restore return 410 and recovery resumes from oldest_cursor', async ({server}) => {
  const A = server.cameras.A.camera_uuid;
  const kept = [seedObservation(server, {camera: A}), seedObservation(server, {camera: A})];
  const src = await installation(server);
  const atBackup = (await walk(server)).cursor;

  await server.kill('SIGTERM');
  const db = path.join(server.directory, 'lightnvr.db'), backup = path.join(server.directory, 'backup.db');
  // A file-level backup of the stopped server: the main file plus any un-checkpointed WAL.
  for (const suffix of ['', '-wal']) if (existsSync(`${db}${suffix}`)) copyFileSync(`${db}${suffix}`, `${backup}${suffix}`);
  await server.relaunch();

  // Newer history the restore will lose. The relay records the head in the sidecar as it relays.
  const lost = [0, 1, 2].map(() => seedObservation(server, {camera: A, publication: 'pending', source: src}));
  await expect.poll(() => publications(server, 'pending'), {timeout: 20000}).toBe(0);
  const sidecar = path.join(server.directory, 'recordings', 'observations', '.cursor_head.json');
  await expect.poll(() => existsSync(sidecar) && JSON.parse(readFileSync(sidecar, 'utf8')).head_seq, {timeout: 20000}).toBe(meta(server).head);
  const latest = await walk(server, atBackup);
  expect(latest.items.map(item => item.id)).toEqual(lost.map(s => s.observation));
  const beforeRestore = latest.cursor, oldEpoch = meta(server).epoch;

  await server.kill('SIGTERM');
  for (const suffix of ['', '-wal', '-shm']) rmSync(`${db}${suffix}`, {force: true});
  for (const suffix of ['', '-wal']) if (existsSync(`${backup}${suffix}`)) copyFileSync(`${backup}${suffix}`, `${db}${suffix}`);
  const offset = serverLog(server).length;
  await server.relaunch();
  expect(serverLog(server).slice(offset)).toContain('epoch regenerated');

  const restored = meta(server);
  expect(restored.epoch).not.toBe(oldEpoch);
  for (const cursor of [beforeRestore, atBackup]) {
    const result = await wire(server, `/api/observations?cursor=${cursor}`);
    valid('ObservationError', result.body);
    expect(result.response.status).toBe(410);
    expect(result.body.code).toBe('cursor_expired');
    expect(decodeCursor(result.body.oldest_cursor)).toEqual({v: 1, seq: restored.oldest - 1, epoch: restored.epoch});
  }
  // The consumer marks recovery and resumes: exactly the restored history, nothing from the lost generation.
  const expired = await wire(server, `/api/observations?cursor=${beforeRestore}`);
  const resumed = await walk(server, expired.body.oldest_cursor);
  expect(resumed.items.map(item => item.id)).toEqual(kept.map(s => s.observation));
  for (const seeded of lost) expect((await wire(server, `/api/observations/${seeded.observation}`)).response.status).toBe(404);

  // Residual window (section 7.6): a same-epoch cursor beyond the head (seq rewound without an
  // epoch change, e.g. a restore newer than the last sidecar write) is also 410.
  const ahead = await wire(server, `/api/observations?cursor=${encodeCursor(restored.head + 5, restored.epoch)}`);
  expect(ahead.response.status).toBe(410);
  expect(ahead.body.code).toBe('cursor_expired');
});

test('A5: with the broker down, 100 observations stay recoverable through the outbox and the cursor', async ({server}) => {
  test.setTimeout(180000);
  const down = await reservePort();
  await down.release();  // nothing listens: the broker is down
  await route(server, down.port, 'outage');
  const src = await installation(server), A = server.cameras.A.camera_uuid;
  const seeded = Array.from({length: 100}, () => seedObservation(server, {camera: A, publication: 'pending', source: src}));
  const expected = new Set(seeded.flatMap(s => [messageId(src, s.observation, 1), messageId(src, s.observation, 2)]));

  // The relay accepts every change into the durable outbox even though nothing can be delivered.
  await expect.poll(() => publications(server, 'pending'), {timeout: 60000}).toBe(0);
  expect(publications(server, 'relayed')).toBe(200);
  expect(new Set(outbox(server).map(row => row.event_id))).toEqual(expected);
  expect(outbox(server).some(row => row.state === 'delivered')).toBe(false);

  // The cursor recovers all 100 while the broker is down.
  const history = await walk(server);
  expect(history.items.map(item => item.id)).toEqual(seeded.map(s => s.observation));
  expect(history.items.every(item => item.revision === 2 && item.evidence.state === 'ready')).toBe(true);

  // A crash during the outage loses nothing either.
  await server.kill('SIGKILL');
  await server.relaunch();
  expect(new Set(outbox(server).map(row => row.event_id))).toEqual(expected);

  // The broker comes back on its address: every change is delivered under its stable ID.
  const sink = await startMqttSink(down.port);
  try {
    await expect.poll(() => new Set(events(sink).map(e => e.id)).size, {timeout: 120000, intervals: [1000]}).toBe(expected.size);
    expect(new Set(events(sink).map(e => e.id))).toEqual(expected);
    await expect.poll(() => outbox(server).every(row => row.state === 'delivered'), {timeout: 30000}).toBe(true);
    expect(outbox(server).length).toBe(expected.size);
  } finally { await sink.stop(); }
});
