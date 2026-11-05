/** First activation must preserve the native checkbox value while selecting its zone. */
const path = require('node:path');
const fixtureZones = [{
  id: 'one',
  name: 'Entry',
  enabled: true,
  color: '#3b82f6',
  polygon: [{
    x: .1,
    y: .1
  }, {
    x: .4,
    y: .1
  }, {
    x: .2,
    y: .4
  }]
}];
const t = key => 'translated:' + key;
// Run with: npm --prefix web run test:e2e -- --testPathPatterns='\.browser\.spec\.js$'
const browserTest = it;
async function openZoneBrowser(deferred = false) {
  const {
    chromium
  } = require('../../../node_modules/playwright');
  const {
    build
  } = require('esbuild');
  const webRoot = path.resolve(__dirname, '../..');
  const output = await build({
    stdin: {
      resolveDir: webRoot,
      loader: 'jsx',
      contents: `
  import {h,render} from 'preact';
  import {ZoneEditor} from './js/components/preact/ZoneEditor.tsx';
  const zones=${JSON.stringify(fixtureZones)};
  window.originalZones=zones;
  window.pendingZones=[];
  window.pendingSnapshots=[];
  window.calls={saved:[],closed:0,events:[],requests:[]};
  window.fetch=async(url,options={})=>{if(options.method==='POST'){window.calls.requests.push({url,method:options.method,body:JSON.parse(options.body)});return{ok:true,json:async()=>({})};}if(${deferred})return new Promise(resolve=>window.pendingZones.push({url,resolve}));return{ok:true,json:async()=>{window.zonesLoaded=true;return{zones};}};};
  window.mount=(streamName='Entry')=>render(h(ZoneEditor,{streamName,streamWidth:640,streamHeight:360,zones,onClose:()=>window.calls.closed++,onZonesChange:value=>window.calls.saved.push(value)}),document.getElementById('root'));
  for(const capture of[true,false])for(const type of['click','input','change'])document.addEventListener(type,event=>{if(event.target.type==='checkbox')window.calls.events.push({type,capture,checked:event.target.checked,selected:String(document.querySelector('input[type=checkbox]')?.closest('div.mb-3.p-3.rounded.border')?.classList.contains('border-primary'))});},capture);
 `
    },
    bundle: true,
    write: false,
    format: 'iife',
    jsx: 'automatic',
    jsxImportSource: 'preact',
    plugins: [{
      name: 'fixture',
      setup(builder) {
        builder.onResolve({
          filter: /(?:i18n|settings-utils)\.ts$/
        }, args => ({
          path: args.path,
          namespace: 'fixture'
        }));
        builder.onLoad({
          filter: /.*/,
          namespace: 'fixture'
        }, () => ({
          contents: "export const t=key=>'translated:'+key;export const useI18n=()=>({t});export const getGo2rtcBaseUrl=()=>window.deferSnapshots?new Promise(resolve=>window.pendingSnapshots.push(resolve)):Promise.resolve('/go2rtc');",
          loader: 'js'
        }));
      }
    }]
  });
  const browser = await chromium.launch({
    headless: true
  });
  const page = await browser.newPage({
    viewport: {
      width: 1200,
      height: 1000
    }
  });
  page.setDefaultTimeout(5000);
  await page.route('http://fixture.invalid/**', route => route.fulfill({
    contentType: route.request().url() === 'http://fixture.invalid/' ? 'text/html' : 'image/svg+xml',
    body: route.request().url() === 'http://fixture.invalid/' ? '<div id="root"></div>' : '<svg xmlns="http://www.w3.org/2000/svg" width="640" height="360"><rect width="640" height="360" fill="gray"/></svg>'
  }));
  await page.goto('http://fixture.invalid/');
  // Standalone fixture layout: no shared build assets, server or operator state.
  await page.addStyleTag({
    content: 'canvas{max-width:640px;max-height:360px}button,input{margin:4px}'
  });
  await page.addScriptTag({
    content: output.outputFiles[0].text
  });
  await page.evaluate(deferred => { window.deferSnapshots = deferred; window.mount(); }, deferred);
  try {
    await page.waitForFunction(deferred => (deferred || window.zonesLoaded) && document.querySelector('input[type=checkbox]'), deferred, {
      timeout: 5000
    });
  } catch (error) {
    await browser.close();
    throw error;
  }
  return {
    browser,
    page
  };
}
browserTest.each([['pointer', false], ['keyboard', false], ['label', false], ['pointer', true], ['keyboard', true], ['label', true]])('first zone checkbox activation preserves value and exact save (method=%s, preselect=%s)', async (method, preselect) => {
  const {
    browser,
    page
  } = await openZoneBrowser();
  try {
    const checkbox = page.getByRole('checkbox', {
      name: t('common.enabled'),
      exact: true
    });
    const card = page.locator('div.mb-3.p-3.rounded.border').first();
    const selectedState = () => card.evaluate(element => String(element.classList.contains('border-primary')));
    expect(await checkbox.isChecked()).toBe(true);
    expect(await selectedState()).toBe('false');
    if (preselect) await card.locator('input[type=text]').click();
    const activate = async () => {
      if (method === 'pointer') await checkbox.click();else if (method === 'label') await checkbox.locator('..').locator('span').click();else {
        // Reach the checkbox solely through native Tab navigation; never click a field/card.
        for (let index = 0; index < 20 && !(await checkbox.evaluate(element => element === document.activeElement)); index++) await page.keyboard.press('Tab');
        expect(await checkbox.evaluate(element => element === document.activeElement)).toBe(true);
        await page.keyboard.press('Space');
      }
    };
    await activate();
    const firstChecked = await checkbox.isChecked();
    expect(await checkbox.evaluate(element => element === document.activeElement)).toBe(true);
    await activate();
    await activate();
    const checked = await checkbox.isChecked();
    const selected = await selectedState();
    await page.getByRole('button', {
      name: t('zoneEditor.saveZones'),
      exact: true
    }).click();
    await page.waitForFunction(() => window.calls.saved.length === 1);
    const calls = await page.evaluate(() => window.calls);
    if (checked !== false) console.log('ZONE_FIRST_TOGGLE_TRACE', JSON.stringify({
      method,
      preselect,
      checked,
      selected,
      calls
    }));
    expect(calls.events.filter(event => event.type === 'change' && !event.capture)).toHaveLength(3);
    expect({
      firstChecked,
      checked,
      selected,
      saved: calls.saved,
      closed: calls.closed,
      requests: calls.requests
    }).toEqual({
      firstChecked: false,
      checked: false,
      selected: 'true',
      saved: [[{
        ...fixtureZones[0],
        enabled: false
      }]],
      closed: 1,
      requests: [{
        url: '/api/streams/Entry/zones',
        method: 'POST',
        body: {
          zones: [{
            ...fixtureZones[0],
            enabled: false
          }]
        }
      }]
    });
  } finally {
    await browser.close();
  }
}, 30000);
it('selects a zone row when keyboard focus enters it', async () => {
  const {
    browser,
    page
  } = await openZoneBrowser();
  try {
    const card = page.locator('div.mb-3.p-3.rounded.border').first();
    const selected = () => card.evaluate(element => element.classList.contains('border-primary'));
    expect(await selected()).toBe(false);
    const nameInput = card.locator('input[type=text]');
    for (let index = 0; index < 20 && !(await nameInput.evaluate(element => element === document.activeElement)); index += 1) await page.keyboard.press('Tab');
    expect(await nameInput.evaluate(element => element === document.activeElement)).toBe(true);
    await page.waitForFunction(() => document.querySelector('div.mb-3.p-3.rounded.border')?.classList.contains('border-primary'));
    expect(await selected()).toBe(true);
  } finally {
    await browser.close();
  }
}, 30000);
browserTest('drawing stops at the point limit the API accepts', async () => {
  const {
    browser,
    page
  } = await openZoneBrowser();
  try {
    // The fixture has no Tailwind, so give the canvas a clickable size.
    await page.addStyleTag({
      content: 'canvas{width:640px;height:360px}'
    });
    await page.getByRole('button', {
      name: t('zoneEditor.drawZone')
    }).click();
    const canvas = page.locator('canvas');
    for (let index = 0; index < 33; index++) await canvas.click({
      position: {
        x: 100 + index * 10,
        y: 100 + index % 2 * 100
      }
    });
    await page.getByRole('status').filter({
      hasText: t('zoneEditor.maxPointsReached')
    }).waitFor();
    await page.getByRole('button', {
      name: t('zoneEditor.completeZoneWithPoints')
    }).click();
    await page.getByRole('button', {
      name: t('zoneEditor.saveZones'),
      exact: true
    }).click();
    await page.waitForFunction(() => window.calls.requests.length === 1);
    const [request] = await page.evaluate(() => window.calls.requests);
    expect(request.body.zones.map(zone => zone.polygon.length)).toEqual([3, 32]);
  } finally {
    await browser.close();
  }
}, 30000);


it('ignores late zone and snapshot loads after switching cameras', async () => {
  const { browser, page } = await openZoneBrowser(true);
  try {
    await page.waitForFunction(() => window.pendingZones.length === 1 && window.pendingSnapshots.length === 1);
    await page.evaluate(() => window.mount('Second'));
    await page.waitForFunction(() => window.pendingZones.length === 2 && window.pendingSnapshots.length === 2);
    await page.evaluate(() => {
      window.pendingZones[1].resolve({ok:true,json:async()=>({zones:[{...window.originalZones[0],name:'Second zone'}]})});
      window.pendingSnapshots[1]('/second');
    });
    await page.waitForFunction(() => document.querySelector('input[type=text]')?.value === 'Second zone');
    await page.locator('img').waitFor({state:'attached'});
    await page.evaluate(() => {
      window.pendingZones[0].resolve({ok:true,json:async()=>({zones:[{...window.originalZones[0],name:'Stale zone'}]})});
      window.pendingSnapshots[0]('/stale');
    });
    await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
    expect(await page.locator('input[type=text]').first().inputValue()).toBe('Second zone');
    expect(await page.locator('img').getAttribute('src')).toContain('/second/api/frame.jpeg?src=Second');
  } finally { await browser.close(); }
});

it.each(['vertex', 'polygon'])('dragging a %s does not mutate the supplied zone objects', async kind => {
  const { browser, page } = await openZoneBrowser();
  try {
    await page.addStyleTag({content:'canvas{width:640px;height:360px}'});
    await page.getByRole('button', {name:t('zoneEditor.selectZone')}).click();
    await page.waitForFunction(() => document.querySelector('img')?.complete);
    await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(resolve))));
    const canvas = page.locator('canvas');
    await canvas.scrollIntoViewIfNeeded();
    const box = await canvas.boundingBox();
    // The canvas has a 400px minimum height; account for the letterboxed image.
    const imageHeight = box.width * 360 / 640;
    const offsetY = (box.height - imageHeight) / 2;
    const start = kind === 'vertex'
      ? {x:box.width * 0.1,y:offsetY + imageHeight * 0.1}
      : {x:box.width * 0.2,y:offsetY + imageHeight / 6};
    await page.mouse.move(box.x + start.x, box.y + start.y);
    await page.mouse.down();
    await page.evaluate(() => new Promise(requestAnimationFrame));
    await page.mouse.move(box.x + start.x + 32, box.y + start.y + 18, {steps:3});
    await page.mouse.up();
    await page.getByRole('button', {name:t('zoneEditor.saveZones'),exact:true}).click();
    await page.waitForFunction(() => window.calls.saved.length === 1);
    const result = await page.evaluate(() => ({original:window.originalZones,saved:window.calls.saved[0]}));
    expect(result.original).toEqual(fixtureZones);
    expect(result.saved[0].polygon).not.toEqual(fixtureZones[0].polygon);
  } finally { await browser.close(); }
});
