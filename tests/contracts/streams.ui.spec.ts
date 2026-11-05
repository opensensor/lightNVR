import {test as base,expect,type Browser,type Page} from '@playwright/test';
import {startFixture,type ContractFixture} from './fixture-server.mjs';
import {createUser,role,policy,cameraScope,type Identity} from './permissionCases';
const test=base.extend<{server:ContractFixture}>({server:async({},use,info)=>{
  const server=await startFixture({webRoot:process.env.LIGHTNVR_CONTRACT_WEB_ROOT});
  try{await use(server);}finally{const failed=info.status!==info.expectedStatus;if(failed)await info.attach('native-log',{path:server.logPath,contentType:'text/plain'});await server.stop({failed});}
}});
async function session(browser:Browser,server:ContractFixture,identity='admin',custom?:Identity){
  const context=await browser.newContext({baseURL:server.baseURL,locale:'en-US'});
  expect((await context.request.post('/api/auth/login',{data:custom?{username:custom.username,password:custom.password}:server.credentials[identity]})).status()).toBe(200);
  return context;
}
const card=(page:Page,name:string)=>page.getByRole('listitem').filter({has:page.locator(`h3[title="${name}"]`)});
async function open(page:Page,server:ContractFixture){await page.goto(`${server.baseURL}/streams.html`);await expect(card(page,'A')).toBeVisible();}
async function configurator(server:ContractFixture,legacyRole:number){
  const identity=await createUser(server,`scoped_configurator_${legacyRole}`,{role:legacyRole});
  const readRole=await role(server,'Browser read',['live.view']),configureRole=await role(server,'Browser configure',['camera.configure']);
  await policy(server,identity,[{role_uuid:readRole,scope:{type:'all'}},{role_uuid:configureRole,scope:cameraScope(server.cameras.A.camera_uuid)}]);
  return identity;
}

test('viewer sees only authorized camera and no mutation controls or docs vendor assets',async({server,browser})=>{
  const context=await session(browser,server,'viewer');const page=await context.newPage();const requests:string[]=[];page.on('request',request=>requests.push(request.url()));
  try{await open(page,server);await expect(card(page,'B')).toHaveCount(0);await expect(page.getByRole('button',{name:'Edit',exact:true})).toHaveCount(0);await expect(page.getByRole('button',{name:'Delete',exact:true})).toHaveCount(0);expect(requests.some(url=>url.includes('api-docs-vendor'))).toBe(false);}finally{await context.close();}
});

for(const legacyRole of [1,2]) test(`policy configurator with legacy role ${legacyRole} gets only camera A controls backed by real permission`,async({server,browser})=>{
  const identity=await configurator(server,legacyRole);const context=await session(browser,server,'custom',identity);const page=await context.newPage();
  try{
    const response=await context.request.get('/api/streams?summary=true');expect(response.status()).toBe(200);
    const summary=await response.json();expect(summary.streams.find((row:{name:string})=>row.name==='A').can_configure).toBe(true);expect(summary.streams.find((row:{name:string})=>row.name==='B').can_configure).toBe(false);
    expect((await context.request.put('/api/streams/A',{data:{set_privacy_mode:false}})).status()).toBe(200);
    expect((await context.request.put('/api/streams/B',{data:{set_privacy_mode:false}})).status()).toBe(403);
    await open(page,server);await expect(card(page,'B')).toBeVisible();await expect(card(page,'A').getByRole('button',{name:'Edit',exact:true})).toBeVisible();await expect(card(page,'B').getByRole('button',{name:'Edit',exact:true})).toHaveCount(0);
    await expect(card(page,'A').getByRole('button',{name:'Enable stream',exact:true})).toBeVisible();
    await expect(card(page,'A').getByRole('link',{name:'Open admin page for A',exact:true})).toBeVisible();
    await expect(card(page,'B').getByRole('link',{name:'Open admin page for B',exact:true})).toHaveCount(0);
    if(legacyRole===2){
      for(const name of ['Clone Stream','Delete','Add Stream','Discover ONVIF Cameras']) await expect(page.getByRole('button',{name,exact:true})).toHaveCount(0);
      await expect(page.getByText('Read-only (insufficient permissions to modify)',{exact:true})).toHaveCount(0);
    }
    await card(page,'A').getByRole('button',{name:'Edit',exact:true}).click();await expect(page.locator('#stream-admin-url')).toHaveValue('http://127.0.0.1:9/admin-a');
    const edited=`http://127.0.0.1:9/scoped-ui-${legacyRole}`;
    await page.locator('#stream-admin-url').fill(edited);await page.locator('#stream-save-btn').click();await expect(page.locator('#stream-modal')).toHaveCount(0);
    await expect.poll(async()=> (await (await context.request.get('/api/streams/A/full')).json()).stream.admin_url).toBe(edited);
    expect((await server.json<{stream:{admin_url:string}}>('/api/streams/B/full')).stream.admin_url).toBe('http://127.0.0.1:9/admin-b');
  }finally{await context.close();}
});

test('admin retains edits after transient save failure and malformed acknowledgment, then saves through real server',async({server,browser})=>{
  const context=await session(browser,server);const page=await context.newPage();
  try{
    await page.setViewportSize({width:390,height:844});await open(page,server);
    const edit=card(page,'A').getByRole('button',{name:'Edit',exact:true});await edit.focus();await page.keyboard.press('Enter');await expect(page.locator('#stream-modal')).toBeVisible();
    await page.locator('#stream-admin-url').fill('http://127.0.0.1:9/browser-edit');
    const before=await (await context.request.get('/api/streams/A/full')).json();
    for(const fault of ['transient','malformed']){
      await page.route('**/api/streams/A',route=>route.request().method()==='PUT'?route.fulfill({status:fault==='transient'?503:202,contentType:'application/json',body:JSON.stringify(fault==='transient'?{error:'Synthetic service unavailable'}:{success:true,private:'do-not-render'})}):route.continue());
      await page.locator('#stream-save-btn').click();await expect(page.getByText(/Error updating stream:/)).toBeVisible();
      await expect(page.locator('#stream-modal')).toBeVisible();await expect(page.locator('#stream-admin-url')).toHaveValue('http://127.0.0.1:9/browser-edit');await expect(page.locator('#stream-save-btn')).toBeEnabled();
      expect((await (await context.request.get('/api/streams/A/full')).json()).stream.admin_url).toBe(before.stream.admin_url);
      await expect(page.getByText('do-not-render',{exact:false})).toHaveCount(0);await page.unroute('**/api/streams/A');
    }
    await page.locator('#stream-save-btn').click();await expect(page.locator('#stream-modal')).toHaveCount(0);
    await expect.poll(async()=> (await (await context.request.get('/api/streams/A/full')).json()).stream.admin_url).toBe('http://127.0.0.1:9/browser-edit');
    expect((await (await context.request.get('/api/streams/B/full')).json()).stream.admin_url).toBe('http://127.0.0.1:9/admin-b');
  }finally{await context.close();}
});

test('malformed summaries fail usefully without private payloads and Retry restores real data',async({server,browser})=>{
  const context=await session(browser,server);const page=await context.newPage();
  try{
    await page.route('**/api/streams?*',route=>route.fulfill({status:200,contentType:'application/json',body:JSON.stringify({streams:[{name:'do-not-render',url:'rtsp://private:secret@private.invalid'}]})}));
    await page.goto(`${server.baseURL}/streams.html`);await expect(page.getByRole('alert').filter({hasText:'Invalid listStreamSummaries response'})).toBeVisible();
    await expect(page.getByText('do-not-render',{exact:false})).toHaveCount(0);await expect(card(page,'A')).toHaveCount(0);
    await page.unroute('**/api/streams?*');await page.getByRole('button',{name:'Retry',exact:true}).click();await expect(card(page,'A')).toBeVisible();await expect(card(page,'B')).toBeVisible();
  }finally{await context.close();}
});

test('malformed full detail cannot open a partially populated editor',async({server,browser})=>{
  const context=await session(browser,server);const page=await context.newPage();
  try{
    await open(page,server);await page.route('**/api/streams/A/full',route=>route.fulfill({status:200,contentType:'application/json',body:JSON.stringify({stream:{name:'A',url:'rtsp://private:secret@private.invalid'}})}));
    await card(page,'A').getByRole('button',{name:'Edit',exact:true}).click();await expect(page.getByText(/Invalid getStreamFull response/)).toBeVisible();await expect(page.locator('#stream-modal')).toHaveCount(0);
    await expect(page.getByText('private:secret',{exact:false})).toHaveCount(0);await page.unroute('**/api/streams/A/full');await card(page,'A').getByRole('button',{name:'Edit',exact:true}).click();await expect(page.locator('#stream-admin-url')).toHaveValue('http://127.0.0.1:9/admin-a');
  }finally{await context.close();}
});
