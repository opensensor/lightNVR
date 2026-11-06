/** Exercise SettingsView's load/save/refetch boundary with a server-supported zero. */
const path = require('node:path');

async function openSettings(initial) {
  const { chromium } = require('../../../node_modules/playwright');
  const { build } = require('esbuild');
  const compiled = await build({
    stdin: {
      resolveDir: path.resolve(__dirname, '../..'), loader: 'jsx',
      contents: `
        import {h,render} from 'preact';
        import {QueryClient,QueryClientProvider} from './js/query-client.ts';
        import {SettingsView} from './js/components/preact/SettingsView.tsx';
        window.saved=[];
        window.serverSettings={log_level:2,storage_path:'/fixture',web_port:8080,
          web_auth_enabled:true,models_path:'/fixture/models-0',${initial}};
        window.fetch=async(input,options={})=>{
          const url=String(input);
          let body={};
          if(url.includes('/api/settings')) {
            if(options.method==='POST') {
              const sent=JSON.parse(options.body);window.saved.push(sent);
              Object.assign(window.serverSettings,sent);
              // A changed response field provides an observable refetch barrier.
              window.serverSettings.models_path='/fixture/models-'+window.saved.length;
              body={success:true};
            } else body=window.serverSettings;
          } else if(url.includes('trusted-devices')) body={devices:[]};
          else if(url.includes('/sessions')) body={sessions:[]};
          return new Response(JSON.stringify(body),{status:200,headers:{'Content-Type':'application/json'}});
        };
        const client=new QueryClient({defaultOptions:{queries:{retry:false}}});
        render(h(QueryClientProvider,{client},h(SettingsView)),document.getElementById('root'));
      `,
    },
    bundle: true, write: false, format: 'iife', jsx: 'automatic', jsxImportSource: 'preact',
    alias: {react: 'preact/compat', 'react-dom': 'preact/compat'},
    plugins: [{name: 'fixture', setup(builder) {
      builder.onResolve({filter: /(?:i18n\.ts|auth-utils\.ts)$/}, args => ({path: args.path, namespace: 'fixture'}));
      builder.onLoad({filter: /.*/, namespace: 'fixture'}, args => ({loader: 'js', contents:
        args.path.endsWith('auth-utils.ts')
          ? 'export const validateSession=async()=>({valid:true,role:"admin"});'
          : 'export const t=key=>key;export const useI18n=()=>({t});',
      }));
    }}],
  });
  const browser = await chromium.launch({headless: true});
  const page = await browser.newPage();
  page.setDefaultTimeout(5000);
  await page.route('http://fixture.invalid/**', route => route.fulfill({
    contentType: 'text/html', body: '<style>svg{width:20px;height:20px}</style><div id="root"></div>',
  }));
  await page.goto('http://fixture.invalid/#detection');
  await page.addScriptTag({content: compiled.outputFiles[0].text});
  await page.locator('#setting-default-detection-threshold').waitFor();
  await page.waitForFunction(() => document.querySelector('[name="detectionModelsPath"]').value === '/fixture/models-0');
  return {browser, page};
}

it('retains zero through save, server refetch and a second save', async () => {
  const {browser, page} = await openSettings('default_detection_threshold:12');
  try {
    const threshold = page.locator('#setting-default-detection-threshold');
    await page.waitForFunction(() => document.querySelector('#setting-default-detection-threshold').value === '12');
    await threshold.press('Home');
    await page.locator('#save-settings-btn').click();
    await page.waitForFunction(() => window.saved.length === 1 &&
      document.querySelector('[name="detectionModelsPath"]').value === '/fixture/models-1');
    expect(await page.evaluate(() => window.saved[0].default_detection_threshold)).toBe(0);
    expect(await threshold.inputValue()).toBe('0');
    await page.locator('[name="detectionGracePeriod"]').fill('3');
    await page.locator('[name="detectionGracePeriod"]').dispatchEvent('change');
    await page.locator('#save-settings-btn').click();
    await page.waitForFunction(() => window.saved.length === 2);
    expect(await page.evaluate(() => window.saved[1].default_detection_threshold)).toBe(0);
  } finally {await browser.close();}
});

it.each([['default_detection_threshold:0', '0'], ['', '50']])(
  'loads the intended threshold for %s', async (initial, expected) => {
    const {browser, page} = await openSettings(initial);
    try {
      await page.waitForFunction(() => document.querySelector('#save-settings-btn').disabled);
      expect(await page.locator('#setting-default-detection-threshold').inputValue()).toBe(expected);
    } finally {await browser.close();}
  },
);
