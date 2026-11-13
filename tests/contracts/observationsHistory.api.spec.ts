/**
 * INT_01 L11 observation history and snapshot contracts against the real
 * native server: A7 (camera-scoped isolation on list, get and snapshot, and
 * rename / delete / name reuse never transferring access), A8 (pending,
 * expired, unavailable, unknown and expired-cursor are distinct outcomes) and
 * A5 cursor pagination including pending -> ready recovery. Contract:
 * docs/integrations/OBSERVATIONS_CONTRACT.md sections 7, 8 and 12.
 */
import {randomUUID} from 'node:crypto';
import {test, expect, ok, admin, viewer, wire, valid, createUser, role, policy, token, cameraScope, database, type Server, type Identity, type Json} from './permissionCases';
import {seedRecordings, seedHistoricalNameReuse} from './fixtures/recordings/seed';
import {seedObservation, completeEvidence, transition, encodeCursor, decodeCursor, meta, sql} from './fixtures/observations/seed';

async function snapshot(server: Server, id: string, identity: Identity, headers: Record<string, string> = {}) {
  const response = await server.request(`/api/observations/${id}/snapshot`, {principal: 'anonymous', headers: {...identity.headers, ...headers}});
  const bytes = Buffer.from(await response.arrayBuffer());
  const json = (response.headers.get('content-type') || '').startsWith('application/json') ? JSON.parse(bytes.toString('utf8')) : undefined;
  if (json) valid('ObservationError', json);
  return {status: response.status, headers: response.headers, bytes, json};
}
async function page(server: Server, query: string, identity: Identity = admin): Promise<Json> {
  const body = await ok(server, `/api/observations${query}`, identity);
  valid('ObservationPage', body);
  return body;
}
const ids = (body: Json) => body.items.map((item: Json) => item.id);
async function error(server: Server, route: string, identity: Identity = admin) {
  const result = await wire(server, route, identity);
  valid('ObservationError', result.body);
  return {status: result.response.status, body: result.body};
}
async function viewerOf(server: Server, name: string, cameraUuid: string, actions = ['live.view', 'snapshot.create']): Promise<Identity> {
  const grants = await role(server, `Observation ${name}`, actions);
  const user = await createUser(server, name);
  await policy(server, user, [{role_uuid: grants, scope: cameraScope(cameraUuid)}]);
  return user;
}

test('A7: list, get and snapshot are isolated per camera for policies and scoped tokens', async ({server}) => {
  const A = server.cameras.A.camera_uuid, B = server.cameras.B.camera_uuid;
  const onA = seedObservation(server, {camera: A, stream: 'A'}), onB = seedObservation(server, {camera: B, stream: 'B'});

  const all = await page(server, '');
  expect(ids(all)).toEqual([onA.observation, onB.observation]);

  // The fixture viewer holds the built-in Viewer role (live.view + snapshot.create) on A only.
  const viewed = await page(server, '', viewer);
  expect(ids(viewed)).toEqual([onA.observation]);
  expect(JSON.stringify(viewed)).not.toContain(B);
  for (const query of [`?camera_uuid=${B}`, `?camera_uuid=${A},${B}`, `?camera_uuid=${randomUUID()}`]) {
    const denied = await error(server, `/api/observations${query}`, viewer);
    expect(denied.status).toBe(403);
    expect(denied.body.code).toBe('forbidden');
  }
  const unknown = await error(server, `/api/observations/${randomUUID()}`, viewer);
  expect(unknown.status).toBe(404);
  expect(unknown.body.code).toBe('observation_not_found');
  const hidden = await error(server, `/api/observations/${onB.observation}`, viewer);
  expect(hidden).toEqual(unknown);
  const hiddenSnapshot = await snapshot(server, onB.observation, viewer);
  const unknownSnapshot = await snapshot(server, randomUUID(), viewer);
  expect(hiddenSnapshot.status).toBe(404);
  expect(hiddenSnapshot.json).toEqual(unknownSnapshot.json);
  expect(hiddenSnapshot.json).toEqual(unknown.body);

  const own = await ok(server, `/api/observations/${onA.observation}`, viewer);
  valid('Observation', own);
  expect(own).toMatchObject({id: onA.observation, camera_uuid: A, revision: 2, evidence: {asset_id: onA.asset, state: 'ready'}});
  expect(JSON.stringify(own)).not.toContain(onA.relativePath);  // the internal path is never serialized
  expect(own.evidence.snapshot_url).toBe(`/api/observations/${onA.observation}/snapshot`);
  const image = await snapshot(server, onA.observation, viewer);
  expect(image.status).toBe(200);
  expect(image.headers.get('content-type')).toBe('image/jpeg');
  expect(image.headers.get('cache-control')).toBe('private, max-age=300');
  expect(image.headers.get('etag')).toBe(`"${onA.asset}"`);
  expect(image.bytes.equals(onA.bytes!)).toBe(true);
  expect((await snapshot(server, onA.observation, viewer, {'If-None-Match': `"${onA.asset}"`})).status).toBe(304);

  // A read-only scoped token (live.view on A): metadata yes, image no (D4 403), B invisible.
  const readA = {headers: {Authorization: `Bearer ${server.tokens.readA}`}};
  expect(ids(await page(server, '', readA))).toEqual([onA.observation]);
  expect((await ok(server, `/api/observations/${onA.observation}`, readA)).id).toBe(onA.observation);
  const noImage = await snapshot(server, onA.observation, readA);
  expect(noImage.status).toBe(403);
  expect(noImage.json.code).toBe('forbidden');
  expect(noImage.bytes.includes(onA.bytes!)).toBe(false);
  expect((await snapshot(server, onB.observation, readA)).json).toEqual(unknown.body);

  // A YA-WAMF-style token minted by the administrator but scoped to B only.
  const tokenB = await token(server, admin, cameraScope(B), ['live.view', 'snapshot.create']);
  expect(ids(await page(server, '', tokenB))).toEqual([onB.observation]);
  expect((await error(server, `/api/observations?camera_uuid=${A}`, tokenB)).status).toBe(403);
  expect((await error(server, `/api/observations/${onA.observation}`, tokenB))).toEqual(unknown);
  expect((await snapshot(server, onA.observation, tokenB)).json).toEqual(unknown.body);
  expect((await snapshot(server, onB.observation, tokenB)).bytes.equals(onB.bytes!)).toBe(true);
  const capabilities = await ok(server, '/api/integrations/capabilities', tokenB);
  valid('IntegrationCapabilities', capabilities);
  expect(capabilities.cameras.map((camera: Json) => camera.camera_uuid)).toEqual([B]);

  // snapshot.create without live.view is not enough to read metadata or list.
  const imageOnly = await viewerOf(server, 'image_only', A, ['snapshot.create']);
  expect(ids(await page(server, '', imageOnly))).toEqual([]);
  expect((await error(server, `/api/observations/${onA.observation}`, imageOnly)).status).toBe(404);
  expect((await snapshot(server, onA.observation, imageOnly)).json).toEqual(unknown.body);
});

test('A7: rename, delete and name reuse never transfer observation access', async ({server}) => {
  const A = server.cameras.A.camera_uuid, B = server.cameras.B.camera_uuid;
  // Historical name reuse (recordings fixture): stream name "A" once belonged to camera B.
  const recordings = seedRecordings(server), historical = seedHistoricalNameReuse(server, recordings);
  const reusedName = seedObservation(server, {camera: B, stream: 'A', recordingId: historical.hidden});
  // Keep the hidden ID's digits in a timestamp to exercise incidental substring matches.
  const observedAt = Math.floor(Date.now() / 1000) * 1000 + historical.hidden;
  const visible = seedObservation(server, {camera: A, stream: 'B', recordingId: historical.visible, observedAt});
  const viewed = await page(server, '', viewer);
  expect(ids(viewed)).toEqual([visible.observation]);
  expect(viewed.items[0]).toMatchObject({camera_uuid: A, stream_name_at_capture: 'B', recording_reference: {recording_id: historical.visible}});
  expect((await error(server, `/api/observations/${reusedName.observation}`, viewer)).status).toBe(404);
  expect((await snapshot(server, reusedName.observation, viewer)).status).toBe(404);
  expect(viewed.items.map((item: Json) => item.recording_reference?.recording_id)).not.toContain(historical.hidden);

  // Rename: a camera keeps its UUID, so its grant follows it and its history keeps the capture name.
  const created = await ok(server, '/api/streams', admin, 'POST', {name: 'C', url: 'rtsp://127.0.0.1:9/contract-c', enabled: false, streaming_enabled: false, record: false});
  const C = created.camera_uuid as string;
  const onC = seedObservation(server, {camera: C, stream: 'C'});
  const holderC = await viewerOf(server, 'holder_c', C);
  // lightNVR has no rename API; the rename is applied to the stream row and picked up by a restart.
  await server.kill('SIGTERM');
  database(server, "UPDATE streams SET name = 'C-renamed' WHERE camera_uuid = ?", [C]);
  await server.relaunch();
  expect((await ok(server, '/api/streams/C-renamed/full')).stream.camera_uuid).toBe(C);
  expect(ids(await page(server, '', holderC))).toEqual([onC.observation]);
  expect((await ok(server, `/api/observations/${onC.observation}`, holderC)).stream_name_at_capture).toBe('C');
  // ...and a new camera that takes the freed name gets nothing of the old one's history.
  const reused = await ok(server, '/api/streams', admin, 'POST', {name: 'C', url: 'rtsp://127.0.0.1:9/contract-c2', enabled: false, streaming_enabled: false, record: false});
  expect(reused.camera_uuid).not.toBe(C);
  const holderNewC = await viewerOf(server, 'holder_new_c', reused.camera_uuid);
  expect(ids(await page(server, '', holderNewC))).toEqual([]);
  expect((await error(server, `/api/observations/${onC.observation}`, holderNewC)).status).toBe(404);
  expect((await snapshot(server, onC.observation, holderNewC)).status).toBe(404);
  expect((await error(server, `/api/observations?camera_uuid=${C}`, holderNewC)).status).toBe(403);

  // Delete + recreate under the same name: the deleted camera's history is gone for everyone
  // (deleted capture camera => 404) and never attaches to the replacement.
  const onB = seedObservation(server, {camera: B, stream: 'B'});
  const holderB = await viewerOf(server, 'holder_b', B);
  expect(ids(await page(server, '', holderB))).toEqual([reusedName.observation, onB.observation]);
  expect((await wire(server, '/api/streams/B?permanent=true', admin, 'DELETE')).response.status).toBe(202);
  await expect.poll(async () => (await server.request('/api/streams/B')).status).toBe(404);
  const newB = await ok(server, '/api/streams', admin, 'POST', {name: 'B', url: 'rtsp://127.0.0.1:9/contract-b2', enabled: false, streaming_enabled: false, record: false});
  expect(newB.camera_uuid).not.toBe(B);
  const holderNewB = await viewerOf(server, 'holder_new_b', newB.camera_uuid);
  for (const identity of [admin, holderB, holderNewB]) {
    const visibleIds = ids(await page(server, '', identity));
    expect(visibleIds).not.toContain(onB.observation);
    expect(visibleIds).not.toContain(reusedName.observation);
    expect((await error(server, `/api/observations/${onB.observation}`, identity)).body.code).toBe('observation_not_found');
    expect((await snapshot(server, onB.observation, identity)).status).toBe(404);
  }
  // The rows are retained (no implicit purge), just unreachable.
  expect(sql(server, 'SELECT count(*) AS n FROM object_observations WHERE camera_uuid = ?', [B])[0].n).toBe(2);
  const onNewB = seedObservation(server, {camera: newB.camera_uuid, stream: 'B'});
  expect(ids(await page(server, '', holderNewB))).toEqual([onNewB.observation]);
});

test('A8: pending, expired, unavailable, unknown and expired-cursor outcomes are distinct', async ({server}) => {
  const A = server.cameras.A.camera_uuid;
  const pending = seedObservation(server, {camera: A, state: 'pending'});
  const expired = seedObservation(server, {camera: A, state: 'expired', reason: 'retention_expired'});
  const unavailable = seedObservation(server, {camera: A, state: 'unavailable', reason: 'queue_full'});
  const ready = seedObservation(server, {camera: A});

  const outcomes: Record<string, {status: number; code?: string}> = {};
  const p = await snapshot(server, pending.observation, admin);
  expect(p.status).toBe(202);
  expect(p.json.code).toBe('evidence_pending');
  const retryAfter = Number(p.headers.get('retry-after'));
  expect(Number.isInteger(retryAfter) && retryAfter >= 1 && retryAfter <= 30).toBe(true);
  expect(p.json.retry_after_seconds).toBe(retryAfter);
  outcomes.pending = {status: p.status, code: p.json.code};

  const e = await snapshot(server, expired.observation, admin);
  expect(e.status).toBe(410);
  expect(e.json).toMatchObject({code: 'evidence_expired', reason: 'retention_expired'});
  expect(Date.parse(e.json.expired_at)).toBeGreaterThan(0);
  outcomes.expired = {status: e.status, code: e.json.code};

  const u = await snapshot(server, unavailable.observation, admin);
  expect(u.status).toBe(404);
  expect(u.json).toMatchObject({code: 'evidence_unavailable', reason: 'queue_full'});
  outcomes.unavailable = {status: u.status, code: u.json.code};

  const n = await snapshot(server, randomUUID(), admin);
  expect(n.status).toBe(404);
  expect(n.json.code).toBe('observation_not_found');
  outcomes.unknown = {status: n.status, code: n.json.code};

  // Expired cursor: history before seq 3 is pruned (L4 advances oldest_seq with the prune).
  const before = meta(server);
  const stale = encodeCursor(1, before.epoch);
  database(server, 'UPDATE observation_meta SET oldest_seq = 3 WHERE id = 1');
  const c = await error(server, `/api/observations?cursor=${stale}`);
  expect(c.status).toBe(410);
  expect(c.body.code).toBe('cursor_expired');
  expect(decodeCursor(c.body.oldest_cursor)).toEqual({v: 1, seq: 2, epoch: before.epoch});
  expect(Date.parse(c.body.oldest_observed_at)).toBeGreaterThan(0);
  outcomes.expired_cursor = {status: c.status, code: c.body.code};
  // Resuming from oldest_cursor works and never re-serves the pruned gap.
  const resumed = await page(server, `?cursor=${c.body.oldest_cursor}`);
  expect(resumed.items.length).toBeGreaterThan(0);
  expect(ids(resumed)).toEqual([expired.observation, unavailable.observation, ready.observation]);

  const invalid = await error(server, '/api/observations?cursor=not-a-cursor');
  expect(invalid.status).toBe(400);
  expect(invalid.body.code).toBe('invalid_cursor');
  outcomes.invalid_cursor = {status: invalid.status, code: invalid.body.code};

  // Every outcome is distinguishable by (status, code); nothing collapses into a generic 404.
  const keys = Object.values(outcomes).map(o => `${o.status}:${o.code}`);
  expect(new Set(keys).size).toBe(keys.length);
  // The observation itself stays readable in every evidence state, carrying the state and reason.
  for (const [seeded, state, reason] of [[pending, 'pending', null], [expired, 'expired', 'retention_expired'], [unavailable, 'unavailable', 'queue_full'], [ready, 'ready', null]] as const) {
    const body = await ok(server, `/api/observations/${seeded.observation}`);
    valid('Observation', body);
    expect(body.evidence).toMatchObject({state, reason});
  }
  // Ready evidence is served only for its own asset, never a substitute.
  expect((await snapshot(server, ready.observation, admin)).bytes.equals(ready.bytes!)).toBe(true);
});

test('A5: cursor pages are stable, deduplicated and recover pending -> ready after the cursor passed', async ({server}) => {
  const A = server.cameras.A.camera_uuid, B = server.cameras.B.camera_uuid;
  const seeded = [0, 1, 2, 3, 4].map(i => seedObservation(server, {camera: i === 2 ? B : A, state: 'pending', observedAt: Date.now() - 60000 + i}));
  const order = seeded.map(s => s.observation);

  // Unfiltered admin walk with limit 2: every observation exactly once, in persistence order.
  const seen: string[] = [];
  let cursor = '', pages = 0, last: Json = {};
  do {
    last = await page(server, `?limit=2${cursor ? `&cursor=${cursor}` : ''}`);
    expect(last.items.length).toBeLessThanOrEqual(2);
    seen.push(...ids(last));
    cursor = last.next_cursor;
    pages++;
    if (pages >= 10 && last.has_more) {
      throw new Error('Pagination did not terminate within 10 pages');
    }
  } while (last.has_more);
  expect(last.has_more).toBe(false);
  expect(seen).toEqual(order);
  expect(last.items.every((item: Json) => item.evidence.state === 'pending')).toBe(true);
  const tail = cursor;
  expect(decodeCursor(tail).seq).toBe(meta(server).head);

  // Filtered walk (camera A only): B's change is scanned and skipped, so cursors still advance.
  const trailingExcluded = seedObservation(server, {camera: B, state: 'pending', observedAt: Date.now() - 59000});
  const filtered = await page(server, `?camera_uuid=${A}&limit=10`);
  expect(ids(filtered)).toEqual([...order, trailingExcluded.observation].filter((id, i) => i !== 2 && id !== trailingExcluded.observation));
  expect(decodeCursor(filtered.next_cursor).seq).toBe(meta(server).head);
  expect(filtered.has_more).toBe(false);
  expect(await page(server, `?cursor=${filtered.next_cursor}`)).toEqual({items: [], next_cursor: filtered.next_cursor, has_more: false});

  // Nothing new: an empty page that keeps the cursor. `head` starts at the same place.
  const idle = await page(server, `?cursor=${tail}`);
  expect(idle).toEqual({items: [], next_cursor: tail, has_more: false});
  expect(await page(server, '?cursor=head')).toEqual({items: [], next_cursor: tail, has_more: false});

  // Evidence finishes after the consumer already read the created changes (A5 recovery).
  const bytes = completeEvidence(server, seeded[1]);
  transition(server, seeded[3], 'unavailable', 'encode_failed');
  completeEvidence(server, seeded[4]);
  transition(server, seeded[4], 'expired', 'retention_expired');  // two changes, one item
  const recovered = await page(server, `?cursor=${tail}&limit=10`);
  expect(ids(recovered)).toEqual([seeded[1], seeded[3], seeded[4]].map(s => s.observation));
  expect(recovered.items.map((item: Json) => [item.revision, item.evidence.state, item.evidence.reason])).toEqual([[2, 'ready', null], [2, 'unavailable', 'encode_failed'], [3, 'expired', 'retention_expired']]);
  expect(recovered.has_more).toBe(false);
  const ready = await snapshot(server, seeded[1].observation, admin);
  expect(ready.status).toBe(200);
  expect(ready.bytes.equals(bytes)).toBe(true);

  // limit counts observations after dedup: walk with limit 1 across page boundaries
  // and ensure each recovered observation appears exactly once.
  const expectedRecoveredIds = [seeded[1], seeded[3], seeded[4]].map(s => s.observation);
  const seenRecoveredIds: string[] = [];
  let recoveryCursor = tail;
  for (let i = 0; i < expectedRecoveredIds.length; i++) {
    const p = await page(server, `?cursor=${recoveryCursor}&limit=1`);
    expect(ids(p).length).toBe(1);
    seenRecoveredIds.push(ids(p)[0]);
    recoveryCursor = p.next_cursor;
    expect(p.has_more).toBe(i < expectedRecoveredIds.length - 1);
  }
  expect(seenRecoveredIds).toEqual(expectedRecoveredIds);
  expect(decodeCursor(recoveryCursor).seq).toBe(meta(server).head);

  // Re-reading from the beginning still yields every observation once (current state).
  const replay = await page(server, '?limit=500');
  expect(ids(replay)).toEqual(order);
  expect(replay.items.find((item: Json) => item.id === seeded[1].observation).evidence.state).toBe('ready');
});
