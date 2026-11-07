/** Exercise RecordingsView with real Preact, query caching and recordings API
 * query construction; only transport and unrelated presentation are replaced. */
const path = require('node:path');
let browser, page, bundle, errors;

beforeAll(async () => {
  const stubs = {
    'i18n.ts': "export const t=key=>key;export const useI18n=()=>({t});",
    'ToastContainer.tsx': 'export const showStatusMessage=()=>{};',
    'UI.tsx': `import {createContext} from 'preact';export const ModalContext=createContext(null);
      export const showVideoModal=()=>{};export const DeleteConfirmationModal=()=>null;`,
    'BatchDownloadModal.tsx': 'export const BatchDownloadModal=()=>null;',
    'RecordingsNavigation.tsx': 'export const RecordingsNavigation=()=>null;',
    'RecordingsGrid.tsx': 'export const RecordingsGrid=()=>null;',
    'RecordingsTable.tsx': `import {h} from 'preact';
      export const RecordingsTable=({recordings})=>h('pre',{id:'rows'},JSON.stringify(recordings));`,
    'FiltersSidebar.tsx': `import {h} from 'preact';
      export const FiltersSidebar=({setFilters})=>h('button',{id:'detect',
        onClick:()=>setFilters(prev=>({...prev,recordingType:'detection'}))},'Only detections');`,
    'LoadingIndicator.tsx': 'export const ContentLoader=({children})=>children;',
    'request-queue.ts': 'export const clearThumbnailQueue=()=>{};',
    'auth-utils.ts': "export const validateSession=async()=>({valid:true,role:'admin'});",
    'fleet.ts': 'export const listCameraCollections=async()=>({collections:[]});',
    'stream-summaries.ts': 'export const fetchAllStreamSummaries=async()=>[];',
    'fetch-utils.ts': `export const fetchJSON=(url)=>window.fetchJSON(url);
      export const enhancedFetch=()=>{throw new Error('Unexpected request');};
      export const createRequestController=()=>new AbortController();`,
  };
  const output = await require('esbuild').build({
    stdin: {resolveDir:path.resolve(__dirname,'../..'),loader:'jsx',contents:`
      import {h,render} from 'preact';
      import {RecordingsView} from './js/components/preact/RecordingsView.tsx';
      import {queryClient,QueryClientProvider} from './js/query-client.ts';
      window.queryClient=queryClient;window.requests=[];
      window.fetch=async()=>({json:async()=>({generate_thumbnails:false})});
      window.fetchJSON=async url=>{
        if(!url.startsWith('/api/recordings?'))throw new Error('Unexpected URL: '+url);
        window.requests.push(url);
        const params=new URL(url,location.href).searchParams;
        const id=params.get('has_detection')==='1'?1:2;
        return Object.freeze({recordings:Object.freeze([Object.freeze({
          id,stream:'Gate',start_time:1700000000,end_time:1700000060,has_detections:false,
        })]),pagination:Object.freeze({limit:20,total:100,pages:5})});
      };
      render(h(QueryClientProvider,{client:queryClient},h(RecordingsView)),document.getElementById('root'));
    `},
    bundle:true,write:false,format:'iife',jsx:'automatic',jsxImportSource:'preact',
    alias:{react:'preact/compat','react-dom':'preact/compat'},
    plugins:[{name:'fixture',setup(builder){
      builder.onResolve({filter:/\.(ts|tsx)$/},args=>{
        const name=path.basename(args.path);
        if(stubs[name])return {path:name,namespace:'fixture'};
      });
      builder.onLoad({filter:/.*/,namespace:'fixture'},args=>({contents:stubs[args.path],loader:'js',resolveDir:path.resolve(__dirname,'../..')}));
    }}],
  });
  bundle=output.outputFiles[0].text;
  browser=await require('../../../node_modules/playwright').chromium.launch({headless:true});
});
beforeEach(async()=>{
  page=await browser.newPage();page.setDefaultTimeout(5000);errors=[];
  page.on('pageerror',error=>errors.push(error.message));
  await page.route('http://fixture.invalid/**',route=>route.fulfill({contentType:'text/html',body:'<div id="root"></div>'}));
});
afterEach(async()=>{await page?.close();});
afterAll(async()=>{await browser?.close();});

async function mount(search){
  await page.goto('http://fixture.invalid/recordings.html?view=table&limit=20&'+search);
  await page.addScriptTag({content:bundle});
  await waitForRows(search.includes('detection=1')?1:2);
  await settled();
}
async function waitForRows(id){
  await page.waitForFunction(id=>JSON.parse(document.querySelector('#rows')?.textContent||'[]')[0]?.id===id,id);
}
async function settled(){
  await page.waitForFunction(()=>window.queryClient.isFetching()===0);
  // Allow effects (including the page reset) and their resulting renders to run.
  await page.evaluate(()=>new Promise(resolve=>requestAnimationFrame(()=>setTimeout(resolve,100))));
  await page.waitForFunction(()=>window.queryClient.isFetching()===0);
}
async function removeType(){
  await page.locator('#active-filters .filter-tag').filter({hasText:'Detection Events Only'}).getByRole('button').click();
}

it('marks detection rows for display without contaminating cached responses',async()=>{
  await mount('detection=1');
  expect(JSON.parse(await page.locator('#rows').textContent())[0].has_detections).toBe(true);
  expect(await page.evaluate(()=>window.queryClient.getQueriesData({queryKey:['recordings']})
    .filter(([,data])=>data).every(([,data])=>data.recordings[0].has_detections===false))).toBe(true);
  await removeType();await waitForRows(2);await settled();
  expect(JSON.parse(await page.locator('#rows').textContent())[0].has_detections).toBe(false);
  await page.locator('#detect').click();await waitForRows(1);await settled();
  expect(JSON.parse(await page.locator('#rows').textContent())[0].has_detections).toBe(true);
  expect(errors).toEqual([]);
});

it('removing a filter updates requests and URL, resets the page, and avoids refetching the old filter',async()=>{
  await mount('detection=1&page=3');
  expect(new URL(page.url()).searchParams.get('page')).toBe('3');
  await page.evaluate(()=>{window.requests=[];});
  await removeType();await waitForRows(2);await settled();
  const params=new URL(page.url()).searchParams;
  expect(params.has('detection')).toBe(false);expect(params.get('page')).toBe('1');
  const requests=await page.evaluate(()=>window.requests);
  expect(requests.length).toBeGreaterThan(0);
  expect(requests.every(url=>!new URL(url,'http://fixture.invalid').searchParams.has('has_detection'))).toBe(true);
  expect(new URL(requests.at(-1),'http://fixture.invalid').searchParams.get('page')).toBe('1');
  expect(errors).toEqual([]);
});
