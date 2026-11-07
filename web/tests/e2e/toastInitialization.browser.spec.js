/** The legacy toast API must be ready synchronously and share its provider root. */
const path = require('node:path');

const FIXTURE = `
  import {h, render} from 'preact';
  import {useLayoutEffect} from 'preact/hooks';
  import {initToasts, showToast, ToastProvider, useToast} from './js/components/preact/Toast.tsx';
  window.toastApi = {initToasts, showToast};
  window.firstToast = showToast('Immediate import toast', 'success', 0);
  function Child() {
    const toast = useToast();
    useLayoutEffect(() => { toast.showWarningToast('Provider mount toast', 0); }, []);
    return null;
  }
  window.mountProviders = () => {
    for (const id of ['provider-a', 'provider-b']) {
      render(h(ToastProvider, null, h(Child)), document.getElementById(id));
    }
  };
  window.unmountProviders = () => {
    for (const id of ['provider-a', 'provider-b']) render(null, document.getElementById(id));
  };
`;
let browser, page, bundle, errors;
beforeAll(async () => {
  const {chromium} = require('../../../node_modules/playwright');
  const {build} = require('esbuild');
  const output = await build({
    stdin: {resolveDir: path.resolve(__dirname, '../..'), loader: 'jsx', contents: FIXTURE},
    bundle: true, write: false, format: 'iife', jsx: 'automatic', jsxImportSource: 'preact',
    plugins: [{name: 'i18n-fixture', setup(builder) {
      builder.onResolve({filter: /\/i18n\.ts$/}, args => ({path: args.path, namespace: 'fixture'}));
      builder.onLoad({filter: /.*/, namespace: 'fixture'}, () => ({contents:
        "export const useI18n=()=>({t:key=>key==='common.close'?'Close':key});", loader: 'js'}));
    }}],
  });
  bundle = output.outputFiles[0].text;
  browser = await chromium.launch({headless: true});
});
beforeEach(async () => {
  page = await browser.newPage();
  page.setDefaultTimeout(5000);
  errors = [];
  page.on('pageerror', error => errors.push(error.message));
  await page.setContent('<div id="provider-a"></div><div id="provider-b"></div>');
  await page.addScriptTag({content: bundle});
});
afterEach(async () => { await page?.close(); expect(errors).toEqual([]); });
afterAll(async () => { await browser?.close(); });

const toast = message => page.locator('[data-toast-id]').filter({hasText: message});

test('imperative toasts work immediately and repeated initialization reuses the class instance', async () => {
  await toast('Immediate import toast').waitFor({state: 'visible'});
  expect(await page.evaluate(() => window.toastApi.initToasts() === window.toastApi.initToasts())).toBe(true);
  expect(await page.locator('#toast-container-root').count()).toBe(1);
  expect(await page.evaluate(() => typeof window.firstToast)).toBe('number');
});

test('providers can toast from child mount hooks and unmount without breaking global callers', async () => {
  await page.evaluate(() => window.mountProviders());
  await page.waitForFunction(() => document.querySelectorAll('[data-toast-id]').length === 3);
  expect(await toast('Provider mount toast').count()).toBe(2);
  expect(await page.locator('#toast-container-root').count()).toBe(1);
  await page.evaluate(() => {
    window.unmountProviders();
    window.toastApi.showToast('After provider unmount', 'info', 0);
  });
  await toast('After provider unmount').waitFor({state: 'visible'});
  expect(await page.locator('#toast-container-root').count()).toBe(1);
});

test('same-tick toast IDs are distinct and closing one preserves its neighbor', async () => {
  const ids = await page.evaluate(() => {
    const originalNow = Date.now;
    Date.now = () => 1234;
    try {
      return [window.toastApi.showToast('First sibling', 'info', 0),
        window.toastApi.showToast('Second sibling', 'info', 0)];
    } finally { Date.now = originalNow; }
  });
  expect(ids[0]).not.toBe(ids[1]);
  await toast('First sibling').getByRole('button').click();
  await toast('First sibling').waitFor({state: 'detached'});
  expect(await toast('Second sibling').count()).toBe(1);
});

test('a detached toast root is remounted and its old expiry timers are cleared', async () => {
  const oldId = await page.evaluate(() => window.toastApi.showToast('Old timed toast', 'info', 600));
  await toast('Old timed toast').waitFor({state: 'visible'});
  await page.evaluate(() => {
    window.oldContainer = window.toastApi.initToasts();
    window.oldTimerFired = false;
    window.oldContainer.removeToast = () => { window.oldTimerFired = true; };
    document.getElementById('toast-container-root').remove();
    window.toastApi.showToast('Replacement root', 'success', 0);
  });
  await toast('Replacement root').waitFor({state: 'visible'});
  expect(await page.locator(`[data-toast-id="${oldId}"]`).count()).toBe(0);
  // A separate new toast expiring proves the old timeout deadline has elapsed.
  await page.evaluate(() => window.toastApi.showToast('New timed toast', 'info', 900));
  await toast('New timed toast').waitFor({state: 'attached'});
  await toast('New timed toast').waitFor({state: 'detached'});
  expect(await page.evaluate(() => window.oldTimerFired)).toBe(false);
  expect(await page.locator('#toast-container-root').count()).toBe(1);
  expect(await toast('Replacement root').count()).toBe(1);
});

async function pauseClock() {
  await page.clock.install({time: new Date('2026-01-01T00:00:00Z')});
  await page.clock.pauseAt(new Date('2026-01-01T00:00:01Z'));
}

test('automatic dismissal enters the exit animation before its deadline', async () => {
  await pauseClock();
  await page.evaluate(() => window.toastApi.showToast('Automatic fade', 'info', 1000));
  await page.clock.runFor(100);
  expect(await toast('Automatic fade').evaluate(el => el.style.opacity)).toBe('1');
  await page.clock.runFor(600);
  expect(await toast('Automatic fade').evaluate(el => el.style.opacity)).toBe('0');
  expect(await toast('Automatic fade').evaluate(el => el.style.transitionDuration)).toBe('300ms, 300ms');
  await page.clock.runFor(299);
  expect(await toast('Automatic fade').count()).toBe(1);
  await page.clock.runFor(1);
  expect(await toast('Automatic fade').count()).toBe(0);
});

test('a short duration bounds both the animation and removal deadline', async () => {
  await pauseClock();
  await page.evaluate(() => window.toastApi.showToast('Short fade', 'info', 100));
  await page.clock.runFor(1);
  expect(await toast('Short fade').evaluate(el => el.style.opacity)).toBe('0');
  expect(await toast('Short fade').evaluate(el => el.style.transitionDuration)).toBe('100ms, 100ms');
  await page.clock.runFor(98);
  expect(await toast('Short fade').count()).toBe(1);
  await page.clock.runFor(1);
  expect(await toast('Short fade').count()).toBe(0);
});

test('persistent notifications expose a named close control and stay until dismissed', async () => {
  await pauseClock();
  await page.clock.runFor(10000);
  expect(await toast('Immediate import toast').count()).toBe(1);
  await toast('Immediate import toast').getByRole('button', {name: 'Close'}).click();
  await page.clock.runFor(299);
  expect(await toast('Immediate import toast').count()).toBe(1);
  await page.clock.runFor(1);
  expect(await toast('Immediate import toast').count()).toBe(0);
});
