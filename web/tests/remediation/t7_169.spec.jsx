/** Explicit accessible names must match the existing labels while preserving form contracts. */
const mockHooks = { cursor: 0, values: {}, setters: [] };
jest.mock('preact/hooks', () => ({
  useState: initial => { const index = mockHooks.cursor++; const setter = jest.fn(); mockHooks.setters[index] = setter;
    return [Object.hasOwn(mockHooks.values, index) ? mockHooks.values[index] : typeof initial === 'function' ? initial() : initial, setter]; },
  useEffect: jest.fn(), useMemo: factory => factory(), useCallback: fn => fn, useRef: value => ({ current: value }), useId: () => 'fixture',
}));
jest.mock('../../js/query-client.js', () => ({ fetchJSON: jest.fn() }));
jest.mock('../../js/i18n.js', () => ({ useI18n: () => ({ t: key => 'translated:' + key }) }));
jest.mock('../../js/components/preact/ToastContainer.jsx', () => ({ showStatusMessage: jest.fn() }));
const { EventDestinationEditor } = require('../../js/components/preact/events/EventDestinationEditor.jsx');
const { EventRouteEditor } = require('../../js/components/preact/events/EventRouteEditor.jsx');
const { BulkOrganizeModal } = require('../../js/components/preact/fleet/BulkOrganizeModal.jsx');
const { CollectionManager } = require('../../js/components/preact/fleet/CollectionManager.jsx');
const { CollectionSelectorBuilder, CameraValuePicker } = require('../../js/components/preact/fleet/CollectionSelectorBuilder.jsx');
const { FleetFilters } = require('../../js/components/preact/fleet/FleetFilters.jsx');
const { EMPTY_DESTINATION_DRAFT, EMPTY_ROUTE_DRAFT, DEFAULT_SCHEDULE_WINDOW } = require('../../js/components/preact/events/eventRouting.js');
const t = key => 'translated:' + key;
const tags = [{ uuid: 'tag-a', label: 'Entrance' }];
const catalog = [
  { type: 'io.lightnvr.detection.object.v1', description: 'Detected object', subject_kind: 'camera' },
  { type: 'io.lightnvr.health.condition.v1', description: 'Health condition', subject_kind: 'system', family: 'system_health', minimum_severity: 'warning', maximum_severity: 'critical' },
];
function nodes(node) { if (Array.isArray(node)) return node.flatMap(nodes); if (!node || typeof node !== 'object') return []; return [node, ...nodes(node.props?.children)]; }
function render(Component, props = {}, values = {}) { mockHooks.cursor = 0; mockHooks.values = values; mockHooks.setters = []; return Component({ t, ...props }); }
function expand(tree, name) { const node = nodes(tree).find(node => typeof node.type === 'function' && node.type.name === name); expect(node).toBeDefined(); return node.type(node.props); }
function fields(tree) { return nodes(tree).filter(node => ['input', 'textarea'].includes(node.type)); }
function named(node, key, localized = true) { expect(node).toBeDefined(); expect(node.props['aria-label']).toBe(localized ? t(key) : key); }
function change(node, value = 'changed') { (node.props.onInput || node.props.oninput || node.props.onChange)({ currentTarget: { value, checked: true } }); }
beforeEach(() => jest.clearAllMocks());
it('all 15 destination controls keep their precise names and draft updates, including credentials and mutual TLS', () => {
  const draft = { ...EMPTY_DESTINATION_DRAFT, tlsMode: 'mutual', passwordConfigured: true };
  const tree = render(EventDestinationEditor, { destination: {} }, { 0: draft });
  const expected = ['common.name', 'events.destination.enabled', 'common.description', 'events.destination.host', 'events.destination.port', 'events.destination.clientId', 'events.destination.topicTemplate', 'events.destination.keepalive', 'events.destination.statusTopicTemplate', 'auth.username', 'auth.password', 'events.destination.clearPassword', 'events.destination.caFile', 'events.destination.certFile', 'events.destination.keyFile'];
  expect(fields(tree)).toHaveLength(expected.length);
  fields(tree).forEach((field, index) => { named(field, expected[index]); change(field); });
  expect(mockHooks.setters[0]).toHaveBeenCalledTimes(15);
  const patches = mockHooks.setters[0].mock.calls.map(([update]) => update(draft));
  expect(patches[0].name).toBe('changed'); expect(patches[3].host).toBe('changed');
  expect(patches[11]).toMatchObject({ clearPassword: true, password: '' }); expect(patches[14].keyFile).toBe('changed');
  const cleared = render(EventDestinationEditor, { destination: {} }, { 0: { ...draft, clearPassword: true } });
  expect(fields(cleared).find(field => field.props.type === 'password').props.disabled).toBe(true);
});
it('route identity, detection, health, schedule and suppression inputs remain named with exact update callbacks', () => {
  const draft = { ...EMPTY_ROUTE_DRAFT, eventTypes: catalog.map(event => event.type), detectionFilterEnabled: true, healthFilterEnabled: true, healthConditionCodes: [], healthSeverities: [], scheduleEnabled: true, windows: [DEFAULT_SCHEDULE_WINDOW] };
  const tree = render(EventRouteEditor, { catalog, healthConditions: [{ code: 'storage.full' }], destinations: [], locations: [], tags }, { 0: draft });
  const expected = ['common.name', 'common.description', 'events.route.enabled', 'detection.object', 'health.condition', 'events.route.detectionFilters', 'events.route.labels', 'events.route.zones', 'events.route.minConfidence', 'events.route.healthFilters', 'storage full', 'warning', 'error', 'critical', 'events.route.restrictSchedule', 'events.schedule.timezone', 'events.route.debounce', 'events.route.cooldown', 'events.route.grouping', 'events.route.rateLimit'];
  expect(fields(tree)).toHaveLength(expected.length);
  fields(tree).forEach((field, index) => { named(field, expected[index], !['detection.object', 'health.condition', 'storage full', 'warning', 'error', 'critical'].includes(expected[index])); change(field); });
  expect(mockHooks.setters[0]).toHaveBeenCalledTimes(expected.length);
  const schedule = expand(tree, 'ScheduleWindow'); const entries = fields(schedule);
  const names = ['sunday', 'monday', 'tuesday', 'wednesday', 'thursday', 'friday', 'saturday', 'start', 'end'];
  expect(entries).toHaveLength(9); entries.forEach((field, index) => { named(field, 'events.schedule.' + names[index]); change(field, '09:00'); });
  const last = mockHooks.setters[0].mock.calls.at(-1)[0](draft); expect(last.windows[0].end).toBe('09:00');
});
it('bulk operation radio names and tag names preserve disabled state and unique selection updates', () => {
  const tree = render(BulkOrganizeModal, { cameras: [{ camera_uuid: 'cam-a' }], locations: [], tags }, { 1: 'add', 4: true });
  const entries = fields(tree); expect(entries).toHaveLength(5);
  entries.slice(0, 4).forEach((field, index) => { named(field, 'fleet.bulk.tagOperation.' + ['none', 'add', 'remove', 'replace'][index]); expect(field.props.disabled).toBe(true); });
  named(entries[4], 'Entrance', false); expect(entries[4].props.disabled).toBe(true);
  change(entries[1]); expect(mockHooks.setters[1]).toHaveBeenCalledWith('add'); change(entries[4]);
  expect(mockHooks.setters[2].mock.calls[0][0]([])).toEqual(['tag-a']);
});
it('collection editor fields keep meaningful names and required create guard', async () => {
  const tree = render(CollectionManager, { collections: [], locations: [], tags }, { 0: { collection: null, members: ['cam-a'] } });
  mockHooks.values = {}; const editor = expand(tree, 'CollectionEditor');
  const entries = fields(editor); expect(entries).toHaveLength(3);
  ['common.name', 'collections.description', 'collections.shared'].forEach((key, index) => { named(entries[index], key); change(entries[index]); });
  expect(entries[0].props.required).toBe(true); expect(nodes(editor).find(node => node.type === 'button' && node.props.type === 'submit').props.disabled).toBe(true);
});
it.each(['health', 'recording_mode', 'tag_any', 'capability_any', 'vendor', 'model'])('selector %s branch names values and preserves rule updates', type => {
  const rule = { type, values: [] };
  const tree = render(CollectionSelectorBuilder, { locations: [], tags, onChange: jest.fn() }, { 1: [rule] });
  const editor = expand(tree, 'RuleValueEditor');
  if (['vendor', 'model'].includes(type)) {
    named(editor, 'collections.rule.' + type); change(editor, 'Alpha, Beta');
    expect(mockHooks.setters[1].mock.calls[0][0]([rule])[0].values).toEqual(['Alpha', 'Beta']);
  } else {
    const checks = fields(expand(editor, 'ValueChecks')); expect(checks.length).toBeGreaterThan(0);
    for (const check of checks) { expect(check.props['aria-label']).toBeTruthy(); expect(check.props['aria-label']).not.toContain('undefined'); }
    named(checks[0], type === 'tag_any' ? 'Entrance' : type === 'health' ? 'fleet.health.unknown' : type === 'recording_mode' ? 'fleet.recording.off' : 'collections.capability.onvif', type !== 'tag_any');
    change(checks[0]); expect(mockHooks.setters[1]).toHaveBeenCalledTimes(1);
  }
});
it('camera search and advanced JSON inputs retain names, updates and invalid-preview guard', () => {
  const picker = render(CameraValuePicker, { values: [], onChange: jest.fn(), idPrefix: 'picker' });
  named(fields(picker)[0], 'access.cameraSearch'); change(fields(picker)[0], 'Entrance'); expect(mockHooks.setters[0]).toHaveBeenCalledWith('Entrance');
  const tree = render(CollectionSelectorBuilder, { locations: [], tags, onChange: jest.fn() }, { 2: true, 3: '{broken' });
  named(fields(tree)[0], 'collections.builder.advancedMode'); change(fields(tree)[0], '{}'); expect(mockHooks.setters[3]).toHaveBeenCalledWith('{}');
  expect(nodes(tree).find(node => node.type === 'button' && node.props.children === t('collections.preview.action')).props.disabled).toBe(true);
});
it('fleet filter names exclude changing counts and preserve each toggle callback', () => {
  const onChange = jest.fn(); const state = { health: [], recordingModes: [], tagUuids: [], availability: 'all', enabled: 'all', collectionUuid: '', locationUuid: '' };
  const tree = render(FleetFilters, { state, facets: { tags }, locations: [], collections: [], onChange });
  const filters = nodes(tree).filter(node => typeof node.type === 'function' && node.type.name === 'FilterCheckbox');
  expect(filters.length).toBeGreaterThan(5);
  for (const filter of filters) { const input = fields(filter.type(filter.props))[0]; named(input, filter.props.label, false); change(input); }
  expect(onChange).toHaveBeenCalledTimes(filters.length);
});
async function openBrowser() {
  const { chromium } = require('../../../node_modules/playwright'); const { build } = require('esbuild');
  const path = require('node:path'); const fs = require('node:fs'); const webRoot = path.resolve(__dirname, '../..');
  const result = await build({ stdin: { resolveDir: webRoot, loader: 'jsx', contents: `
    import { h, render } from 'preact'; import { useState } from 'preact/hooks';
    import { EventDestinationEditor } from './js/components/preact/events/EventDestinationEditor.jsx';
    import { EventRouteEditor } from './js/components/preact/events/EventRouteEditor.jsx';
    import { BulkOrganizeModal } from './js/components/preact/fleet/BulkOrganizeModal.jsx';
    import { CollectionManager } from './js/components/preact/fleet/CollectionManager.jsx';
    import { CollectionSelectorBuilder, CameraValuePicker } from './js/components/preact/fleet/CollectionSelectorBuilder.jsx';
    import { FleetFilters } from './js/components/preact/fleet/FleetFilters.jsx';
    const root = document.getElementById('root'); const t = key => 'translated:' + key;
    // Observe real effect installation; do not race Preact's post-paint Escape listener updates.
    window.keydownEffects=0;const addListener=window.addEventListener.bind(window);
    window.addEventListener=(type,listener,options)=>{if(type==='keydown')window.keydownEffects++;return addListener(type,listener,options);};
    const tags = ${JSON.stringify(tags)}; const catalog = ${JSON.stringify(catalog)};
    const all = {version:1,expression:{op:'all'}};
    const destination = {uuid:'destination-a', name:'Broker', broker:{host:'mqtt.example.invalid',client_id:'fixture-client'}, authentication:{password_configured:true},tls:{mode:'mutual',ca_file:'/certs/ca.pem',cert_file:'/certs/client.pem',key_file:'/certs/key.pem'}};
    const route = {uuid:'route-a',name:'Alarm',event_types:catalog.map(e=>e.type),predicate:{detection:{labels_any:['person']},health:{condition_codes_any:['storage.full'],severities_any:['warning']}},schedule:{timezone:'UTC',windows:[{days:[1],start:'08:00',end:'18:00'}]}};
    window.fixtureJSON = async (url, options={}) => {
      window.calls.requests.push({url,method:options.method||'GET',body:options.body ? JSON.parse(options.body) : null});
      if (options.method === 'PUT' && url.endsWith('/tags')) return new Promise(resolve=>window.finish=resolve);
      if (url.endsWith('/tags')) return {tags:[]};
      if (url === '/api/camera-collections') return new Promise(resolve=>window.finish=()=>resolve({uuid:'new-collection'}));
      if (url.endsWith('/members')) return {};
      if (url === '/api/fleet/cameras/query') return {cameras:[{camera_uuid:'cam-a',name:'Entrance camera'}],total:1};
      if (url === '/api/fleet/selectors/preview') return {matched_count:1,sample:[]};
      return {cameras:[],collections:[],tags:[],locations:[]};
    };
    function FilterHost() { const [state,setState]=useState({health:[],recordingModes:[],tagUuids:[],availability:'all',enabled:'all',collectionUuid:'',locationUuid:''});
      return h(FleetFilters,{state,facets:{tags:tags.map(tag=>({...tag,count:17}))},locations:[],collections:[],t,onChange:change=>{window.calls.changes.push(change);setState({...state,...change});}}); }
    function CameraHost() { const [values,setValues]=useState([]);return h(CameraValuePicker,{values,onChange:values=>{setValues(values);window.calls.changes.push(values);},idPrefix:'camera',t}); }
    window.mount = kind => {
      render(null,root); window.calls={requests:[],changes:[],close:0,save:[],preview:[],refresh:0};
      const close=()=>window.calls.close++; const save=payload=>{window.calls.save.push(payload);return new Promise(resolve=>window.finish=resolve);};
      const common={locations:[],tags,t,onClose:close};
      if(kind==='destination') return render(h(EventDestinationEditor,{destination,onSave:save,...common}),root);
      if(kind==='route') return render(h(EventRouteEditor,{route,catalog,healthConditions:[{code:'storage.full'}],destinations:[],onSave:save,onPreview:async payload=>{window.calls.preview.push(payload);return {};},...common}),root);
      if(kind==='bulk') return render(h(BulkOrganizeModal,{cameras:[{camera_uuid:'cam-a',name:'Entrance camera'}],onComplete:()=>window.calls.refresh++,...common}),root);
      if(kind==='manager') return render(h(CollectionManager,{collections:[],isAdmin:true,onRefresh:()=>window.calls.refresh++,...common}),root);
      if(kind==='filters') return render(h(FilterHost),root);
      if(kind==='camera') return render(h(CameraHost),root);
      return render(h(CollectionSelectorBuilder,{initialSelector:all,onChange:(value,error)=>window.calls.changes.push({value,error}),allowCameraSelection:true,...common}),root);
    };
  ` }, bundle: true, write: false, format: 'iife', jsx: 'automatic', jsxImportSource: 'preact', plugins: [{name:'services',setup(builder){
    builder.onResolve({filter:/(?:i18n|query-client|ToastContainer)\.(?:js|jsx)$/},args=>({path:args.path,namespace:'service'}));
    builder.onLoad({filter:/.*/,namespace:'service'},()=>({loader:'js',contents:`export const useI18n=()=>({t:key=>'translated:'+key});export const fetchJSON=(url,options)=>window.fixtureJSON(url,options);export const showStatusMessage=()=>{};export const useQuery=()=>({data:{cameras:[],total:0},isLoading:false});export const useMutation=()=>({});export const useQueryClient=()=>({});`}));
  }}] });
  const browser = await chromium.launch({headless:true});
  try {
    const page = await browser.newPage({viewport:{width:1440,height:1100}}); page.setDefaultTimeout(5000); const errors=[]; page.on('pageerror',error=>errors.push(error.message));
    await page.route('http://fixture.invalid/**',route=>route.fulfill({contentType:'text/html',body:'<div id="root"></div>'})); await page.goto('http://fixture.invalid/');
    const styles = new Set(); for(const file of ['settings.html','streams.html']) {const html=fs.readFileSync(path.join(webRoot,'dist',file),'utf8');for(const tag of html.match(/<link[^>]*rel="stylesheet"[^>]*>/g)||[]) styles.add(tag.match(/href="([^"]+)"/)[1]);}
    expect(styles.size).toBeGreaterThan(0);for(const href of styles) await page.addStyleTag({content:fs.readFileSync(path.join(webRoot,'dist',href),'utf8')});
    await page.addScriptTag({content:result.outputFiles[0].text});return {browser,page,errors};
  }catch(error){await browser.close();throw error;}
}
const browserTest = process.env.T7_169_BROWSER === '1' ? it : it.skip;
async function expectName(page,key,role='textbox') { const field=page.getByRole(role,{name:t(key),exact:true}); await field.waitFor(); expect(await field.count()).toBe(1); return field; }
browserTest('destination names survive title removal and preserve password clearing, payload and pending Escape guard',async()=>{
  const {browser,page,errors}=await openBrowser();try{
    await page.evaluate(()=>window.mount('destination'));await page.locator('[title]').evaluateAll(nodes=>nodes.forEach(node=>node.removeAttribute('title')));
    for(const key of ['common.name','common.description','events.destination.host','events.destination.clientId','events.destination.topicTemplate','events.destination.statusTopicTemplate','auth.username','events.destination.caFile','events.destination.certFile','events.destination.keyFile']) await expectName(page,key);
    for(const key of ['events.destination.port','events.destination.keepalive']) await expectName(page,key,'spinbutton');
    const password=page.getByLabel(t('auth.password'),{exact:true});expect(await password.count()).toBe(1);
    await (await expectName(page,'common.name')).fill('Updated broker');await (await expectName(page,'events.destination.host')).fill('changed.example.invalid');
    const enabled=await expectName(page,'events.destination.enabled','checkbox');await enabled.press('Space');expect(await enabled.isChecked()).toBe(false);
    const clear=await expectName(page,'events.destination.clearPassword','checkbox');await clear.press('Space');expect(await password.isDisabled()).toBe(true);
    await page.waitForFunction(()=>window.keydownEffects>0);const beforeSaving=await page.evaluate(()=>window.keydownEffects);
    await page.getByRole('button',{name:t('common.saveChanges'),exact:true}).press('Enter');await page.waitForFunction(()=>window.calls.save.length===1);await page.waitForFunction(count=>window.keydownEffects>count,beforeSaving);
    expect(await page.evaluate(()=>window.calls.save[0])).toMatchObject({name:'Updated broker',enabled:false,broker:{host:'changed.example.invalid'},authentication:{password:null},tls:{mode:'mutual',key_file:'/certs/key.pem'}});
    await page.getByRole('button',{name:t('common.saving'),exact:true}).waitFor();await page.keyboard.press('Escape');expect(await page.evaluate(()=>window.calls.close)).toBe(0);
    expect(await page.getByRole('button',{name:t('common.cancel'),exact:true}).isDisabled()).toBe(true);const beforeFinished=await page.evaluate(()=>window.keydownEffects);await page.evaluate(()=>window.finish());await page.waitForFunction(count=>window.keydownEffects>count,beforeFinished);await page.getByRole('button',{name:t('common.saveChanges'),exact:true}).waitFor();
    await page.keyboard.press('Escape');await page.waitForFunction(()=>window.calls.close===1);
  }finally{await browser.close();expect(errors).toEqual([]);}
});
browserTest('route named health/detection and schedule controls preserve selections, times, suppression and preview payload',async()=>{
  const {browser,page,errors}=await openBrowser();try{
    await page.evaluate(()=>window.mount('route'));
    for(const key of ['common.name','common.description','events.route.labels','events.route.zones','events.schedule.timezone']) await expectName(page,key);
    for(const key of ['events.route.minConfidence','events.route.debounce','events.route.cooldown','events.route.grouping','events.route.rateLimit']) await expectName(page,key,'spinbutton');
    for(const key of ['events.route.enabled','events.route.detectionFilters','events.route.healthFilters','events.route.restrictSchedule']) await expectName(page,key,'checkbox');
    for(const name of ['detection.object','health.condition','storage full','warning','error','critical']) expect(await page.getByRole('checkbox',{name,exact:true}).count()).toBe(1);
    for(const day of ['sunday','monday','tuesday','wednesday','thursday','friday','saturday']) await expectName(page,'events.schedule.'+day,'checkbox');
    await (await expectName(page,'events.route.labels')).fill('person, vehicle');await page.getByRole('checkbox',{name:'critical',exact:true}).press('Space');
    await (await expectName(page,'events.schedule.tuesday','checkbox')).press('Space');
    await page.getByLabel(t('events.schedule.start'),{exact:true}).fill('09:00');expect(await page.getByLabel(t('events.schedule.end'),{exact:true}).inputValue()).toBe('18:00');
    await (await expectName(page,'events.route.cooldown','spinbutton')).fill('12');
    const preview=page.getByRole('button',{name:t('events.route.preview'),exact:true});expect(await preview.isDisabled()).toBe(true);
    await (await expectName(page,'events.route.healthFilters','checkbox')).press('Space');await preview.press('Enter');
    await page.waitForFunction(()=>window.calls.preview.length===1);expect(await page.evaluate(()=>window.calls.preview[0])).toMatchObject({predicate:{detection:{labels_any:['person','vehicle']}},schedule:{windows:[{days:[1,2],start:'09:00',end:'18:00'}]},suppression:{cooldown_seconds:12}});
    await page.getByRole('checkbox',{name:'detection.object',exact:true}).press('Space');
    await (await expectName(page,'events.route.healthFilters','checkbox')).press('Space');await preview.press('Enter');await page.waitForFunction(()=>window.calls.preview.length===2);
    expect(await page.evaluate(()=>window.calls.preview[1])).toMatchObject({event_types:['io.lightnvr.health.condition.v1'],predicate:{health:{severities_any:['warning','critical'],condition_codes_any:['storage.full']}}});
    expect(await page.evaluate(()=>window.calls.close)).toBe(0);
  }finally{await browser.close();expect(errors).toEqual([]);}
});
browserTest('bulk radio and tag keyboard activation preserves one assignment and pending dismissal guards',async()=>{
  const {browser,page,errors}=await openBrowser();try{
    await page.evaluate(()=>window.mount('bulk'));
    expect(await page.getByRole('button',{name:t('fleet.bulk.apply'),exact:true}).isDisabled()).toBe(true);
    for(const operation of ['none','add','remove','replace']) await expectName(page,'fleet.bulk.tagOperation.'+operation,'radio');
    await (await expectName(page,'fleet.bulk.tagOperation.add','radio')).press('Space');const tag=page.getByRole('checkbox',{name:'Entrance',exact:true});await tag.press('Space');expect(await tag.isChecked()).toBe(true);
    await page.waitForFunction(()=>window.keydownEffects>0);const beforeWorking=await page.evaluate(()=>window.keydownEffects);
    await page.getByRole('button',{name:t('fleet.bulk.apply'),exact:true}).press('Enter');await page.waitForFunction(()=>typeof window.finish==='function');await page.waitForFunction(count=>window.keydownEffects>count,beforeWorking);
    expect(await page.locator('input:enabled').count()).toBe(0);await page.keyboard.press('Escape');await page.locator('#root > div').click({position:{x:1,y:1}});expect(await page.evaluate(()=>window.calls.close)).toBe(0);
    expect(await page.evaluate(()=>window.calls.requests.filter(r=>r.method==='PUT'))).toEqual([{url:'/api/cameras/cam-a/tags',method:'PUT',body:{tag_uuids:['tag-a']}}]);
    await page.evaluate(()=>window.finish({}));await page.waitForFunction(()=>window.calls.close===1);expect(await page.evaluate(()=>window.calls.refresh)).toBe(1);
  }finally{await browser.close();expect(errors).toEqual([]);}
});
browserTest('collection editor names preserve required and pending controls and static membership write',async()=>{
  const {browser,page,errors}=await openBrowser();try{
    await page.evaluate(()=>window.mount('manager'));await page.getByRole('button',{name:t('collections.create'),exact:true}).press('Enter');
    const name=await expectName(page,'common.name');await name.fill('  Lobby  ');await (await expectName(page,'collections.description')).fill('  Cameras  ');
    const shared=await expectName(page,'collections.shared','checkbox');await shared.press('Space');expect(await shared.isChecked()).toBe(false);
    await page.getByRole('button',{name:t('common.saveChanges'),exact:true}).press('Enter');await page.waitForFunction(()=>window.calls.requests.some(r=>r.method==='POST'));
    expect(await page.getByRole('button',{name:t('common.cancel'),exact:true}).isDisabled()).toBe(true);expect(await page.getByRole('button',{name:t('collections.backToList'),exact:true}).isDisabled()).toBe(true);
    expect(await page.evaluate(()=>window.calls.requests.find(r=>r.method==='POST').body)).toEqual({name:'Lobby',description:'Cameras',type:'static',shared:false});
    await page.evaluate(()=>window.finish());await page.waitForFunction(()=>window.calls.refresh===1);expect(await page.evaluate(()=>window.calls.requests.find(r=>r.url.endsWith('/members')).body)).toEqual({camera_uuids:[]});
  }finally{await browser.close();expect(errors).toEqual([]);}
});
browserTest('selector vendor/model names and advanced JSON preserve rule changes and invalid preview prevention',async()=>{
  const {browser,page,errors}=await openBrowser();try{
    await page.evaluate(()=>window.mount('selector'));await page.getByRole('button',{name:t('collections.builder.addRule'),exact:true}).press('Enter');
    const ruleType=page.locator('select').last();await ruleType.selectOption('vendor');await (await expectName(page,'collections.rule.vendor')).fill('Acme, Example');
    await ruleType.selectOption('model');await (await expectName(page,'collections.rule.model')).fill('Cam100');
    await ruleType.selectOption('health');await (await expectName(page,'fleet.health.up','checkbox')).press('Space');
    await page.getByRole('button',{name:t('collections.builder.advancedMode'),exact:true}).press('Enter');const json=await expectName(page,'collections.builder.advancedMode');
    expect(JSON.parse(await json.inputValue()).expression).toMatchObject({op:'health',values:['up']});await json.fill('{broken');
    expect(await page.getByRole('button',{name:t('collections.preview.action'),exact:true}).isDisabled()).toBe(true);
    await json.fill('{"version":1,"expression":{"op":"all"}}');await page.getByRole('button',{name:t('collections.preview.action'),exact:true}).press('Enter');
    await page.waitForFunction(()=>window.calls.requests.some(r=>r.url==='/api/fleet/selectors/preview'));expect(await page.evaluate(()=>window.calls.requests.at(-1).body.selector)).toEqual({version:1,expression:{op:'all'}});
  }finally{await browser.close();expect(errors).toEqual([]);}
});
browserTest('camera search name supports keyboard search and a single selected camera callback',async()=>{
  const {browser,page,errors}=await openBrowser();try{
    await page.evaluate(()=>window.mount('camera'));const search=await expectName(page,'access.cameraSearch','searchbox');await search.fill('Entrance');
    const camera=page.getByRole('button',{name:/Entrance camera.*translated:access.cameraAdd/});await camera.press('Enter');await page.waitForFunction(()=>window.calls.changes.length===1);
    expect(await page.evaluate(()=>window.calls.changes)).toEqual([['cam-a']]);expect(await page.getByRole('button',{name:/Entrance camera.*translated:access.cameraSelected/}).isDisabled()).toBe(true);
  }finally{await browser.close();expect(errors).toEqual([]);}
});
browserTest('fleet checkbox names remain independent of counts and keyboard toggles update exactly once',async()=>{
  const {browser,page,errors}=await openBrowser();try{
    await page.evaluate(()=>window.mount('filters'));const health=await expectName(page,'fleet.health.up','checkbox');await health.press('Space');expect(await health.isChecked()).toBe(true);
    const recording=await expectName(page,'fleet.recording.continuous','checkbox');await recording.press('Space');expect(await recording.isChecked()).toBe(true);
    const tag=page.getByRole('checkbox',{name:'Entrance',exact:true});await tag.press('Space');expect(await tag.isChecked()).toBe(true);
    expect(await page.evaluate(()=>window.calls.changes)).toEqual([{health:['up']},{recordingModes:['continuous']},{tagUuids:['tag-a']}]);
  }finally{await browser.close();expect(errors).toEqual([]);}
});
