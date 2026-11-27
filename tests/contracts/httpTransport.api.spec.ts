import {connect} from 'node:net';
import {test, expect} from '@playwright/test';
import {startFixture} from './fixture-server.mjs';

function rawExchange(port: number, bytes: string): Promise<string> {
  return new Promise((resolve, reject) => {
    const socket = connect(port, '127.0.0.1');
    const chunks: Buffer[] = [];
    const timer = setTimeout(() => socket.destroy(new Error('HTTP response timed out')), 10000);
    socket.on('connect', () => socket.write(bytes));
    socket.on('data', chunk => chunks.push(Buffer.isBuffer(chunk) ? chunk : Buffer.from(chunk)));
    socket.on('error', reject);
    socket.on('close', hadError => {
      clearTimeout(timer);
      if (!hadError) resolve(Buffer.concat(chunks).toString('utf8'));
    });
  });
}

function stagedExchange(port: number, parts: string[]): Promise<string> {
  return new Promise((resolve, reject) => {
    const socket = connect(port, '127.0.0.1');
    socket.setNoDelay(true);
    const chunks: Buffer[] = [];
    const timer = setTimeout(() => socket.destroy(new Error('HTTP response timed out')), 10000);

    const writePart = (index: number) => {
      if (index >= parts.length) {
        socket.end();
        return;
      }
      socket.write(parts[index], error => {
        if (error) {
          reject(error);
          return;
        }
        // Best-effort pacing only: TCP is a byte stream, so writes may still be
        // coalesced/split before parser reads. Parser-boundary behavior must be
        // validated with parser-level tests that feed fragments explicitly.
        setTimeout(() => writePart(index + 1), 5);
      });
    };

    socket.on('connect', () => writePart(0));
    socket.on('data', chunk => chunks.push(Buffer.isBuffer(chunk) ? chunk : Buffer.from(chunk)));
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
      'GET /api/health?liveness=false HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n');
    expect(response.match(/HTTP\/1\.1 200 /g)).toHaveLength(2);
    const firstBodyIndex = response.indexOf('"liveness":true');
    const secondBodyIndex = response.indexOf('"streams":', firstBodyIndex + 1);
    expect(firstBodyIndex).toBeGreaterThan(-1);
    expect(secondBodyIndex).toBeGreaterThan(-1);
    expect(firstBodyIndex).toBeLessThan(secondBodyIndex);
  } finally {
    await server.stop();
  }
});

test('chunked request body is assembled without transfer framing', async () => {
  const server = await startFixture();
  try {
    const body = JSON.stringify(server.credentials.admin);
    const middle = Math.floor(body.length / 2);
    const first = body.slice(0, middle);
    const second = body.slice(middle);
    const firstLenHex = Buffer.byteLength(first).toString(16);
    const secondLenHex = Buffer.byteLength(second).toString(16);

    const response = await stagedExchange(server.port, [
      'POST /api/auth/',
      'login HTTP/1.1\r\nHost: localhost\r\nContent-Ty',
      'pe: application/json\r\nTransfer-Encoding: chu',
      'nked\r\nConnection: close\r\n\r\n',
      `${firstLenHex}\r\n${first.slice(0, Math.floor(first.length / 2))}`,
      `${first.slice(Math.floor(first.length / 2))}\r\n`,
      `${secondLenHex}\r\n${second.slice(0, Math.floor(second.length / 2))}`,
      `${second.slice(Math.floor(second.length / 2))}\r\n0\r\n`,
      '\r\n',
    ]);

    expect(response).toMatch(/^HTTP\/1\.1 200 /);
  } finally {
    await server.stop();
  }
});

test('double-slash static paths are rejected before file lookup', async () => {
  const server = await startFixture();
  try {
    const response = await rawExchange(server.port,
      'GET //backups/unknown.json HTTP/1.1\r\nHost: localhost\r\nConnection: close\r\n\r\n');
    expect(response).toMatch(/^HTTP\/1\.1 400 /);
  } finally {
    await server.stop();
  }
});

test('compressible responses vary by accepted content encoding', async () => {
  const server = await startFixture();
  try {
    for (const encoding of ['identity', 'gzip']) {
      const response = await server.request('/api/openapi.json', {
        headers: {'Accept-Encoding': encoding},
      });
      expect(response.status).toBe(200);
      expect(response.headers.get('vary')?.toLowerCase()).toContain('accept-encoding');
      expect(response.headers.get('content-encoding')).toBe(encoding === 'gzip' ? 'gzip' : null);
      const body = await response.text();
      const openapi = JSON.parse(body) as {openapi?: unknown; paths?: unknown};
      expect(typeof openapi.openapi).toBe('string');
      expect(openapi.paths).toBeTruthy();
    }
  } finally {
    await server.stop();
  }
});

test('gzip quality values follow the HTTP grammar', async () => {
  const server = await startFixture();
  try {
    const cases: [string, boolean][] = [
      ['gzip;q=0.5', true], ['gzip;q=0.5 ', true], ['gzip;q=0.001', true],
      ['gzip;q=1.', true], ['gzip;q=0', false], ['gzip;q=0.000', false],
      ['gzip;q=0.5junk', false], ['gzip;q=1.001', false],
    ];
    for (const [header, acceptsGzip] of cases) {
      const response = await server.request('/api/openapi.json', {
        headers: {'Accept-Encoding': header},
      });
      expect(response.status).toBe(200);
      expect(response.headers.get('content-encoding'), header)
        .toBe(acceptsGzip ? 'gzip' : null);
    }
  } finally {
    await server.stop();
  }
});
