import {mkdir} from 'node:fs/promises';
import path from 'node:path';
import {test,expect,ok,admin} from './permissionCases';
test('storage settings reject malformed target catalogs and recover through Retry',async({server,browser})=>{
 const directory=path.join(server.directory,'visible-storage');await mkdir(directory);await ok(server,'/api/storage-targets',admin,'POST',{name:'Owned UI target',root_path:directory});
 const context=await server.browserContext(browser),page=await context.newPage();
 try{
  await page.route('**/api/storage-targets',route=>route.fulfill({json:{targets:[{name:'do-not-render-private'}]}}));
  await page.goto(`${server.baseURL}/settings.html`);await page.locator('#settings-tab-storage').click();
  await expect(page.getByText('Invalid listStorageTargets response',{exact:false}).first()).toBeVisible();await expect(page.getByText('do-not-render-private')).toHaveCount(0);
  await page.unroute('**/api/storage-targets');await page.getByRole('button',{name:'Retry',exact:true}).first().click();
  await expect(page.getByText('Owned UI target',{exact:true}).first()).toBeVisible();
 }finally{await context.close();}
});
