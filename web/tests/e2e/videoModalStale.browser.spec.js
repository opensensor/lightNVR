/** Resolve response bodies after navigation even when cancellation is ignored. */
const path = require('node:path');

const FIXTURE = `
  import {h,render} from 'preact';
  import {VideoModal} from './js/components/preact/UI.tsx';
  window.requests=[];
  window.fetch=async(url,options={})=>{
    const request={url:String(url),signal:options.signal};
    window.requests.push(request);
    return {ok:true,status:200,headers:new Headers({'Content-Type':'application/json'}),
      json:()=>new Promise((resolve,reject)=>{request.resolve=resolve;request.reject=reject;})};
  };
  const close=()=>window.show(null);
  window.show=id=>render(h(VideoModal,{isOpen:id!==null,onClose:close,
    videoUrl:id===null?undefined:'/api/recordings/play/'+id,title:'Recording'}),document.getElementById('root'));
`;

let browser, page, bundle;
beforeAll(async()=>{
  const {chromium}=require('../../../node_modules/playwright');
  const {build}=require('esbuild');
  const result=await build({
    stdin:{resolveDir:path.resolve(__dirname,'../..'),loader:'jsx',contents:FIXTURE},
    bundle:true,write:false,format:'iife',jsx:'automatic',jsxImportSource:'preact',
    alias:{react:'preact/compat','react-dom':'preact/compat'},
    plugins:[{name:'fixture-i18n',setup(builder){
      builder.onResolve({filter:/(?:\/i18n|ToastContainer)\.tsx?$/},args=>({path:args.path,namespace:'fixture'}));
      builder.onLoad({filter:/.*/,namespace:'fixture'},()=>({loader:'js',contents:
        "export const t=(key,params)=>params?key+JSON.stringify(params):key;export const useI18n=()=>({t});export const getTextDirection=()=>'ltr';export const showStatusMessage=()=>{};export const showSuccessToast=()=>{};export const showErrorToast=()=>{};export const showWarningToast=()=>{};export const showInfoToast=()=>{};"}));
    }}],
  });
  bundle=result.outputFiles[0].text;
  browser=await chromium.launch({headless:true});
},60000);
beforeEach(async()=>{
  page=await browser.newPage();page.setDefaultTimeout(5000);
  await page.route('http://fixture.invalid/**',route=>route.fulfill({contentType:'text/html',body:'<div id="root"></div>'}));
  await page.goto('http://fixture.invalid/');await page.addScriptTag({content:bundle});
});
afterEach(async()=>{await page?.close();});
afterAll(async()=>{await browser?.close();});

const recording=id=>({id,stream:'Gate'+id,start_time:1700000000,end_time:1700000060,protected:id===1});
const detectionBody=count=>({timestamp:'2026-11-07T10:00:00Z',stream_status:'Running',detections:
  Array.from({length:count},()=>({label:'person',confidence:.9,x:0,y:0,width:.5,height:.5,timestamp:1700000001,end_timestamp:1700000002}))});
async function pending(fragment){
  await page.waitForFunction(part=>window.requests.some(r=>r.url.includes(part)&&r.resolve),fragment);
}
async function show(id){
  await page.evaluate(id=>window.show(id),id);
  if(id!==null)await pending('/api/recordings/'+id);
}
async function release(fragment,body){
  await page.evaluate(([part,value])=>window.requests.find(r=>r.url.includes(part)).resolve(value),[fragment,body]);
  await page.evaluate(()=>new Promise(resolve=>requestAnimationFrame(()=>setTimeout(resolve))));
}
async function reject(fragment){
  await pending(fragment);
  await page.evaluate(part=>window.requests.find(r=>r.url.includes(part)).reject(new Error('Body interrupted')),fragment);
  // Let the rejection reach the component and its state update render before
  // checking that a stale error did not replace the current success message.
  await page.evaluate(()=>new Promise(resolve=>requestAnimationFrame(()=>setTimeout(resolve))));
}
async function assertCurrent(){
  await page.waitForFunction(()=>document.body.textContent.includes('Gate2 -'));
  expect(await page.getByTitle('recordings.protect').isEnabled()).toBe(true);
  expect(await page.getByTitle('recordings.unprotect').count()).toBe(0);
}

test('late metadata cannot overwrite the current recording or start stale detections',async()=>{
  await show(1);await show(2);
  expect(await page.evaluate(()=>window.requests[0].signal.aborted)).toBe(true);
  await release('/api/recordings/2',recording(2));
  await pending('/api/detection/results/Gate2');
  await release('/api/detection/results/Gate2',detectionBody(0));
  await release('/api/recordings/1',recording(1));
  await assertCurrent();
  expect(await page.evaluate(()=>window.requests.some(r=>r.url.includes('/Gate1')))).toBe(false);
});

test('late detections cannot replace the new recording after URL change',async()=>{
  await show(1);await release('/api/recordings/1',recording(1));
  await pending('/api/detection/results/Gate1');
  await show(2);
  expect(await page.getByTitle('recordings.protect').isDisabled()).toBe(true);
  expect(await page.getByTitle('recordings.viewInTimelineTitle').count()).toBe(0);
  expect(await page.evaluate(()=>window.requests.find(r=>r.url.includes('/Gate1')).signal.aborted)).toBe(true);
  await release('/api/recordings/2',recording(2));
  await pending('/api/detection/results/Gate2');
  await release('/api/detection/results/Gate2',detectionBody(0));
  await release('/api/detection/results/Gate1',detectionBody(1));
  await assertCurrent();
  expect(await page.getByText('recordings.noDetectionsForRecording',{exact:true}).count()).toBe(1);
  expect(await page.getByText(/recordings.detectionAvailableOne/).count()).toBe(0);
});

test('close cancels detections and reopening does not retain old metadata',async()=>{
  await show(1);await release('/api/recordings/1',recording(1));
  await pending('/api/detection/results/Gate1');
  await show(null);
  await page.locator('#video-preview-modal').waitFor({state:'detached'});
  await page.waitForFunction(()=>window.requests.find(r=>r.url.includes('/Gate1')).signal.aborted);
  await release('/api/detection/results/Gate1',detectionBody(1));
  await show(2);
  expect(await page.getByTitle('recordings.viewInTimelineTitle').count()).toBe(0);
  expect(await page.getByTitle('recordings.protect').isDisabled()).toBe(true);
  expect(await page.getByText(/recordings.detectionAvailableOne/).count()).toBe(0);
});

test('current detection failure is visible and switching recordings recovers',async()=>{
  await show(1);await release('/api/recordings/1',recording(1));
  await reject('/api/detection/results/Gate1');
  await page.getByText('recordings.errorLoadingDetections',{exact:true}).waitFor();

  await show(2);await release('/api/recordings/2',recording(2));
  await pending('/api/detection/results/Gate2');
  await release('/api/detection/results/Gate2',detectionBody(0));
  await page.getByText('recordings.noDetectionsForRecording',{exact:true}).waitFor();
  expect(await page.getByText('recordings.errorLoadingDetections',{exact:true}).count()).toBe(0);
  await assertCurrent();
});

test('stale metadata failure cannot overwrite the current success',async()=>{
  await show(1);await show(2);
  await release('/api/recordings/2',recording(2));
  await pending('/api/detection/results/Gate2');
  await release('/api/detection/results/Gate2',detectionBody(0));
  await page.getByText('recordings.noDetectionsForRecording',{exact:true}).waitFor();

  await reject('/api/recordings/1');
  expect(await page.getByText('recordings.noDetectionsForRecording',{exact:true}).count()).toBe(1);
  expect(await page.getByText('recordings.errorLoadingDetections',{exact:true}).count()).toBe(0);
  expect(await page.evaluate(()=>window.requests.some(r=>r.url.includes('/Gate1')))).toBe(false);
  await assertCurrent();
});

test('stale detection failure cannot overwrite the current success',async()=>{
  await show(1);await release('/api/recordings/1',recording(1));
  await pending('/api/detection/results/Gate1');

  await show(2);await release('/api/recordings/2',recording(2));
  await pending('/api/detection/results/Gate2');
  await release('/api/detection/results/Gate2',detectionBody(0));
  await page.getByText('recordings.noDetectionsForRecording',{exact:true}).waitFor();

  await reject('/api/detection/results/Gate1');
  expect(await page.getByText('recordings.noDetectionsForRecording',{exact:true}).count()).toBe(1);
  expect(await page.getByText('recordings.errorLoadingDetections',{exact:true}).count()).toBe(0);
  await assertCurrent();
});
