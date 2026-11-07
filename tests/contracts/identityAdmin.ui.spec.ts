/** Actual UsersView and administration modals, built UI against owned native fixtures. */
import {test,expect,ok,database} from './permissionCases';
import {readFileSync} from 'node:fs';

test('creating an account honors password lock and legacy administrator editing uses policy-safe payload',async({server,browser})=>{
  database(server,"UPDATE users SET authorization_mode='legacy' WHERE id=1");
  const context=await server.browserContext(browser);const page=await context.newPage();
  try{
    await page.goto(`${server.baseURL}/users.html`);
    await page.getByRole('button',{name:'Add User',exact:true}).click();const dialog=page.getByRole('dialog');
    await dialog.getByLabel('Username',{exact:true}).fill('browser_locked');await dialog.getByLabel('Password',{exact:true}).fill('BrowserContract123!');await dialog.getByLabel('Lock Password Changes',{exact:true}).check();
    await dialog.getByRole('button',{name:'Add User',exact:true}).click();await expect(dialog).toHaveCount(0);
    const row=page.getByRole('row').filter({has:page.getByText('browser_locked',{exact:true})});await expect(row).toBeVisible();
    expect((await ok(server,'/api/auth/users')).users.find((u:any)=>u.username==='browser_locked').password_change_locked).toBe(true);
    const adminRow=page.getByRole('row').filter({has:page.getByRole('cell',{name:'admin',exact:true})});await adminRow.getByRole('button',{name:'Edit User',exact:true}).click();
    await page.getByRole('dialog').getByLabel('Email',{exact:true}).fill('browser@example.test');await page.getByRole('dialog').getByRole('button',{name:'Update User',exact:true}).click();
    await expect(page.getByRole('dialog')).toHaveCount(0);expect((await ok(server,'/api/auth/users/1')).email).toBe('browser@example.test');
  }finally{await context.close();}
});

test('malformed user lists and failed write acknowledgments never render payloads or claim success',async({server,browser})=>{
  const context=await server.browserContext(browser);const page=await context.newPage();const secret='DO-NOT-RENDER-ADMIN-SECRET';
  try{
    await page.route('**/api/auth/users',route=>route.fulfill({status:200,contentType:'application/json',body:JSON.stringify({users:[{username:secret}],api_key:secret})}));
    await page.goto(`${server.baseURL}/users.html`);await expect(page.getByText('Invalid listAdminUsers response: AdminUsers (shape).')).toBeVisible();await expect(page.getByText(secret,{exact:false})).toHaveCount(0);
    await page.unroute('**/api/auth/users');await page.getByRole('button',{name:'Retry',exact:true}).click();await expect(page.getByText('contract_viewer',{exact:true})).toBeVisible();
    const before=await ok(server,'/api/auth/users');await page.getByRole('button',{name:'Add User',exact:true}).click();const dialog=page.getByRole('dialog');
    await dialog.getByLabel('Username',{exact:true}).fill('bad_ack');await dialog.getByLabel('Password',{exact:true}).fill('BrowserContract123!');
    let calls=0;await page.route('**/api/auth/users',route=>{if(route.request().method()!=='POST')return route.continue();calls++;return route.fulfill({status:200,contentType:'application/json',body:JSON.stringify({error:secret,id:999})});});
    await dialog.getByRole('button',{name:'Add User',exact:true}).click();await expect(dialog.getByText('Invalid createAdminUser response: AdminUser (shape).',{exact:true})).toBeVisible();expect(calls).toBe(1);await expect(dialog).toBeVisible();
    expect((await ok(server,'/api/auth/users')).users.map((u:any)=>({...u,last_login:0}))).toEqual(before.users.map((u:any)=>({...u,last_login:0})));await expect(page.getByText('User added successfully',{exact:true})).toHaveCount(0);await expect(page.getByText(secret,{exact:false})).toHaveCount(0);
  }finally{await context.close();}
});

test('role manager consumes typed catalog and audit history downloads actual CSV',async({server,browser})=>{
  const context=await server.browserContext(browser);const page=await context.newPage();
  try{
    await page.goto(`${server.baseURL}/users.html`);await page.getByRole('button',{name:'Manage Roles',exact:true}).click();const dialog=page.getByRole('dialog');
    await expect(page.getByRole('heading',{name:'Authorization Roles',exact:true})).toBeVisible();await dialog.getByRole('button',{name:'Create Role',exact:true}).click();
    await dialog.getByLabel('Name',{exact:true}).fill('Browser audit reader');await dialog.getByRole('checkbox',{name:/^system.admin /}).check();
    await dialog.getByRole('button',{name:'Save Changes',exact:true}).click();await expect(page.getByText('Role created',{exact:true})).toBeVisible();
    expect((await ok(server,'/api/authorization/roles')).roles.some((r:any)=>r.name==='Browser audit reader')).toBe(true);
    await dialog.getByRole('button',{name:'Close',exact:true}).click();await page.getByRole('button',{name:'Audit History',exact:true}).click();
    const downloadPromise=page.waitForEvent('download');await page.getByRole('button',{name:'Export this page',exact:true}).click();const download=await downloadPromise;
    expect(download.suggestedFilename()).toMatch(/^lightnvr-audit-.*\.csv$/);const file=await download.path();expect(file).toBeTruthy();expect(readFileSync(file!,'utf8')).toContain('uuid,occurred_at,request_id,principal_user_id,principal_username,');
  }finally{await context.close();}
});

test('policy editor refuses malformed acknowledgment then applies and exposes the real revised policy',async({server,browser})=>{
  const context=await server.browserContext(browser);const page=await context.newPage();
  try{
    const before=await ok(server,'/api/authorization/users/2');
    await page.goto(`${server.baseURL}/users.html`);await page.getByRole('row').filter({has:page.getByText('contract_viewer',{exact:true})}).getByRole('button',{name:'Manage Access',exact:true}).click();
    const dialog=page.getByRole('dialog');await dialog.getByRole('combobox',{name:'Camera scope',exact:true}).selectOption('all');
    let calls=0;await page.route('**/api/authorization/users/2',route=>{if(route.request().method()!=='PUT')return route.continue();calls++;return route.fulfill({status:200,contentType:'application/json',body:'{"policy_version":"private-invalid-version"}'});});
    await dialog.getByRole('button',{name:'Save Changes',exact:true}).click();await expect(page.getByText(/Could not save access policy: Invalid setUserAuthorizationPolicy response/)).toBeVisible();expect(calls).toBe(1);
    expect(await ok(server,'/api/authorization/users/2')).toEqual(before);await expect(page.getByText('Access policy saved',{exact:true})).toHaveCount(0);await expect(page.getByText('private-invalid-version')).toHaveCount(0);
    await page.unroute('**/api/authorization/users/2');await dialog.getByRole('button',{name:'Save Changes',exact:true}).click();await expect(page.getByText('Access policy saved',{exact:true})).toBeVisible();
    const after=await ok(server,'/api/authorization/users/2');expect(after.policy_version).toBeGreaterThan(before.policy_version);expect(after.grants[0].scope.type).toBe('all');
  }finally{await context.close();}
});
