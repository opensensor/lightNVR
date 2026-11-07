import {test as base,expect} from '@playwright/test';
import {startFixture,type ContractFixture} from './fixture-server.mjs';
const test=base.extend<{server:ContractFixture}>({server:async({},use,info)=>{const server=await startFixture({webRoot:process.env.LIGHTNVR_CONTRACT_WEB_ROOT});try{await use(server);}finally{await server.stop({failed:info.status!==info.expectedStatus});}}});
test('ONVIF discovery consumer rejects malformed device rows and recovers without device effects',async({server,browser})=>{
 const context=await server.browserContext(browser);const page=await context.newPage();let calls=0;
 try{
  await page.goto(`${server.baseURL}/streams.html`);await page.locator('#discover-onvif-btn').click();
  await page.route('**/api/onvif/discovery/discover',route=>{calls++;return route.fulfill({status:200,contentType:'application/json',body:JSON.stringify({devices:[{manufacturer:'private-wire-marker',ip_address:'127.0.0.1'}]})});});
  await page.locator('#discover-btn').click();await expect(page.getByText(/Invalid discoverOnvifDevices response/)).toBeVisible();await expect(page.getByText('private-wire-marker')).toHaveCount(0);expect(calls).toBe(1);
  await page.unroute('**/api/onvif/discovery/discover');await page.route('**/api/onvif/discovery/discover',route=>route.fulfill({status:200,contentType:'application/json',body:'{"devices":[]}'}));
  await page.locator('#discover-btn').click();await expect(page.locator('#discover-btn')).toBeEnabled();await expect(page.getByText('No devices discovered yet')).toBeVisible();
 }finally{await context.close();}
});
