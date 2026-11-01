/** Real component effects with delayed API and parent recording snapshots. */
// Run with: npm --prefix web run test:e2e -- --testPathPatterns='\.browser\.spec\.js$'
const browserTest = it;
async function openBrowser({
  delayedCatalog = false
} = {}) {
  const {
    chromium
  } = require('../../../node_modules/playwright');
  const {
    build
  } = require('esbuild');
  const path = require('node:path');
  const webRoot = path.resolve(__dirname, '../..');
  const compiled = await build({
    stdin: {
      resolveDir: webRoot,
      loader: 'jsx',
      contents: `
    import { h, render } from 'preact'; import { useState } from 'preact/hooks';
    import { BulkTagsOverlay } from './js/components/preact/recordings/TagsOverlay.jsx';
    window.calls = {requests:[],changed:0,messages:[]};
    window.api = {
      getAllRecordingTags:()=>${delayedCatalog ? 'new Promise(resolve=>window.finishCatalog=resolve)' : "Promise.resolve(['shared','partial','known'])"},
      batchUpdateRecordingTags:(...args)=>{window.calls.requests.push(args);return new Promise((resolve,reject)=>{window.succeed=resolve;window.fail=()=>reject(new Error('fixture failure'));});}
    };
    function App(){
      const [recordings,setRecordings]=useState([{id:1,tags:['shared','partial']},{id:2,tags:['shared']},{id:3,tags:['other']}]);
      const [selected,setSelected]=useState({1:true,2:true});window.refresh=setRecordings;window.select=setSelected;
      return h(BulkTagsOverlay,{recordings,selectedRecordings:selected,onClose:()=>{},onTagsChanged:()=>window.calls.changed++});
    }
    render(h(App),document.getElementById('root'));
  `
    },
    bundle: true,
    write: false,
    format: 'iife',
    jsx: 'automatic',
    jsxImportSource: 'preact',
    plugins: [{
      name: 'fixture',
      setup(build) {
        build.onResolve({
          filter: /(?:i18n\.js|recordingsAPI\.jsx|ToastContainer\.jsx)$/
        }, args => ({
          path: args.path,
          namespace: 'fixture'
        }));
        build.onLoad({
          filter: /.*/,
          namespace: 'fixture'
        }, args => ({
          contents: args.path.endsWith('i18n.js') ? 'export const useI18n=()=>({t:key=>key});' : args.path.endsWith('recordingsAPI.jsx') ? 'export const recordingsAPI={getAllRecordingTags:()=>window.api.getAllRecordingTags(),batchUpdateRecordingTags:(...args)=>window.api.batchUpdateRecordingTags(...args)};' : 'export const showStatusMessage=(...args)=>window.calls.messages.push(args);',
          loader: 'js'
        }));
      }
    }]
  });
  const browser = await chromium.launch({
    headless: true
  });
  const page = await browser.newPage();
  await page.setContent('<div id="root"></div>');
  await page.addScriptTag({
    content: compiled.outputFiles[0].text
  });
  await page.getByRole('checkbox', {
    name: 'partial',
    exact: true
  }).waitFor();
  await settle(page);
  return {
    browser,
    page
  };
}
async function settle(page) {
  await page.evaluate(() => new Promise(resolve => requestAnimationFrame(() => requestAnimationFrame(() => setTimeout(resolve, 60)))));
}
async function state(page, tag) {
  return page.getByRole('checkbox', {
    name: tag,
    exact: true
  }).evaluate(el => ({
    checked: el.checked,
    mixed: el.indeterminate,
    disabled: el.disabled
  }));
}
async function enter(page, tag) {
  const field = page.getByPlaceholder('recordings.addNewTagPlaceholder', {
    exact: true
  });
  await field.fill(tag);
  await field.press('Enter');
}
async function complete(page, success = true) {
  await page.evaluate(success => success ? window.succeed() : window.fail(), success);
  await settle(page);
}
browserTest('successful new tag survives stale rows and late autocomplete catalog, then reconciles same-count refresh', async () => {
  const {
    browser,
    page
  } = await openBrowser({
    delayedCatalog: true
  });
  try {
    await enter(page, ' new ');
    expect(await page.evaluate(() => window.calls.requests)).toEqual([[[1, 2], ['new'], []]]);
    await complete(page);
    expect(await state(page, 'new')).toMatchObject({
      checked: true,
      mixed: false,
      disabled: false
    });
    await page.evaluate(() => window.finishCatalog(['shared', 'partial', 'catalog-only']));
    await settle(page);
    expect(await state(page, 'new')).toMatchObject({
      checked: true,
      mixed: false
    });
    expect(await state(page, 'catalog-only')).toMatchObject({
      checked: false,
      mixed: false
    });
    await page.evaluate(() => window.refresh([{
      id: 1,
      tags: ['shared', 'new']
    }, {
      id: 2,
      tags: ['new']
    }, {
      id: 3,
      tags: ['other']
    }]));
    await settle(page);
    expect(await state(page, 'new')).toMatchObject({
      checked: true,
      mixed: false
    });
    expect(await state(page, 'shared')).toMatchObject({
      checked: false,
      mixed: true
    });
    expect(await state(page, 'partial')).toMatchObject({
      checked: false,
      mixed: false
    });
    await page.evaluate(() => window.refresh([{
      id: 1,
      tags: []
    }, {
      id: 2,
      tags: ['new']
    }, {
      id: 3,
      tags: ['other']
    }]));
    await settle(page);
    expect(await state(page, 'new')).toMatchObject({
      checked: false,
      mixed: true
    });
    expect(await state(page, 'shared')).toMatchObject({
      checked: false,
      mixed: false
    });
    expect(await page.evaluate(() => window.calls.changed)).toBe(1);
  } finally {
    await browser.close();
  }
}, 30000);
browserTest('same-size selection replacement recomputes actual tags and late old-selection completion cannot mark new rows', async () => {
  const {
    browser,
    page
  } = await openBrowser();
  try {
    await enter(page, 'old-selection');
    await page.evaluate(() => window.select({
      2: true,
      3: true
    }));
    await settle(page);
    expect(await state(page, 'partial')).toMatchObject({
      checked: false,
      mixed: false
    });
    expect(await state(page, 'shared')).toMatchObject({
      checked: false,
      mixed: true
    });
    expect(await state(page, 'other')).toMatchObject({
      checked: false,
      mixed: true
    });
    await complete(page);
    expect(await state(page, 'old-selection')).toMatchObject({
      checked: false,
      mixed: false
    });
    await page.getByRole('checkbox', {
      name: 'other',
      exact: true
    }).click();
    await complete(page);
    expect(await state(page, 'other')).toMatchObject({
      checked: true,
      mixed: false
    });
    expect(await page.evaluate(() => window.calls.requests)).toEqual([[[1, 2], ['old-selection'], []], [[2, 3], ['other'], []]]);
    await page.evaluate(() => window.select({
      3: true
    }));
    await settle(page);
    expect(await state(page, 'other')).toMatchObject({
      checked: true,
      mixed: false
    });
    expect(await state(page, 'shared')).toMatchObject({
      checked: false,
      mixed: false
    });
  } finally {
    await browser.close();
  }
}, 30000);
browserTest('failed mixed-add, all-remove and new-tag saves retain state, input and callback counts; pending controls cannot resubmit', async () => {
  const {
    browser,
    page
  } = await openBrowser();
  try {
    const partial = page.getByRole('checkbox', {
      name: 'partial',
      exact: true
    });
    await partial.focus();
    await page.keyboard.press('Space');
    expect(await partial.isDisabled()).toBe(true);
    await partial.evaluate(el => el.click());
    expect(await page.evaluate(() => window.calls.requests.length)).toBe(1);
    await complete(page, false);
    expect(await state(page, 'partial')).toMatchObject({
      checked: false,
      mixed: true,
      disabled: false
    });
    expect(await page.evaluate(() => window.calls.changed)).toBe(0);
    const shared = page.getByRole('checkbox', {
      name: 'shared',
      exact: true
    });
    await shared.click();
    await complete(page, false);
    expect(await state(page, 'shared')).toMatchObject({
      checked: true,
      mixed: false
    });
    await enter(page, 'failed-new');
    expect(await page.getByPlaceholder('recordings.addNewTagPlaceholder', {
      exact: true
    }).isDisabled()).toBe(true);
    await complete(page, false);
    expect(await page.getByRole('checkbox', {
      name: 'failed-new',
      exact: true
    }).count()).toBe(0);
    expect(await page.getByPlaceholder('recordings.addNewTagPlaceholder', {
      exact: true
    }).inputValue()).toBe('failed-new');
    expect(await page.evaluate(() => window.calls.changed)).toBe(0);
    expect(await page.evaluate(() => window.calls.messages)).toEqual([]);
    expect(await page.evaluate(() => window.calls.requests)).toEqual([[[1, 2], ['partial'], []], [[1, 2], [], ['shared']], [[1, 2], ['failed-new'], []]]);
    await shared.click();
    await complete(page);
    expect(await state(page, 'shared')).toMatchObject({
      checked: false,
      mixed: false
    });
    await partial.click();
    await complete(page);
    expect(await state(page, 'partial')).toMatchObject({
      checked: true,
      mixed: false
    });
    expect(await page.evaluate(() => window.calls.changed)).toBe(2);
  } finally {
    await browser.close();
  }
}, 30000);
