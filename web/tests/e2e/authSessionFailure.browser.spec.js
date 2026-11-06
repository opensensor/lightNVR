/** Actual AuthGate/auth utilities in Chromium; only HTTP and translations are fixtures. */
async function openBrowser() {
  const {chromium} = require('../../../node_modules/playwright');
  const {build} = require('esbuild');
  const path = require('node:path');
  const compiled = await build({
    stdin: {resolveDir: path.resolve(__dirname, '../..'), loader: 'jsx', contents: `
      import {h,render} from 'preact';
      import {AuthGate} from './js/components/preact/AuthGate.tsx';
      import {clearAuthState,setupSessionValidation} from './js/utils/auth-utils.ts';
      window.requests=[];
      window.fetch=(url,options={})=>new Promise((resolve,reject)=>{
        const request={url,aborted:false,resolve:(body,status=200)=>resolve(new Response(JSON.stringify(body),{status}))};
        options.signal?.addEventListener('abort',()=>{request.aborted=true;reject(new DOMException('Aborted','AbortError'));},{once:true});
        window.requests.push(request);
      });
      localStorage.setItem('auth','fixture-credential');
      window.mount=()=>render(h(AuthGate,null,h('main',{'data-testid':'protected'},'Protected content')),document.getElementById('root'));
      window.unmount=()=>render(null,document.getElementById('root'));
      window.periodic=()=>setupSessionValidation(1000);
      window.logout=clearAuthState;
      window.mount();
    `}, bundle: true, write: false, format: 'iife', jsx: 'automatic', jsxImportSource: 'preact',
    plugins: [{name: 'translations', setup(builder) {
      builder.onResolve({filter: /i18n\.ts$/}, args => ({path: args.path, namespace: 'translations'}));
      builder.onLoad({filter: /.*/, namespace: 'translations'}, () => ({contents: 'const t=key=>key; export const useI18n=()=>({t}); export {t};'}));
    }}],
  });
  const browser = await chromium.launch({headless: true});
  const page = await browser.newPage();
  page.setDefaultTimeout(4000);
  const errors = [];
  page.on('pageerror', error => errors.push(error.message));
  await page.route('http://fixture.invalid/**', route => route.fulfill({contentType: 'text/html', body: '<div id="root"></div>'}));
  await page.goto('http://fixture.invalid/streams.html');
  await page.addScriptTag({content: compiled.outputFiles[0].text});
  await page.waitForFunction(() => window.requests.length === 1);
  return {browser, page, errors};
}
const valid = {authenticated: true, id: 7, username: 'Fixture', role: 'viewer', is_active: true};
async function respond(page, index, body, status = 200) {
  await page.evaluate(({index, body, status}) => window.requests[index].resolve(body, status), {index, body, status});
}

it.each([503, 429, 200])('unknown initial session stays protected after HTTP %s and can retry', async status => {
  const {browser, page, errors} = await openBrowser();
  try {
    await respond(page, 0, {}, status);
    await page.getByRole('alert').waitFor();
    expect(await page.getByTestId('protected').count()).toBe(0);
    expect(await page.evaluate(() => localStorage.getItem('auth'))).toBe('fixture-credential');
    expect(page.url()).toBe('http://fixture.invalid/streams.html');
    await page.getByRole('button', {name: 'common.retry'}).click();
    await page.waitForFunction(() => window.requests.length === 2);
    expect(await page.getByRole('button', {name: 'common.retry'}).count()).toBe(0);
    await respond(page, 1, valid);
    await page.getByTestId('protected').waitFor();
    expect(errors).toEqual([]);
  } finally {await browser.close();}
});

it('keeps a verified session through outages, then clears on actual rejection', async () => {
  const {browser, page, errors} = await openBrowser();
  try {
    await respond(page, 0, valid);
    await page.getByTestId('protected').waitFor();
    await page.evaluate(() => window.periodic());
    await page.waitForFunction(() => window.requests.length === 2);
    await respond(page, 1, {error: 'Unavailable'}, 503);
    await page.waitForFunction(() => window.requests.length === 3);
    expect(await page.getByTestId('protected').count()).toBe(1);
    expect(await page.evaluate(() => localStorage.getItem('auth'))).toBe('fixture-credential');
    await respond(page, 2, {error: 'Expired'}, 401);
    await page.waitForURL('**/login.html?**');
    expect(await page.evaluate(() => localStorage.getItem('auth'))).toBeNull();
    expect(await page.getByTestId('protected').count()).toBe(0);
    expect(errors).toEqual([]);
  } finally {await browser.close();}
});

it('aborts pending verification on unmount and ignores late replies', async () => {
  const {browser, page, errors} = await openBrowser();
  try {
    await page.evaluate(() => window.unmount());
    await page.waitForFunction(() => window.requests[0].aborted);
    await respond(page, 0, {...valid, role: 'admin', auth_enabled: false});
    expect(await page.getByTestId('protected').count()).toBe(0);
    expect(await page.evaluate(() => window._authDisabled === true)).toBe(false);
    expect(await page.evaluate(() => localStorage.getItem('auth'))).toBe('fixture-credential');
    expect(errors).toEqual([]);
  } finally {await browser.close();}
});

it('bounds a stalled initial verification and offers retry without unlocking', async () => {
  const {browser, page, errors} = await openBrowser();
  try {
    // The actual deadline is ten seconds; allow it to abort the fetch in Chromium.
    await page.getByRole('alert').waitFor({timeout: 12000});
    expect(await page.evaluate(() => window.requests[0].aborted)).toBe(true);
    expect(await page.getByTestId('protected').count()).toBe(0);
    expect(await page.evaluate(() => localStorage.getItem('auth'))).toBe('fixture-credential');
    expect(await page.evaluate(() => window.requests.length)).toBe(1);
    expect(errors).toEqual([]);
  } finally {await browser.close();}
});

it('explicit logout immediately hides protected content', async () => {
  const {browser, page, errors} = await openBrowser();
  try {
    await respond(page, 0, valid);
    await page.getByTestId('protected').waitFor();
    await page.evaluate(() => window.logout());
    await page.getByTestId('protected').waitFor({state: 'detached'});
    expect(await page.evaluate(() => localStorage.getItem('auth'))).toBeNull();
    expect(errors).toEqual([]);
  } finally {await browser.close();}
});
