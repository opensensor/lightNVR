import {test, expect} from '@playwright/test';
import {startFixture} from './fixture-server.mjs';

interface BaselineStream { name: string; enabled: boolean; record: boolean; admin_url: string; onvif_password: string; can_control_privacy: boolean; privacy_mode: boolean; [key: string]: unknown }
interface BaselineFull { stream: BaselineStream }
interface BaselineSummary { total: number; streams: BaselineStream[] }

test('Streams baseline enforces scope, redaction and write boundaries', async ({playwright}, testInfo) => {
  const server = await startFixture();
  let failed = true;
  try {
    expect((await server.request('/api/streams', {principal:'anonymous'})).status).toBe(401);
    const all = await server.json<BaselineStream[]>('/api/streams');
    expect(all.map(row => row.name).sort()).toEqual(['A','B']);
    expect(all.every(row => row.enabled === false && row.record === false)).toBe(true);
    for (const principal of ['viewer','readA']) {
      expect((await server.json<BaselineStream[]>('/api/streams',{principal})).map(row => row.name)).toEqual(['A']);
      const summary = await server.json<BaselineSummary>('/api/streams?summary=true&include_admin_url=true',{principal});
      expect(summary.total).toBe(1); expect(summary.streams.map(row => row.name)).toEqual(['A']);
      const full = (await server.json<BaselineFull>('/api/streams/A/full',{principal})).stream;
      expect(full.admin_url).toBe(''); expect(full.onvif_password).toBe(''); expect(full.can_control_privacy).toBe(false);
      expect((await server.request('/api/streams/B/full',{principal})).status).toBe(403);
      const before = (await server.json<BaselineFull>('/api/streams/A/full')).stream;
      expect(before.admin_url).toBe('http://127.0.0.1:9/admin-a');
      expect((await server.request('/api/streams/A',{principal,method:'PUT',body:{set_privacy_mode:true}})).status).toBe(403);
      expect((await server.json<BaselineFull>('/api/streams/A/full')).stream).toEqual(before);
    }
    const before = (await server.json<BaselineFull>('/api/streams/A/full')).stream;
    expect((await server.request('/api/streams/A',{method:'PUT',body:{set_privacy_mode:true,enabled:true}})).status).toBe(400);
    expect((await server.json<BaselineFull>('/api/streams/A/full')).stream).toEqual(before);
    expect((await server.request('/api/streams/A',{method:'PUT',body:{set_privacy_mode:true}})).status).toBe(200);
    expect((await server.json<BaselineFull>('/api/streams/A/full')).stream.privacy_mode).toBe(true);
    expect((await server.json<BaselineFull>('/api/streams/B/full')).stream.privacy_mode).toBe(false);
    const context = await server.apiContext(playwright.request,'viewer');
    try { expect((await context.get('/api/streams/B/full')).status()).toBe(403); }
    finally { await context.dispose(); }
    failed = false;
  } finally {
    if (failed) await testInfo.attach('server-log',{path:server.logPath,contentType:'text/plain'});
    await server.stop({failed});
  }
});
