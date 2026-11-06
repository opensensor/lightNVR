/** Real StreamsView/query mutations in Chromium; unrelated child views are reduced fixtures. */
async function openBrowser() {
  const {chromium} = require('../../../node_modules/playwright');
  const {build} = require('esbuild');
  const path = require('node:path');
  const baseline=require('../../../tests/contracts/fixtures/baseline-payloads.json').cases;
  const compiled = await build({
    stdin: {resolveDir:path.resolve(__dirname,'../..'),loader:'jsx',contents:`
      import {h,render} from 'preact';
      import {QueryClient,QueryClientProvider} from './js/query-client.ts';
      import {StreamsView} from './js/components/preact/StreamsView.tsx';
      const full={...${JSON.stringify(baseline['admin-full'].body.stream)},name:'Existing',url:'rtsp://existing/live',enabled:true,record:true};
      const summary=${JSON.stringify(baseline['admin-summary'].body)};summary.streams[0]={...summary.streams[0],name:'Existing',enabled:true};summary.streams=[summary.streams[0]];summary.total=1;summary.total_pages=1;
      const devices=['A','B'].map(name=>({name,ip_address:name==='A'?'192.0.2.10':'192.0.2.11',device_service:'http://'+name+'.invalid/onvif/device_service',online:true,last_seen_at:0}));
      window.calls=[];window.waiting={};window.delay={};window.fixtureFull=full;
      const json=(value,status=200)=>new Response(JSON.stringify(value),{status,headers:{'Content-Type':'application/json'}});
      window.fetch=async(input,options={})=>{
        const url=String(input),method=options.method||'GET';
        window.calls.push({url,method,body:options.body?JSON.parse(options.body):null});
        const key=url==='/api/onvif/devices'?'inventory':url.endsWith('/full')?'full':url==='/api/onvif/device/profiles'?options.headers['X-Device-URL']:url==='/api/onvif/discovery/discover'?'discovery':null;
        if(key&&window.delay[key]){window.delay[key]=false;return new Promise(resolve=>window.waiting[key]=value=>resolve(json(value)));}
        if(url.includes('/full'))return json({stream:full});
        if(url.startsWith('/api/streams?'))return json(summary);
        if(method==='PUT'&&url.startsWith('/api/streams/'))return json({success:true,message:'Accepted'},202);
        if(url==='/api/onvif/devices'||url==='/api/onvif/discovery/discover')return json({devices});
        if(url==='/api/onvif/device/test')return json({success:true});
        if(url==='/api/onvif/device/profiles'){const name=options.headers['X-Device-URL'].includes('B.invalid')?'B':'A';return json({profiles:[{name:'Profile '+name,token:name,stream_uri:'rtsp://'+name+'.invalid/live',width:640,height:480,fps:15,encoding:'H264'}]});}
        if(url==='/api/detection/models')return json({models:[]});
        return json({});
      };
      const client=new QueryClient({defaultOptions:{queries:{retry:false},mutations:{retry:false}}});
      render(h(QueryClientProvider,{client},h(StreamsView)),document.getElementById('root'));
    `},bundle:true,write:false,format:'iife',jsx:'automatic',jsxImportSource:'preact',
    alias:{react:'preact/compat','react-dom':'preact/compat'},
    plugins:[{name:'fixture',setup(builder){
      builder.onResolve({filter:/(?:i18n\.ts|auth-utils\.ts|ToastContainer\.tsx|StreamCard\.tsx|StreamConfigModal\.tsx|HealthView\.tsx|FleetView\.tsx|EventRoutingView\.tsx)$/},args=>({path:args.path,namespace:'fixture'}));
      builder.onLoad({filter:/.*/,namespace:'fixture'},args=>({loader:'jsx',resolveDir:path.resolve(__dirname,'../..'),contents:
        args.path.endsWith('i18n.ts')?'export const useI18n=()=>({t:key=>key});':
        args.path.endsWith('auth-utils.ts')?'export const validateSession=async()=>({valid:true,role:"admin"});':
        args.path.endsWith('ToastContainer.tsx')?'export const showStatusMessage=()=>{};':
        args.path.endsWith('StreamCard.tsx')?'export const StreamCard=({stream,onEdit,onClone})=><div><button onClick={()=>onEdit(stream.name)}>Edit existing</button><button onClick={()=>onClone(stream.name)}>Clone existing</button></div>;':
        args.path.endsWith('StreamConfigModal.tsx')?'export const StreamConfigModal=({currentStream,isEditing,isCloning,onClose,onSave,onInputChange})=><div data-testid="editor"><span data-testid="mode">{isEditing?"edit":isCloning?"clone":"create"}</span><input aria-label="Editor name" name="name" value={currentStream.name} onInput={onInputChange}/><button onClick={onClose}>Cancel editor</button><button onClick={onSave}>Save editor</button></div>;':
        'export const HealthView=()=>null;export const FleetView=()=>null;export const EventRoutingView=()=>null;'
      }));
    }}]
  });
  const browser=await chromium.launch({headless:true});const page=await browser.newPage();page.setDefaultTimeout(5000);
  const errors=[];page.on('pageerror',error=>errors.push(error.message));
  await page.route('http://fixture.invalid/**',route=>route.fulfill({contentType:'text/html',body:'<style>svg{width:16px;height:16px}button{margin:4px}</style><div id="root"></div>'}));
  await page.goto('http://fixture.invalid/');await page.addScriptTag({content:compiled.outputFiles[0].text});
  await page.getByRole('button',{name:'Edit existing',exact:true}).waitFor();
  return {browser,page,errors};
}
async function settle(page){await page.evaluate(()=>new Promise(resolve=>requestAnimationFrame(()=>requestAnimationFrame(resolve))));}
async function connect(page,name){
  await page.locator('#onvif-modal tr').filter({hasText:name==='A'?'192.0.2.10':'192.0.2.11'}).getByRole('button',{name:'streams.connect',exact:true}).click();
}
async function writes(page){return page.evaluate(()=>window.calls.filter(x=>x.url.startsWith('/api/streams')&&['POST','PUT'].includes(x.method)));}

it('edit/cancel then ONVIF add uses POST; edit stays PUT and clone/manual creation stay POST',async()=>{
  const {browser,page,errors}=await openBrowser();
  try{
    await page.getByRole('button',{name:'Edit existing',exact:true}).click();await page.getByTestId('editor').waitFor();
    await page.getByRole('button',{name:'Cancel editor',exact:true}).click();
    await page.locator('#discover-onvif-btn').click();await connect(page,'A');
    await page.getByRole('button',{name:'streams.addAsStream',exact:true}).click();
    await page.locator('#custom-stream-name').fill('Discovered');
    await page.locator('#custom-name-modal').getByRole('button',{name:'streams.addStream',exact:true}).click();
    await page.waitForFunction(()=>window.calls.some(x=>x.url.startsWith('/api/streams')&&x.method!=='GET'));
    expect((await writes(page))[0]).toMatchObject({url:'/api/streams',method:'POST',body:{name:'Discovered',url:'rtsp://A.invalid/live',isOnvif:true}});
    await page.locator('#onvif-close-btn').click();
    await page.getByRole('button',{name:'Edit existing',exact:true}).click();await page.getByTestId('editor').waitFor();
    expect(await page.getByTestId('mode').innerText()).toBe('edit');
    await page.getByRole('button',{name:'Save editor',exact:true}).click();await page.getByTestId('editor').waitFor({state:'detached'});
    expect((await writes(page))[1]).toMatchObject({url:'/api/streams/Existing',method:'PUT'});
    expect((await writes(page))[1].body).not.toHaveProperty('name');
    await page.getByRole('button',{name:'Clone existing',exact:true}).click();await page.getByTestId('editor').waitFor();
    expect(await page.getByTestId('mode').innerText()).toBe('clone');
    await page.getByRole('textbox',{name:'Editor name'}).fill('Cloned');await page.getByRole('button',{name:'Save editor',exact:true}).click();await page.getByTestId('editor').waitFor({state:'detached'});
    expect((await writes(page))[2]).toMatchObject({url:'/api/streams',method:'POST',body:{name:'Cloned'}});
    await page.locator('#add-stream-btn').click();expect(await page.getByTestId('mode').innerText()).toBe('create');
    await page.getByRole('button',{name:'Cancel editor',exact:true}).click();await page.locator('#add-stream-btn').click();
    expect(await page.getByRole('textbox',{name:'Editor name'}).inputValue()).toBe('');expect(errors).toEqual([]);
  }finally{await browser.close();}
});

it('late discovery opening cannot replace a newer manual editor',async()=>{
  const {browser,page}=await openBrowser();try{
    await page.evaluate(()=>window.delay.inventory=true);await page.locator('#discover-onvif-btn').click();
    await page.waitForFunction(()=>!!window.waiting.inventory);
    await page.locator('#add-stream-btn').click();await page.getByTestId('editor').waitFor();
    await page.evaluate(()=>window.waiting.inventory({devices:[]}));await settle(page);
    expect(await page.locator('#onvif-modal').count()).toBe(0);expect(await page.getByTestId('mode').innerText()).toBe('create');
  }finally{await browser.close();}
});

it('late profile result from a closed discovery session cannot replace the new device selection',async()=>{
  const {browser,page}=await openBrowser();try{
    await page.locator('#discover-onvif-btn').click();
    await page.evaluate(()=>window.delay['http://A.invalid/onvif/device_service']=true);await connect(page,'A');
    await page.waitForFunction(()=>!!window.waiting['http://A.invalid/onvif/device_service']);
    await page.locator('#onvif-close-btn').click();await page.locator('#discover-onvif-btn').click();await connect(page,'B');
    await page.getByText('Profile B',{exact:true}).waitFor();
    await page.evaluate(()=>window.waiting['http://A.invalid/onvif/device_service']({profiles:[{name:'Stale Profile A',token:'stale',stream_uri:'rtsp://stale/live'}]}));await settle(page);
    expect(await page.getByText('Stale Profile A',{exact:true}).count()).toBe(0);expect(await page.getByText('Profile B',{exact:true}).count()).toBe(1);
  }finally{await browser.close();}
});

it('late edit or clone loading cannot reopen over a new manual form',async()=>{
  for(const action of ['Edit existing','Clone existing']){
    const {browser,page}=await openBrowser();try{
      await page.evaluate(()=>window.delay.full=true);
      await page.getByRole('button',{name:action,exact:true}).click();await page.waitForFunction(()=>!!window.waiting.full);
      await page.locator('#add-stream-btn').click();await page.getByTestId('editor').waitFor();
      await page.evaluate(()=>window.waiting.full({stream:{...window.fixtureFull,name:'Stale camera',url:'rtsp://stale/live'}}));await settle(page);
      expect(await page.getByTestId('mode').innerText()).toBe('create');expect(await page.getByRole('textbox',{name:'Editor name'}).inputValue()).toBe('');
    }finally{await browser.close();}
  }
});

it('a scan finishing after Escape cannot populate a reopened discovery session',async()=>{
  const {browser,page}=await openBrowser();try{
    await page.locator('#discover-onvif-btn').click();
    await page.evaluate(()=>window.delay.discovery=true);await page.locator('#discover-btn').click();
    await page.waitForFunction(()=>!!window.waiting.discovery);await page.keyboard.press('Escape');
    await page.locator('#onvif-modal').waitFor({state:'detached'});await page.locator('#discover-onvif-btn').click();
    await page.locator('#onvif-modal').getByText('192.0.2.11',{exact:true}).waitFor();
    await page.evaluate(()=>window.waiting.discovery({devices:[{name:'Stale scan',ip_address:'192.0.2.99',online:true}]}));await settle(page);
    expect(await page.getByText('192.0.2.99',{exact:true}).count()).toBe(0);expect(await page.locator('#discover-btn').isDisabled()).toBe(false);
  }finally{await browser.close();}
});
