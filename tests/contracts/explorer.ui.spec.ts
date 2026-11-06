import {test as base,expect,type Browser,type Page,type BrowserContext} from '@playwright/test';
import {startFixture,type ContractFixture} from './fixture-server.mjs';
import {createUser,role,policy,cameraScope,ok,identityOps,recordingCommonOps,recordingReplayOps,recordingActionOps,recordingJobOps,recordingExportOps,type Identity} from './permissionCases';
import {startProxy} from './proxy-fixture.mjs';
const test=base.extend<{server:ContractFixture}>({server:async({},use,info)=>{
  const server=await startFixture({webRoot:process.env.LIGHTNVR_CONTRACT_WEB_ROOT});
  try{await use(server);}finally{const failed=info.status!==info.expectedStatus;if(failed)await info.attach('native-log',{path:server.logPath,contentType:'text/plain'});await server.stop({failed});}
}});
interface RenderRecord {ops:string[];schemas:string[];servers:string[];forbidden:boolean}
interface Probe {renders:RenderRecord[];forbidWrites:boolean}
declare global {interface Window {contractExplorerProbe:Probe}}
async function instrument(page:Page) {
  await page.addInitScript(()=>{
    window.contractExplorerProbe={renders:[],forbidWrites:false};
    let factory:((options:any)=>unknown)|undefined;
    Object.defineProperty(window,'SwaggerUIBundle',{configurable:true,get:()=>factory,set(original:(options:any)=>unknown){
      factory=(options:any)=>{
        const ops=Object.values(options.spec.paths).flatMap((item:any)=>Object.values(item).filter((operation:any)=>operation?.operationId).map((operation:any)=>operation.operationId)).sort();
        window.contractExplorerProbe.renders.push({ops,schemas:Object.keys(options.spec.components?.schemas||{}),servers:options.spec.servers.map((s:{url:string})=>s.url),forbidden:window.contractExplorerProbe.forbidWrites&&ops.includes('updateStream')});
        return original(options);
      };
    }});
  });
}
async function session(browser:Browser,server:ContractFixture,identity='admin',baseURL=server.baseURL,custom?:Identity):Promise<BrowserContext>{
  const context=await browser.newContext({baseURL,locale:'en-US'});
  const credentials=custom?{username:custom.username,password:custom.password}:server.credentials[identity];
  expect((await context.request.post(`${baseURL}/api/auth/login`,{data:credentials})).status()).toBe(200);
  return context;
}
async function rendered(page:Page,ops:string[]) {
  await expect(page.locator('#docs-status')).toContainText('covers selected APIs');
  await expect(page.locator('.opblock')).toHaveCount(ops.length);
  await expect.poll(()=>page.evaluate(()=>window.contractExplorerProbe.renders.at(-1)?.ops)).toEqual([...ops].sort());
}
const pilotRead=['getOpenApi','getStreamFull','listStreams'];
const read=[...identityOps,...recordingCommonOps,...recordingReplayOps,...recordingJobOps,...recordingExportOps,...pilotRead];
const all=[...identityOps,...recordingCommonOps,...recordingReplayOps,...recordingActionOps,...pilotRead,'updateStream'];
const configured=[...identityOps,...recordingCommonOps,...recordingJobOps,...pilotRead,'updateStream'];
for(const principal of ['admin','viewer','configurator']) test(`${principal}: real-session explorer operations, conditional models, keyboard and read-only network`,async({server,browser})=>{
  let custom:Identity|undefined;
  if(principal==='configurator'){
    custom=await createUser(server,'docs_configurator');const readRole=await role(server,'Docs read',['live.view']),configureRole=await role(server,'Docs configure',['camera.configure']);
    await policy(server,custom,[{role_uuid:readRole,scope:{type:'all'}},{role_uuid:configureRole,scope:cameraScope(server.cameras.A.camera_uuid)}]);
  }
  const context=await session(browser,server,principal,server.baseURL,custom);const page=await context.newPage();await instrument(page);
  const requests:{url:string;method:string}[]=[];page.on('request',request=>requests.push({url:request.url(),method:request.method()}));
  try{
    await page.setViewportSize({width:390,height:844});await page.goto(`${server.baseURL}/api-docs.html`);
    await rendered(page,principal==='viewer'?read:principal==='configurator'?configured:all);
    const snapshot=await page.evaluate(()=>window.contractExplorerProbe.renders.at(-1)!);
    expect(snapshot.schemas.includes('StreamFullPrivilegedResponse')).toBe(principal!=='viewer');
    expect(snapshot.servers).toEqual([`${server.baseURL}/`]);
    await expect(page.getByRole('button',{name:/^(?:authorize|try it out|execute)$/i})).toHaveCount(0);
    await page.locator('#docs-refresh').focus();await page.keyboard.press('Enter');await rendered(page,principal==='viewer'?read:principal==='configurator'?configured:all);
    await expect(page.locator('#docs-refresh')).toBeFocused();
    expect(await page.evaluate(()=>document.documentElement.scrollWidth)).toBeLessThanOrEqual(390);
    expect(requests.every(request=>new URL(request.url).origin===server.baseURL)).toBe(true);
    expect(requests.every(request=>request.method==='GET')).toBe(true);
    expect(requests.some(request=>/validator\.swagger\.io|swagger\.json|configUrl/.test(request.url))).toBe(false);
    if(custom){expect((await ok(server,'/api/streams/A/full',custom)).stream.admin_url).not.toBe('');expect((await ok(server,'/api/streams/B/full',custom)).stream.admin_url).toBe('');}
  }finally{await context.close();}
});

test('prefix deployment resolves spec, verification, vendor and server base within private proxy',async({server,browser},info)=>{
  const proxy=await startProxy(server.baseURL,{prefix:'/camera-site/'});let context:BrowserContext|undefined;let failed=true;
  try{
    context=await session(browser,server,'viewer',proxy.baseURL);const page=await context.newPage();await instrument(page);
    await page.goto(`${proxy.baseURL}/api-docs.html`);await rendered(page,read);
    expect((await page.evaluate(()=>window.contractExplorerProbe.renders.at(-1)!)).servers).toEqual([`${proxy.baseURL}/`]);
    expect(proxy.requests.some(request=>request.url==='/camera-site/api/openapi.json')).toBe(true);
    expect(proxy.requests.some(request=>request.url==='/camera-site/api/auth/verify')).toBe(true);
    expect(proxy.requests.some(request=>request.url.includes('/camera-site/api-docs-vendor/'))).toBe(true);
    expect(proxy.requests.every(request=>request.status!==404)).toBe(true);failed=false;
  }finally{await context?.close();await proxy.stop({failed});}
});

test('logout, login as viewer and back navigation never restore the administrator document',async({server,browser})=>{
  const context=await session(browser,server);const page=await context.newPage();await instrument(page);
  try{
    await page.goto(`${server.baseURL}/api-docs.html`);await rendered(page,all);
    await page.locator('#docs-logout').click();await page.waitForURL('**/login.html**');await expect(page.locator('#username')).toBeVisible();
    await page.locator('#username').fill(server.credentials.viewer.username);await page.locator('#password').fill(server.credentials.viewer.password);
    await page.locator('#login-form button[type=submit]').click();await page.waitForURL(url=>!url.pathname.endsWith('/login.html'));
    await page.goBack();
    // Successful login must replace its history entry: revisiting the login
    // page would clear the newly issued session on mount.
    await expect(page).toHaveURL(`${server.baseURL}/api-docs.html`);
    // Viewer self-service now includes PUT password; stream writes remain hidden.
    await rendered(page,read);await expect(page.locator('.opblock[id$="-updateStream"]')).toHaveCount(0);
    await page.evaluate(()=>{window.dispatchEvent(new PageTransitionEvent('pagehide',{persisted:true}));});
    await expect(page.locator('#api-reference')).toBeEmpty();
    await page.evaluate(()=>{window.dispatchEvent(new PageTransitionEvent('pageshow',{persisted:true}));});
    await rendered(page,read);await expect(page.locator('.opblock[id$="-updateStream"]')).toHaveCount(0);
  }finally{await context.close();}
});

test('real cookie account change while an admin response is delayed cannot publish that response',async({server,browser})=>{
  const context=await session(browser,server);const page=await context.newPage();await instrument(page);
  let release!:()=>void,arrived!:()=>void;const held=new Promise<void>(resolve=>{release=resolve;}),captured=new Promise<void>(resolve=>{arrived=resolve;});let hold=true;
  try{
    await page.goto(`${server.baseURL}/api-docs.html`);await rendered(page,all);
    await page.route('**/api/openapi.json',async route=>{
      if(!hold){await route.continue();return;}hold=false;
      const response=await route.fetch();expect(response.status()).toBe(200);arrived();await held;await route.fulfill({response});
    });
    await page.locator('#docs-refresh').click();await captured;await expect(page.locator('#api-reference')).toBeEmpty();
    expect((await context.request.post(`${server.baseURL}/api/auth/logout`)).status()).toBe(200);
    expect((await context.request.post(`${server.baseURL}/api/auth/login`,{data:server.credentials.viewer})).status()).toBe(200);
    await page.evaluate(()=>{window.contractExplorerProbe.renders=[];window.contractExplorerProbe.forbidWrites=true;});release();
    await rendered(page,read);
    expect(await page.evaluate(()=>window.contractExplorerProbe.renders.some(record=>record.forbidden))).toBe(false);
    await expect(page.locator('.opblock[id$="-updateStream"]')).toHaveCount(0);
  }finally{release?.();await context.close();}
});

test('malformed and unavailable responses clear old spec; real discovery 401 clears it with denied state',async({server,browser})=>{
  const context=await session(browser,server);const page=await context.newPage();await instrument(page);let releaseDenied=()=>{};
  try{
    await page.goto(`${server.baseURL}/api-docs.html`);await rendered(page,all);
    for(const fault of ['malformed','unavailable']){
      await page.route('**/api/openapi.json',route=>route.fulfill({status:fault==='malformed'?200:503,contentType:'application/json',body:JSON.stringify(fault==='malformed'?{openapi:'invalid'}:{error:'Synthetic service failure'})}));
      await page.locator('#docs-refresh').click();await expect(page.locator('#docs-status')).toContainText('Unable to load');await expect(page.locator('#api-reference')).toBeEmpty();
      await page.unroute('**/api/openapi.json');await page.locator('#docs-refresh').click();await rendered(page,all);
    }
    // Let the first verification succeed, then revoke the actual server session
    // before its discovery request reaches the backend. This is a real 401,
    // rather than a synthetic rejection or only an invalid verification body.
    let entered!:()=>void;
    const waiting=new Promise<void>(resolve=>{entered=resolve;});
    const release=new Promise<void>(resolve=>{releaseDenied=resolve;});
    await page.route('**/api/openapi.json',async route=>{entered();await release;await route.continue();});
    const denied=page.waitForResponse(response=>response.url().endsWith('/api/openapi.json')&&response.status()===401);
    await page.locator('#docs-refresh').click();await waiting;
    expect((await context.request.post(`${server.baseURL}/api/auth/logout`)).status()).toBe(200);
    releaseDenied();await denied;
    await expect(page.locator('#docs-status')).toContainText('Sign in');await expect(page.locator('#api-reference')).toBeEmpty();
  }finally{releaseDenied();await context.close();}
});
