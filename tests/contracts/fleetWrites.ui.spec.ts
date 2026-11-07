/** A real session and real UI must not treat invalid mutation acknowledgements as success. */
import {test,expect,ok} from './permissionCases';
for (const fullscreen of [false,true]) test(`building plans can be created and renamed through the name dialogs (fullscreen=${fullscreen})`,async({server,browser})=>{
 const context=await server.browserContext(browser),page=await context.newPage();
 try {
  await page.goto(`${server.baseURL}/index.html${fullscreen ? '' : '?surface=plan'}`);
  if(fullscreen) {
   await page.locator('#fullscreen-btn').click();
   await expect(page.locator('#live-page')).toHaveClass(/fullscreen-mode/);
   await page.getByRole('tab',{name:'Plan',exact:true}).click();
  }
  await page.getByRole('button',{name:'Create building plan',exact:true}).click();
  const dialog=page.getByRole('dialog');
  await expect(dialog).toBeVisible();
  await dialog.getByRole('textbox',{name:'Plan name',exact:true}).fill('New building');
  await dialog.getByRole('button',{name:'Create plan',exact:true}).click();
  await expect(dialog).toHaveCount(0);
  await expect(page.locator('.live-plan-toolbar')).toContainText('New building');
  await page.getByRole('button',{name:'Rename',exact:true}).click();
  await dialog.getByRole('textbox',{name:'Plan name',exact:true}).fill('Renamed building');
  await dialog.getByRole('button',{name:'Rename plan',exact:true}).click();
  await expect(dialog).toHaveCount(0);
  expect((await ok(server,'/api/live/plans')).plans.map((plan:{name:string})=>plan.name)).toEqual(['Renamed building']);
  await page.getByRole('button',{name:'Delete',exact:true}).click();
  await dialog.getByRole('button',{name:'Cancel',exact:true}).click();
  await expect(dialog).toHaveCount(0);
  if(fullscreen) await expect(page.locator('#live-page')).toHaveClass(/fullscreen-mode/);
  await page.getByRole('button',{name:'New plan',exact:true}).click();
  await dialog.getByRole('textbox',{name:'Plan name',exact:true}).fill('Second building');
  await dialog.getByRole('button',{name:'Create plan',exact:true}).click();
  await expect(dialog).toHaveCount(0);
  expect((await ok(server,'/api/live/plans')).plans.map((plan:{name:string})=>plan.name).sort()).toEqual(['Renamed building','Second building']);
  await page.getByRole('button',{name:'Rename',exact:true}).click();
  await expect(dialog.getByRole('textbox',{name:'Plan name',exact:true})).toHaveValue('Second building');
  await page.keyboard.press('Escape');
  await expect(dialog).toHaveCount(0);
  if(fullscreen) await expect(page.locator('#live-page')).toHaveClass(/fullscreen-mode/);
 } finally {await context.close();}
});

test('saved-view form remains editable after malformed success and saves after recovery',async({server,browser})=>{
 const context=await server.browserContext(browser);const page=await context.newPage();
 try{
  await page.goto(`${server.baseURL}/streams.html`);await page.locator('#inventory-tab').click();
  await page.getByRole('button',{name:'Save current view',exact:true}).click();
  await page.getByRole('textbox',{name:'View name',exact:true}).fill('Contract view');
  let writes=0;
  await page.route('**/api/fleet/views',async route=>{if(route.request().method()==='POST'){writes++;await route.fulfill({status:201,json:{uuid:'do-not-render-private'}});}else await route.continue();});
  await page.getByRole('button',{name:'Save Changes',exact:true}).click();
  await expect(page.getByText('Invalid createFleetView response',{exact:false})).toBeVisible();
  await expect(page.getByRole('textbox',{name:'View name',exact:true})).toHaveValue('Contract view');
  expect(writes).toBe(1);expect((await ok(server,'/api/fleet/views')).views).toEqual([]);
  await page.unroute('**/api/fleet/views');
  await page.getByRole('button',{name:'Save Changes',exact:true}).click();
  await expect(page.getByRole('textbox',{name:'View name',exact:true})).toHaveCount(0);
  await expect(page.getByRole('combobox',{name:'Saved views',exact:true}).locator('option')).toContainText(['Contract view']);
  expect((await ok(server,'/api/fleet/views')).views).toHaveLength(1);
 }finally{await context.close();}
});
