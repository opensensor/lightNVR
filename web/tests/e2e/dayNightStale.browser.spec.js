/** Day/night requests must remain owned by the camera that started them. */
const path = require('node:path');
const {validateDaynightResponse, validateDaynightUpdated} = require('../../generated/api/validators.js');

const FIXTURE = `
  import {h,render} from 'preact';
  import {DayNightControl} from './js/components/preact/DayNightControl.tsx';
  window.requests=[];
  window.toasts=[];
  window.fetch=async(url,options={})=>{
    const request={url:String(url),method:options.method,body:options.body,signal:options.signal};
    window.requests.push(request);
    // Deliberately ignore cancellation while the response body is pending.
    return {ok:true,status:200,headers:new Headers({'Content-Type':'application/json'}),
      json:()=>new Promise((resolve,reject)=>{request.resolve=resolve;request.reject=reject;})};
  };
  window.show=(name,isOnvif=true)=>render(h(DayNightControl,{stream:name?{name,isOnvif}:null}),document.getElementById('root'));
  window.unmount=()=>render(null,document.getElementById('root'));
`;
let browser, page, bundle;
beforeAll(async()=>{
  const {chromium}=require('../../../node_modules/playwright');
  const {build}=require('esbuild');
  const output=await build({
    stdin:{resolveDir:path.resolve(__dirname,'../..'),loader:'jsx',contents:FIXTURE},
    bundle:true,write:false,format:'iife',jsx:'automatic',jsxImportSource:'preact',
    plugins:[{name:'fixture',setup(builder){
      builder.onResolve({filter:/(?:\/i18n|ToastContainer)\.tsx?$/},args=>({path:args.path,namespace:'fixture'}));
      builder.onLoad({filter:/.*/,namespace:'fixture'},()=>({loader:'js',contents:
        "export const useI18n=()=>({t:(key)=>key});export const showStatusMessage=(...args)=>window.toasts.push(args);"}));
    }}],
  });
  bundle=output.outputFiles[0].text;
  browser=await chromium.launch({headless:true});
},60000);
beforeEach(async()=>{
  page=await browser.newPage();page.setDefaultTimeout(5000);
  await page.route('http://fixture.invalid/**',route=>route.fulfill({contentType:'text/html',body:'<div id="root" style="margin-top:200px"></div>'}));
  await page.goto('http://fixture.invalid/');await page.addScriptTag({content:bundle});
});
afterEach(async()=>{await page?.close();});
afterAll(async()=>{await browser?.close();});

const modeButton=mode=>page.getByRole('button',{name:`live.dayNightMode_${mode}`,exact:true});
const settle=()=>page.evaluate(()=>new Promise(resolve=>requestAnimationFrame(()=>setTimeout(resolve))));
async function show(name,isOnvif=true){
  await page.evaluate(([n,o])=>window.show(n,o),[name,isOnvif]);
  await settle();
}
async function pending(index){
  await page.waitForFunction(i=>Boolean(window.requests[i]?.resolve),index);
}
async function open(name,index){
  await show(name);
  expect(await modeButton('day').count()).toBe(0);
  await page.getByTitle('live.dayNightControl').click();
  await pending(index);
  expect(await page.evaluate(i=>window.requests[i].url,index)).toBe(`/api/streams/${name}/daynight`);
}
async function complete(index,mode='day'){
  const method=await page.evaluate(i=>window.requests[i].method,index);
  const body=method==='PUT'
    ?{success:true,source:'onvif-imaging',mode,ir_cut_filter:{day:'Off',night:'On',auto:'Auto'}[mode]}
    :{success:true,source:'onvif-imaging',mode,supported_modes:['auto','day','night'],options_available:true,video_source_token:'source'};
  const guard=method==='PUT'?validateDaynightUpdated:validateDaynightResponse;
  expect(guard(body)).toBe(true);
  await page.evaluate(([i,b])=>window.requests[i].resolve(b),[index,body]);
  await settle();
}
async function reject(index){
  await page.evaluate(i=>window.requests[i].reject(new Error('Body interrupted')),index);
  await settle();
}
const toasts=()=>page.evaluate(()=>window.toasts);
const aborted=index=>page.evaluate(i=>window.requests[i].signal.aborted,index);
const selected=async mode=>expect(await modeButton(mode).evaluate(el=>el.style.backgroundColor)).toBe('rgba(59, 130, 246, 0.8)');

test.each(['success','failure'])('switching during GET ignores its late %s and keeps the new read pending',async(outcome)=>{
  await open('Gate',0);
  await open('Barn',1);
  expect(await aborted(0)).toBe(true);
  if(outcome==='success')await complete(0,'night');else await reject(0);
  expect(await modeButton('day').isDisabled()).toBe(true);
  expect(await toasts()).toEqual([]);
  await complete(1,'day');
  expect(await modeButton('day').isEnabled()).toBe(true);
  await selected('day');
  expect(await page.evaluate(()=>window.requests.length)).toBe(2);
});

test.each(['success','failure'])('switching during PUT ignores its late %s without clearing the new write',async(outcome)=>{
  await open('Gate',0);await complete(0);
  await modeButton('night').click();await pending(1);
  await open('Barn',2);
  expect(await aborted(1)).toBe(true);
  await complete(2);
  await modeButton('auto').click();await pending(3);
  expect(await page.evaluate(()=>JSON.parse(window.requests[3].body))).toEqual({mode:'auto'});
  if(outcome==='success')await complete(1,'night');else await reject(1);
  expect(await modeButton('day').isDisabled()).toBe(true);
  expect(await page.getByRole('button',{name:'live.dayNightApplying'}).count()).toBe(1);
  expect(await toasts()).toEqual([]);
  await complete(3,'auto');await selected('auto');
  expect(await modeButton('day').isEnabled()).toBe(true);
  expect((await toasts()).map(t=>t[1])).toEqual(['success']);
  expect(await page.evaluate(()=>window.requests.length)).toBe(4);
});

test('non-ONVIF transition and unmount cancel requests; returning to the same name starts fresh',async()=>{
  await open('Gate',0);
  await show('Gate',false);
  expect(await page.getByTitle('live.dayNightControl').count()).toBe(0);
  expect(await aborted(0)).toBe(true);
  await open('Gate',1);
  await complete(1,'day');await complete(0,'night');await selected('day');
  await modeButton('night').click();await pending(2);
  await page.evaluate(()=>window.unmount());
  expect(await aborted(2)).toBe(true);
  await reject(2);
  expect(await toasts()).toEqual([]);
});

test('current failures remain visible and a failed write can be retried explicitly',async()=>{
  await open('Gate',0);await reject(0);
  expect((await toasts())[0][0]).toContain('live.dayNightLoadFailed');
  expect(await modeButton('night').isEnabled()).toBe(true);
  await modeButton('night').click();await pending(1);await reject(1);
  expect((await toasts())[1][0]).toContain('live.dayNightApplyFailed');
  expect(await modeButton('night').isEnabled()).toBe(true);
  await modeButton('night').click();await pending(2);await complete(2,'night');
  await selected('night');
  expect((await toasts()).map(t=>t[1])).toEqual(['error','error','success']);
  expect(await page.evaluate(()=>window.requests.length)).toBe(3);
});
