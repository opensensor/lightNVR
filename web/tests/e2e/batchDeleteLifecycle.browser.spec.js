/** Real modal lifecycle with deferred HTTP responses, including deliberately late replies. */
const path = require('node:path');
const {build} = require('esbuild');
const {chromium} = require('playwright');
const english = require('../../public/locales/en.json');

let bundle;
beforeAll(async()=>{
  bundle=(await build({
    stdin:{resolveDir:path.resolve(__dirname,'../..'),loader:'jsx',contents:`
      import {h,render} from 'preact';
      import {BatchDeleteModal,batchDeleteRecordings} from './js/components/preact/BatchDeleteModal.tsx';
      window.calls={requests:[],messages:[],invalidations:[],outcomes:[]};
      window.fetch=(url,options={})=>new Promise((resolve,reject)=>{
        // Intentionally ignore abort: identity guards must also handle late replies.
        window.calls.requests.push({url,body:options.body,signal:options.signal,
          succeed:(body,status=200)=>resolve({ok:status<400,status,statusText:'fixture',json:async()=>body}),
          fail:()=>reject(new Error('network fixture failure'))});
      });
      window.openDelete=(ids=['9007199254740993'])=>{
        const index=window.calls.outcomes.push({status:'pending'})-1;
        window.batchDeleteRecordingsByHttpRequest(ids===null?{filter:{stream:'A'}}:{ids}).then(
          value=>window.calls.outcomes[index]={status:'resolved',value},
          error=>window.calls.outcomes[index]={status:'rejected',message:error.message});
      };
      window.openViaWrapper=()=>batchDeleteRecordings(['42']);
      window.mount=()=>render(h(BatchDeleteModal),document.getElementById('root'));
      window.unmount=()=>render(null,document.getElementById('root'));
      window.mount();
    `},bundle:true,write:false,format:'iife',jsx:'automatic',jsxImportSource:'preact',
    plugins:[{name:'fixture',setup(build){
      build.onResolve({filter:/(?:i18n\.ts|query-client\.ts|ToastContainer\.tsx)$/},args=>({path:args.path,namespace:'fixture'}));
      build.onLoad({filter:/.*/,namespace:'fixture'},args=>({loader:'js',contents:
        args.path.endsWith('i18n.ts')?`export const t=key=>key==='recordings.batchDeleteProgress'?${JSON.stringify(english['recordings.batchDeleteProgress'])}:key; export const useI18n=()=>({t});`:
        args.path.endsWith('query-client.ts')?'const client={invalidateQueries:async args=>window.calls.invalidations.push(args)};export const useQueryClient=()=>client;':
        'export const showStatusMessage=(...args)=>window.calls.messages.push(args);'
      }));
    }}]
  })).outputFiles[0].text;
});

async function fixture() {
  const browser=await chromium.launch({headless:true});
  try {
    const page=await browser.newPage();
    await page.setContent('<div id="root"></div>');
    await page.addScriptTag({content:bundle});
    await page.waitForFunction(()=>typeof window.batchDeleteRecordingsByHttpRequest==='function');
    return {browser,page};
  }catch(error){await browser.close();throw error;}
}
async function open(page,filtered=false) {
  await page.evaluate(filtered=>window.openDelete(filtered?null:undefined),filtered);
  await page.getByRole('button',{name:'recordings.deleteDescription',exact:true}).waitFor();
}
async function proceed(page) {
  await page.getByRole('button',{name:'recordings.deleteDescription',exact:true}).click();
}
async function respond(page,index,body,status=200) {
  await page.evaluate(({index,body,status})=>window.calls.requests[index].succeed(body,status),{index,body,status});
}
async function requestCount(page,count) {
  await page.waitForFunction(count=>window.calls.requests.length===count,count);
}
async function outcome(page,index=0) {
  await page.waitForFunction(index=>window.calls.outcomes[index]?.status!=='pending',index);
  return page.evaluate(index=>window.calls.outcomes[index],index);
}
const completed={status:'complete',complete:true,total:1,current:1,succeeded:1,failed:0};

test.each(['button','escape','global','unmount'])('confirmation cancellation via %s settles once without HTTP',async(method)=>{
  const {browser,page}=await fixture();
  try {
    await open(page);
    if(method==='button')await page.getByRole('button',{name:'common.cancel',exact:true}).click();
    if(method==='escape')await page.keyboard.press('Escape');
    if(method==='global')await page.evaluate(()=>window.closeBatchDeleteModal());
    if(method==='unmount')await page.evaluate(()=>window.unmount());
    expect(await outcome(page)).toEqual({status:'resolved',value:{status:'cancelled',complete:true,current:0,total:0,succeeded:0,failed:0,pending_deletions:0}});
    expect(await page.evaluate(()=>window.calls.requests.length)).toBe(0);
    expect(await page.evaluate(()=>window.calls.invalidations.length)).toBe(0);
    expect(await page.getByRole('dialog').count()).toBe(0);
  }finally{await browser.close();}
});

test('duplicate starts are guarded and slow progress requests never overlap',async()=>{
  const {browser,page}=await fixture();
  try {
    await open(page);
    // A second global request cannot replace the first promise or its IDs.
    await page.evaluate(()=>window.openDelete(['different']));
    expect((await outcome(page,1)).status).toBe('rejected');
    await page.evaluate(()=>window.openViaWrapper()); // Rejection must not close the original dialog.
    await page.evaluate(()=>{
      const button=[...document.querySelectorAll('button')].find(el=>el.textContent==='recordings.deleteDescription');
      button.click();button.click();
    });
    await requestCount(page,1);
    expect(await page.evaluate(()=>JSON.parse(window.calls.requests[0].body))).toEqual({ids:['9007199254740993']});
    await respond(page,0,{job_id:'job-one'});await requestCount(page,2);
    await page.waitForTimeout(1100);
    expect(await page.evaluate(()=>window.calls.requests.length)).toBe(2);
    await page.evaluate(()=>window.closeBatchDeleteModal());await page.keyboard.press('Escape');
    expect(await page.getByRole('dialog').count()).toBe(1);
    expect(await page.evaluate(()=>window.calls.outcomes[0].status)).toBe('pending');
    await respond(page,1,{status:'running',complete:false,total:1,current:0});
    await requestCount(page,3);
    await page.waitForTimeout(600);
    expect(await page.evaluate(()=>window.calls.requests.length)).toBe(3);
    await respond(page,2,completed);
    expect(await outcome(page)).toEqual({status:'resolved',value:completed});
    await page.waitForTimeout(600);
    expect(await page.evaluate(()=>window.calls.requests.length)).toBe(3);
    expect(await page.evaluate(()=>window.calls.invalidations)).toEqual([{queryKey:['recordings']}]);
    expect(await page.evaluate(()=>window.calls.messages.map(m=>m[1]))).toEqual(['success']);
    await page.getByRole('button',{name:'common.done',exact:true}).click();
    await open(page);await page.getByRole('button',{name:'common.cancel',exact:true}).click();
    expect((await outcome(page,2)).value.status).toBe('cancelled');
  }finally{await browser.close();}
},30000);

test.each(['start','poll','poll-error'])('unmount during %s aborts monitoring and late replies cannot affect a new modal',async(stage)=>{
  const {browser,page}=await fixture();
  try {
    await open(page);await proceed(page);await requestCount(page,1);
    if(stage!=='start'){await respond(page,0,{job_id:'old-job'});await requestCount(page,2);}
    const oldIndex=stage==='start'?0:1;
    await page.evaluate(()=>window.unmount());
    expect((await outcome(page)).message).toMatch(/server job may continue/);
    expect(await page.evaluate(index=>window.calls.requests[index].signal.aborted,oldIndex)).toBe(true);
    await page.evaluate(()=>window.mount());
    await page.waitForFunction(()=>typeof window.batchDeleteRecordingsByHttpRequest==='function');
    await open(page);await proceed(page);await requestCount(page,oldIndex+2);
    if(stage==='poll-error')await page.evaluate(index=>window.calls.requests[index].fail(),oldIndex);
    else await respond(page,oldIndex,stage==='poll'?completed:{job_id:'late-job'});
    await page.waitForTimeout(600);
    expect(await page.evaluate(()=>window.calls.messages)).toEqual([]);
    expect(await page.evaluate(()=>window.calls.invalidations)).toEqual([]);
    expect(await page.evaluate(()=>window.calls.outcomes[1].status)).toBe('pending');
    expect(await page.evaluate(()=>window.calls.requests.length)).toBe(oldIndex+2);
    await respond(page,oldIndex+1,{job_id:'new-job'});await requestCount(page,oldIndex+3);
    await respond(page,oldIndex+2,completed);
    expect((await outcome(page,1)).value).toEqual(completed);
  }finally{await browser.close();}
},30000);

test.each(['start-http','missing-job','poll-network','job-error'])('%s rejects once and stops polling',async(mode)=>{
  const {browser,page}=await fixture();
  try {
    await open(page);await proceed(page);await requestCount(page,1);
    if(mode==='start-http')await respond(page,0,{},500);
    else if(mode==='missing-job')await respond(page,0,{});
    else {
      await respond(page,0,{job_id:'failed-job'});await requestCount(page,2);
      if(mode==='poll-network')await page.evaluate(()=>window.calls.requests[1].fail());
      else await respond(page,1,{status:'error',complete:true,error_message:'Database unavailable',total:0,current:0});
    }
    expect((await outcome(page)).status).toBe('rejected');
    const bar=page.getByRole('progressbar',{name:'Recording deletion progress'});
    expect(await bar.getAttribute('aria-valuenow')).toBeNull();
    expect(await bar.getAttribute('aria-valuetext')).toMatch(/^Error:/);
    await page.waitForTimeout(600);
    expect(await page.evaluate(()=>window.calls.requests.length)).toBe(mode.startsWith('start')||mode==='missing-job'?1:2);
    expect(await page.evaluate(()=>window.calls.messages.map(m=>m[1]))).toEqual(['error']);
    expect(await page.evaluate(()=>window.calls.invalidations.length)).toBe(0);
    expect(await page.getByRole('button',{name:'common.done',exact:true}).count()).toBe(1);
  }finally{await browser.close();}
},30000);


test('unmount clears an already scheduled poll timer',async()=>{
  const {browser,page}=await fixture();
  try {
    await open(page);await proceed(page);await requestCount(page,1);
    await respond(page,0,{job_id:'timer-job'});await requestCount(page,2);
    await respond(page,1,{status:'running',complete:false,total:1,current:0});
    await page.evaluate(()=>window.unmount());
    expect((await outcome(page)).status).toBe('rejected');
    await page.waitForTimeout(700);
    expect(await page.evaluate(()=>window.calls.requests.length)).toBe(2);
  }finally{await browser.close();}
},30000);


test('accessible progress stays indeterminate until a total is known and reports bounded percentages',async()=>{
  const {browser,page}=await fixture();
  try {
    await open(page,true);await proceed(page);await requestCount(page,1);
    const bar=page.getByRole('progressbar',{name:'Recording deletion progress'});
    expect(await bar.getAttribute('aria-valuenow')).toBeNull();
    expect(await bar.getAttribute('aria-valuetext')).toBe('recordings.inProgress');
    expect(await bar.getAttribute('aria-valuemin')).toBe('0');
    expect(await bar.getAttribute('aria-valuemax')).toBe('100');
    await respond(page,0,{job_id:'accessible-job'});await requestCount(page,2);
    await respond(page,1,{status:'running',complete:false,total:4,current:1,succeeded:1});
    await page.waitForFunction(()=>document.querySelector('[role=progressbar]')?.getAttribute('aria-valuenow')==='25');
    expect(await bar.getAttribute('aria-valuetext')).toBe('25%');
    await requestCount(page,3);
    await respond(page,2,{status:'running',complete:false,total:4,current:5,succeeded:4});
    await page.waitForFunction(()=>document.querySelector('[role=progressbar]')?.getAttribute('aria-valuenow')==='100');
    expect(await bar.getAttribute('aria-valuetext')).toBe('100%');
    await requestCount(page,4);
    await respond(page,3,{...completed,total:4,current:4,succeeded:4});await outcome(page);
    expect(await bar.getAttribute('aria-valuenow')).toBe('100');
  }finally{await browser.close();}
},30000);

test('an empty successful job announces completion rather than remaining indeterminate',async()=>{
  const {browser,page}=await fixture();
  try {
    await open(page,true);await proceed(page);await requestCount(page,1);
    await respond(page,0,{job_id:'empty-job'});await requestCount(page,2);
    await respond(page,1,{...completed,total:0,current:0,succeeded:0});await outcome(page);
    const bar=page.getByRole('progressbar',{name:'Recording deletion progress'});
    expect(await bar.getAttribute('aria-valuenow')).toBe('100');
    expect(await bar.getAttribute('aria-valuetext')).toBe('100%');
  }finally{await browser.close();}
},30000);
