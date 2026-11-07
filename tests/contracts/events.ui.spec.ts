import {test,expect,ok,admin} from './permissionCases';
test('event destination editor rejects malformed acknowledgment and preserves retry intent',async({server,browser})=>{
 const context=await server.browserContext(browser),page=await context.newPage();
 try {
  await page.goto(`${server.baseURL}/streams.html`);await page.locator('#events-tab').click();
  await page.getByRole('tab',{name:/Destinations/}).click();
  await page.getByRole('button',{name:'Add destination',exact:true}).first().click();
  const dialog=page.getByRole('dialog');await dialog.getByLabel('Name',{exact:true}).fill('Browser destination');
  await dialog.getByPlaceholder('mqtt.example.net').fill('127.0.0.1');
  let writes=0;await page.route('**/api/event-destinations',route=>{
   if(route.request().method()!=='POST')return route.continue();
   writes++;return route.fulfill({status:201,json:{uuid:'do-not-render-private',name:'private'}});
  });
  await dialog.getByRole('button',{name:'Create destination',exact:true}).click();
  await expect(page.getByText('Invalid createEventDestination response',{exact:false}).first()).toBeVisible();
  await expect(dialog).toBeVisible();expect(writes).toBe(1);expect((await ok(server,'/api/event-destinations')).count).toBe(0);
  await page.unroute('**/api/event-destinations');await dialog.getByRole('button',{name:'Create destination',exact:true}).click();
  await expect(dialog).toHaveCount(0);await expect(page.getByText('Browser destination',{exact:true})).toBeVisible();
 }finally{await context.close();}
});
test('editing an existing route previews without sending its mutation revision',async({server,browser})=>{
 await ok(server,'/api/event-routes',admin,'POST',{name:'Existing route',event_types:['io.lightnvr.detection.object.v1']});
 const context=await server.browserContext(browser),page=await context.newPage();
 try {
  await page.goto(`${server.baseURL}/streams.html`);await page.locator('#events-tab').click();
  await page.getByRole('button',{name:'Edit',exact:true}).click();
  const pending=page.waitForResponse(response=>response.url().endsWith('/api/event-routes/preview'));
  await page.getByRole('dialog').getByRole('button',{name:'Preview route',exact:true}).click();
  const response=await pending;expect(response.status()).toBe(200);expect(response.request().postDataJSON()).not.toHaveProperty('revision');
  await expect(page.getByRole('dialog').getByText('A',{exact:true})).toBeVisible();
  expect((await ok(server,'/api/event-routes')).routes[0].revision).toBe(1);
 }finally{await context.close();}
});
