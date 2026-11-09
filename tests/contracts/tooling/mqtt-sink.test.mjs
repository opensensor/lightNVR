import test from 'node:test';
import assert from 'node:assert/strict';
import net from 'node:net';
import {on, once} from 'node:events';
import {startMqttSink} from '../mqtt-sink.ts';

async function connect(t) {
  const sink = await startMqttSink();
  const client = net.connect(sink.port, '127.0.0.1');
  t.after(async () => { client.destroy(); await sink.stop(); });
  await once(client, 'connect', {signal: t.signal});
  return {sink, client};
}

const qos0 = Buffer.from([0x30, 6, 0, 1, 0x78, 0, 0x80, 0xff]);
const qos1 = Buffer.from([0x32, 8, 0, 1, 0x78, 0, 7, 0, 0x80, 0xff]);
const payload = Buffer.from([0, 0x80, 0xff]);
const ping = Buffer.from([0xc0, 0]);

async function receive(client, size, signal) {
  let bytes = Buffer.alloc(0);
  for await (const [chunk] of on(client, 'data', {signal})) {
    bytes = Buffer.concat([bytes, chunk]);
    if (bytes.length >= size) return bytes;
  }
}

test('MQTT sink closes reserved QoS 3 without recording it or later packets', {timeout: 5000}, async t => {
  const {sink, client} = await connect(t);
  const invalid = Buffer.from(qos1);
  invalid[0] = 0x36;
  const closed = once(client, 'close', {signal: t.signal});
  client.write(Buffer.concat([invalid, qos0]));
  await closed;
  assert.deepEqual(sink.publications, []);
});

test('MQTT sink preserves QoS 0 binary payloads and responds to the following ping', {timeout: 5000}, async t => {
  const {sink, client} = await connect(t);
  const response = receive(client, 2, t.signal);
  client.write(Buffer.concat([qos0, ping]));
  assert.deepEqual(await response, Buffer.from([0xd0, 0]));
  assert.deepEqual(sink.publications, [{topic: 'x', payload, qos: 0}]);
});

test('MQTT sink acknowledges QoS 1 with its packet ID and can suppress PUBACK', {timeout: 5000}, async t => {
  const {sink, client} = await connect(t);
  const ack = receive(client, 4, t.signal);
  client.write(qos1);
  assert.deepEqual(await ack, Buffer.from([0x40, 2, 0, 7]));
  assert.deepEqual(sink.publications, [{topic: 'x', payload, qos: 1}]);

  sink.acknowledge(false);
  const response = receive(client, 2, t.signal);
  client.write(Buffer.concat([qos1, ping]));
  assert.deepEqual(await response, Buffer.from([0xd0, 0]));
  assert.deepEqual(sink.publications, [
    {topic: 'x', payload, qos: 1}, {topic: 'x', payload, qos: 1},
  ]);
});
