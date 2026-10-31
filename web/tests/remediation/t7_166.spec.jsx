/** Stable names retain native auth/download/search/media behavior. */
const mockHooks={cursor:0,values:{},setters:[]};
const mockZoom={isZoomed:false,scale:1,reset:jest.fn(),containerRef:{},touchAction:'auto'};
jest.mock('preact/hooks',()=>({
  useState:initial=>{const index=mockHooks.cursor++;const setter=jest.fn();mockHooks.setters[index]=setter;return [Object.hasOwn(mockHooks.values,index)?mockHooks.values[index]:typeof initial==='function'?initial():initial,setter];},
  useEffect:jest.fn(),useLayoutEffect:jest.fn(),useMemo:fn=>fn(),useCallback:fn=>fn,useRef:value=>({current:value}),useImperativeHandle:jest.fn(),useId:()=> 'fixture',
}));
jest.mock('../../js/i18n.js',()=>({t:key=>'translated:'+key,useI18n:()=>({t:key=>'translated:'+key,locale:'en'})}));
jest.mock('../../js/query-client.js',()=>({fetchJSON:jest.fn(),useQuery:()=>({data:{},isLoading:false,refetch:jest.fn()}),useQueryClient:()=>({invalidateQueries:jest.fn()})}));
jest.mock('../../js/components/preact/ToastContainer.jsx',()=>({showStatusMessage:jest.fn()}));
jest.mock('../../js/components/preact/useVideoZoom.js',()=>({useVideoZoom:()=>mockZoom}));
jest.mock('../../js/components/preact/useAutoRetry.js',()=>({useAutoRetry:()=>null}));
jest.mock('../../js/components/preact/MobileTileGestures.jsx',()=>({useMobileTileGestures:()=>({onClick:()=>false,chromeVisible:true}),MobileTileContextMenu:()=>null}));
jest.mock('hls.js',()=>({}));
const {RequiredPasswordChange}=require('../../js/components/preact/AuthGate.jsx');
const {BatchDownloadModal}=require('../../js/components/preact/BatchDownloadModal.jsx');
const {DetectionOverlay}=require('../../js/components/preact/DetectionOverlay.jsx');
const {FleetView}=require('../../js/components/preact/FleetView.jsx');
const {FullscreenButton}=require('../../js/components/preact/FullscreenManager.jsx');
const {HLSVideoCell}=require('../../js/components/preact/HLSVideoCell.jsx');
const t=key=>'translated:'+key;
const stream={name:'Entrance',enabled:true,status:'Running',can_control_privacy:true,detection_based_recording:true,detection_model:'fixture'};
function nodes(node){if(Array.isArray(node))return node.flatMap(nodes);return !node||typeof node!=='object'?[]:[node,...nodes(node.props?.children)];}
function render(Component,props={},values={}){mockHooks.cursor=0;mockHooks.values=values;mockHooks.setters=[];return Component(props);}
function named(node,name){expect(node).toBeDefined();expect(node.props['aria-label']).toBe(name);}
function change(node,value='fixture'){const target={value,checked:true};(node.props.onInput||node.props.oninput||node.props.onChange)({target,currentTarget:target});}
beforeEach(()=>{jest.clearAllMocks();mockZoom.isZoomed=false;});
it('required password fields retain names, autocomplete, length and pending disabled states',()=>{
  const tree=render(RequiredPasswordChange,{session:{id:7}},{4:true});const fields=nodes(tree).filter(node=>node.type==='input');expect(fields).toHaveLength(3);
  ['currentPassword','newPassword','confirmPassword'].forEach((key,index)=>{named(fields[index],t('passwordChangeRequired.'+key));expect(fields[index].props.disabled).toBe(true);change(fields[index]);expect(mockHooks.setters[index]).toHaveBeenCalledWith('fixture');});
  expect(fields[0].props.autoComplete).toBe('current-password');expect(fields[1].props.minLength).toBe('8');expect(fields[2].props.autoComplete).toBe('new-password');
});
it('download mode/filename names retain selection, suffix input and busy/empty guards',()=>{
  let tree=render(BatchDownloadModal,{isOpen:true,selectedIds:{1:true},recordings:[{id:1}]},{2:true});const fields=nodes(tree).filter(node=>node.type==='input');expect(fields).toHaveLength(3);
  ['recordings.zipArchiveRecommended','recordings.sequentialDownloads','recordings.zipFilename'].forEach((key,index)=>named(fields[index],t(key)));
  change(fields[1]);expect(mockHooks.setters[0]).toHaveBeenCalledWith('sequential');change(fields[2],'evidence');expect(mockHooks.setters[1]).toHaveBeenCalledWith('evidence');expect(fields[2].props.disabled).toBe(true);
  tree=render(BatchDownloadModal,{isOpen:true,selectedIds:{},recordings:[]});expect(nodes(tree).find(node=>node.type==='button'&&node.props.children===t('common.download')).props.disabled).toBe(true);
  expect(render(BatchDownloadModal,{isOpen:false})).toBeNull();
});
it.each([false,true])('mounted detection graphic remains named and pointer-transparent while polling enabled=%s',enabled=>{
  const tree=render(DetectionOverlay,{streamName:'Entrance',videoRef:{current:null},enabled,detectionModel:'fixture'});
  const canvas=nodes(tree).find(node=>node.type==='canvas');expect(canvas).toBeDefined();const graphic=nodes(tree).find(node=>node.props?.role==='img');expect(graphic).not.toBe(canvas);named(graphic,'Entrance '+t('recordings.detections'));expect(canvas.props['aria-hidden']).toBe('true');expect(graphic.props.style).toMatchObject({position:'absolute',inset:0,pointerEvents:'none',zIndex:2});
  expect(canvas.ref).toBeDefined();expect(canvas.props.style).toMatchObject({position:'absolute',pointerEvents:'none',width:'100%',height:'100%',zIndex:2});
});
it('fleet search name retains its label ID, input value and page-reset update',()=>{
  const tree=render(FleetView,{});const input=nodes(tree).find(node=>node.props?.id==='fleet-search');named(input,t('fleet.searchLabel'));change(input,'Rear');
  expect(mockHooks.setters[0].mock.calls[0][0]({page:3,search:'old'})).toMatchObject({page:1,search:'Rear'});
});
it('fullscreen action has a localized name and dispatches once without changing the hidden state',()=>{
  const onToggle=jest.fn();const button=render(FullscreenButton,{isFullscreen:false,onToggle});named(button,t('live.toggleFullscreen'));button.props.onClick();expect(onToggle).toHaveBeenCalledTimes(1);expect(button.props.onKeyDown).toBeUndefined();
  expect(render(FullscreenButton,{isFullscreen:true,onToggle})).toBeNull();
});
it('HLS video/privacy names retain media properties and the separate confirmation transition',()=>{
  const tree=render(HLSVideoCell,{stream,streamId:'entry',onToggleFullscreen:jest.fn()});const video=nodes(tree).find(node=>node.type==='video');named(video,'Entrance '+t('live.stream'));
  expect(video.props.autoPlay).toBe(true);expect(video.props.playsInline).toBe(true);expect(video.props.controls).toBeUndefined();expect(video.props.muted).toBe(true);
  const privacy=nodes(tree).find(node=>node.props?.title===t('live.pauseForPrivacy'));named(privacy,t('live.pauseForPrivacy'));privacy.props.onClick();expect(mockHooks.setters[6]).toHaveBeenCalledWith(true);
  const restricted=render(HLSVideoCell,{stream:{...stream,can_control_privacy:false},streamId:'entry',onToggleFullscreen:jest.fn()});expect(nodes(restricted).find(node=>node.props?.title===t('live.pauseForPrivacy'))).toBeUndefined();
});
it.each([false,true])('HLS detection visibility follows its existing global toggle=%s',visible=>{
  const tree=render(HLSVideoCell,{stream,streamId:'entry',onToggleFullscreen:jest.fn(),globalShowDetections:visible});
  expect(nodes(tree).filter(node=>node.type===DetectionOverlay)).toHaveLength(visible?1:0);
});
async function openBrowser(){
  const {chromium}=require('../../../node_modules/playwright');const {build}=require('esbuild');const fs=require('node:fs');const path=require('node:path');const webRoot=path.resolve(__dirname,'../..');
  const built=await build({stdin:{resolveDir:webRoot,loader:'jsx',contents:`
    import {h,render} from 'preact';import {useRef,useState} from 'preact/hooks';
    import {RequiredPasswordChange} from './js/components/preact/AuthGate.jsx';
    import {BatchDownloadModal} from './js/components/preact/BatchDownloadModal.jsx';
    import {DetectionOverlay} from './js/components/preact/DetectionOverlay.jsx';
    import {FleetView} from './js/components/preact/FleetView.jsx';
    import {FullscreenButton} from './js/components/preact/FullscreenManager.jsx';
    import {HLSVideoCell} from './js/components/preact/HLSVideoCell.jsx';
    const root=document.getElementById('root');window.options={playing:true};
    const zone={name:'Entrance zone',enabled:true,color:'#3b82f6',polygon:[{x:.2,y:.2},{x:.8,y:.2},{x:.8,y:.8},{x:.2,y:.8}]};
    window.fetch=async(url,options={})=>{
      window.calls.requests.push({url:String(url),method:options.method||'GET',body:options.body?JSON.parse(options.body):null});
      if(String(url).endsWith('/zones'))return {ok:true,json:async()=>({zones:[zone]})};
      if(String(url).includes('/batch-download/status/'))return {ok:true,json:async()=>({status:'complete',current:2})};
      if(String(url).endsWith('/batch-download'))return new Promise(resolve=>window.finish=()=>resolve({ok:true,json:async()=>({token:'fixture',total:2})}));
      return new Promise(resolve=>window.finish=()=>resolve({ok:true,json:async()=>({})}));
    };
    document.addEventListener('click',event=>{const anchor=event.target.closest?.('a[download]');if(anchor){event.preventDefault();window.calls.downloads.push({href:anchor.getAttribute('href'),name:anchor.download});}},true);
    window.fixtureQuery=arg=>{const key=Array.isArray(arg)?arg:arg.queryKey;if(key[0]==='fleet-cameras'){window.calls.queries.push(key[1]);return {data:{cameras:[],total:0,total_pages:5,facets:{}},isLoading:false,refetch:()=>{}};}return {data:{},isLoading:false,refetch:()=>{}};};
    window.fixtureZoom={isZoomed:false,scale:1,containerRef:{},touchAction:'auto',reset:()=>{}};
    window.fixtureMobile={chromeVisible:true,onClick:()=>false,onPointerDown:()=>{},onPointerUp:()=>{},onPointerMove:()=>{},onPointerCancel:()=>{},onContextMenu:()=>{}};
    function Overlay(){const video=useRef(null);return h('div',{style:{position:'relative',width:'320px',height:'180px'}},h('video',{ref:video,src:'/fixture.mp4',muted:true,style:{width:'320px',height:'180px'}}),h(DetectionOverlay,{streamName:'Entrance',videoRef:video,enabled:false,detectionModel:'fixture'}));}
    function FullscreenHost(){const [active,setActive]=useState(false);return h(FullscreenButton,{isFullscreen:active,onToggle:()=>{window.calls.fullscreen++;setActive(true);}});}
    window.mount=(kind,options={})=>{
      render(null,root);window.options={playing:true,...options};window.calls={requests:[],queries:[],downloads:[],close:0,fullscreen:0,invalidations:0};
      if(kind==='auth')return render(h(RequiredPasswordChange,{session:{id:7}}),root);
      if(kind==='download')return render(h(BatchDownloadModal,{isOpen:true,onClose:()=>window.calls.close++,recordings:[{id:1,stream:'Entrance'},{id:2,stream:'Rear'}],selectedIds:{1:true,2:true}}),root);
      if(kind==='overlay')return render(h(Overlay),root);
      if(kind==='fullscreen')return render(h(FullscreenHost),root);
      if(kind==='fleet'){history.replaceState({},'','/?page=3');return render(h(FleetView),root);}
      const stream={name:'Entrance',enabled:true,status:'Running',can_control_privacy:options.privacy!==false,detection_based_recording:true,detection_model:'fixture'};
      return render(h('div',{style:{width:'640px',height:'360px',position:'relative'}},h(HLSVideoCell,{stream,streamId:'entry',globalShowDetections:options.detections!==false,onToggleFullscreen:()=>window.calls.fullscreen++})),root);
    };
  `},bundle:true,write:false,format:'iife',jsx:'automatic',jsxImportSource:'preact',plugins:[{name:'fixtures',setup(builder){
    // The transport/state fixture leaves the real rendered media, handlers, privacy
    // confirmation, DetectionOverlay and its drawing effects under test.
    builder.onLoad({filter:/HLSVideoCell\.jsx$/},args=>{let source=fs.readFileSync(args.path,'utf8');source=source.replace('useState, useEffect, useRef','useState, useRef');source='const useEffect=()=>{};\n'+source;source=source.replace('useState(true);','useState(false);').replace('const [isPlaying, setIsPlaying] = useState(false)','const [isPlaying, setIsPlaying] = useState(Boolean(window.options.playing))');return {contents:source,loader:'jsx',resolveDir:path.dirname(args.path)};});
    builder.onResolve({filter:/(?:i18n|query-client|auth-utils|useVideoZoom|useAutoRetry|MobileTileGestures|ToastContainer|detection-polling)\.(?:js|jsx)$/},args=>({path:args.path,namespace:'service'}));
    builder.onLoad({filter:/.*/,namespace:'service'},()=>({loader:'js',contents:`
      export const t=key=>'translated:'+key;export const useI18n=()=>({t,locale:'en'});export const fetchJSON=async()=>({});export const useQuery=arg=>window.fixtureQuery(arg);
      export const useQueryClient=()=>({invalidateQueries:()=>window.calls.invalidations++});export const useMutation=()=>({});export const usePostMutation=()=>({});
      export const clearAuthState=()=>sessionStorage.setItem('fixture-auth-cleared','yes');export const redirectToLogin=()=>{};export const validateSession=async()=>({valid:true,role:'viewer'});
      export const useVideoZoom=()=>window.fixtureZoom;export const useAutoRetry=()=>null;export const useMobileTileGestures=()=>window.fixtureMobile;export const MobileTileContextMenu=()=>null;
      export const startDetectionPolling=()=>()=>{};export const showStatusMessage=()=>{};`}));
    builder.onResolve({filter:/(?:SnapshotManager|ManualRecordingButton|PTZControls|DayNightControl|UI|LiveTileStatus|PictureInPictureButton|TileAudioButton|FisheyeEptzCanvas)\.jsx$/},args=>({path:args.path,namespace:'child'}));
    builder.onLoad({filter:/.*/,namespace:'child'},()=>({loader:'js',contents:`export const SnapshotButton=()=>null;export const ManualRecordingButton=()=>null;export const PTZControls=()=>null;export const DayNightControl=()=>null;export const ConfirmDialog=()=>null;export const LiveTileStatus=()=>null;export const PictureInPictureButton=()=>null;export const TileAudioButton=()=>null;export const FisheyeEptzCanvas=()=>null;`}));
  }}]});
  const browser=await chromium.launch({headless:true});try{
    const page=await browser.newPage({viewport:{width:1440,height:1100}});page.setDefaultTimeout(5000);const errors=[];page.on('pageerror',error=>errors.push(error.message));
    await page.route('http://fixture.invalid/**',route=>route.fulfill({contentType:'text/html',body:'<div id="root"></div>'}));await page.route('http://fixture.invalid/fixture.mp4',route=>route.fulfill({contentType:'video/mp4',body:fs.readFileSync(path.resolve(webRoot,'../tests/integration/fixtures/recording.mp4'))}));
    await page.goto('http://fixture.invalid/');const styles=new Set();for(const file of ['index.html','streams.html','recordings.html','login.html']){const html=fs.readFileSync(path.join(webRoot,'dist',file),'utf8');for(const tag of html.match(/<link[^>]*rel="stylesheet"[^>]*>/g)||[])styles.add(tag.match(/href="([^"]+)"/)[1]);}
    expect(styles.size).toBeGreaterThan(0);for(const href of styles)await page.addStyleTag({content:fs.readFileSync(path.join(webRoot,'dist',href),'utf8')});await page.addScriptTag({content:built.outputFiles[0].text});return {browser,page,errors};
  }catch(error){await browser.close();throw error;}
}
const browserTest=process.env.T7_166_BROWSER==='1'?it:it.skip;
browserTest('password names preserve mismatch validation, exact request, busy fields and successful redirect',async()=>{
  const {browser,page,errors}=await openBrowser();try{
    await page.evaluate(()=>window.mount('auth'));const old=page.getByLabel(t('passwordChangeRequired.currentPassword'),{exact:true});const password=page.getByLabel(t('passwordChangeRequired.newPassword'),{exact:true});const confirmation=page.getByLabel(t('passwordChangeRequired.confirmPassword'),{exact:true});
    await old.fill('Synthetic-old-42');await password.fill('Synthetic-new-42');await confirmation.fill('Synthetic-other-42');await page.getByRole('button',{name:t('passwordChangeRequired.submit'),exact:true}).press('Enter');await page.getByRole('alert').waitFor();expect(await page.evaluate(()=>window.calls.requests)).toEqual([]);
    await confirmation.fill('Synthetic-new-42');await page.getByRole('button',{name:t('passwordChangeRequired.submit'),exact:true}).press('Enter');await page.waitForFunction(()=>window.calls.requests.length===1);
    expect(await page.evaluate(()=>window.calls.requests[0])).toEqual({url:'/api/auth/users/7/password',method:'PUT',body:{old_password:'Synthetic-old-42',new_password:'Synthetic-new-42'}});for(const field of [old,password,confirmation])expect(await field.isDisabled()).toBe(true);
    await Promise.all([page.waitForURL('**/login.html?password_changed=true'),page.evaluate(()=>window.finish())]);expect(await page.evaluate(()=>sessionStorage.getItem('fixture-auth-cleared'))).toBe('yes');
  }finally{await browser.close();expect(errors).toEqual([]);}
});
browserTest('batch download names preserve native mode switching, ZIP suffix/payload, busy controls and sequential filenames',async()=>{
  const {browser,page,errors}=await openBrowser();try{
    await page.evaluate(()=>window.mount('download'));const sequential=page.getByRole('radio',{name:t('recordings.sequentialDownloads'),exact:true});const zip=page.getByRole('radio',{name:t('recordings.zipArchiveRecommended'),exact:true});await sequential.press('Space');expect(await sequential.isChecked()).toBe(true);expect(await page.getByRole('textbox',{name:t('recordings.zipFilename'),exact:true}).count()).toBe(0);
    await zip.press('Space');const filename=page.getByRole('textbox',{name:t('recordings.zipFilename'),exact:true});await filename.fill('  incident  ');await page.getByRole('button',{name:t('common.download'),exact:true}).press('Enter');await page.waitForFunction(()=>window.calls.requests.length===1);
    expect(await page.evaluate(()=>window.calls.requests[0].body)).toEqual({ids:[1,2],filename:'incident.zip'});expect(await filename.isDisabled()).toBe(true);expect(await page.getByRole('button',{name:t('common.cancel'),exact:true}).isDisabled()).toBe(true);
    await page.evaluate(()=>window.finish());await page.waitForFunction(()=>window.calls.close===1);expect(await page.evaluate(()=>window.calls.downloads)).toEqual([{href:'/api/recordings/batch-download/result/fixture',name:'incident.zip'}]);
    await page.evaluate(()=>window.mount('download'));await sequential.press('Space');await page.getByRole('button',{name:t('common.download'),exact:true}).press('Enter');await page.waitForFunction(()=>window.calls.close===1);
    expect(await page.evaluate(()=>window.calls.downloads)).toEqual([{href:'/api/recordings/download/1',name:'Entrance_1.mp4'},{href:'/api/recordings/download/2',name:'Rear_2.mp4'}]);
  }finally{await browser.close();expect(errors).toEqual([]);}
});
browserTest('mounted detection canvas has a meaningful graphic name and still draws zones with polling paused',async()=>{
  const {browser,page,errors}=await openBrowser();try{
    await page.evaluate(()=>window.mount('overlay'));await page.waitForFunction(()=>document.querySelector('video')?.readyState>=2);await page.waitForFunction(()=>window.calls.requests.some(r=>r.url.endsWith('/zones')));
    const graphic=page.getByRole('img',{name:'Entrance '+t('recordings.detections'),exact:true});await graphic.waitFor();const canvas=graphic.locator('canvas[aria-hidden="true"]');await canvas.waitFor();await page.evaluate(()=>window.dispatchEvent(new Event('resize')));
    expect(await canvas.evaluate(el=>getComputedStyle(el).pointerEvents)).toBe('none');expect(await canvas.evaluate(el=>[el.width,el.height])).toEqual([320,180]);expect(await graphic.boundingBox()).toEqual(await canvas.boundingBox());expect(await canvas.evaluate(el=>{const r=el.getBoundingClientRect();return document.elementFromPoint(r.x+r.width/2,r.y+r.height/2)?.tagName;})).toBe('VIDEO');
    await page.waitForFunction(()=>{const c=document.querySelector('canvas');return c&&c.getContext('2d').getImageData(160,90,1,1).data[3]>0;});
  }finally{await browser.close();expect(errors).toEqual([]);}
});
browserTest('fleet search remains named, debounced and resets the URL/query page',async()=>{
  const {browser,page,errors}=await openBrowser();try{
    await page.evaluate(()=>window.mount('fleet'));const search=page.getByRole('searchbox',{name:t('fleet.searchLabel'),exact:true});await search.fill('Rear');await page.waitForFunction(()=>window.calls.queries.some(query=>query.search==='Rear'));
    expect(await page.evaluate(()=>window.calls.queries.at(-1))).toMatchObject({search:'Rear',page:1});expect(new URL(page.url()).searchParams.get('q')).toBe('Rear');expect(new URL(page.url()).searchParams.get('page')).toBeNull();
  }finally{await browser.close();expect(errors).toEqual([]);}
});
browserTest('fullscreen icon has its localized name without title and keyboard activation dispatches once',async()=>{
  const {browser,page,errors}=await openBrowser();try{
    await page.evaluate(()=>window.mount('fullscreen'));await page.locator('[title]').evaluateAll(nodes=>nodes.forEach(node=>node.removeAttribute('title')));const button=page.getByRole('button',{name:t('live.toggleFullscreen'),exact:true});await button.press('Enter');expect(await page.evaluate(()=>window.calls.fullscreen)).toBe(1);expect(await button.count()).toBe(0);
    await page.evaluate(()=>window.mount('fullscreen'));await button.press('Space');expect(await page.evaluate(()=>window.calls.fullscreen)).toBe(1);expect(await button.count()).toBe(0);
  }finally{await browser.close();expect(errors).toEqual([]);}
});
browserTest('HLS video exposes its playable AX name and the named privacy action keeps confirmation/permission boundaries',async()=>{
  const {browser,page,errors}=await openBrowser();try{
    await page.evaluate(()=>window.mount('hls',{detections:false}));const video=page.locator('video');await video.evaluate(el=>{el.src='/fixture.mp4';el.load();});await page.waitForFunction(()=>document.querySelector('video')?.readyState>=2);
    const session=await page.context().newCDPSession(page);const {root}=await session.send('DOM.getDocument');const {nodeId}=await session.send('DOM.querySelector',{nodeId:root.nodeId,selector:'video'});const {nodes:ax}=await session.send('Accessibility.getPartialAXTree',{nodeId,fetchRelatives:false});expect(ax[0].name?.value).toBe('Entrance '+t('live.stream'));await session.detach();expect(await video.evaluate(el=>el.controls)).toBe(false);
    await page.locator('[title]').evaluateAll(nodes=>nodes.forEach(node=>node.removeAttribute('title')));const privacy=page.getByRole('button',{name:t('live.pauseForPrivacy'),exact:true});await privacy.press('Space');await page.getByText(t('live.pauseForPrivacyConfirm'),{exact:true}).waitFor();expect(await page.evaluate(()=>window.calls.requests.length)).toBe(0);
    await page.getByRole('button',{name:t('common.cancel'),exact:true}).click();await privacy.press('Enter');const confirm=page.getByRole('button',{name:t('live.pauseForPrivacy'),exact:true}).last();await confirm.press('Enter');await page.waitForFunction(()=>window.calls.requests.length===1);expect(await confirm.isDisabled()).toBe(true);
    expect(await page.evaluate(()=>window.calls.requests[0])).toMatchObject({url:'/api/streams/Entrance',method:'PUT',body:{set_privacy_mode:true}});await page.evaluate(()=>window.finish());await page.getByText(t('live.streamPausedForPrivacy'),{exact:true}).waitFor();expect(await page.evaluate(()=>window.calls.invalidations)).toBe(1);expect(await page.evaluate(()=>window.calls.fullscreen)).toBe(0);
    await page.evaluate(()=>window.mount('hls',{privacy:false,detections:false}));expect(await page.getByRole('button',{name:t('live.pauseForPrivacy'),exact:true}).count()).toBe(0);expect(await page.locator('canvas.detection-overlay').count()).toBe(0);
  }finally{await browser.close();expect(errors).toEqual([]);}
});
browserTest('HLS only exposes the named detection graphic while the existing overlay toggle renders it',async()=>{
  const {browser,page,errors}=await openBrowser();try{
    await page.evaluate(()=>window.mount('hls',{privacy:false}));const graphic=page.getByRole('img',{name:'Entrance '+t('recordings.detections'),exact:true});await graphic.waitFor();
    await page.getByRole('button',{name:t('live.hideDetections'),exact:true}).press('Enter');await graphic.waitFor({state:'detached'});await page.getByRole('button',{name:t('live.showDetections'),exact:true}).press('Space');await graphic.waitFor();
  }finally{await browser.close();expect(errors).toEqual([]);}
});
