/** Explicit setting names preserve the existing permission, transport and editing contracts. */
const mockHooks = { cursor: 0, values: {}, setters: [] };
jest.mock('preact/hooks', () => ({
  useState: initial => { const index = mockHooks.cursor++; const setter = jest.fn(); mockHooks.setters[index] = setter; return [Object.hasOwn(mockHooks.values, index) ? mockHooks.values[index] : typeof initial === 'function' ? initial() : initial, setter]; },
  useEffect: jest.fn(), useCallback: fn => fn, useRef: value => ({ current: value }),
}));
jest.mock('../../js/i18n.js', () => ({ useI18n: () => ({ t: key => 'translated:' + key }) }));
jest.mock('../../js/query-client.js', () => ({ fetchJSON: jest.fn() }));
jest.mock('../../js/components/preact/ToastContainer.jsx', () => ({ showStatusMessage: jest.fn() }));
jest.mock('../../js/components/preact/recordings/recordingsAPI.jsx', () => ({ recordingsAPI: { getAllRecordingTags: jest.fn(), setRecordingTags: jest.fn(), batchUpdateRecordingTags: jest.fn() } }));
const { TagsOverlay, BulkTagsOverlay } = require('../../js/components/preact/recordings/TagsOverlay.jsx');
const components = {
  AdvancedTab: require('../../js/components/preact/settings/AdvancedTab.jsx').AdvancedTab,
  AuthTab: require('../../js/components/preact/settings/AuthTab.jsx').AuthTab,
  DetectionTab: require('../../js/components/preact/settings/DetectionTab.jsx').DetectionTab,
  GeneralTab: require('../../js/components/preact/settings/GeneralTab.jsx').GeneralTab,
  Go2rtcTab: require('../../js/components/preact/settings/Go2rtcTab.jsx').Go2rtcTab,
};
const { recordingsAPI } = require('../../js/components/preact/recordings/recordingsAPI.jsx');
const t = key => 'translated:' + key;
const NAMES = {
  "AdvancedTab": {
    "setting-buffer-size": "settings.bufferSizeKb",
    "setting-use-swap": "settings.useSwapFile",
    "setting-swap-size": "settings.swapSizeMb",
    "setting-turn-enabled": "settings.enableTurnRelay",
    "setting-turn-server-url": "settings.turnServerUrl",
    "setting-turn-username": "auth.username",
    "setting-turn-password": "auth.password",
    "setting-onvif-discovery-enabled": "settings.enableOnvifDiscovery",
    "setting-onvif-discovery-interval": "settings.discoveryIntervalSeconds",
    "setting-onvif-discovery-network": "settings.discoveryNetwork"
  },
  "AuthTab": {
    "setting-auth-enabled": "settings.enableAuthentication",
    "setting-demo-mode": "auth.demoMode",
    "setting-auth-timeout": "settings.sessionIdleTimeoutHours",
    "setting-auth-absolute-timeout": "settings.absoluteSessionLifetimeHours",
    "setting-trusted-device-days": "settings.rememberDeviceDays",
    "setting-trusted-proxy-cidrs": "settings.trustedProxyCidrs",
    "setting-force-mfa": "settings.forceMfaOnLogin",
    "setting-rate-limit-enabled": "settings.loginRateLimiting",
    "setting-rate-limit-max": "settings.maxLoginAttempts",
    "setting-rate-limit-window": "settings.rateLimitWindowSeconds"
  },
  "DetectionTab": {
    "setting-detection-models-path": "settings.detectionModelsPath",
    "setting-api-detection-url": "settings.apiDetectionUrl",
    "setting-api-detection-detector-name": "settings.apiDetectionDetectorName",
    "setting-default-detection-threshold": "settings.defaultDetectionThreshold",
    "setting-detection-grace-period": "settings.detectionGracePeriod",
    "setting-detection-engine-enabled": "settings.detectionEngineEnabled",
    "setting-detection-engine-threads": "settings.detectionEngineThreads"
  },
  "GeneralTab": {
    "setting-syslog-enabled": "settings.enableSyslog",
    "setting-syslog-ident": "settings.syslogIdent",
    "setting-web-port": "settings.webPort",
    "setting-web-bind-ip": "settings.webBindIp",
    "setting-thread-pool": "settings.threadPoolSize",
    "setting-max-streams": "settings.maxStreams"
  },
  "Go2rtcTab": {
    "setting-go2rtc-enabled": "settings.enableGo2rtc",
    "setting-go2rtc-force-native-hls": "settings.forceNativeHls",
    "setting-go2rtc-binary-path": "settings.binaryPath",
    "setting-go2rtc-config-dir": "settings.configDirectory",
    "setting-go2rtc-api-port": "settings.apiPort",
    "setting-go2rtc-rtsp-port": "settings.rtspPort",
    "setting-go2rtc-webrtc-enabled": "settings.enableWebrtc",
    "setting-go2rtc-webrtc-listen-port": "settings.webrtcListenPort",
    "setting-go2rtc-stun-enabled": "settings.enableStun",
    "setting-go2rtc-stun-server": "settings.stunServer",
    "setting-go2rtc-external-ip": "settings.externalIp",
    "setting-go2rtc-ice-servers": "settings.iceServers",
    "setting-go2rtc-config-override": "settings.go2rtcConfigOverride"
  }
};
const SETTINGS = { authEnabled: true, demoMode: false, loginRateLimitEnabled: true, forceMfaOnLogin: true, syslogEnabled: true, syslogIdent: 'fixture', logLevel: 2, webPort: 8080, webBindIp: '127.0.0.1', webThreadPoolSize: 4, maxStreams: 8, maxStreamsCeiling: 64, bufferSize: 512, useSwap: false, swapSize: 64, turnEnabled: true, turnServerUrl: 'turn:relay.invalid', turnUsername: 'fixture', turnPassword: 'synthetic', onvifDiscoveryEnabled: true, onvifDiscoveryInterval: 60, authTimeoutHours: 24, authAbsoluteTimeoutHours: 72, trustedDeviceDays: 14, loginRateLimitMaxAttempts: 5, loginRateLimitWindowSeconds: 60, detectionEngineEnabled: true, detectionEngineThreads: 2, detectionEngineDelegate: 'none', apiDetectionFormat: 'doods2', defaultDetectionThreshold: 70, detectionGracePeriod: 5, go2rtcEnabled: true, go2rtcWebrtcEnabled: true, go2rtcStunEnabled: true, go2rtcApiPort: 1984, go2rtcRtspPort: 8554, go2rtcWebrtcListenPort: 8555, go2rtcConfigOverride: 'log:\n  level: info\n' };
function nodes(node) { if (Array.isArray(node)) return node.flatMap(nodes); return !node || typeof node !== 'object' ? [] : [node, ...nodes(node.props?.children)]; }
function render(Component, props = {}, values = {}) { mockHooks.cursor = 0; mockHooks.values = values; mockHooks.setters = []; return Component(props); }
function props(overrides = {}) { return { settings: SETTINGS, t, canModifySettings: true, handleInputChange: jest.fn(), handleThresholdChange: jest.fn(), setSettings: jest.fn(), ...overrides }; }
function byId(tree, id) { return nodes(tree).find(node => node.props?.id === id); }
function named(node, name) { expect(node).toBeDefined(); expect(node.props['aria-label']).toBe(name); }
beforeEach(() => jest.clearAllMocks());
it.each(Object.keys(components))('%s fields retain each localized name, exact change event and read-only guard', name => {
  const input = props(); const tree = render(components[name], input);
  for (const [id, key] of Object.entries(NAMES[name])) {
    const field = byId(tree, id); named(field, t(key)); expect(field.props.disabled).toBe(false);
    const event = { target: { name: field.props.name, value: 'changed', checked: true } }; field.props.onChange(event);
    expect(id === 'setting-default-detection-threshold' ? input.handleThresholdChange : input.handleInputChange).toHaveBeenLastCalledWith(event);
  }
  const readOnly = render(components[name], props({ canModifySettings: false }));
  for (const [id, key] of Object.entries(NAMES[name])) { const field = byId(readOnly, id); named(field, t(key)); expect(field.props.disabled).toBe(true); }
  if (name === 'AdvancedTab') expect(byId(tree, 'setting-turn-password').props.type).toBe('password');
  if (name === 'GeneralTab') expect(byId(tree, 'setting-max-streams').props.max).toBe(64);
  if (name === 'Go2rtcTab') expect(byId(tree, 'setting-go2rtc-config-override').props.maxLength).toBe(65535);
});
it('view methods retain positive names, inverted change flags and the last-concrete-transport guard', () => {
  const input = props({ settings: { ...SETTINGS, webrtcDisabled: true, mseDisabled: true, hlsDisabled: false, autoDisabled: false, authEnabled: false, loginRateLimitEnabled: false } });
  const tree = render(components.AuthTab, input);
  for (const [method, key] of [['auto', 'Auto'], ['webrtc', 'Webrtc'], ['hls', 'Hls'], ['mse', 'Mse']]) {
    const field = byId(tree, 'setting-' + method + '-enabled'); named(field, t('settings.viewMethod' + key)); expect(field.props.disabled).toBe(method === 'hls');
    field.props.onChange({ target: { checked: true } }); expect(input.handleInputChange).toHaveBeenLastCalledWith({ target: { name: method + 'Disabled', type: 'checkbox', checked: false } });
  }
  for (const id of ['setting-demo-mode', 'setting-rate-limit-max', 'setting-rate-limit-window']) expect(byId(tree, id).props.disabled).toBe(true);
});
it('conditional settings preserve hidden engine, syslog and go2rtc controls and dependent disablement', () => {
  const disabled = { ...SETTINGS, detectionEngineEnabled: false, apiDetectionFormat: 'light-object-detect', syslogEnabled: false, go2rtcEnabled: false };
  const detection = render(components.DetectionTab, props({ settings: disabled })); expect(byId(detection, 'setting-api-detection-detector-name')).toBeUndefined(); expect(byId(detection, 'setting-api-detection-backend')).toBeDefined(); expect(byId(detection, 'setting-detection-engine-threads').props.disabled).toBe(true);
  expect(byId(render(components.GeneralTab, props({ settings: disabled })), 'setting-syslog-ident')).toBeUndefined();
  const go = render(components.Go2rtcTab, props({ settings: disabled })); expect(byId(go, 'setting-go2rtc-enabled')).toBeDefined(); expect(byId(go, 'setting-go2rtc-config-override')).toBeUndefined();
});
it('single tag entry keeps localized name, Enter save, Escape and pending guard', async () => {
  const onClose = jest.fn(); const onTagsChanged = jest.fn(); const input = { recording: { id: 1, tags: ['old'] }, onClose, onTagsChanged };
  const tree = render(TagsOverlay, input, { 1: '  new  ' }); const field = nodes(tree).find(node => node.type === 'input'); named(field, t('recordings.addTagPlaceholder'));
  const preventDefault = jest.fn(); field.props.onKeyDown({ key: 'Enter', preventDefault }); await Promise.resolve();
  expect(preventDefault).toHaveBeenCalledTimes(1); expect(recordingsAPI.setRecordingTags).toHaveBeenCalledWith(1, ['old', 'new']); expect(onTagsChanged).toHaveBeenCalledWith(1, ['old', 'new']);
  field.props.onKeyDown({ key: 'Escape' }); expect(onClose).toHaveBeenCalledTimes(1);
  const pending = render(TagsOverlay, input, { 4: true }); expect(nodes(pending).find(node => node.type === 'input').props.disabled).toBe(true);
});
it('bulk tag names preserve tri-state ref, exact selected IDs, add/remove and pending state', async () => {
  const input = { recordings: [{ id: 1 }, { id: 2 }, { id: 3 }], selectedRecordings: { 1: true, 2: true, 3: false }, onClose: jest.fn(), onTagsChanged: jest.fn() };
  const tree = render(BulkTagsOverlay, input, { 1: { partial: 'some', all: 'all' }, 2: 'new' }); const fields = nodes(tree).filter(node => node.type === 'input');
  expect(fields).toHaveLength(3); named(fields[0], 'all'); named(fields[1], 'partial'); named(fields[2], t('recordings.addNewTagPlaceholder'));
  const checkbox = {}; fields[1].ref(checkbox); expect(checkbox.indeterminate).toBe(true);
  await fields[0].props.onChange(); await fields[1].props.onChange(); fields[2].props.onKeyDown({ key: 'Enter', preventDefault: jest.fn() }); await Promise.resolve();
  expect(recordingsAPI.batchUpdateRecordingTags.mock.calls).toEqual([[[1, 2], [], ['all']], [[1, 2], ['partial'], []], [[1, 2], ['new'], []]]); expect(input.onTagsChanged).toHaveBeenCalledTimes(3);
  const pending = render(BulkTagsOverlay, input, { 1: { partial: 'some' }, 4: true }); expect(nodes(pending).filter(node => node.type === 'input').every(node => node.props.disabled)).toBe(true);
});
async function openBrowser() {
  const { chromium } = require('../../../node_modules/playwright'); const { build } = require('esbuild');
  const fs = require('node:fs'); const path = require('node:path'); const webRoot = path.resolve(__dirname, '../..');
  const built = await build({ stdin: { resolveDir: webRoot, loader: 'jsx', contents: `
    import { h, render } from 'preact'; import { useState } from 'preact/hooks';
    import { AdvancedTab } from './js/components/preact/settings/AdvancedTab.jsx';
    import { AuthTab } from './js/components/preact/settings/AuthTab.jsx';
    import { DetectionTab } from './js/components/preact/settings/DetectionTab.jsx';
    import { GeneralTab } from './js/components/preact/settings/GeneralTab.jsx';
    import { Go2rtcTab } from './js/components/preact/settings/Go2rtcTab.jsx';
    import { TagsOverlay, BulkTagsOverlay } from './js/components/preact/recordings/TagsOverlay.jsx';
    const components = { AdvancedTab, AuthTab, DetectionTab, GeneralTab, Go2rtcTab }; const root = document.getElementById('root'); const t = key => 'translated:' + key;
    window.fixtureJSON = async (url, options = {}) => {
      window.calls.requests.push({url, method: options.method || 'GET', body: options.body ? JSON.parse(options.body) : null});
      if (url.endsWith('/validate')) return {valid:false,error:{line:2,column:3,message:'fixture invalid YAML'},warnings:[],libyaml_available:true};
      if (url.endsWith('/effective-config')) return {base:'api: {}',override:'log: {}',effective:'api: {}'};
      if (url.endsWith('/override-status')) return {db_bytes:8,file_present:true,content_matches:true,process_state_known:false};
      throw new Error('Unexpected request ' + url);
    };
    window.fixtureTags = {getAllRecordingTags: async()=>['partial','all','suggested'],setRecordingTags:async(...args)=>{window.calls.tags.push(args);return new Promise(resolve=>window.finish=resolve);},batchUpdateRecordingTags:async(...args)=>{window.calls.tags.push(args);return new Promise(resolve=>window.finish=resolve);}};
    function App({kind,options}) {
      const [settings,setSettings] = useState({...${JSON.stringify(SETTINGS)},...options.settings});window.currentSettings=settings;
      const [records,setRecords] = useState([{id:1,tags:['all','partial']},{id:2,tags:['all']},{id:3,tags:['outside']}]);
      const handleInputChange = event => {const {name,type,value,checked}=event.target;const next=type==='checkbox'?checked:value;window.calls.changes.push({name,value:next});setSettings(prev=>({...prev,[name]:next}));};
      const common={settings,setSettings,handleInputChange,handleThresholdChange:event=>{window.calls.threshold++;handleInputChange(event);},canModifySettings:!options.readonly,t};
      if(kind==='single') return h(TagsOverlay,{recording:{id:1,tags:['all']},onClose:()=>window.calls.close++,onTagsChanged:(...args)=>window.calls.updated.push(args)});
      if(kind==='bulk') return h(BulkTagsOverlay,{recordings:records,selectedRecordings:{1:true,2:true,3:false},onClose:()=>window.calls.close++,onTagsChanged:()=>{
        window.calls.updated.push([]);
        // Model delivered post-save recording data. The opt-in reproducer withholds it,
        // matching the real parent's pending query invalidation rather than claiming
        // this fixture exercises the parent/query integration.
        if(!options.staleRecords){const [ids,added,removed]=window.calls.tags.at(-1);setRecords(prev=>prev.map(record=>ids.includes(record.id)?{...record,tags:[...new Set([...record.tags.filter(tag=>!removed.includes(tag)),...added])]}:record));}
      }});
      return h(components[kind],common);
    }
    window.mount=(kind,options={})=>{render(null,root);window.calls={changes:[],threshold:0,requests:[],tags:[],updated:[],close:0};render(h(App,{kind,options}),root);};
  ` }, bundle: true, write: false, format: 'iife', jsx: 'automatic', jsxImportSource: 'preact', plugins: [{ name: 'fixture-services', setup(build) {
    build.onResolve({ filter: /(?:query-client\.js|i18n\.js|ToastContainer\.jsx|recordingsAPI\.jsx)$/ }, args => ({ path: args.path, namespace: 'fixture' }));
    build.onLoad({ filter: /.*/, namespace: 'fixture' }, args => ({ contents: args.path.endsWith('query-client.js') ? 'export const fetchJSON=(...args)=>window.fixtureJSON(...args);' : args.path.endsWith('i18n.js') ? 'export const useI18n=()=>({t:key=>"translated:"+key});' : args.path.endsWith('recordingsAPI.jsx') ? 'export const recordingsAPI={getAllRecordingTags:(...a)=>window.fixtureTags.getAllRecordingTags(...a),setRecordingTags:(...a)=>window.fixtureTags.setRecordingTags(...a),batchUpdateRecordingTags:(...a)=>window.fixtureTags.batchUpdateRecordingTags(...a)};' : 'export const showStatusMessage=()=>{};', loader: 'js' }));
  } }] });
  const browser = await chromium.launch({ headless: true }); const page = await browser.newPage();
  await page.route('http://fixture.invalid/**', route => route.fulfill({ contentType: 'text/html', body: '<html><body><div id="root"></div><button id="outside">Outside</button></body></html>' }));
  await page.goto('http://fixture.invalid/settings'); const css = fs.readdirSync(path.join(webRoot,'dist/assets')).find(file=>/^settings-.*\.css$/.test(file)); if(css)await page.addStyleTag({content:fs.readFileSync(path.join(webRoot,'dist/assets',css),'utf8')});
  await page.addScriptTag({ content: built.outputFiles[0].text }); return { browser, page };
}
const browserTest = process.env.T7_172_BROWSER === '1' ? it : it.skip;
browserTest.each(Object.keys(components))('%s browser names resolve to the original fields and permissions block activation', async name => {
  const {browser,page}=await openBrowser();try{
    await page.evaluate(name=>window.mount(name),name);
    for(const [id,key] of Object.entries(NAMES[name])){const field=page.locator('#'+id);expect(await field.getAttribute('aria-label')).toBe(t(key));expect(await page.getByLabel(t(key),{exact:true}).getAttribute('id')).toBe(id);expect(await field.isEnabled()).toBe(true);}
    const first=page.locator('#'+Object.keys(NAMES[name])[0]);const fieldName=await first.getAttribute('name');const checkbox=await first.getAttribute('type')==='checkbox';if(checkbox){await first.focus();await page.keyboard.press('Space');}else{await first.fill('1024');await first.press('Tab');}
    expect(await page.evaluate(()=>window.calls.changes)).toEqual([{name:fieldName,value:checkbox?false:'1024'}]);
    await page.evaluate(name=>window.mount(name,{readonly:true}),name);
    for(const id of Object.keys(NAMES[name]))expect(await page.locator('#'+id).isDisabled()).toBe(true);
    await page.locator('#'+Object.keys(NAMES[name])[0]).evaluate(el=>el.click());expect(await page.evaluate(()=>window.calls.changes)).toEqual([]);
  }finally{await browser.close();}
},30000);
browserTest('auth keyboard toggles preserve inverted flags, last transport, demo and rate-limit dependencies',async()=>{
 const {browser,page}=await openBrowser();try{
  await page.evaluate(()=>window.mount('AuthTab',{settings:{autoDisabled:false,webrtcDisabled:true,mseDisabled:true,hlsDisabled:false,authEnabled:false,loginRateLimitEnabled:false}}));
  const hls=page.getByRole('checkbox',{name:t('settings.viewMethodHls'),exact:true});expect(await hls.isChecked()).toBe(true);expect(await hls.isDisabled()).toBe(true);
  for(const id of ['setting-demo-mode','setting-rate-limit-max','setting-rate-limit-window'])expect(await page.locator('#'+id).isDisabled()).toBe(true);
  const webrtc=page.getByRole('checkbox',{name:t('settings.viewMethodWebrtc'),exact:true});await webrtc.focus();await page.keyboard.press('Space');expect(await hls.isEnabled()).toBe(true);
  await hls.focus();await page.keyboard.press('Space');expect(await webrtc.isDisabled()).toBe(true);
  await page.getByRole('checkbox',{name:t('settings.viewMethodAuto'),exact:true}).uncheck();expect(await webrtc.isDisabled()).toBe(true);
  expect(await page.evaluate(()=>window.calls.changes)).toEqual([{name:'webrtcDisabled',value:false},{name:'hlsDisabled',value:true},{name:'autoDisabled',value:true}]);
  await page.getByRole('checkbox',{name:t('settings.enableAuthentication'),exact:true}).check();expect(await page.locator('#setting-demo-mode').isEnabled()).toBe(true);
  await page.getByRole('checkbox',{name:t('settings.loginRateLimiting'),exact:true}).check();expect(await page.locator('#setting-rate-limit-max').isEnabled()).toBe(true);
 }finally{await browser.close();}
},30000);
browserTest('detection format, threshold keyboard and engine dependency preserve exact state callbacks',async()=>{
 const {browser,page}=await openBrowser();try{
  await page.evaluate(()=>window.mount('DetectionTab'));const threshold=page.getByRole('slider',{name:t('settings.defaultDetectionThreshold'),exact:true});await threshold.focus();await page.keyboard.press('ArrowRight');expect(await threshold.inputValue()).toBe('71');expect(await page.evaluate(()=>window.calls.threshold)).toBe(1);
  await page.locator('#setting-api-detection-format').selectOption('light-object-detect');expect(await page.locator('#setting-api-detection-detector-name').count()).toBe(0);expect(await page.locator('#setting-api-detection-backend').isVisible()).toBe(true);
  await page.getByRole('checkbox',{name:t('settings.detectionEngineEnabled'),exact:true}).uncheck();expect(await page.getByRole('spinbutton',{name:t('settings.detectionEngineThreads'),exact:true}).isDisabled()).toBe(true);
  expect(await page.evaluate(()=>window.calls.changes)).toEqual([{name:'defaultDetectionThreshold',value:'71'},{name:'apiDetectionFormat',value:'light-object-detect'},{name:'detectionEngineEnabled',value:false}]);
 }finally{await browser.close();}
},30000);
browserTest('override editor retains blur validation, preset append, effective preview and native close',async()=>{
 const {browser,page}=await openBrowser();try{
  await page.evaluate(()=>window.mount('Go2rtcTab'));const editor=page.getByRole('textbox',{name:t('settings.go2rtcConfigOverride'),exact:true});await editor.fill('bad: [');await editor.press('Tab');await page.getByText('fixture invalid YAML',{exact:true}).waitFor();
  expect(await page.evaluate(()=>window.calls.requests)).toEqual([{url:'/api/settings/go2rtc/validate',method:'POST',body:{override:'bad: ['}}]);
  await page.locator('select').selectOption('1');expect(await editor.inputValue()).toBe('bad: [\nlog:\n  level: trace\n');expect(await page.getByText('fixture invalid YAML',{exact:true}).count()).toBe(0);
  await page.getByRole('button',{name:t('settings.go2rtcShowEffectiveConfig'),exact:true}).focus();await page.keyboard.press('Enter');await page.getByText('Override is saved and on disk. go2rtc is busy starting or reconfiguring, so its runtime state is not readable yet.',{exact:true}).waitFor();
  expect((await page.evaluate(()=>window.calls.requests)).map(call=>call.url)).toEqual(['/api/settings/go2rtc/validate','/api/system/go2rtc/effective-config','/api/system/go2rtc/override-status']);
  await page.getByRole('button',{name:t('common.close'),exact:true}).focus();await page.keyboard.press('Space');expect(await page.getByText(t('settings.go2rtcEffectiveConfig'),{exact:true}).count()).toBe(0);
 }finally{await browser.close();}
},30000);
browserTest('single tags keep focused named input, Enter save, disabled pending field and Escape callback',async()=>{
 const {browser,page}=await openBrowser();try{
  await page.evaluate(()=>window.mount('single'));const field=page.getByRole('textbox',{name:t('recordings.addTagPlaceholder'),exact:true});await field.waitFor();await page.waitForFunction(()=>document.activeElement===document.querySelector('input'));
  await field.fill(' new ');await field.press('Enter');await page.waitForFunction(()=>window.calls.tags.length===1);expect(await field.isDisabled()).toBe(true);expect(await page.evaluate(()=>window.calls.tags)).toEqual([[1,['all','new']]]);
  await page.evaluate(()=>window.finish());await page.waitForFunction(()=>window.calls.updated.length===1);expect(await page.evaluate(()=>window.calls.updated)).toEqual([[1,['all','new']]]);expect(await field.inputValue()).toBe('');await field.focus();await field.press('Escape');expect(await page.evaluate(()=>window.calls.close)).toBe(1);
 }finally{await browser.close();}
},30000);
browserTest('bulk tags retain mixed state, selected IDs, one keyboard update, remove and new-tag save',async()=>{
 const {browser,page}=await openBrowser();try{
  await page.evaluate(staleRecords=>window.mount('bulk',{staleRecords}),process.env.T7_172_STALE_REPRO==='1');const partial=page.getByRole('checkbox',{name:'partial',exact:true});await partial.waitFor();expect(await partial.evaluate(el=>el.indeterminate)).toBe(true);
  await partial.focus();await page.keyboard.press('Space');expect(await partial.isDisabled()).toBe(true);expect(await page.getByRole('textbox',{name:t('recordings.addNewTagPlaceholder'),exact:true}).isDisabled()).toBe(true);expect(await page.evaluate(()=>window.calls.tags)).toEqual([[[1,2],['partial'],[]]]);
  await page.evaluate(()=>window.finish());await page.waitForFunction(()=>window.calls.updated.length===1);expect(await partial.isChecked()).toBe(true);expect(await partial.evaluate(el=>el.indeterminate)).toBe(false);
  await partial.focus();await page.keyboard.press('Space');await page.evaluate(()=>window.finish());await page.waitForFunction(()=>window.calls.updated.length===2);expect(await partial.isChecked()).toBe(false);
  const field=page.getByRole('textbox',{name:t('recordings.addNewTagPlaceholder'),exact:true});await field.fill(' new ');await field.press('Enter');await page.evaluate(()=>window.finish());await page.waitForFunction(()=>window.calls.updated.length===3);
  expect(await page.evaluate(()=>window.calls.tags)).toEqual([[[1,2],['partial'],[]],[[1,2],[],['partial']],[[1,2],['new'],[]]]);await page.evaluate(()=>new Promise(resolve=>requestAnimationFrame(()=>requestAnimationFrame(resolve))));expect(await page.getByRole('checkbox',{name:'new',exact:true}).isChecked()).toBe(true);
  await field.focus();await field.press('Escape');expect(await page.evaluate(()=>window.calls.close)).toBe(1);
 }finally{await browser.close();}
},30000);
