/** Real authentication consumers; no secret-bearing browser trace/screenshot artifacts. */
import {test,expect,database} from './permissionCases';
test.use({trace:'off',screenshot:'off'});
const privateValue='do-not-render-private';
test('login validates config and success, retries configuration, and preserves safe history redirects',async({server,browser})=>{
  const context=await server.browserContext(browser,'anonymous');const page=await context.newPage();
  try {
    await page.route('**/api/auth/login/config',route=>route.fulfill({json:{force_mfa_on_login:'false'}}));
    await page.goto(`${server.baseURL}/login.html?redirect=https://example.invalid/escape`);
    await page.locator('#username').fill('admin');await page.locator('#password').fill('ContractAdmin123!');
    const submit=page.locator('#login-form button[type=submit]');
    await expect(submit).toBeDisabled();await page.unroute('**/api/auth/login/config');await page.getByRole('button',{name:'Retry',exact:true}).click();await expect(submit).toBeEnabled();
    let requests=0;
    await page.route('**/api/auth/login',route=>{requests++;return route.fulfill({json:{success:true,secret:privateValue}});});
    await submit.click();await expect(submit).toBeEnabled();expect(requests).toBe(1);await expect(page).toHaveURL(/login\.html/);await expect(page.getByText(privateValue,{exact:false})).toHaveCount(0);
    await page.unroute('**/api/auth/login');await submit.click();await expect(page).toHaveURL(`${server.baseURL}/index.html`);
    expect((await context.request.get('/api/auth/verify')).status()).toBe(200);
    // History replacement must not revisit LoginView and revoke this session.
    await page.goto(`${server.baseURL}/settings.html`);await page.goBack();await expect(page).toHaveURL(`${server.baseURL}/index.html`);
    expect((await context.request.get('/api/auth/verify')).status()).toBe(200);
  }finally{await context.close();}
});

test('MFA completion cannot navigate on malformed success or expose server diagnostics',async({server,browser})=>{
  const context=await server.browserContext(browser,'anonymous');const page=await context.newPage();
  try {
    await page.route('**/api/auth/login',route=>route.fulfill({json:{totp_required:true,totp_token:'fixture-pending'}}));
    await page.route('**/api/auth/login/totp',route=>route.fulfill({json:{success:false,error:privateValue}}));
    await page.goto(`${server.baseURL}/login.html`);await page.locator('#username').fill('admin');await page.locator('#password').fill('ContractAdmin123!');
    await page.locator('#login-form button[type=submit]').click();
    await page.locator('#totp-code').fill('123456');await page.locator('#totp-form button[type=submit]').click();
    await expect(page.locator('#totp-code')).toHaveValue('');await expect(page).toHaveURL(/login\.html/);
    await expect(page.getByText(privateValue,{exact:false})).toHaveCount(0);
    await page.getByRole('button',{name:'Back to Login',exact:false}).click();await expect(page.locator('#login-form')).toBeVisible();
  }finally{await context.close();}
});

test('MFA status and mutation errors require refresh instead of claiming disabled or enabled',async({server,browser})=>{
  const context=await server.browserContext(browser);const page=await context.newPage();
  try {
    const status='**/api/auth/users/2/totp/status',setup='**/api/auth/users/2/totp/setup',verify='**/api/auth/users/2/totp/verify',disable='**/api/auth/users/2/totp/disable';
    await page.route(status,route=>route.fulfill({json:{totp_enabled:'false',secret:privateValue}}));
    await page.goto(`${server.baseURL}/users.html`);
    await page.getByRole('row').filter({has:page.getByText('contract_viewer',{exact:true})}).getByRole('button',{name:'Manage MFA'}).click();
    await expect(page.getByRole('button',{name:'Retry',exact:true})).toBeVisible();
    await expect(page.getByRole('button',{name:'Set Up Two-Factor Authentication'})).toHaveCount(0);
    await page.unroute(status);await page.getByRole('button',{name:'Retry',exact:true}).click();
    const start=page.getByRole('button',{name:'Set Up Two-Factor Authentication'});
    await expect(start).toBeVisible();
    await page.route(setup,route=>route.fulfill({json:{success:true,secret:privateValue,otpauth_uri:'https://example.invalid'}}));
    await start.click();await expect(page.getByRole('button',{name:'Retry',exact:true})).toBeVisible();await expect(page.getByText(privateValue,{exact:false})).toHaveCount(0);
    await page.unroute(setup);await page.getByRole('button',{name:'Retry',exact:true}).click();await start.click();
    await page.route(verify,route=>route.fulfill({json:{success:false,error:privateValue}}));
    await page.getByPlaceholder('000000').fill('123456');await page.getByRole('button',{name:'Verify & Enable'}).click();
    await expect(page.getByRole('button',{name:'Retry',exact:true})).toBeVisible();await expect(page.locator('canvas')).toHaveCount(0);
    database(server,'UPDATE users SET totp_enabled=1 WHERE id=2');
    await page.getByRole('button',{name:'Retry',exact:true}).click();
    await page.route(disable,route=>route.fulfill({json:{success:false,error:privateValue}}));
    const turnOff=page.getByRole('button',{name:'Disable Two-Factor Authentication'});
    await turnOff.click();await expect(page.getByRole('button',{name:'Retry',exact:true})).toBeVisible();await expect(start).toHaveCount(0);
    await page.unroute(disable);await page.getByRole('button',{name:'Retry',exact:true}).click();await turnOff.click();await expect(start).toBeVisible();
    await expect(page.getByText(privateValue,{exact:false})).toHaveCount(0);
  }finally{await context.close();}
});

test('forced password change requires a validated acknowledgment before leaving the gate',async({server,browser})=>{
  database(server,'UPDATE users SET must_change_password=1 WHERE id=1');
  const context=await server.browserContext(browser);const page=await context.newPage();
  try {
    await page.route('**/api/auth/users/1/password',route=>route.fulfill({json:{success:true,error:privateValue}}));
    await page.goto(`${server.baseURL}/index.html`);
    await page.locator('#required-current-password').fill('ContractAdmin123!');await page.locator('#required-new-password').fill('ChangedContract456!');await page.locator('#required-confirm-password').fill('ChangedContract456!');
    await page.getByRole('button',{name:'Change password',exact:true}).click();await expect(page.getByRole('alert')).toBeVisible();
    await expect(page.locator('#required-current-password')).toBeVisible();await expect(page.getByText(privateValue,{exact:false})).toHaveCount(0);
    await page.unroute('**/api/auth/users/1/password');await page.getByRole('button',{name:'Change password',exact:true}).click();
    await expect(page).toHaveURL(/login\.html\?password_changed=true/);expect((await context.request.get('/api/auth/verify')).status()).toBe(401);
  }finally{await context.close();}
});


test('closing enrollment discards a delayed secret response before another account opens',async({server,browser})=>{
  const context=await server.browserContext(browser);const page=await context.newPage();
  let release!:()=>void,started!:()=>void;
  const waiting=new Promise<void>(resolve=>{release=resolve;}),entered=new Promise<void>(resolve=>{started=resolve;});
  const enrollmentSecret='JBSWY3DPEHPK3PXPJBSWY3DPEHPK3PXP';
  try {
    await page.route('**/api/auth/users/2/totp/setup',async route=>{
      started();await waiting;
      await route.fulfill({json:{success:true,secret:enrollmentSecret,otpauth_uri:`otpauth://totp/fixture?secret=${enrollmentSecret}`,message:'Scan'}}).catch(()=>{});
    });
    await page.goto(`${server.baseURL}/users.html`);
    await page.getByRole('row').filter({has:page.getByText('contract_viewer',{exact:true})}).getByRole('button',{name:'Manage MFA'}).click();
    await page.getByRole('button',{name:'Set Up Two-Factor Authentication'}).click();await entered;
    await page.getByRole('button',{name:'Close',exact:true}).click();
    await page.getByRole('row').filter({has:page.getByRole('cell',{name:'admin',exact:true})}).getByRole('button',{name:'Manage MFA'}).click();
    release();await expect(page.getByRole('heading',{name:'MFA Setup — admin'})).toBeVisible();
    await expect(page.getByRole('button',{name:'Set Up Two-Factor Authentication'})).toBeVisible();
    await expect(page.getByText(enrollmentSecret,{exact:false})).toHaveCount(0);await expect(page.locator('canvas')).toHaveCount(0);
  }finally{release();await context.close();}
});
