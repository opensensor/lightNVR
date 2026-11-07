import {test,expect,ok} from './permissionCases';
test('settings rejects malformed load and write acknowledgments, then recovers explicitly',async({server,browser})=>{
 const context=await server.browserContext(browser),page=await context.newPage();
 try {
  await page.route('**/api/settings',route=>route.fulfill({json:{storage_path:'do-not-render-private'}}));
  await page.goto(`${server.baseURL}/settings.html`);
  await expect(page.getByRole('alert').filter({hasText:'Invalid getSystemSettings response'})).toBeVisible();
  await expect(page.locator('#save-settings-btn')).toHaveCount(0);
  await expect(page.getByText('do-not-render-private')).toHaveCount(0);
  await page.unroute('**/api/settings');await page.getByRole('button',{name:'Retry',exact:true}).click();
  await expect(page.locator('#setting-log-level')).toBeVisible();
  const before=await ok(server,'/api/settings');
  await page.locator('#setting-log-level').selectOption(String(before.log_level===3?2:3));
  let writes=0;
  await page.route('**/api/settings',route=>{
   if(route.request().method()!=='POST')return route.continue();
   writes++;return route.fulfill({json:{success:true,message:'do-not-render-private'}});
  });
  await page.locator('#save-settings-btn').click();
  await expect(page.getByText('Error saving settings: Invalid updateSystemSettings response',{exact:false})).toBeVisible();
  expect(writes).toBe(1);expect((await ok(server,'/api/settings')).log_level).toBe(before.log_level);
  await expect(page.getByText('do-not-render-private',{exact:false})).toHaveCount(0);
  await page.unroute('**/api/settings');await page.locator('#save-settings-btn').click();
  await expect(page.locator('#save-settings-btn')).toBeDisabled();
 } finally {await context.close();}
});
test('system view rejects malformed information and Retry loads the real server',async({server,browser})=>{
 const context=await server.browserContext(browser),page=await context.newPage();
 try {
  await page.route('**/api/system/info',route=>route.fulfill({json:{version:'do-not-render-private'}}));
  await page.goto(`${server.baseURL}/system.html`);
  await expect(page.getByRole('alert').filter({hasText:'Invalid getSystemInformation response'})).toBeVisible();
  await expect(page.getByText('do-not-render-private')).toHaveCount(0);
  await page.unroute('**/api/system/info');await page.getByRole('button',{name:'Retry',exact:true}).click();
  await expect(page.getByRole('alert').filter({hasText:'Invalid getSystemInformation response'})).toHaveCount(0);
  await expect(page.locator('#system-panel')).toBeVisible();
 } finally {await context.close();}
});
