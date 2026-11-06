import { readFileSync } from 'node:fs';
import path from 'node:path';
import { build } from 'esbuild';
import { chromium } from '../../node_modules/playwright';
import type { Browser, BrowserContext, Page } from '../../node_modules/playwright';
import {
  bindExplorerLifecycle, createExplorerSession, explorerUrls,
  prepareExplorerDocument, readOnlySwaggerOptions,
} from '../js/api/explorerSession.ts';
import type { SessionInfo } from '../js/types/core.ts';
import { clearAuthState, validateSession } from '../js/utils/auth-utils.ts';

const admin: SessionInfo = { valid: true, status: 'valid', id: 1, username: 'admin-fixture', role: 'admin', authenticated: true };
const viewer: SessionInfo = { valid: true, status: 'valid', id: 2, username: 'viewer-fixture', role: 'viewer', authenticated: true };
const minimal = (title = 'Fixture reference') => ({
  openapi: '3.1.1', info: { title, version: '1' }, servers: [{ url: '../' }],
  paths: { '/api/streams': { get: { operationId: 'listStreams', responses: { 200: { description: 'OK' } } } } },
  'x-lightnvr-coverage': { partial: true, canonical_revision: 'fixture-revision', covered_operations: ['listStreams'] },
});
const response = (value: unknown, status = 200) => new Response(JSON.stringify(value), { status });
function deferred<T>() {
  let resolve!: (value: T) => void;
  const promise = new Promise<T>(done => { resolve = done; });
  return { promise, resolve };
}
function harness(overrides: Partial<Parameters<typeof createExplorerSession>[0]> = {}) {
  const clear = jest.fn(); const status = jest.fn(); const render = jest.fn();
  const fetcher = jest.fn<ReturnType<typeof fetch>, Parameters<typeof fetch>>().mockResolvedValue(response(minimal()));
  const verify = jest.fn<Promise<SessionInfo>, [{ signal: AbortSignal; verifyUrl: string }]>().mockResolvedValue(admin);
  const session = createExplorerSession({ pageUrl: 'https://fixture.invalid/prefix/api-docs.html?url=https://evil.invalid',
    clear, status, render, fetch: fetcher, verify, ...overrides });
  return { session, clear, status, render, fetcher, verify };
}

afterEach(() => { jest.restoreAllMocks(); });

test('page-relative discovery and verification preserve direct and proxy roots without query configuration', () => {
  expect(explorerUrls('https://fixture.invalid/api-docs.html').spec).toBe('https://fixture.invalid/api/openapi.json');
  expect(explorerUrls('https://fixture.invalid/nvr/api-docs.html?url=https://evil.invalid')).toMatchObject({
    spec: 'https://fixture.invalid/nvr/api/openapi.json', verify: 'https://fixture.invalid/nvr/api/auth/verify',
  });
  expect(() => explorerUrls('file:///api-docs.html')).toThrow();
  const prepared = prepareExplorerDocument(minimal(), 'https://fixture.invalid/nvr/api/openapi.json');
  expect(prepared.servers).toEqual([{ url: 'https://fixture.invalid/nvr/' }]);
});

test('only local references and the same deployment server reach Swagger', () => {
  for (const unsafe of [
    { ...minimal(), components: { schemas: { Secret: { $ref: 'https://evil.invalid/schema' } } } },
    { ...minimal(), components: { schemas: { Secret: { $ref: '/other/spec' } } } },
    { ...minimal(), servers: [{ url: 'https://evil.invalid/' }] },
    { ...minimal(), servers: [{ url: '/' }] },
    { paths: {} },
  ]) expect(() => prepareExplorerDocument(unsafe, 'https://fixture.invalid/nvr/api/openapi.json')).toThrow();
});

test('docs-specific session verification explicitly bypasses the HTTP cache', async () => {
  const previous = Object.getOwnPropertyDescriptor(globalThis, 'window');
  Object.defineProperty(globalThis, 'window', { configurable: true, value: {} });
  const fetcher = jest.spyOn(globalThis, 'fetch').mockResolvedValue(response({ authenticated: true, id: 1, username: 'fixture', role: 'admin' }));
  try {
    expect((await validateSession({ verifyUrl: 'https://fixture.invalid/prefix/api/auth/verify' })).valid).toBe(true);
    expect(fetcher).toHaveBeenCalledWith('https://fixture.invalid/prefix/api/auth/verify', expect.objectContaining({ cache: 'no-store', credentials: 'same-origin' }));
  } finally {
    if (previous) Object.defineProperty(globalThis, 'window', previous); else Reflect.deleteProperty(globalThis, 'window');
  }
});

test('Swagger options remove authorization components and disable all vendor network paths', () => {
  const options = readOnlySwaggerOptions(prepareExplorerDocument(minimal(), 'https://fixture.invalid/api/openapi.json'), {} as HTMLElement);
  expect(options).toMatchObject({ supportedSubmitMethods: [], persistAuthorization: false,
    queryConfigEnabled: false, validatorUrl: null, configUrl: null, url: '', urls: [], tryItOutEnabled: false });
  expect(() => options.requestInterceptor()).toThrow('requests are disabled');
  for (const component of Object.values(options.plugins[0]!().components)) expect(component()).toBeNull();
});

test('clears before verification, fetches without caching, then renders only after a second identity check', async () => {
  const h = harness(); await h.session.reload();
  expect(h.clear).toHaveBeenCalledTimes(1);
  expect(h.verify).toHaveBeenCalledTimes(2);
  expect(h.fetcher).toHaveBeenCalledWith('https://fixture.invalid/prefix/api/openapi.json', expect.objectContaining({
    credentials: 'same-origin', cache: 'no-store', redirect: 'error', method: 'GET',
  }));
  expect(h.verify.mock.calls[0]?.[0].verifyUrl).toBe('https://fixture.invalid/prefix/api/auth/verify');
  expect(h.status.mock.calls.map(([state]) => state)).toEqual(['loading', 'partial']);
  expect(h.render).toHaveBeenCalledTimes(1);
  h.session.dispose();
});

test.each([401, 403, 500])('discovery HTTP %i never renders the previous or error document', async status => {
  const h = harness({ fetch: jest.fn<ReturnType<typeof fetch>, Parameters<typeof fetch>>().mockResolvedValue(response({ error: 'private server detail' }, status)) });
  await h.session.reload();
  expect(h.render).not.toHaveBeenCalled();
  expect(h.status).toHaveBeenLastCalledWith(status === 500 ? 'unavailable' : 'denied');
  h.session.dispose();
});

test('malformed, unavailable-session, password-change and empty results have explicit safe states', async () => {
  for (const [session, spec, state] of [
    [{ valid: false, status: 'unavailable' }, minimal(), 'unavailable'],
    [{ ...admin, must_change_password: true }, minimal(), 'denied'],
    [admin, { private: 'malformed' }, 'unavailable'],
    [admin, { ...minimal(), paths: {} }, 'empty'],
  ] as const) {
    const h = harness({ verify: async () => session, fetch: jest.fn<ReturnType<typeof fetch>, Parameters<typeof fetch>>().mockResolvedValue(response(spec)) });
    await h.session.reload(); expect(h.status).toHaveBeenLastCalledWith(state);
    expect(h.render).not.toHaveBeenCalled(); h.session.dispose();
  }
});

test('an admin response completing after logout cannot restore cleared DOM even if fetch ignores abort', async () => {
  const pending = deferred<Response>(); const arrived = deferred<void>();
  let signal: AbortSignal | null | undefined;
  const h = harness({ fetch: jest.fn<ReturnType<typeof fetch>, Parameters<typeof fetch>>().mockImplementation((_url, options) => {
    signal = options?.signal; arrived.resolve(); return pending.promise;
  }) });
  const reload = h.session.reload(); await arrived.promise;
  h.session.clear('denied'); expect(signal?.aborted).toBe(true);
  pending.resolve(response(minimal('Old admin reference'))); await reload;
  expect(h.render).not.toHaveBeenCalled(); expect(h.status).toHaveBeenLastCalledWith('denied');
  h.session.dispose();
});

test('account change during body loading revalidates and renders only the newer identity document', async () => {
  const oldBody = deferred<unknown>(); const reading = deferred<void>();
  const oldResponse = response({});
  jest.spyOn(oldResponse, 'json').mockImplementation(() => { reading.resolve(); return oldBody.promise; });
  const fetcher = jest.fn<ReturnType<typeof fetch>, Parameters<typeof fetch>>().mockResolvedValueOnce(oldResponse).mockResolvedValue(response(minimal('Viewer reference')));
  const h = harness({ fetch: fetcher });
  const first = h.session.reload(); await reading.promise;
  h.verify.mockResolvedValue(viewer);
  const second = h.session.reload(); await second;
  oldBody.resolve(minimal('Old admin reference')); await first;
  expect(h.render).toHaveBeenCalledTimes(1);
  expect(h.render.mock.calls[0]?.[0].info.title).toBe('Viewer reference');
  h.session.dispose();
});

test('identity switching without a browser event is detected by the post-fetch verification', async () => {
  const h = harness();
  h.verify.mockResolvedValueOnce(admin).mockResolvedValue(viewer);
  h.fetcher.mockResolvedValueOnce(response(minimal('Old admin reference'))).mockResolvedValue(response(minimal('Viewer reference')));
  await h.session.reload();
  expect(h.render).toHaveBeenCalledTimes(1);
  expect(h.render.mock.calls[0]?.[0].info.title).toBe('Viewer reference');
  h.session.dispose();
});

test('deadline clears the view even when a loader ignores abort', async () => {
  jest.useFakeTimers();
  const pending = deferred<void>();
  const h = harness({ prepare: () => pending.promise, timeout: 100 });
  try {
    const reload = h.session.reload();
    await jest.advanceTimersByTimeAsync(100);
    expect(h.status).toHaveBeenLastCalledWith('unavailable');
    expect(h.render).not.toHaveBeenCalled();
    pending.resolve(); await reload;
    expect(h.verify).not.toHaveBeenCalled();
  } finally { h.session.dispose(); jest.useRealTimers(); }
});

test('auth clear publishes only a nonpersistent notification and closes its temporary channel', () => {
  const previous = Object.fromEntries(['window', 'document', 'localStorage', 'BroadcastChannel'].map(key => [key, Object.getOwnPropertyDescriptor(globalThis, key)]));
  const postMessage = jest.fn(); const close = jest.fn();
  class Channel { postMessage = postMessage; close = close; }
  Object.defineProperties(globalThis, {
    window: { configurable: true, value: new EventTarget() },
    document: { configurable: true, value: { cookie: '' } },
    localStorage: { configurable: true, value: { removeItem: jest.fn() } },
    BroadcastChannel: { configurable: true, value: Channel },
  });
  try {
    clearAuthState(); expect(postMessage).toHaveBeenCalledWith('cleared');
    expect(close).toHaveBeenCalledTimes(1);
  } finally {
    for (const [key, descriptor] of Object.entries(previous)) {
      if (descriptor) Object.defineProperty(globalThis, key, descriptor); else Reflect.deleteProperty(globalThis, key);
    }
  }
});

test('lifecycle and broadcast listeners clear immediately, periodically revalidate, and dispose completely', async () => {
  jest.useFakeTimers();
  const original = globalThis.BroadcastChannel;
  const channels: Array<EventTarget & { close: jest.Mock }> = [];
  class Channel extends EventTarget { close = jest.fn(); constructor() { super(); channels.push(this); } }
  Object.defineProperty(globalThis, 'BroadcastChannel', { configurable: true, value: Channel });
  const win = Object.assign(new EventTarget(), { setInterval, clearInterval }) as unknown as Window;
  const doc = Object.assign(new EventTarget(), { visibilityState: 'visible' }) as unknown as Document;
  const session = { reload: jest.fn(async () => {}), clear: jest.fn(), dispose: jest.fn() };
  try {
    const dispose = bindExplorerLifecycle(session, win, doc);
    win.dispatchEvent(new Event('pagehide')); expect(session.clear).toHaveBeenCalled();
    win.dispatchEvent(new Event('pageshow')); expect(session.reload).toHaveBeenCalledTimes(1);
    channels[0]?.dispatchEvent(new MessageEvent('message', { data: 'cleared' }));
    expect(session.clear).toHaveBeenLastCalledWith('denied');
    await jest.advanceTimersByTimeAsync(30000); expect(session.reload).toHaveBeenCalledTimes(2);
    dispose(); expect(channels[0]?.close).toHaveBeenCalledTimes(1);
    const count = session.reload.mock.calls.length;
    const clears = session.clear.mock.calls.length;
    channels[0]?.dispatchEvent(new MessageEvent('message', { data: 'cleared' }));
    expect(session.clear).toHaveBeenCalledTimes(clears);
    win.dispatchEvent(new Event('focus')); win.dispatchEvent(new Event('pageshow'));
    await jest.advanceTimersByTimeAsync(30000);
    expect(session.reload).toHaveBeenCalledTimes(count); expect(session.dispose).toHaveBeenCalledTimes(1);
  } finally { Object.defineProperty(globalThis, 'BroadcastChannel', { configurable: true, value: original }); jest.useRealTimers(); }
});

describe('standalone Swagger browser', () => {
  let browser: Browser; let context: BrowserContext; let page: Page; let bundle: string;
  let requested: string[];
  beforeAll(async () => {
    const result = await build({
      entryPoints: [path.resolve(__dirname, '../js/pages/api-docs-page.ts')],
      bundle: true, write: false, format: 'iife', platform: 'browser',
    });
    bundle = result.outputFiles[0]!.text;
    browser = await chromium.launch({ headless: true });
  });
  afterAll(async () => browser?.close());
  afterEach(async () => context?.close());

  async function open(prefix: string, spec: unknown = minimal()) {
    context = await browser.newContext(); page = await context.newPage(); page.setDefaultTimeout(5000);
    requested = [];
    await context.route('**/*', async route => {
      const url = new URL(route.request().url()); requested.push(url.href);
      if (url.origin !== 'http://fixture.invalid') { await route.abort(); return; }
      const pathname = url.pathname.slice(prefix.length);
      if (pathname === '/api/auth/verify') { await route.fulfill({ json: { authenticated: true, id: 1, username: 'fixture', role: 'admin' } }); return; }
      if (pathname === '/api/openapi.json') { await route.fulfill({ json: spec, headers: { 'Cache-Control': 'private, no-store' } }); return; }
      if (pathname === '/fixture.js') { await route.fulfill({ contentType: 'text/javascript', body: bundle }); return; }
      if (pathname.startsWith('/locales/')) {
        await route.fulfill({ contentType: 'application/json', body: readFileSync(path.resolve(__dirname, '../public', pathname.slice(1)), 'utf8') }); return;
      }
      if (pathname === '/api-docs-vendor/swagger-ui-bundle-5.34.1.js') {
        await route.fulfill({ contentType: 'text/javascript', body: readFileSync(path.resolve(__dirname, '../node_modules/swagger-ui-dist/swagger-ui-bundle.js')) }); return;
      }
      if (pathname === '/api-docs-vendor/swagger-ui-5.34.1.css') {
        await route.fulfill({ contentType: 'text/css', body: readFileSync(path.resolve(__dirname, '../node_modules/swagger-ui-dist/swagger-ui.css')) }); return;
      }
      if (pathname === '/css/api-docs.css') {
        await route.fulfill({ contentType: 'text/css', body: readFileSync(path.resolve(__dirname, '../css/api-docs.css')) }); return;
      }
      if (pathname === '/api-docs.html') {
        const html = readFileSync(path.resolve(__dirname, '../api-docs.html'), 'utf8')
          .replace('__API_DOCS_LEGACY_HASHES__', '').replace('./js/pages/api-docs-page.ts', './fixture.js');
        await route.fulfill({ contentType: 'text/html', body: html }); return;
      }
      await route.fulfill({ status: 404, body: '' });
    });
    await page.goto(`http://fixture.invalid${prefix}/api-docs.html?url=https://evil.invalid/spec&configUrl=https://evil.invalid/config`);
  }

  test.each(['', '/proxy/nvr'])('standalone renderer uses %s prefix and has no authorize, submit, config or validation requests', async prefix => {
    const spec = { ...minimal(), components: { securitySchemes: { bearerAuth: { type: 'http', scheme: 'bearer' } } }, security: [{ bearerAuth: [] }] };
    await open(prefix, spec);
    await page.locator('.swagger-ui .opblock').waitFor();
    await page.locator('.opblock-summary').click();
    expect(await page.getByRole('button', { name: /authorize|try it out|execute/i }).count()).toBe(0);
    expect(await page.locator('input[type=password]').count()).toBe(0);
    expect(await page.locator('#docs-status').innerText()).toContain('selected APIs');
    expect(requested).toContain(`http://fixture.invalid${prefix}/api/auth/verify`);
    expect(requested).toContain(`http://fixture.invalid${prefix}/api/openapi.json`);
    expect(requested.filter(url => !url.startsWith(`http://fixture.invalid${prefix}/`))).toEqual([]);
    expect(requested.some(url => /validator|evil\.invalid/.test(new URL(url).hostname))).toBe(false);
    expect(await page.evaluate(() => localStorage.getItem('authorized'))).toBeNull();
  });

  test('logout broadcast clears rendered operations and lifecycle re-entry obtains a fresh document', async () => {
    await open(''); await page.locator('.swagger-ui .opblock').waitFor();
    await page.evaluate(() => window.dispatchEvent(new Event('lightnvr:auth-cleared')));
    expect(await page.locator('#api-reference').textContent()).toBe('');
    await page.evaluate(() => window.dispatchEvent(new Event('pageshow')));
    await page.locator('.swagger-ui .opblock').waitFor();
    const other = await context.newPage();
    await other.goto('http://fixture.invalid/empty.html');
    await other.evaluate(() => { const channel = new BroadcastChannel('lightnvr-auth'); channel.postMessage('cleared'); channel.close(); });
    await page.waitForFunction(() => document.getElementById('api-reference')?.childElementCount === 0);
    expect(await page.locator('#docs-status').innerText()).toContain('Sign in');
  });

  test('remote schema references fail closed before the vendor resolves or requests them', async () => {
    await open('', { ...minimal(), components: { schemas: { Remote: { $ref: 'https://evil.invalid/private-schema' } } } });
    await page.getByRole('alert').waitFor();
    expect(await page.locator('#api-reference').textContent()).toBe('');
    expect(requested.some(url => url.startsWith('https://evil.invalid'))).toBe(false);
  });
});
