/**
 * System page storage: per-tier rows when recordings span a hot disk and a
 * cold bucket, and the legacy single bar otherwise (single disk, or no
 * permission to read storage targets).
 * Run with: npm --prefix web run test:e2e -- --testPathPatterns='memoryStorageTiers\.browser\.spec\.js$'
 */
const path = require('node:path');

const GB = 1024 ** 3;
const systemInfo = {
  memory: { used: 200e6, total: 8e9 }, go2rtcMemory: { used: 80e6, total: 8e9 },
  detectorMemory: { used: 0, total: 8e9 }, systemMemory: { used: 4e9, total: 8e9, capability: 'available' },
  // Legacy figure: every recording byte (hot + cold) over the local disk.
  disk: { used: 2240 * GB, total: 500 * GB, capability: 'available' },
  systemDisk: { used: 20 * GB, total: 100 * GB, capability: 'available' },
};
const hot = {
  uuid: 'hot', name: 'NVMe', target_type: 'filesystem', storage_class: 'hot', enabled: true,
  is_default: true, recording_count: 1200, recording_bytes: 180 * GB, replica_bytes: 0,
  archive_budget_bytes: 0,
  health: { status: 'healthy', capacity_known: true, capacity_bytes: 500 * GB, used_bytes: 230 * GB, pressure: 'normal' },
};
const cold = {
  uuid: 'cold', name: 'Archive', target_type: 's3', storage_class: 'cold', enabled: true,
  recording_count: 9000, recording_bytes: 2048 * GB, replica_bytes: 0, archive_budget_bytes: 4096 * GB,
  health: { status: 'healthy', capacity_known: false, capacity_bytes: 0, used_bytes: 0, pressure: 'normal' },
};

async function openPanel(targetsResponse) {
  const { chromium } = require('../../../node_modules/playwright');
  const { build } = require('esbuild');
  const webRoot = path.resolve(__dirname, '../..');
  const output = await build({
    stdin: {
      resolveDir: webRoot,
      loader: 'jsx',
      contents: `
  import { h, render } from 'preact';
  import { QueryClient, QueryClientProvider } from './js/query-client.js';
  import { MemoryStorage } from './js/components/preact/system/MemoryStorage.jsx';
  const targets = ${JSON.stringify(targetsResponse)};
  window.fetch = async (url) => String(url) === '/api/storage-targets'
    ? new Response(JSON.stringify(targets.body), { status: targets.status, headers: { 'Content-Type': 'application/json' } })
    : new Response('', { status: 404 });
  const formatBytes = (bytes) => (bytes / ${GB}).toFixed(0) + ' GB';
  const client = new QueryClient({ defaultOptions: { queries: { retry: false } } });
  window.mount = () => render(h(QueryClientProvider, { client },
    h(MemoryStorage, { systemInfo: ${JSON.stringify(systemInfo)}, formatBytes })), document.getElementById('root'));
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
        builder.onResolve({ filter: /\/i18n\.js$/ }, (args) => ({ path: args.path, namespace: 'fixture' }));
        builder.onLoad({ filter: /.*/, namespace: 'fixture' }, () => ({
          contents: "export const t=(key,params)=>params?key+JSON.stringify(params):key;export const useI18n=()=>({t});",
          loader: 'js',
        }));
      },
    }],
  });
  const browser = await chromium.launch({ headless: true });
  const page = await browser.newPage();
  page.setDefaultTimeout(5000);
  await page.route('http://fixture.invalid/**', (route) => route.fulfill({
    contentType: 'text/html', body: '<div id="root"></div>',
  }));
  await page.goto('http://fixture.invalid/');
  await page.addScriptTag({ content: output.outputFiles[0].text });
  await page.evaluate(() => window.mount());
  return { browser, page };
}

it('shows each tier against its own capacity when recordings span disk and bucket', async () => {
  const { browser, page } = await openPanel({ status: 200, body: { count: 2, targets: [cold, hot] } });
  try {
    const tiers = page.getByTestId('storage-tiers');
    await tiers.waitFor();
    const text = await tiers.textContent();
    expect(text).toContain('system.storageTiers.summary{"total":"2228 GB","count":2}');
    // Hot disk first, measured against its filesystem.
    const rows = tiers.locator('.border-l-2');
    expect(await rows.nth(0).textContent()).toContain('NVMe');
    expect(await rows.nth(0).textContent()).toContain('230 GB / 500 GB');
    expect(await rows.nth(1).textContent()).toContain('Archive');
    expect(await rows.nth(1).textContent()).toContain('2048 GB / 4096 GB');
    expect(await rows.nth(1).textContent()).toContain('system.storageTiers.budgetUsage');
    // The misleading legacy "all recordings / local disk" bar is gone.
    expect(await page.getByText('system.lightNvrStorage:').count()).toBe(0);
  } finally {
    await browser.close();
  }
}, 30000);

it('keeps the single bar for one disk and when targets are not readable', async () => {
  for (const response of [
    { status: 200, body: { count: 1, targets: [hot] } },
    { status: 403, body: { error: 'forbidden' } },
  ]) {
    const { browser, page } = await openPanel(response);
    try {
      await page.getByText('system.lightNvrStorage:').waitFor();
      await page.waitForTimeout(200);
      expect(await page.getByTestId('storage-tiers').count()).toBe(0);
    } finally {
      await browser.close();
    }
  }
}, 30000);
