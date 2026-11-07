/** Built production investigation/bookmark/LPR consumers against owned native fixtures. */
import {test,expect,ok} from './permissionCases';
import {seedRecordings} from './fixtures/recordings/seed';
test('actual investigation loads media, saves a revisioned bookmark and deletes it',async({server,browser})=>{
 const fixture=seedRecordings(server),context=await server.browserContext(browser),page=await context.newPage();
 try{
  await page.goto(`${server.baseURL}/investigation.html?cameras=${server.cameras.A.camera_uuid}&start=${fixture.start-1}&end=${fixture.end+1}&cursor=${fixture.start+1}`);
  await expect(page.locator('.investigation-player').first()).toBeVisible();await expect(page.getByRole('button',{name:'Save bookmark',exact:true})).toBeEnabled();
  await page.getByRole('button',{name:'Save bookmark',exact:true}).click();const dialog=page.getByRole('dialog');await dialog.getByLabel('Bookmark name',{exact:true}).fill('Browser contract evidence');await dialog.getByLabel('Private note (optional)',{exact:true}).fill('Synthetic browser note');await dialog.getByRole('button',{name:'Save bookmark',exact:true}).click();await expect(dialog).toHaveCount(0);
  const saved=await ok(server,'/api/investigation-bookmarks');expect(saved.count).toBe(1);expect(saved.bookmarks[0]).toMatchObject({title:'Browser contract evidence',holds_recordings:false});
  await page.getByRole('button',{name:'Bookmarks',exact:true}).click();await expect(page.getByRole('heading',{name:'Browser contract evidence',exact:true})).toBeVisible();await dialog.getByRole('button',{name:'Delete',exact:true}).click();await dialog.getByRole('button',{name:'Delete bookmark?',exact:true}).click();await expect(page.getByText('You have no saved investigation bookmarks.')).toBeVisible();expect((await ok(server,'/api/investigation-bookmarks')).count).toBe(0);
 }finally{await context.close();}
});
test('actual bookmark screen rejects malformed reads and write acknowledgments without success or disclosure',async({server,browser})=>{
 const fixture=seedRecordings(server),context=await server.browserContext(browser),page=await context.newPage(),secret='DO-NOT-RENDER-EVIDENCE-SECRET';
 try{
  await page.goto(`${server.baseURL}/investigation.html?cameras=${server.cameras.A.camera_uuid}&start=${fixture.start-1}&end=${fixture.end+1}&cursor=${fixture.start+1}`);await expect(page.getByRole('button',{name:'Save bookmark',exact:true})).toBeEnabled();
  await page.route('**/api/investigation-bookmarks',route=>route.fulfill({status:200,contentType:'application/json',body:JSON.stringify({bookmarks:[{title:secret}],count:1})}));
  await page.getByRole('button',{name:'Bookmarks',exact:true}).click();await expect(page.getByRole('alert')).toContainText('Invalid listEvidenceBookmarks response');await expect(page.getByText(secret,{exact:false})).toHaveCount(0);await page.getByRole('dialog').getByRole('button',{name:'Close',exact:true}).click();
  await page.unroute('**/api/investigation-bookmarks');let calls=0;await page.route('**/api/investigation-bookmarks',route=>{if(route.request().method()!=='POST')return route.continue();calls++;return route.fulfill({status:201,contentType:'application/json',body:JSON.stringify({title:secret})});});
  await page.getByRole('button',{name:'Save bookmark',exact:true}).click();const dialog=page.getByRole('dialog');await dialog.getByLabel('Bookmark name',{exact:true}).fill('Invalid acknowledgment');await dialog.getByRole('button',{name:'Save bookmark',exact:true}).click();await expect(dialog.getByRole('alert')).toContainText('Invalid createEvidenceBookmark response');expect(calls).toBe(1);expect((await ok(server,'/api/investigation-bookmarks')).count).toBe(0);await expect(page.getByText(secret,{exact:false})).toHaveCount(0);
 }finally{await context.close();}
});
test('actual plates screen refuses malformed sensitive search results',async({server,browser})=>{
 const context=await server.browserContext(browser),page=await context.newPage(),secret='DO-NOT-RENDER-PLATE';
 try{
  await page.route('**/api/lpr/search',route=>route.fulfill({status:200,contentType:'application/json',body:JSON.stringify({reads:[{plate:secret}],count:1})}));
  await page.goto(`${server.baseURL}/plates.html`);await expect(page.getByLabel('Camera',{exact:true})).not.toHaveValue('');await page.getByRole('button',{name:'Search',exact:true}).click();await expect(page.getByText('The plate search failed. Try again.',{exact:true})).toBeVisible();await expect(page.getByText(secret,{exact:false})).toHaveCount(0);
 }finally{await context.close();}
});
