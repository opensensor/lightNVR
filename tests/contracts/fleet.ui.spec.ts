/** Real inventory UI and validated reads against an owned native server. */
import {test,expect,ok,admin,viewer} from './permissionCases';
test('inventory renders only visible cameras and refuses malformed query rows until Retry succeeds',async({server,browser})=>{
  const context=await server.browserContext(browser,'viewer');const page=await context.newPage();
  try {
    await page.goto(`${server.baseURL}/streams.html`);await page.locator('#inventory-tab').click();
    const table=page.locator('#inventory-panel table');
    await expect(table.getByText('A',{exact:true})).toBeVisible();await expect(table.getByText('B',{exact:true})).toHaveCount(0);
    await page.route('**/api/fleet/cameras/query',route=>route.fulfill({json:{cameras:[{name:'do-not-render-private'}],total:1}}));
    await page.getByRole('button',{name:'Refresh',exact:true}).click();
    await expect(page.getByText('Invalid queryFleetCameras response',{exact:false})).toBeVisible();
    await expect(page.getByText('do-not-render-private',{exact:false})).toHaveCount(0);await expect(table).toHaveCount(0);
    await page.unroute('**/api/fleet/cameras/query');await page.getByRole('button',{name:'Retry',exact:true}).click();
    await expect(table.getByText('A',{exact:true})).toBeVisible();
    await page.locator('#fleet-search').fill(server.cameras.B.camera_uuid);await expect(table.getByText('A',{exact:true})).toHaveCount(0);await expect(table.getByText('B',{exact:true})).toHaveCount(0);
    expect((await ok(server,'/api/fleet/cameras/query',viewer,'POST',{search:server.cameras.B.camera_uuid})).total).toBe(0);
  }finally{await context.close();}
});

test('malformed collection and saved-view catalogs are unavailable, never partially rendered',async({server,browser})=>{
  const collection=await ok(server,'/api/camera-collections',admin,'POST',{name:'Visible collection',type:'static',shared:true});
  await ok(server,`/api/camera-collections/${collection.uuid}/members`,admin,'PUT',{camera_uuids:[server.cameras.A.camera_uuid]});
  await ok(server,'/api/fleet/views',admin,'POST',{name:'First saved view',selector:{version:1,expression:{op:'all'}}});
  await ok(server,'/api/fleet/views',admin,'POST',{name:'Second saved view',selector:{version:1,expression:{op:'all'}}});
  const context=await server.browserContext(browser);const page=await context.newPage();
  try {
    for(const endpoint of ['camera-collections','fleet/views'])await page.route(`**/api/${endpoint}`,route=>route.fulfill({json:{collections:[{name:'do-not-render-private'}],views:[{name:'do-not-render-private'}]}}));
    await page.goto(`${server.baseURL}/streams.html`);await page.locator('#inventory-tab').click();
    await expect(page.getByRole('alert')).toBeVisible();await expect(page.getByText('do-not-render-private',{exact:false})).toHaveCount(0);
    for(const endpoint of ['camera-collections','fleet/views'])await page.unroute(`**/api/${endpoint}`);
    await page.getByRole('button',{name:'Retry',exact:true}).click();await expect(page.getByRole('alert')).toHaveCount(0);
    await expect(page.getByRole('combobox',{name:'Saved views',exact:true}).locator('option')).toContainText(['First saved view']);await expect(page.getByRole('combobox',{name:'Saved views',exact:true}).locator('option')).toContainText(['Second saved view']);
  }finally{await context.close();}
});
