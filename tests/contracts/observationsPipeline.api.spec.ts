/**
 * INT_01 L11 end-to-end in the native binary, with no SQL seeding: an owned
 * HTTP MPEG-TS source feeds a detection stream whose HTTP API detector is an
 * owned loopback server. The real unified detection thread submits the exact
 * JPEG it sent for inference (L7) to the real evidence worker (L3), which
 * writes it, commits pending -> ready (L2) and wakes the relay (L5), which
 * delivers to an owned MQTT sink. Covers A1 (both objects of one frame share
 * one asset), A2 HTTP variant (the stored snapshot is byte-identical to the
 * detector's input, and the detector was asked exactly once per frame) and
 * the lightNVR side of A5 (cursor + one message ID per revision).
 */
import {execFileSync} from 'node:child_process';
import {readFileSync} from 'node:fs';
import {createServer, type IncomingMessage, type ServerResponse} from 'node:http';
import path from 'node:path';
import {startMqttSink} from './mqtt-sink';
import {test, expect, ok, admin, valid, type Json} from './permissionCases';
import {OBSERVATION_EVENT, messageId} from './fixtures/observations/seed';

test.skip(process.env.LIGHTNVR_CONTRACT_MQTT !== '1', 'Requires the explicitly MQTT-enabled contract build (enabled in CI).');

function jpegOf(body: Buffer): Buffer | null {
  const start = body.indexOf(Buffer.from([0xff, 0xd8, 0xff]));
  const end = body.lastIndexOf(Buffer.from([0xff, 0xd9]));
  return start >= 0 && end > start ? Buffer.from(body.subarray(start, end + 2)) : null;
}
async function listen(handler: (req: IncomingMessage, res: ServerResponse) => void) {
  const server = createServer(handler);
  await new Promise<void>(resolve => server.listen(0, '127.0.0.1', resolve));
  const address = server.address();
  if (!address || typeof address === 'string') throw new Error('owned fixture did not bind');
  return {server, port: address.port, async close() { server.closeAllConnections(); await new Promise<void>(resolve => server.close(() => resolve())); }};
}

test('A1/A2/A5: a real detection frame becomes two observations sharing the exact inference JPEG, delivered once per revision', async ({server}) => {
  test.setTimeout(150000);
  const media = path.join(server.directory, 'feeder.ts');
  execFileSync('ffmpeg', ['-nostdin', '-hide_banner', '-loglevel', 'error', '-f', 'lavfi', '-i', 'testsrc=size=320x240:rate=10', '-t', '90', '-c:v', 'libx264', '-preset', 'ultrafast', '-pix_fmt', 'yuv420p', '-g', '10', '-f', 'mpegts', media], {timeout: 60000});
  const bytes = readFileSync(media);
  const source = await listen((req, res) => {
    res.writeHead(200, {'Content-Type': 'video/mp2t'});
    let offset = 0;
    const timer = setInterval(() => { res.write(bytes.subarray(offset, offset + 188 * 40)); offset += 188 * 40; if (offset >= bytes.length) { clearInterval(timer); res.end(); } }, 20);
    res.on('close', () => clearInterval(timer));
  });
  const inputs: Buffer[] = [];
  const detector = await listen((req, res) => {
    const chunks: Buffer[] = [];
    req.on('data', chunk => chunks.push(chunk));
    req.on('end', () => {
      const jpeg = jpegOf(Buffer.concat(chunks));
      if (jpeg) inputs.push(jpeg);
      res.writeHead(200, {'Content-Type': 'application/json'});
      res.end(JSON.stringify({model: 'contract-birds', model_version: '1.0', detections: [
        {label: 'bird', confidence: 0.91, x_min: 0.1, y_min: 0.2, x_max: 0.3, y_max: 0.5},
        {label: 'bird', confidence: 0.82, x_min: 0.6, y_min: 0.2, x_max: 0.8, y_max: 0.4},
      ]}));
    });
  });
  const sink = await startMqttSink();
  try {
    const created = await ok(server, '/api/streams', admin, 'POST', {name: 'Feeder', url: `http://127.0.0.1:${source.port}/feeder.ts`, enabled: true, streaming_enabled: false, record: false,
      detection_based_recording: true, detection_model: `http://127.0.0.1:${detector.port}/detect`, detection_threshold: 0.5, detection_interval: 1,
      observation_evidence_enabled: true, observation_labels: 'bird'});
    const camera = created.camera_uuid as string;
    const destination = await ok(server, '/api/event-destinations', admin, 'POST', {name: 'Pipeline sink', broker: {host: '127.0.0.1', port: sink.port, client_id: 'pipeline-sink', topic_template: 'fixture/{type}/{subject_id}', qos: 1, keepalive_seconds: 5}, tls: {mode: 'disabled'}});
    await ok(server, '/api/event-routes', admin, 'POST', {name: 'Pipeline route', destination: destination.key, event_types: [OBSERVATION_EVENT], camera_scope: {type: 'selector', selector: {version: 1, expression: {op: 'camera_uuid', values: [camera]}}}});
    const src = (await ok(server, '/api/integrations/capabilities')).source.id as string;

    let ready: Json[] = [];
    await expect.poll(async () => {
      const page = await ok(server, `/api/observations?camera_uuid=${camera}&limit=500`);
      valid('ObservationPage', page);
      ready = page.items.filter((item: Json) => item.evidence.state === 'ready');
      return ready.length;
    }, {timeout: 90000, intervals: [1000]}).toBeGreaterThanOrEqual(2);

    // A1: one frame -> one batch -> two observations sharing one asset.
    const first = ready[0];
    const siblings = ready.filter(item => item.batch_id === first.batch_id);
    expect(siblings.map(item => item.object_index).sort()).toEqual([0, 1]);
    expect(new Set(siblings.map(item => item.evidence.asset_id)).size).toBe(1);
    for (const item of siblings) {
      valid('Observation', item);
      expect(item).toMatchObject({camera_uuid: camera, stream_name_at_capture: 'Feeder', label: 'bird', revision: 2,
        detector: {engine_type: 'api', model: 'contract-birds', model_version: '1.0'}, evidence: {state: 'ready', origin: 'submitted_image', media_type: 'image/jpeg'}});
    }
    // A2 (HTTP): the stored evidence is exactly an image the detector received; no later frame substituted.
    const response = await server.request(`/api/observations/${first.id}/snapshot`);
    expect(response.status).toBe(200);
    const snapshot = Buffer.from(await response.arrayBuffer());
    expect(snapshot.length).toBe(first.evidence.bytes);
    expect(inputs.filter(input => input.equals(snapshot)).length).toBe(1);
    expect(first.evidence.width).toBe(320);
    expect(first.evidence.height).toBe(240);
    expect(first.box).toEqual({format: 'xywh_normalized', x: 0.1, y: 0.2, width: 0.2, height: 0.3});

    // A5: each revision of each sibling reaches the broker under its deterministic ID.
    const expected = siblings.flatMap(item => [messageId(src, item.id, 1), messageId(src, item.id, 2)]);
    const received = () => sink.publications.map(p => JSON.parse(p.payload.toString())).filter(e => e.type === OBSERVATION_EVENT);
    await expect.poll(() => expected.every(id => received().some(e => e.id === id)), {timeout: 30000}).toBe(true);
    for (const item of siblings) {
      const mine = received().filter(e => e.data.observation_id === item.id);
      expect(new Set(mine.map(e => e.id))).toEqual(new Set([messageId(src, item.id, 1), messageId(src, item.id, 2)]));
      expect(mine.find(e => e.data.revision === 1).data).toMatchObject({change_cause: 'created', evidence_state: 'pending', asset_id: item.evidence.asset_id});
      expect(mine.find(e => e.data.revision === 2).data).toMatchObject({change_cause: 'evidence', evidence_state: 'ready', asset_id: item.evidence.asset_id});
    }
  } finally {
    await ok(server, '/api/streams/Feeder?permanent=true', admin, 'DELETE').catch(() => undefined);
    await sink.stop(); await detector.close(); await source.close();
  }
});
