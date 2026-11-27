import {connect} from 'node:net';
import {test, expect} from '@playwright/test';
import {startFixture} from './fixture-server.mjs';

function rawExchange(port: number, bytes: string): Promise<string> {
  return new Promise((resolve, reject) => {
    const socket = connect(port, '127.0.0.1');
    const chunks: Buffer[] = [];
    const timer = setTimeout(() => socket.destroy(new Error('HTTP response timed out')), 10000);
    socket.on('connect', () => socket.write(bytes));
    socket.on('data', chunk => chunks.push(chunk));
    socket.on('error', reject);
    socket.on('close', hadError => {
      clearTimeout(timer);
      if (!hadError) resolve(Buffer.concat(chunks).toString('utf8'));
    });
  });
}

test('pipelined requests are replayed after the first response', async () => {
  const server = await startFixture();
  try {
    const response = await rawExchange(server.port,
      'GET /api/health?liveness=true HTTP/1.1\r\nHost: localhost\r\nConnection: keep-alive\r\n\r\n' +
      'GET /api/health?liveness=true HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n');
    expect(response.match(/HTTP\/1\.1 200 /g)).toHaveLength(2);
  } finally {
    await server.stop();
  }
});

test('chunked request body is assembled without transfer framing', async () => {
  const server = await startFixture();
  try {
    const body = JSON.stringify(server.credentials.admin);
    const middle = Math.floor(body.length / 2);
    const chunks = [body.slice(0, middle), body.slice(middle)]
      .map(part => `${Buffer.byteLength(part).toString(16)}\r\n${part}\r\n`).join('');
    const response = await rawExchange(server.port,
      'POST /api/auth/login HTTP/1.1\r\nHost: localhost\r\nContent-Type: application/json\r\n' +
      `Transfer-Encoding: chunked\r\nConnection: close\r\n\r\n${chunks}0\r\n\r\n`);
    expect(response).toMatch(/^HTTP\/1\.1 200 /);
  } finally {
    await server.stop();
  }
});
