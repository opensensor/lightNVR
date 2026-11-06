/** Actual SettingsView, generated validators and owned server in Chromium. */
import {test,expect,wire,admin} from './permissionCases';
import {seedCredentials,credentialInventory} from './credentialFixture';

test('Settings revokes owned sessions/devices through the validated adapter and preserves other owners',async({server,browser})=>{
  const mine=seedCredentials(server,1)[0]!,other=seedCredentials(server,2)[0]!;
  const context=await server.browserContext(browser);const page=await context.newPage();
  try {
    await page.goto(`${server.baseURL}/settings.html#auth`);
    const sessions=page.getByRole('list',{name:'Active Sessions'}),devices=page.getByRole('list',{name:'Trusted Devices',exact:true});
    const session=sessions.getByRole('listitem').filter({has:page.getByText(`Session #${mine.session}`,{exact:true})});
    const device=devices.getByRole('listitem').filter({has:page.getByText(`Trusted device #${mine.device}`,{exact:true})});
    await expect(session).toBeVisible();await expect(device).toBeVisible();
    await session.getByRole('button',{name:'Revoke',exact:true}).click();await expect(session).toHaveCount(0);
    expect((await wire(server,'/api/auth/verify',{headers:{Cookie:`session=${mine.sessionToken}`}})).response.status).toBe(401);
    await device.getByRole('button',{name:'Revoke',exact:true}).click();await expect(device).toHaveCount(0);
    const state=credentialInventory(server);expect(state.sessions.some(row=>row.id===other.session)).toBe(true);expect(state.devices.some(row=>row.id===other.device)).toBe(true);
    const current=sessions.getByRole('listitem').filter({has:page.getByText('Current session',{exact:true})});
    await current.getByRole('button',{name:'Revoke',exact:true}).click();
    await expect(page).toHaveURL(/login\.html/);expect((await context.request.get('/api/auth/verify')).status()).toBe(401);
  }finally{await context.close();}
});

test('malformed credential lists fail usefully without rendering private payloads',async({server,browser})=>{
  const context=await server.browserContext(browser);const page=await context.newPage();
  try {
    await page.route('**/api/auth/sessions',route=>route.fulfill({status:200,contentType:'application/json',body:JSON.stringify({sessions:[{id:999,user_agent:'do-not-render-private'}]})}));
    await page.route('**/api/auth/trusted-devices',route=>route.fulfill({status:200,contentType:'application/json',body:JSON.stringify({trusted_devices:null,token:'do-not-render-private'})}));
    await page.goto(`${server.baseURL}/settings.html#auth`);
    await expect(page.getByRole('alert').filter({hasText:'Invalid listAuthSessions response'})).toBeVisible();
    await expect(page.getByRole('alert').filter({hasText:'Invalid listTrustedDevices response'})).toBeVisible();
    await expect(page.getByText('do-not-render-private',{exact:false})).toHaveCount(0);
    await expect(page.getByRole('list',{name:'Active Sessions'}).getByRole('listitem')).toHaveCount(0);
    await page.unroute('**/api/auth/sessions');await page.unroute('**/api/auth/trusted-devices');await page.reload();
    await expect(page.getByText('Current session',{exact:true})).toBeVisible();
  }finally{await context.close();}
});

test('failed or malformed revoke acknowledgments never show success or remove unchanged rows',async({server,browser})=>{
  const mine=seedCredentials(server,1)[0]!;
  const context=await server.browserContext(browser);const page=await context.newPage();
  try {
    await page.goto(`${server.baseURL}/settings.html#auth`);
    for(const [route,id,label,success] of [['sessions',mine.session,`Session #${mine.session}`,'Session revoked'],['trusted-devices',mine.device,`Trusted device #${mine.device}`,'Trusted device revoked']] as const) {
      const row=page.getByRole('listitem').filter({has:page.getByText(label,{exact:true})});
      await expect(row).toBeVisible();
      for(const status of [503,200]) {
        let requests=0;const before=credentialInventory(server);
        await page.route(`**/api/auth/${route}/${id}`,async route=>{requests++;await route.fulfill({status,contentType:'application/json',body:JSON.stringify({error:'do-not-render-private',success:false})});});
        await row.getByRole('button',{name:'Revoke',exact:true}).click();
        const operation=route==='sessions'?'revokeAuthSession':'revokeTrustedDevice';
        await expect(row.getByText(status===503?new RegExp(`^${operation} failed`):new RegExp(`^Invalid ${operation} response`))).toBeVisible();
        await expect(row.getByRole('button',{name:'Revoke',exact:true})).toBeEnabled();
        expect(requests).toBe(1);expect(credentialInventory(server)).toEqual(before);
        await expect(page.getByText(success,{exact:true})).toHaveCount(0);await expect(page.getByText('do-not-render-private',{exact:false})).toHaveCount(0);
        await page.unroute(`**/api/auth/${route}/${id}`);
      }
    }
    expect((await wire(server,'/api/auth/sessions',admin)).response.status).toBe(200);
  }finally{await context.close();}
});
