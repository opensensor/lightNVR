import {test,expect} from '@playwright/test';
import {startFixture} from './fixture-server.mjs';

test('create consumer preserves intent and form after malformed acknowledgment, then creates through native server',async({browser})=>{
 const server=await startFixture();const context=await server.browserContext(browser);const page=await context.newPage();let failed=true;
 try{
  await page.goto(`${server.baseURL}/streams.html`);
  const camera=page.getByRole('listitem').filter({has:page.locator('h3[title="A"]')});await expect(camera).toBeVisible();
  await camera.getByRole('button',{name:'Edit',exact:true}).click();await expect(page.locator('#stream-name')).toHaveValue('A');
  await page.locator('#stream-modal').getByRole('button',{name:'Cancel',exact:true}).click();
  await page.getByRole('button',{name:'Add Stream',exact:true}).click();
  await page.locator('#stream-name').fill('BrowserOwned');await page.locator('#stream-url').fill('rtsp://127.0.0.1:9/browser-owned');await page.locator('#stream-enabled').uncheck();
  let postCount=0,putCount=0;page.on('request',r=>{if(new URL(r.url()).pathname==='/api/streams'&&r.method()==='POST')postCount++;if(r.method()==='PUT'&&new URL(r.url()).pathname==='/api/streams/A')putCount++;});
  await page.route('**/api/streams',route=>route.request().method()==='POST'?route.fulfill({status:201,contentType:'application/json',body:JSON.stringify({success:true,private:'secret-wire-value'})}):route.continue());
  await page.locator('#stream-save-btn').click();await expect(page.getByText(/^Error adding stream: createStream response failed/)).toBeVisible();
  await expect(page.locator('#stream-modal')).toBeVisible();await expect(page.locator('#stream-name')).toHaveValue('BrowserOwned');expect(postCount).toBe(1);expect(putCount).toBe(0);await expect(page.getByText('secret-wire-value',{exact:false})).toHaveCount(0);
  await page.unroute('**/api/streams');await page.locator('#stream-save-btn').click();await expect(page.locator('#stream-modal')).toHaveCount(0);
  await expect.poll(async()=> (await server.request('/api/streams/BrowserOwned')).status).toBe(200);expect(postCount).toBe(2);expect(putCount).toBe(0);
  const original=await server.json<{stream:{camera_uuid:string}}>('/api/streams/A/full');expect(original.stream.camera_uuid).toBe(server.cameras.A.camera_uuid);
  failed=false;
 }finally{await context.close();await server.stop({failed});}
});
