/** Existing localized names must survive real form, selection and media interactions. */
const mockHooks = { cursor: 0, values: {}, setters: [] };
let mockQueryData = [];
jest.mock('preact/hooks', () => ({
  useState: initial => { const index = mockHooks.cursor++; const setter = jest.fn(); mockHooks.setters[index] = setter;
    return [Object.hasOwn(mockHooks.values, index) ? mockHooks.values[index] : typeof initial === 'function' ? initial() : initial, setter]; },
  useEffect: jest.fn(), useLayoutEffect: jest.fn(), useMemo: factory => factory(), useCallback: fn => fn, useRef: value => ({ current: value }), useId: () => 'fixture',
}));
jest.mock('../../js/query-client.js', () => ({ fetchJSON: jest.fn(), useQuery: () => ({ data: mockQueryData, isLoading: false }) }));
jest.mock('../../js/i18n.js', () => ({ useI18n: () => ({ t: key => 'translated:' + key }) }));
jest.mock('../../js/components/preact/ToastContainer.jsx', () => ({ showStatusMessage: jest.fn() }));
const { OrganizationManager } = require('../../js/components/preact/fleet/OrganizationManager.jsx');
const { SavedViewsBar } = require('../../js/components/preact/fleet/SavedViewsBar.jsx');
const { StaticCollectionMembers } = require('../../js/components/preact/fleet/StaticCollectionMembers.jsx');
const { InvestigationActions } = require('../../js/components/preact/investigation/InvestigationActions.jsx');
const { InvestigationBookmarks } = require('../../js/components/preact/investigation/InvestigationBookmarks.jsx');
const { InvestigationView } = require('../../js/components/preact/investigation/InvestigationView.jsx');
const t = key => 'translated:' + key;
const start = 1793448000;
const track = { camera_uuid: 'cam-a', name: 'Entrance', segment_count: 1, segments: [{ id: 1, start_time: start, end_time: start + 60, duration: 60 }] };
const timeline = { start_time: start, end_time: start + 60, tracks: [track] };
const filters = { eventType: '', location: '', label: '', zone: '', source: '', captureMethod: '', recordingTag: '', protection: '', minConfidence: '', region: { cameraUuid: 'cam-a', x: .1, y: .2, width: .3, height: .4, match: 'minimum_intersection', minIntersection: .5 } };
function nodes(node) { if (Array.isArray(node)) return node.flatMap(nodes); if (!node || typeof node !== 'object') return []; return [node, ...nodes(node.props?.children)]; }
function render(Component, props = {}, values = {}) { mockHooks.cursor = 0; mockHooks.values = values; mockHooks.setters = []; return Component({ t, ...props }); }
function child(tree, name) { const node = nodes(tree).find(node => typeof node.type === 'function' && node.type.name === name); expect(node).toBeDefined(); return node; }
function fields(tree) { return nodes(tree).filter(node => ['input', 'textarea'].includes(node.type)); }
function named(node, key, localized = true) { expect(node).toBeDefined(); expect(node.props['aria-label']).toBe(localized ? t(key) : key); }
function change(node, value = 'changed', checked = true) { const target = { value, checked }; (node.props.onInput || node.props.oninput || node.props.onChange)({ target, currentTarget: target }); }
beforeEach(() => { jest.clearAllMocks(); mockQueryData = []; });
it.each(['locations','tags'])('organization %s fields have meaningful names and retain form constraints/updates', tab => {
  const manager = render(OrganizationManager, { locations: [], tags: [] }, { 0: tab });
  const tabNode = child(manager, tab === 'locations' ? 'LocationsTab' : 'TagsTab');
  const tabTree = render(tabNode.type, tabNode.props, { 0: { item: null } }); const editorNode = child(tabTree, tab === 'locations' ? 'LocationEditor' : 'TagEditor');
  const editor = render(editorNode.type, editorNode.props); const entries = fields(editor);
  const names = tab === 'locations' ? ['common.name','fleet.organization.sortOrder'] : ['fleet.organization.tagLabel','fleet.organization.color','fleet.organization.tagDescription'];
  expect(entries).toHaveLength(names.length); entries.forEach((field,index)=>{named(field,names[index]);change(field);});
  expect(entries[0].props.required).toBe(true); expect(mockHooks.setters[0]).toHaveBeenCalledTimes(names.length);
  expect(nodes(editor).find(node=>node.type==='button'&&node.props.type==='submit').props.disabled).toBe(true);
  if(tab==='tags') expect(entries[1].props.pattern).toBe('^#[0-9A-Fa-f]{6}$');
});
it('saved-view names preserve sharing permission, required name and pending save guard',()=>{
  let tree=render(SavedViewsBar,{data:{can_share:true},state:{}},{1:true,4:true});const entries=fields(tree);expect(entries).toHaveLength(2);
  named(entries[0],'fleet.views.name');named(entries[1],'fleet.views.share');change(entries[0],'Gate');change(entries[1]);
  expect(mockHooks.setters[2]).toHaveBeenCalledWith('Gate');expect(mockHooks.setters[3]).toHaveBeenCalledWith(true);
  expect(nodes(tree).find(node=>node.type==='button'&&node.props.type==='submit').props.disabled).toBe(true);
  tree=render(SavedViewsBar,{data:{can_share:false},state:{}},{1:true});expect(fields(tree)).toHaveLength(1);
});
it('membership names preserve indeterminate/page selection and search page reset',()=>{
  mockQueryData={cameras:[{camera_uuid:'cam-a',name:'Entrance'},{camera_uuid:'cam-b',name:'Rear'}],total:2,total_pages:1};const onChange=jest.fn();
  const tree=render(StaticCollectionMembers,{selectedUuids:['cam-a','off-page'],onChange});const entries=fields(tree);expect(entries).toHaveLength(4);
  named(entries[0],'collections.members.search');named(entries[1],'collections.members.selectPage');named(entries[2],'Entrance',false);named(entries[3],'Rear',false);
  const element={};entries[1].ref(element);expect(element.indeterminate).toBe(true);change(entries[1]);expect(onChange).toHaveBeenLastCalledWith(['cam-a','off-page','cam-b']);
  change(entries[2],'',false);expect(onChange).toHaveBeenLastCalledWith(['off-page']);change(entries[0],'Rear');expect(mockHooks.setters[0]).toHaveBeenCalledWith('Rear');expect(mockHooks.setters[1]).toHaveBeenCalledWith(1);
});
it('investigation action controls preserve scope permissions, time bounds and busy states',()=>{
  const tree=render(InvestigationActions,{timeline,selectedResult:null},{0:true,6:'protect'});const entries=fields(tree);expect(entries).toHaveLength(4);
  ['investigation.actions.selectedResult','investigation.actions.fullWindow','investigation.start','investigation.end'].forEach((name,index)=>{named(entries[index],name);expect(entries[index].props.disabled).toBe(true);});
  change(entries[2],'2026-10-31T08:00:00');expect(mockHooks.setters[2]).toHaveBeenCalledWith('2026-10-31T08:00:00');expect(mockHooks.setters[4]).toHaveBeenCalledWith(null);
  expect(entries[2].props.min).toBeTruthy();expect(entries[3].props.max).toBeTruthy();
});
it('bookmark title/note names retain required/length constraints and exact input values',()=>{
  const tree=render(InvestigationBookmarks,{timeline,cursor:start,primaryCameraUuid:'cam-a',searchFilters:filters},{0:'save'});const entries=fields(tree);expect(entries).toHaveLength(2);
  named(entries[0],'investigation.bookmarks.title');named(entries[1],'investigation.bookmarks.note');expect(entries[0].props.required).toBe(true);expect(entries[1].props.maxLength).toBe('2047');
  change(entries[0],'Incident');change(entries[1],'Details');expect(mockHooks.setters[1]).toHaveBeenCalledWith('Incident');expect(mockHooks.setters[2]).toHaveBeenCalledWith('Details');
});
function investigationTree() {
  global.window={location:{search:'',href:'http://fixture.invalid/'}};mockQueryData=[{camera_uuid:'cam-a',name:'Entrance'}];
  return render(InvestigationView,{}, {0:start,1:start+60,2:['cam-a'],3:timeline,4:start,5:['cam-a'],6:'cam-a',12:filters});
}
it('investigation camera/time/confidence/region inputs keep names and original update limits',()=>{
  const tree=investigationTree();const entries=fields(tree).filter(field=>field.props.type!=='range');const expected=['Entrance','investigation.start','investigation.end','investigation.minimumConfidence','investigation.regionLeft','investigation.regionTop','investigation.regionWidth','investigation.regionHeight','investigation.regionMinimumOverlap'];
  expect(entries).toHaveLength(expected.length);entries.forEach((field,index)=>named(field,expected[index],index!==0));
  change(entries[0]);expect(mockHooks.setters[2]).toHaveBeenCalledTimes(1);change(entries[3],'.7');expect(mockHooks.setters[12]).toHaveBeenCalled();
  change(entries.at(-1),'25');expect(mockHooks.setters[12].mock.calls.at(-1)[0](filters).region.minIntersection).toBe(.25);
  const before=mockHooks.setters[12].mock.calls.length;change(entries.at(-1),'101');expect(mockHooks.setters[12]).toHaveBeenCalledTimes(before);
});
it('player and track selection are named without adding native playback controls or changing delegation',()=>{
  const tree=investigationTree();const player=child(tree,'InvestigationPlayer');const rendered=render(player.type,player.props);const video=nodes(rendered).find(node=>node.type==='video');
  named(video,'Entrance '+t('timeline.timelinePlayback'),false);expect(video.props.controls).toBeUndefined();expect(typeof video.props.onPlay).toBe('function');expect(typeof video.props.onPause).toBe('function');
  const row=child(tree,'InvestigationTrack');const onToggleActive=jest.fn();const rowTree=render(row.type,{...row.props,onToggleActive});const selection=fields(rowTree)[0];named(selection,'Entrance',false);change(selection);expect(onToggleActive).toHaveBeenCalledTimes(1);
  const slider=nodes(rowTree).find(node=>node.props?.role==='slider');expect(slider.props.tabIndex).toBe('0');expect(typeof slider.props.onKeyDown).toBe('function');
});
async function openBrowser() {
  const {chromium}=require('../../../node_modules/playwright');const {build}=require('esbuild');const fs=require('node:fs');const path=require('node:path');const webRoot=path.resolve(__dirname,'../..');
  const built=await build({stdin:{resolveDir:webRoot,loader:'jsx',contents:`
    import {h,render} from 'preact';import {useState} from 'preact/hooks';
    import {OrganizationManager} from './js/components/preact/fleet/OrganizationManager.jsx';
    import {SavedViewsBar} from './js/components/preact/fleet/SavedViewsBar.jsx';
    import {StaticCollectionMembers} from './js/components/preact/fleet/StaticCollectionMembers.jsx';
    import {DEFAULT_FLEET_STATE} from './js/components/preact/fleet/fleetQuery.js';
    import {InvestigationActions} from './js/components/preact/investigation/InvestigationActions.jsx';
    import {InvestigationBookmarks} from './js/components/preact/investigation/InvestigationBookmarks.jsx';
    import {InvestigationView} from './js/components/preact/investigation/InvestigationView.jsx';
    const root=document.getElementById('root');const t=key=>'translated:'+key;const timeline=${JSON.stringify(timeline)};
    const uuid='aaaaaaaa-aaaa-4aaa-8aaa-aaaaaaaaaaaa';window.cameraUuid=uuid;
    window.streams=[{camera_uuid:uuid,name:'Entrance'},{camera_uuid:'bbbbbbbb-bbbb-4bbb-8bbb-bbbbbbbbbbbb',name:'Rear'}];
    window.memberData={cameras:[{camera_uuid:'cam-a',name:'Entrance'},{camera_uuid:'cam-b',name:'Rear'}],total:2,total_pages:1};
    window.viewTimeline={...timeline,tracks:[{...timeline.tracks[0],camera_uuid:uuid}]};
    window.fixtureJSON=async(url,options={})=>{
      const body=options.body?JSON.parse(options.body):null;window.calls.requests.push({url,method:options.method||'GET',body});
      if(url==='/api/investigations/timeline')return window.viewTimeline;
      if(url==='/api/investigations/search')return {results:[],page:{total:0},facets:{},histogram:[],coverage:{}};
      if(url==='/api/investigations/recordings/preview')return {start_time:timeline.start_time,end_time:timeline.end_time,recordings:[{id:1,can_protect:true,can_export:false,protected:false}]};
      if(options.method==='POST')return new Promise(resolve=>window.finish=()=>resolve({uuid:'saved',success_count:1,fail_count:0}));
      return {};
    };
    function Members(){const [selected,setSelected]=useState(['cam-a','off-page']);return h(StaticCollectionMembers,{selectedUuids:selected,onChange:next=>{setSelected(next);window.calls.changes.push(next);},t});}
    window.mount=(kind,sharing=true)=>{
      render(null,root);window.calls={requests:[],changes:[],refresh:0,close:0};
      const common={t,onClose:()=>window.calls.close++,onRefresh:()=>window.calls.refresh++};
      if(kind==='organization')return render(h(OrganizationManager,{locations:[],tags:[],...common}),root);
      if(kind==='saved')return render(h(SavedViewsBar,{data:{can_share:sharing},state:{...DEFAULT_FLEET_STATE,search:'Entry',health:['up']},onApply:()=>true,...common}),root);
      if(kind==='members')return render(h(Members),root);
      if(kind==='actions')return render(h(InvestigationActions,{timeline,selectedResult:{camera_uuid:'cam-a',start_time:timeline.start_time+5,end_time:timeline.start_time+10},t}),root);
      if(kind==='bookmarks')return render(h(InvestigationBookmarks,{timeline,cursor:timeline.start_time+5,primaryCameraUuid:'cam-a',searchFilters:{},t}),root);
      const url=new URL(location.href);url.search='?cameras='+uuid+'&start='+timeline.start_time+'&end='+timeline.end_time+'&region_camera='+uuid+'&region_rect=0.1,0.2,0.3,0.4&region_match=minimum_intersection&region_min=0.5';history.replaceState({},'',url);
      return render(h(InvestigationView),root);
    };
  `},bundle:true,write:false,format:'iife',jsx:'automatic',jsxImportSource:'preact',plugins:[{name:'services',setup(builder){
    builder.onResolve({filter:/(?:i18n|query-client|ToastContainer)\.(?:js|jsx)$/},args=>({path:args.path,namespace:'service'}));
    builder.onLoad({filter:/.*/,namespace:'service'},()=>({loader:'js',contents:`const t=key=>'translated:'+key;export const useI18n=()=>({t});export const fetchJSON=(url,options)=>window.fixtureJSON(url,options);export const showStatusMessage=()=>{};export const useQuery=({queryKey})=>({data:queryKey[0]==='investigation-streams'?window.streams:window.memberData,isLoading:false});`}));
  }}]});
  const browser=await chromium.launch({headless:true});try{
    const page=await browser.newPage({viewport:{width:1440,height:1100},timezoneId:'UTC'});page.setDefaultTimeout(5000);const errors=[];page.on('pageerror',error=>errors.push(error.message));
    await page.route('http://fixture.invalid/**',route=>route.fulfill({contentType:'text/html',body:'<div id="root"></div>'}));
    await page.route('http://fixture.invalid/api/recordings/play/*',route=>route.fulfill({contentType:'video/mp4',body:fs.readFileSync(path.resolve(webRoot,'../tests/integration/fixtures/recording.mp4'))}));
    await page.goto('http://fixture.invalid/');const styles=new Set();for(const file of ['streams.html','investigation.html']){const html=fs.readFileSync(path.join(webRoot,'dist',file),'utf8');for(const tag of html.match(/<link[^>]*rel="stylesheet"[^>]*>/g)||[])styles.add(tag.match(/href="([^"]+)"/)[1]);}
    expect(styles.size).toBeGreaterThan(0);for(const href of styles)await page.addStyleTag({content:fs.readFileSync(path.join(webRoot,'dist',href),'utf8')});await page.addScriptTag({content:built.outputFiles[0].text});return {browser,page,errors};
  }catch(error){await browser.close();throw error;}
}
const browserTest=process.env.T7_170_BROWSER==='1'?it:it.skip;
async function control(scope,key,role='textbox'){const input=scope.getByRole(role,{name:t(key),exact:true});await input.waitFor();expect(await input.count()).toBe(1);return input;}
browserTest('organization names preserve exact location/tag payloads, required/color validation and pending Cancel',async()=>{
  const {browser,page,errors}=await openBrowser();try{
    await page.evaluate(()=>window.mount('organization'));await page.getByRole('button',{name:t('fleet.organization.addRoot'),exact:true}).click();
    const save=page.getByRole('button',{name:t('common.saveChanges'),exact:true});expect(await save.isDisabled()).toBe(true);
    await (await control(page,'common.name')).fill('  Lobby  ');await (await control(page,'fleet.organization.sortOrder','spinbutton')).fill('7');await save.press('Enter');await page.waitForFunction(()=>window.calls.requests.length===1);
    expect(await page.evaluate(()=>window.calls.requests[0])).toMatchObject({url:'/api/locations',body:{name:'Lobby',type:'area',parent_uuid:null,sort_order:7}});expect(await page.getByRole('button',{name:t('common.cancel'),exact:true}).isDisabled()).toBe(true);
    await page.evaluate(()=>window.finish());await page.waitForFunction(()=>window.calls.refresh===1);
    await page.getByRole('tab',{name:t('fleet.organization.tags'),exact:true}).click();await page.getByRole('button',{name:t('fleet.organization.addTag'),exact:true}).click();
    await (await control(page,'fleet.organization.tagLabel')).fill('  Entry  ');const color=await control(page,'fleet.organization.color');await color.fill('invalid');expect(await color.evaluate(el=>el.validity.patternMismatch)).toBe(true);
    await color.fill('#123456');await (await control(page,'fleet.organization.tagDescription')).fill('  Door cameras  ');await save.press('Enter');await page.waitForFunction(()=>window.calls.requests.length===2);
    expect(await page.evaluate(()=>window.calls.requests[1])).toMatchObject({url:'/api/camera-tags',body:{label:'Entry',color:'#123456',description:'Door cameras'}});await page.evaluate(()=>window.finish());await page.waitForFunction(()=>window.calls.refresh===2);
  }finally{await browser.close();expect(errors).toEqual([]);}
});
browserTest('saved-view names retain sharing permissions and exact fleet selector payload',async()=>{
  const {browser,page,errors}=await openBrowser();try{
    await page.evaluate(()=>window.mount('saved'));await page.getByRole('button',{name:t('fleet.views.saveCurrent'),exact:true}).click();
    const name=await control(page,'fleet.views.name');expect(await name.evaluate(el=>el.required)).toBe(true);await name.fill('  Morning  ');
    const share=await control(page,'fleet.views.share','checkbox');await share.press('Space');expect(await share.isChecked()).toBe(true);
    await page.getByRole('button',{name:t('common.saveChanges'),exact:true}).press('Enter');await page.waitForFunction(()=>window.calls.requests.length===1);
    expect(await page.evaluate(()=>window.calls.requests[0].body)).toMatchObject({name:'Morning',is_shared:true,search:'Entry',selector:{version:1,expression:{op:'health',values:['up']}}});
    expect(await page.getByRole('button',{name:t('common.saving'),exact:true}).isDisabled()).toBe(true);await page.evaluate(()=>window.finish());await page.waitForFunction(()=>window.calls.refresh===1);
    await page.evaluate(()=>window.mount('saved',false));await page.getByRole('button',{name:t('fleet.views.saveCurrent'),exact:true}).click();expect(await page.getByRole('checkbox',{name:t('fleet.views.share'),exact:true}).count()).toBe(0);
  }finally{await browser.close();expect(errors).toEqual([]);}
});
browserTest('membership names preserve mixed selection, keyboard page toggles and off-page members',async()=>{
  const {browser,page,errors}=await openBrowser();try{
    await page.evaluate(()=>window.mount('members'));const search=await control(page,'collections.members.search','searchbox');await search.fill('Entry');
    const pageToggle=await control(page,'collections.members.selectPage','checkbox');expect(await pageToggle.evaluate(el=>el.indeterminate)).toBe(true);
    await pageToggle.press('Space');expect(await pageToggle.isChecked()).toBe(true);expect(await page.getByRole('checkbox',{name:'Rear',exact:true}).isChecked()).toBe(true);
    await page.getByRole('checkbox',{name:'Entrance',exact:true}).press('Space');expect(await pageToggle.evaluate(el=>el.indeterminate)).toBe(true);
    expect(await page.evaluate(()=>window.calls.changes)).toEqual([['cam-a','off-page','cam-b'],['off-page','cam-b']]);expect(await search.inputValue()).toBe('Entry');
  }finally{await browser.close();expect(errors).toEqual([]);}
});
browserTest('investigation action names preserve range/scope previews and protection busy guards',async()=>{
  const {browser,page,errors}=await openBrowser();try{
    await page.evaluate(()=>window.mount('actions'));await page.getByRole('button',{name:t('investigation.actions.open'),exact:true}).click();
    const dialog=page.getByRole('dialog');const result=await control(dialog,'investigation.actions.selectedResult','radio');expect(await result.isChecked()).toBe(true);
    const full=await control(dialog,'investigation.actions.fullWindow','radio');await full.press('Space');await page.waitForFunction(()=>window.calls.requests.length>=2);
    const first=dialog.getByLabel(t('investigation.start'),{exact:true});const last=dialog.getByLabel(t('investigation.end'),{exact:true});await first.fill('2026-10-31T12:00:15');await last.fill('2026-10-31T12:00:30');
    await dialog.getByRole('button',{name:t('investigation.actions.preview'),exact:true}).press('Enter');await page.waitForFunction(()=>window.calls.requests.length>=3);
    expect(await page.evaluate(()=>window.calls.requests.at(-1).body)).toEqual({camera_uuids:['cam-a'],start_time:1793448015,end_time:1793448030});
    await dialog.getByRole('button',{name:t('investigation.actions.protectButton'),exact:true}).press('Enter');await page.waitForFunction(()=>window.calls.requests.some(r=>r.url.endsWith('batch-protect')));
    for(const input of [result,full,first,last])expect(await input.isDisabled()).toBe(true);expect(await dialog.getByRole('button',{name:t('common.close'),exact:true}).isDisabled()).toBe(true);
    expect(await page.evaluate(()=>window.calls.requests.find(r=>r.url.endsWith('batch-protect')).body)).toEqual({ids:[1],protected:true});await page.evaluate(()=>window.finish());await dialog.getByRole('button',{name:t('common.close'),exact:true}).click();expect(await dialog.count()).toBe(0);
  }finally{await browser.close();expect(errors).toEqual([]);}
});
browserTest('bookmark names retain required title, exact trimmed payload and pending cancellation guard',async()=>{
  const {browser,page,errors}=await openBrowser();try{
    await page.evaluate(()=>window.mount('bookmarks'));await page.getByRole('button',{name:t('investigation.bookmarks.save'),exact:true}).click();const dialog=page.getByRole('dialog');
    const title=await control(dialog,'investigation.bookmarks.title');await title.fill('  Incident  ');await (await control(dialog,'investigation.bookmarks.note')).fill('  Details  ');
    await dialog.getByRole('button',{name:t('investigation.bookmarks.save'),exact:true}).press('Enter');await page.waitForFunction(()=>window.calls.requests.length===1);
    expect(await page.evaluate(()=>window.calls.requests[0].body)).toMatchObject({title:'Incident',note:'Details',camera_uuids:['cam-a'],cursor_time:1793448005,primary_camera_uuid:'cam-a'});
    await dialog.getByRole('button',{name:t('common.cancel'),exact:true}).click();expect(await dialog.count()).toBe(1);expect(await dialog.getByRole('button',{name:t('common.close'),exact:true}).isDisabled()).toBe(true);
    await page.evaluate(()=>window.finish());await dialog.waitFor({state:'detached'});
  }finally{await browser.close();expect(errors).toEqual([]);}
});
browserTest('investigation names retain camera selection, region limits and exact confidence/region search request',async()=>{
  const {browser,page,errors}=await openBrowser();try{
    await page.evaluate(()=>window.mount('view'));await page.locator('.investigation-track').waitFor();
    const picker=page.locator('.investigation-camera-picker');const camera=picker.getByRole('checkbox',{name:'Rear',exact:true});await camera.press('Space');expect(await camera.isChecked()).toBe(true);await camera.press('Space');expect(await camera.isChecked()).toBe(false);
    const query=page.locator('.investigation-query-card');expect(await query.getByLabel(t('investigation.start'),{exact:true}).count()).toBe(1);expect(await query.getByLabel(t('investigation.end'),{exact:true}).count()).toBe(1);
    await (await control(query,'investigation.minimumConfidence','spinbutton')).fill('.7');
    for(const [key,value] of [['regionLeft','15'],['regionTop','25'],['regionWidth','35'],['regionHeight','45'],['regionMinimumOverlap','75']])await (await control(query,'investigation.'+key,'spinbutton')).fill(value);
    await page.getByRole('button',{name:t('investigation.applyFilters'),exact:true}).press('Enter');await page.waitForFunction(()=>window.calls.requests.some(r=>r.url.endsWith('/search')&&r.body.filters.min_confidence===.7));
    expect(await page.evaluate(()=>window.calls.requests.filter(r=>r.url.endsWith('/search')&&!r.body.summary_only).at(-1).body)).toMatchObject({filters:{min_confidence:.7},region:{x:.15,y:.25,width:.35,height:.45,match:'minimum_intersection',min_intersection:.75}});
    const overlap=await control(query,'investigation.regionMinimumOverlap','spinbutton');await overlap.fill('101');await page.getByRole('button',{name:t('investigation.applyFilters'),exact:true}).press('Enter');
    expect(await page.evaluate(()=>window.calls.requests.filter(r=>r.url.endsWith('/search')).at(-1).body.region.min_intersection)).toBe(.75);
  }finally{await browser.close();expect(errors).toEqual([]);}
});
browserTest('playable investigation video has a camera-specific AX name and keeps shared controls and track activation',async()=>{
  const {browser,page,errors}=await openBrowser();try{
    await page.evaluate(()=>window.mount('view'));const video=page.locator('video');await video.waitFor();await page.waitForFunction(()=>document.querySelector('video')?.readyState>=2);
    const session=await page.context().newCDPSession(page);const {root}=await session.send('DOM.getDocument');const {nodeId}=await session.send('DOM.querySelector',{nodeId:root.nodeId,selector:'video'});const {nodes:ax}=await session.send('Accessibility.getPartialAXTree',{nodeId,fetchRelatives:false});
    expect(ax[0].ignored).toBe(false);expect(ax[0].name?.value).toBe('Entrance '+t('timeline.timelinePlayback'));await session.detach();expect(await video.evaluate(el=>el.controls)).toBe(false);
    const controls=page.getByRole('region',{name:t('investigation.playbackControls'),exact:true});await controls.getByRole('button',{name:/translated:investigation\.play$/}).press('Enter');await controls.getByRole('button',{name:/translated:investigation\.pause$/}).waitFor();await controls.getByRole('button',{name:/translated:investigation\.pause$/}).press('Space');
    const active=page.locator('.investigation-track').getByRole('checkbox',{name:'Entrance',exact:true});await active.press('Space');await video.waitFor({state:'detached'});await active.press('Space');await video.waitFor();expect(await active.isChecked()).toBe(true);
  }finally{await browser.close();expect(errors).toEqual([]);}
});
