/** Real StreamsView, cards, modal, query observers and adapters in Chromium. */
import { readFileSync } from 'node:fs';
import path from 'node:path';
import { build } from 'esbuild';
import { chromium } from '../../../node_modules/playwright';
import type { Browser, Page } from '../../../node_modules/playwright';

type Mode = 'admin' | 'viewer' | 'empty' | 'loading' | 'malformed-summary' | 'malformed-full' | 'redacted-full' | 'forbidden-full';
interface CapturedCall { url: string; method: string; body: unknown }
declare global {
  interface Window {
    contractMode: Mode;
    contractCalls: CapturedCall[];
    contractMessages: string[];
    contractAborted: boolean;
    contractReleaseSummary: () => void;
    contractRestoreSummary: () => void;
    contractUnmount: () => void;
    contractMapping: { form: Record<string, unknown>; dto: Record<string, unknown> };
  }
}

let browser: Browser;
let compiled: string;
let page: Page;
let errors: string[];
let logs: string[];

beforeAll(async () => {
  const baseline = readFileSync(path.resolve(__dirname, '../../../tests/contracts/fixtures/baseline-payloads.json'), 'utf8');
  const output = await build({
    stdin: { resolveDir: path.resolve(__dirname, '../..'), loader: 'jsx', contents: `
      import {h,render} from 'preact';
      import {QueryClient,QueryClientProvider} from './js/query-client.ts';
      import {StreamsView,streamFormFromFull,streamSavePayloadFromForm} from './js/components/preact/StreamsView.tsx';
      const baseline=${baseline}.cases;
      const mode=window.contractMode;
      let summary=structuredClone(baseline[mode==='viewer'?'viewer-summary':'admin-summary'].body);
      let full=structuredClone(baseline[mode==='redacted-full'?'viewer-full':'admin-full'].body);
      Object.assign(full.stream,{retention_days:0,detection_retention_days:0,detection_threshold:0,
        detection_interval:0,pre_detection_buffer:0,post_detection_buffer:0,tier_critical_multiplier:0,
        future_private_field:'do-not-copy-into-form'});
      if(mode==='empty'||mode==='loading')summary={streams:[],total:0,page:1,page_size:50,total_pages:0};
      if(mode==='malformed-summary')summary={streams:[{name:'sensitive-malformed-row',url:'rtsp://private:secret@internal.invalid'}]};
      if(mode==='malformed-full')full={stream:{name:'sensitive-malformed-detail',url:'rtsp://private:secret@internal.invalid'}};
      window.contractCalls=[];window.contractMessages=[];window.contractAborted=false;
      window.contractRestoreSummary=()=>{summary=structuredClone(baseline['admin-summary'].body)};
      const json=(value,status=200)=>new Response(JSON.stringify(value),{status,headers:{'Content-Type':'application/json'}});
      window.fetch=async(input,options={})=>{
        const url=String(input),method=options.method||'GET';
        window.contractCalls.push({url,method,body:options.body?JSON.parse(options.body):null});
        if(url.startsWith('/api/streams?')){
          if(mode==='loading')return new Promise((resolve,reject)=>{
            window.contractReleaseSummary=()=>resolve(json(summary));
            options.signal.addEventListener('abort',()=>{window.contractAborted=true;reject(new DOMException('Aborted','AbortError'))},{once:true});
          });
          return json(summary);
        }
        if(url.endsWith('/full'))return mode==='forbidden-full'?json({error:'private-server-detail'},403):json(full);
        if(url.endsWith('/zones'))return json({zones:[]});
        if(method==='PUT')return json({success:true,message:'Accepted'},202);
        if(url==='/api/detection/models')return json({models:[]});
        return json({});
      };
      const client=new QueryClient({defaultOptions:{queries:{retry:false},mutations:{retry:false}}});
      window.contractUnmount=()=>render(null,document.getElementById('root'));
      if(!['malformed-full'].includes(mode)){
        const form=streamFormFromFull(full.stream);
        window.contractMapping={form,dto:streamSavePayloadFromForm(form,true)};
      }
      render(h(QueryClientProvider,{client},h(StreamsView)),document.getElementById('root'));
    ` }, bundle: true, write: false, format: 'iife', jsx: 'automatic', jsxImportSource: 'preact',
    alias: { react: 'preact/compat', 'react-dom': 'preact/compat' },
    plugins: [{ name: 'unrelated-fixtures', setup(builder) {
      builder.onResolve({ filter: /(?:i18n\.ts|auth-utils\.ts|ToastContainer\.tsx|HealthView\.tsx|FleetView\.tsx|EventRoutingView\.tsx|ZoneEditor\.tsx)$/ }, args => ({ path: args.path, namespace: 'fixture' }));
      builder.onLoad({ filter: /.*/, namespace: 'fixture' }, args => ({ loader: 'jsx', contents:
        args.path.endsWith('i18n.ts') ? 'export const t=(key,params)=>key+(params?JSON.stringify(params):"");export const useI18n=()=>({t});' :
        args.path.endsWith('auth-utils.ts') ? 'export const validateSession=async()=>({valid:true,role:"admin"});' :
        args.path.endsWith('ToastContainer.tsx') ? 'export const showStatusMessage=(message)=>window.contractMessages.push(message);' :
        `export const ${path.basename(args.path, '.tsx')}=()=>null;`,
      }));
    } }],
  });
  const file = output.outputFiles[0];
  if (!file) throw new Error('Missing compiled consumer fixture');
  compiled = file.text;
  browser = await chromium.launch({ headless: true });
});
afterAll(async () => browser?.close());
afterEach(async () => { await page?.close(); });

async function open(mode: Mode): Promise<void> {
  page = await browser.newPage(); page.setDefaultTimeout(5000);
  errors = []; logs = [];
  page.on('pageerror', error => errors.push(error.message));
  page.on('console', message => logs.push(message.text()));
  await page.route('http://fixture.invalid/**', route => route.fulfill({ contentType: 'text/html', body: '<style>svg{width:16px;height:16px}button{margin:3px}</style><div id="root"></div>' }));
  await page.goto('http://fixture.invalid/');
  await page.evaluate(value => { window.contractMode = value; }, mode);
  await page.addScriptTag({ content: compiled });
}

test('loading and valid empty inventory have distinct visible states', async () => {
  await open('loading');
  await page.getByText('streams.loadingStreams', { exact: true }).waitFor();
  expect(await page.getByText('streams.noStreamsConfiguredYetAdd', { exact: true }).count()).toBe(0);
  await page.waitForFunction(() => typeof window.contractReleaseSummary === 'function');
  await page.evaluate(() => window.contractReleaseSummary());
  await page.getByText('streams.noStreamsConfiguredYetAdd', { exact: true }).waitFor();
  expect(await page.getByRole('alert').count()).toBe(0);
  expect(errors).toEqual([]);
});

test('malformed summary displays a safe retryable error instead of an empty inventory or unvalidated row', async () => {
  await open('malformed-summary');
  await page.getByRole('alert').waitFor();
  expect(await page.getByRole('alert').innerText()).toContain('Invalid listStreamSummaries response');
  expect(await page.locator('body').innerText()).not.toMatch(/sensitive-malformed|private:secret|internal.invalid/);
  expect(await page.getByText('streams.noStreamsConfiguredYetAdd', { exact: true }).count()).toBe(0);
  await page.evaluate(() => window.contractRestoreSummary());
  await page.getByRole('button', { name: 'common.retry', exact: true }).click();
  await page.getByRole('button', { name: 'common.edit', exact: true }).first().waitFor();
  expect(await page.getByRole('alert').count()).toBe(0);
  expect(logs.join('\n')).not.toMatch(/sensitive-malformed|private:secret|internal.invalid/);
  expect(errors).toEqual([]);
});

test('per-camera redacted summary hides mutation controls even for a global admin', async () => {
  await open('viewer');
  await page.locator('#streams-grid [data-stream]').waitFor();
  expect(await page.getByRole('button', { name: 'common.edit', exact: true }).first().count()).toBe(0);
  expect(await page.getByRole('button', { name: 'streams.cloneStream', exact: true }).count()).toBe(0);
  expect(await page.locator('#streams-grid a[target="_blank"]').count()).toBe(0);
  expect(errors).toEqual([]);
});

test('full details map zero-valued settings, keep UI metadata out of PUT, and report queued acknowledgement', async () => {
  await open('admin');
  await page.getByRole('button', { name: 'common.edit', exact: true }).first().click();
  await page.locator('#stream-name').waitFor();
  const mapping = await page.evaluate(() => window.contractMapping);
  expect(mapping.form).toMatchObject({ retentionDays: 0, detectionThreshold: 0, detectionInterval: 0, preBuffer: 0, postBuffer: 0, tierCriticalMultiplier: 0 });
  expect(mapping.form).not.toHaveProperty('future_private_field');
  expect(mapping.dto).toMatchObject({ retention_days: 0, detection_threshold: 0, detection_interval: 0, tier_critical_multiplier: 0 });
  expect(mapping.dto).not.toHaveProperty('canControlPrivacy');
  await page.locator('#stream-save-btn').click();
  await page.locator('#stream-name').waitFor({ state: 'detached' });
  const calls = await page.evaluate(() => window.contractCalls);
  expect(calls.find(call => call.url.endsWith('/full'))?.url).toBe('/api/streams/A/full');
  const write = calls.find(call => call.method === 'PUT');
  expect(write).toMatchObject({ url: '/api/streams/A', body: { retention_days: 0, detection_threshold: 0 } });
  expect(write?.body).not.toHaveProperty('name');
  expect(write?.body).not.toHaveProperty('future_private_field');
  expect(write?.body).not.toHaveProperty('can_control_privacy');
  expect(await page.evaluate(() => window.contractMessages)).toContain('streams.streamUpdateQueued');
  expect(errors).toEqual([]);
});

test.each(['malformed-full', 'redacted-full', 'forbidden-full'] as const)('%s never opens an editable form with untrusted or newly redacted values', async mode => {
  await open(mode);
  await page.getByRole('button', { name: 'common.edit', exact: true }).first().click();
  await page.waitForFunction(() => window.contractMessages.length > 0);
  const messages = (await page.evaluate(() => window.contractMessages)).join('\n');
  expect(messages).toContain(mode === 'redacted-full' ? 'streams.readOnlyInsufficientPermissions' : 'streams.errorLoadingStreamDetails');
  if (mode === 'forbidden-full') expect(messages).toContain('HTTP 403');
  expect(messages).not.toMatch(/sensitive-malformed|private:secret|private-server-detail/);
  expect(await page.locator('#stream-name').count()).toBe(0);
  expect(errors).toEqual([]);
});

test('query cancellation reaches the adapter transport when the consumer unmounts', async () => {
  await open('loading');
  await page.getByText('streams.loadingStreams', { exact: true }).waitFor();
  await page.waitForFunction(() => typeof window.contractReleaseSummary === 'function');
  await page.evaluate(() => window.contractUnmount());
  await page.waitForFunction(() => window.contractAborted);
  expect(errors).toEqual([]);
});
