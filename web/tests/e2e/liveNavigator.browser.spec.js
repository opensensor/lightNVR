/**
 * Live Navigator tabs: plan-based camera groups, Events states (empty scope,
 * error + retry, results) and updating a saved view, in a real browser.
 * Run with: npm --prefix web run test:e2e -- --testPathPatterns='liveNavigator\.browser\.spec\.js$'
 */
const path = require('node:path');

const streams = [
  { camera_uuid: 'cam-1', name: 'Gate', availability: 'live', location_uuid: null },
  { camera_uuid: 'cam-2', name: 'Barn', availability: 'live', location_uuid: null },
  { camera_uuid: 'cam-3', name: 'Shed', availability: 'live', location_uuid: null },
  { camera_uuid: 'cam-4', name: 'Office', availability: 'live', location_uuid: null },
];

async function openNavigator() {
  const { chromium } = require('../../../node_modules/playwright');
  const { build } = require('esbuild');
  const webRoot = path.resolve(__dirname, '../..');
  const output = await build({
    stdin: {
      resolveDir: webRoot,
      loader: 'jsx',
      contents: `
  import { h, render } from 'preact';
  import { QueryClient, QueryClientProvider } from './js/query-client.ts';
  import { LiveOperatorNavigator } from './js/components/preact/live/LiveOperatorNavigator.tsx';
  const streams = ${JSON.stringify(streams)};
  window.calls = { searches: 0, layoutPuts: [], applied: [] };
  const json = (body, status = 200) => new Response(JSON.stringify(body), { status, headers: { 'Content-Type': 'application/json' } });
  window.fetch = async (url, options = {}) => {
    const target = String(url);
    if (target === '/api/ui/workspaces') return json({});
    if (target === '/api/locations') return json({ locations: [{ uuid: 'loc-1', name: 'Orchard', parent_uuid: null, sort_order: 0 }] });
    if (target === '/api/live/plans') return json({ can_modify: true, plans: [{ uuid: 'p1', name: 'orchard48', location_uuid: 'loc-1',
      cameras: [{ camera_uuid: 'cam-1', x: 0.2, y: 0.2 }, { camera_uuid: 'cam-2', x: 0.4, y: 0.4 }, { camera_uuid: 'cam-3', x: 0.6, y: 0.6 }] }] });
    if (target.startsWith('/api/live/layouts/') && options.method === 'PUT') {
      const body = JSON.parse(options.body);
      window.calls.layoutPuts.push(body);
      return json({ uuid: 'v1', owned: true, revision: 4, ...body });
    }
    if (target === '/api/live/layouts') return json({ can_modify: true, can_share: false, layouts: [{
      uuid: 'v1', name: 'Yard', owned: true, revision: 3, columns: 2, rows: 1, availability: 'live',
      is_shared: false, location_uuid: null, camera_slots: [{ camera_uuid: 'cam-1' }] }] });
    if (target === '/api/investigations/search') {
      window.calls.searches += 1;
      if (window.calls.searches === 1) return json({ error: 'search backend down' }, 500);
      return json({ results: [{ result_id: 'r1', event_type: 'detection', start_time: 1700000000,
        detection: { label: 'person' }, camera: { name: 'Gate' } }] });
    }
    return new Response('', { status: 404 });
  };
  const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
  window.mount = () => render(h(QueryClientProvider, { client }, h(LiveOperatorNavigator, {
    streams, orderedStreams: streams.slice(0, 2), columns: 2, rows: 1, operatorSurface: 'grid',
    selectedPlanUuid: '', onFilterChange: () => {}, onSurfaceChange: () => {}, onSelectPlan: () => {},
    onApplyLayout: (layout) => window.calls.applied.push(layout),
  })), document.getElementById('root'));
 `,
    },
    bundle: true,
    write: false,
    format: 'iife',
    jsx: 'automatic',
    jsxImportSource: 'preact',
    loader: { '.js': 'jsx' },
    alias: { react: 'preact/compat', 'react-dom': 'preact/compat' },
    plugins: [{
      name: 'fixture',
      setup(builder) {
        builder.onResolve({ filter: /(?:\/i18n|ToastContainer)\.tsx?$/ }, (args) => ({
          path: args.path, namespace: 'fixture',
        }));
        builder.onLoad({ filter: /.*/, namespace: 'fixture' }, () => ({
          contents: "export const t=(key,params)=>params?key+JSON.stringify(params):key;export const useI18n=()=>({t});export const showStatusMessage=()=>{};",
          loader: 'js',
        }));
      },
    }],
  });
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage({ viewport: { width: 1000, height: 1000 } });
  page.setDefaultTimeout(5000);
  await page.route('http://fixture.invalid/**', (route) => route.fulfill({
    contentType: 'text/html', body: '<div id="root"></div>',
  }));
  await page.goto('http://fixture.invalid/');
  await page.addScriptTag({ content: output.outputFiles[0].text });
  await page.evaluate(() => {
    localStorage.clear();
    window.mount();
  });
  return { browser, page };
}

const tab = (page, name) => page.getByRole('tab', { name: `live.navigator.${name}`, exact: true });

it('groups plan cameras, never spins on an empty Events scope, and updates views', async () => {
  const { browser, page } = await openNavigator();
  try {
    // Cameras placed on orchard48 are listed under the plan, with its location.
    const planRow = page.locator('.live-plan-scope-list .live-location-row', { hasText: 'orchard48' });
    await planRow.waitFor();
    expect(await planRow.locator('> small').textContent()).toBe('3/3');
    expect(await planRow.textContent()).toContain('Orchard');
    await planRow.click();
    await page.waitForFunction(() => document.querySelectorAll('.live-camera-row').length === 3);
    expect(await page.locator('.live-camera-row').first().textContent()).toContain('orchard48');

    // The empty Orchard location: Events explains instead of loading forever.
    await page.locator('.live-location-tree:not(.live-plan-scope-list) .live-location-row', { hasText: 'Orchard' }).click();
    await tab(page, 'events').click();
    await page.getByText('live.navigator.eventsNoScope').waitFor();
    expect(await page.getByText('live.navigator.loadingEvents').count()).toBe(0);
    expect(await page.evaluate(() => window.calls.searches)).toBe(0);

    // With cameras in scope, a failing search shows an error with Retry.
    await tab(page, 'cameras').click();
    await page.locator('.live-location-row', { hasText: 'live.navigator.allCameras' }).click();
    await tab(page, 'events').click();
    await page.getByRole('alert').waitFor();
    expect(await page.getByRole('alert').textContent()).toContain('search backend down');
    await page.getByRole('alert').getByRole('button', { name: 'live.navigator.retry' }).click();
    await page.locator('.live-event-row', { hasText: 'person' }).waitFor();

    // Views: camera count, and update-from-current-grid sends the revision.
    await tab(page, 'views').click();
    const view = page.locator('.live-layout-row', { hasText: 'Yard' });
    await view.waitFor();
    expect(await view.textContent()).toContain('live.navigator.layoutCameraCount{"count":1}');
    await view.getByRole('button', { name: 'live.navigator.updateLayoutTitle' }).click();
    await page.waitForFunction(() => window.calls.layoutPuts.length === 1);
    const [put] = await page.evaluate(() => window.calls.layoutPuts);
    expect(put).toMatchObject({
      name: 'Yard', revision: 3, columns: 2, rows: 1,
      camera_slots: [{ camera_uuid: 'cam-1' }, { camera_uuid: 'cam-2' }],
    });
  } finally {
    await browser.close();
  }
}, 30000);
