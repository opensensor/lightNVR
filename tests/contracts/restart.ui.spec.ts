import {test,expect} from './permissionCases';

test('lost restart acknowledgment keeps reconnecting without resubmitting the write',async({server,browser})=>{
 const context=await server.browserContext(browser),page=await context.newPage();
 try {
  let writes=0,statusReads=0,reloads=0;
  await page.route('**/api/system/restart',route=>{writes++;return route.abort('connectionreset');});
  await page.route('**/api/system/status',route=>{
   statusReads++;
   return statusReads===1?route.abort('connectionrefused'):route.fulfill({json:{status:'ok',message:'System running'}});
  });
  await page.goto(`${server.baseURL}/system.html`);
  page.on('framenavigated',frame=>{if(frame===page.mainFrame())reloads++;});
  await page.locator('#restart-btn').click();
  await page.getByRole('button',{name:'Restart',exact:true}).click();
  await expect(page.getByRole('heading',{name:'Restarting LightNVR',exact:true})).toBeVisible();
  await expect.poll(()=>statusReads).toBeGreaterThanOrEqual(2);
  await expect.poll(()=>reloads).toBe(1);
  await expect(page.getByRole('heading',{name:'Restarting LightNVR',exact:true})).toHaveCount(0);
  expect(writes).toBe(1);
 }finally{await context.close();}
});

test('denied restart returns to confirmation instead of polling indefinitely',async({server,browser})=>{
 const context=await server.browserContext(browser),page=await context.newPage();
 try {
  let writes=0;
  await page.route('**/api/system/restart',route=>{writes++;return route.fulfill({status:403,json:{error:'Denied'}});});
  await page.goto(`${server.baseURL}/system.html`);
  await page.locator('#restart-btn').click();
  await page.getByRole('button',{name:'Restart',exact:true}).click();
  await expect(page.getByText('Error restarting system: restartSystem failed (HTTP 403).',{exact:true})).toBeVisible();
  await expect(page.getByRole('button',{name:'Restart',exact:true})).toBeEnabled();
  expect(writes).toBe(1);
 }finally{await context.close();}
});
