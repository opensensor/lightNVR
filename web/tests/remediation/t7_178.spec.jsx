/** Native label associations, rather than aria-label-only name assertions. */
const mockHooks = { component: '', cursor: 0, overrides: {}, nextId: 0 };
jest.mock('preact/hooks', () => ({
  useState: initial => { const index = mockHooks.cursor++; return [Object.hasOwn(mockHooks.overrides[mockHooks.component] || {}, index) ? mockHooks.overrides[mockHooks.component][index] : typeof initial === 'function' ? initial() : initial, jest.fn()]; },
  useEffect: jest.fn(), useMemo: factory => factory(), useCallback: fn => fn,
  useRef: value => ({ current: value }), useId: () => `fixture-${++mockHooks.nextId}`,
}));
jest.mock('../../js/query-client.js', () => ({ fetchJSON: jest.fn(), useQuery: (key, url) => ({ data: mockData[url] || {}, isLoading: false, isError: false, refetch: jest.fn() }) }));
jest.mock('../../js/i18n.js', () => ({ useI18n: () => ({ t: key => 'translated:' + key, locale: 'en-US' }) }));
jest.mock('../../js/components/preact/ToastContainer.jsx', () => ({ showStatusMessage: jest.fn() }));
jest.mock('../../js/components/preact/common/ModalDialog.jsx', () => ({ ConfirmDialog: () => null }));
const { h } = require('preact');
const { HealthTab, HEALTH_SETTINGS_DEFAULTS } = require('../../js/components/preact/settings/HealthTab.jsx');
const { StorageTab } = require('../../js/components/preact/settings/StorageTab.jsx');
const { ApiKeyModal } = require('../../js/components/preact/users/ApiKeyModal.jsx');
const { createTokenDraft } = require('../../js/components/preact/users/apiTokenPolicy.js');
const t = key => 'translated:' + key;
const action = { key: 'recordings:view', description: 'Read recordings', category: 'recordings', enforced: true, camera_scoped: true };
const targets = [{ uuid: 'disk', name: 'Disk', target_type: 'filesystem', root_path: '/fixture', enabled: true, storage_class: 'hot', low_watermark_pct: 80, high_watermark_pct: 90 }];
const mockData = { '/api/storage-targets': { targets }, '/api/storage-policies': { policies: [] }, '/api/storage-pools': { pools: [] } };
const condition = { code: 'storage.free', unit: 'ratio', warning: .2, critical: .1, recovery: .3, warning_for_seconds: 60, critical_for_seconds: 30, recovery_for_seconds: 90 };
const selector = { version: 1, expression: { op: 'health', values: ['up'] } };
const settings = { ...HEALTH_SETTINGS_DEFAULTS, healthEffectivePolicy: { conditions: [condition] }, healthConditionOverrides: { version: 1, conditions: [condition] }, storagePath: '/fixture', storagePathHls: '/hls', maxStorage: '100', retention: '30', autoDelete: false, minFreePct: '10', pressureWarningPct: '80', pressureCriticalPct: '90', pressureEmergencyPct: '95', generateThumbnails: true, thumbnailsPerRecording: '3', dbPath: '/fixture/db', dbBackupIntervalMinutes: '60', dbBackupRetentionCount: '3', dbBackupVerify: 'quick', dbPostBackupScript: '', mp4DirectoryFormat: 'flat' };
function expand(node) {
  if (Array.isArray(node)) return node.flatMap(expand);
  if (!node || typeof node !== 'object') return node;
  if (typeof node.type === 'function') {
    const previous = [mockHooks.component, mockHooks.cursor];
    mockHooks.component = node.type.name; mockHooks.cursor = 0;
    const result = node.type(node.props);
    [mockHooks.component, mockHooks.cursor] = previous;
    return expand(result);
  }
  return { ...node, props: { ...node.props, children: expand(node.props?.children) } };
}
function nodes(node) { return Array.isArray(node) ? node.flatMap(nodes) : !node || typeof node !== 'object' ? [] : [node, ...nodes(node.props?.children)]; }
function render(Component, props = {}) { return expand(h(Component, { settings, t, canModifySettings: true, handleInputChange: jest.fn(), setSettings: jest.fn(), currentUser: { id: 7, username: 'Fixture' }, getAuthHeaders: () => ({}), ...props })); }
const isControl = node => ['input', 'select', 'textarea', 'button'].includes(node.type);
function associations(roots) {
  const all = roots.flatMap(nodes); const controls = all.filter(isControl);
  for (const root of roots) {
    const local = nodes(root);
    for (const label of local.filter(node => node.type === 'label')) {
      const reference = label.props.for || label.props.htmlFor;
      const linked = reference ? controls.filter(node => node.props.id === reference) : nodes(label.props.children).filter(isControl);
      expect(linked).toHaveLength(1); expect(local).toContain(linked[0]);
    }
  }
}
beforeEach(() => { mockHooks.overrides = {}; mockHooks.nextId = 0; });
it('health implicit labels remain local across repeated conditions and removed custom branches', () => {
  associations([render(HealthTab), render(HealthTab)]);
  associations([render(HealthTab, { settings: { ...settings, healthConditionOverrides: { version: 1, conditions: [{ code: condition.code, profile: 'disabled' }] } } })]);
});
it('storage global explicit label IDs belong to exactly one repeated form', () => {
  const roots = [render(StorageTab, { canModifySettings: false }), render(StorageTab, { canModifySettings: false })];
  const all = roots.flatMap(nodes);
  for (const root of roots) for (const label of nodes(root).filter(node => node.type === 'label' && node.props.for)) {
    const matches = all.filter(node => isControl(node) && node.props.id === label.props.for);
    expect(matches).toHaveLength(1); expect(nodes(root)).toContain(matches[0]);
  }
});
it('storage pressure group does not misuse an orphan single-control label', () => {
  associations([render(StorageTab, { canModifySettings: false })]);
  const group = nodes(render(StorageTab, { canModifySettings: false })).find(node => node.props.role === 'group' && node.props['aria-label'] === t('settings.pressureThresholds'));
  expect(group).toBeDefined(); expect(nodes(group).filter(node => node.type === 'input')).toHaveLength(3);
});
it('API conditional selectors keep real child labels unique across repeated forms', () => {
  mockHooks.overrides.ApiKeyModal = { 0: false, 3: [action], 7: { ...createTokenDraft(), description: 'Fixture', actionKeys: [action.key], scopeType: 'selector', selector } };
  associations([render(ApiKeyModal), render(ApiKeyModal)]);
});
it('storage policy editors keep real child labels unique for repeated new policies', () => {
  mockHooks.overrides.StoragePoliciesPanel = { 0: { name: 'Fixture', enabled: true, primary_target_uuid: 'disk', fallback_mode: 'default', required_copy_count: 1, selector } };
  const roots = [render(StorageTab), render(StorageTab)];
  // This case isolates the real conditional selector from the global fields.
  const selectorRoots = roots.map(root => nodes(root).find(node => node.type === 'div' && nodes(node.props.children).some(child => child.type === 'h3' && child.props.children === t('collections.builder.title')) && nodes(node).some(child => child.type === 'label' && String(child.props.for || child.props.htmlFor).includes('rule-0-up'))));
  expect(selectorRoots.every(Boolean)).toBe(true);
  const labels = roots.flatMap(nodes).filter(node => node.type === 'label' && String(node.props.for || node.props.htmlFor).includes('rule-0-up'));
  expect(labels).toHaveLength(2); expect(labels[0].props.for || labels[0].props.htmlFor).not.toBe(labels[1].props.for || labels[1].props.htmlFor);
});

async function openBrowser() {
  const { chromium } = require('../../../node_modules/playwright'); const { build } = require('esbuild');
  const path = require('node:path'); const fs = require('node:fs'); const webRoot = path.resolve(__dirname, '../..');
  const compiled = await build({ stdin: { resolveDir: webRoot, loader: 'jsx', contents: `
    import { h, render } from 'preact'; import { useState } from 'preact/hooks';
    import { HealthTab } from './js/components/preact/settings/HealthTab.jsx';
    import { StorageTab } from './js/components/preact/settings/StorageTab.jsx';
    import { ApiKeyModal } from './js/components/preact/users/ApiKeyModal.jsx';
    const components={health:HealthTab,storage:StorageTab,api:ApiKeyModal}; const headers=()=>({});
    window.fixtureData=${JSON.stringify(mockData)}; window.emptyData={}; window.locale='translated';
    window.translate=key=>window.locale+':'+key;
    const action=${JSON.stringify(action)};
    window.fixtureJSON=async(url,options={})=>{
      if(options.method==='POST' && url.endsWith('/tokens')){window.calls.created.push({url,body:JSON.parse(options.body)});return {token:{uuid:'new',description:'Fixture',expires_at:2000000000},secret:'synthetic'};}
      if(url.endsWith('/actions'))return {actions:[action]};
      if(url.endsWith('/tokens'))return {tokens:[{uuid:'old',description:'Old',expires_at:1,actions:[action.key],scope:{type:'all'}}]};
      if(url==='/api/camera-collections')return {collections:[{uuid:'collection',name:'Collection',shared:true,effective_count:1}]};
      return window.fixtureData[url]||{locations:[],tags:[]};
    };
    function Host({kind,instance,readonly}){
      const [settings,setSettings]=useState(${JSON.stringify(settings)});window.current[instance]=settings;
      const change=event=>{const {name,type,checked,value}=event.currentTarget;const next=type==='checkbox'?checked:value;window.calls.changes.push({instance,name,value:next});setSettings(previous=>({...previous,[name]:next}));};
      return h('div',{'data-instance':instance},h(components[kind],{settings,setSettings,handleInputChange:change,canModifySettings:!readonly,t:window.translate,currentUser:{id:7+instance,username:'User'+instance},getAuthHeaders:headers,onClose:()=>window.calls.closed.push(instance)}));
    }
    window.mount=(kind,{readonly=false,locale='translated'}={})=>{render(null,document.getElementById('root'));window.locale=locale;window.calls={changes:[],created:[],closed:[]};window.current={};render(h('main',null,[0,1].map(instance=>h(Host,{key:instance,kind,instance,readonly}))),document.getElementById('root'));};
  ` }, bundle: true, write: false, format: 'iife', jsx: 'automatic', jsxImportSource: 'preact', plugins: [{ name: 'services', setup(build) {
    build.onResolve({ filter: /(?:query-client\.js|i18n\.js|ToastContainer\.jsx)$/ }, args => ({ path: args.path, namespace: 'fixture' }));
    build.onLoad({ filter: /.*/, namespace: 'fixture' }, args => ({ contents: args.path.endsWith('query-client.js') ? 'export const fetchJSON=(...args)=>window.fixtureJSON(...args);export const useQuery=(key,url)=>({data:window.fixtureData[url]||window.emptyData,isLoading:false,isError:false,refetch:async()=>{}});' : args.path.endsWith('i18n.js') ? 'export const useI18n=()=>({t:window.translate,locale:"en-US"});' : 'export const showStatusMessage=()=>{};', loader: 'js' }));
  } }] });
  const browser = await chromium.launch({ headless: true }); const page = await browser.newPage(); page.setDefaultTimeout(5000);
  const errors = []; page.on('pageerror', error => errors.push(error.message));
  await page.route('http://fixture.invalid/**', route => route.fulfill({ contentType: 'text/html', body: '<html><body><div id="root"></div></body></html>' }));
  await page.goto('http://fixture.invalid/settings.html');
  for(const css of fs.readdirSync(path.join(webRoot,'dist/assets')).filter(file=>/^(settings|users)-.*\.css$/.test(file)))await page.addStyleTag({content:fs.readFileSync(path.join(webRoot,'dist/assets',css),'utf8')});
  // Display repeated modal instances side by side in document flow so native
  // pointer activation is observable; control/label markup remains unchanged.
  await page.addStyleTag({content:'[data-instance] > .fixed { position: static !important; }'});
  await page.addScriptTag({ content: compiled.outputFiles[0].text });
  return { browser, page, errors };
}
async function nativeAssociations(page) {
  const problems = await page.evaluate(() => [...document.querySelectorAll('[data-instance]')].flatMap(root => [...root.querySelectorAll('label')].flatMap(label => {
    if (!label.control) return ['orphan:' + label.textContent.trim()];
    if (!root.contains(label.control)) return ['cross-instance:' + label.textContent.trim()];
    if (![...label.control.labels].includes(label)) return ['reverse-association'];
    if (label.htmlFor && [...document.querySelectorAll('input,select,textarea,button')].filter(control => control.id === label.htmlFor).length !== 1) return ['duplicate-id:' + label.htmlFor];
    return [];
  })));
  expect(problems).toEqual([]);
}
const selectByLabel = (root, key) => root.locator('label').filter({ hasText: t(key) }).locator('select');
const browserTest = process.env.T7_178_BROWSER === '1' ? it : it.skip;
browserTest('storage label activation stays in its form and group semantics survive read-only rerender', async () => {
  const {browser,page,errors}=await openBrowser();try{
    await page.evaluate(()=>window.mount('storage'));await nativeAssociations(page);
    const second=page.locator('[data-instance="1"]');
    await second.locator('label').filter({hasText:t('settings.autoDeleteOldest')}).click();
    expect(await page.evaluate(()=>window.calls.changes)).toEqual([{instance:1,name:'autoDelete',value:true}]);
    expect(await page.locator('[data-instance="0"] input[name="autoDelete"]').isChecked()).toBe(false);
    await second.locator('label').filter({hasText:t('settings.storagePath')}).click();
    expect(await second.locator('input[name="storagePath"]').evaluate(control=>document.activeElement===control)).toBe(true);
    const before=await second.locator('input[name="storagePath"]').getAttribute('id');
    await second.locator('input[name="storagePath"]').fill('/updated');await second.locator('input[name="storagePath"]').press('Tab');
    expect(await second.locator('input[name="storagePath"]').getAttribute('id')).toBe(before);
    for(const root of [page.locator('[data-instance="0"]'),second])expect(await root.getByRole('group',{name:t('settings.pressureThresholds'),exact:true}).locator('input').count()).toBe(3);
    await page.evaluate(()=>window.mount('storage',{readonly:true,locale:'localized'}));await nativeAssociations(page);
    expect(await page.locator('input,select').evaluateAll(fields=>fields.every(field=>field.disabled))).toBe(true);
    expect(await page.locator('[data-instance="1"]').getByLabel('localized:settings.storagePath',{exact:true}).count()).toBe(1);expect(errors).toEqual([]);
  }finally{await browser.close();}
},30000);
browserTest('health labels survive custom threshold removal and remount without cross-form activation',async()=>{
  const {browser,page,errors}=await openBrowser();try{
    await page.evaluate(()=>window.mount('health'));await nativeAssociations(page);
    const second=page.locator('[data-instance="1"]');const detail=second.locator('details');await detail.locator('summary').click();
    await selectByLabel(detail,'settings.health.overrideMode').selectOption('disabled');expect(await detail.locator('input').count()).toBe(0);await nativeAssociations(page);
    await selectByLabel(detail,'settings.health.overrideMode').selectOption('custom');expect(await detail.locator('input').count()).toBe(6);await nativeAssociations(page);
    await second.locator('label').filter({has:page.locator('input[name="healthEnabled"]')}).click();
    expect(await page.evaluate(()=>window.calls.changes)).toEqual([{instance:1,name:'healthEnabled',value:false}]);
    expect(await page.locator('[data-instance="0"] input[name="healthEnabled"]').isChecked()).toBe(true);expect(errors).toEqual([]);
  }finally{await browser.close();}
},30000);
browserTest('API real conditional selector labels are unique and remain local after scope changes',async()=>{
  const {browser,page,errors}=await openBrowser();try{
    await page.evaluate(()=>window.mount('api'));await page.waitForFunction(()=>document.querySelectorAll('input[maxlength="127"]').length===2);
    for(const index of [0,1]){const root=page.locator('[data-instance="'+index+'"]');await selectByLabel(root,'tokens.scope.label').selectOption('selector');await root.getByRole('button',{name:t('collections.builder.addRule'),exact:true}).click();}
    await nativeAssociations(page);
    const second=page.locator('[data-instance="1"]');const label=second.locator('label').filter({has:page.getByRole('checkbox',{name:t('fleet.health.up'),exact:true})});
    await label.click();expect(await second.getByRole('checkbox',{name:t('fleet.health.up'),exact:true}).isChecked()).toBe(true);expect(await page.locator('[data-instance="0"]').getByRole('checkbox',{name:t('fleet.health.up'),exact:true}).isChecked()).toBe(false);
    await selectByLabel(second,'tokens.scope.label').selectOption('collection');await selectByLabel(second,'tokens.scope.chooseCollection').selectOption('collection');await nativeAssociations(page);
    await selectByLabel(second,'tokens.scope.label').selectOption('all');expect(await selectByLabel(second,'tokens.scope.chooseCollection').count()).toBe(0);
    await second.getByRole('textbox',{name:t('tokens.name'),exact:true}).fill('Fixture');await second.getByRole('checkbox',{name:'recordings:view Read recordings',exact:true}).check();await second.getByRole('button',{name:t('tokens.create'),exact:true}).click();
    await second.getByRole('checkbox',{name:t('tokens.secret.savedConfirm')+' '+t('tokens.secret.savedHelp'),exact:true}).waitFor();await nativeAssociations(page);
    await second.locator('label').filter({has:page.getByRole('checkbox',{name:t('tokens.secret.savedConfirm')+' '+t('tokens.secret.savedHelp'),exact:true})}).click();expect(await second.getByRole('button',{name:t('common.done'),exact:true}).isEnabled()).toBe(true);
    expect(await page.evaluate(()=>window.calls.created.map(row=>row.url))).toEqual(['/api/authorization/users/8/tokens']);expect(errors).toEqual([]);
  }finally{await browser.close();}
},30000);
browserTest('repeated storage policy editors namespace the real selector labels through conditional rules',async()=>{
  const {browser,page,errors}=await openBrowser();try{
    await page.evaluate(()=>window.mount('storage'));
    for(const index of [0,1]){const root=page.locator('[data-instance="'+index+'"]');const panel=root.locator('[data-setting-label="'+t('settings.storagePolicies.title')+'"]');await panel.getByRole('button',{name:t('settings.storagePolicies.add'),exact:true}).click();await panel.getByRole('button',{name:t('collections.builder.addRule'),exact:true}).click();}
    await nativeAssociations(page);
    const first=page.locator('[data-instance="0"]');const second=page.locator('[data-instance="1"]');
    await second.locator('label').filter({has:page.getByRole('checkbox',{name:t('fleet.health.up'),exact:true})}).click();
    expect(await second.getByRole('checkbox',{name:t('fleet.health.up'),exact:true}).isChecked()).toBe(true);expect(await first.getByRole('checkbox',{name:t('fleet.health.up'),exact:true}).isChecked()).toBe(false);
    await second.getByRole('button',{name:t('collections.builder.removeRule'),exact:true}).click();await nativeAssociations(page);expect(await second.getByRole('checkbox',{name:t('fleet.health.up'),exact:true}).count()).toBe(0);expect(errors).toEqual([]);
  }finally{await browser.close();}
},30000);
