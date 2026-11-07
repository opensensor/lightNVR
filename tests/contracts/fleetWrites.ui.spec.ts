/** A real session and real UI must not treat invalid mutation acknowledgements as success. */
import {test,expect,ok} from './permissionCases';
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
