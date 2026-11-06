/** F1d canonical/projected identity contract acceptance, independent of generator output. */
import {createHmac} from 'node:crypto';
import {test,expect,admin,viewer,anonymous,wire,ok,createUser,token,policy,database,operationIds,projectedResponse,valid,identityManagementOps,lintProjections,discovery,type Server} from './permissionCases';
function otp(secret:string) {
  const alphabet='ABCDEFGHIJKLMNOPQRSTUVWXYZ234567';
  const bits=[...secret].map(c=>alphabet.indexOf(c).toString(2).padStart(5,'0')).join('');
  const key=Buffer.from(bits.match(/.{8}/g)!.map(b=>parseInt(b,2)));
  const counter=Buffer.alloc(8);counter.writeBigUInt64BE(BigInt(Math.floor(Date.now()/30000)));
  const digest=createHmac('sha1',key).update(counter).digest();
  return ((digest.readUInt32BE(digest[19]!&15)&0x7fffffff)%1000000).toString().padStart(6,'0');
}
const pathFor=(suffix:string)=>`/api/auth/users/{id}/${suffix}`;
test('identity discovery distinguishes self-service, scoped managers and lost owner permission',async({server},info)=>{
  const own=await discovery(server,viewer,info,'self');
  for(const id of identityManagementOps)expect(operationIds(own)).toContain(id);
  expect(own.components.schemas).not.toHaveProperty('PasswordAdminReset');
  const mine=await createUser(server,'identity_projection');
  const managed=await token(server,admin,{type:'all'},['users.manage']);managed.identityManagement=true;
  const doc=await discovery(server,managed,info,'manager');
  expect(doc.components.schemas).toHaveProperty('PasswordAdminReset');
  expect(operationIds(doc)).not.toContain('listAuthSessions');
  const status=await wire(server,`/api/auth/users/${mine.id}/totp/status`,managed);
  expect(status.response.status).toBe(200);projectedResponse(doc,pathFor('totp/status'),'get',200,status.body);
  const denied=await token(server,admin,{type:'all'},['live.view']);
  const narrow=await discovery(server,denied,info,'narrow');
  for(const id of identityManagementOps)expect(operationIds(narrow)).not.toContain(id);
  expect((await wire(server,`/api/auth/users/${admin.id}/totp/status`,denied)).response.status).toBe(403);
  // Revoking the owner's effective permission removes delegated token authority.
  const owner=await createUser(server,'identity_delegation',{role:0});
  const delegated=await token(server,owner,{type:'all'},['users.manage']);
  expect((await wire(server,`/api/auth/users/${mine.id}/totp/status`,delegated)).response.status).toBe(200);
  await policy(server,owner,[]);
  const after=await ok(server,'/api/openapi.json',delegated);
  for(const id of identityManagementOps)expect(operationIds(after)).not.toContain(id);
  expect((await wire(server,`/api/auth/users/${mine.id}/totp/status`,delegated)).response.status).toBe(403);
  lintProjections(info);
});

test('enrollment, completion, disable and password responses satisfy projected contracts and revoke credentials',async({server},info)=>{
  const user=await createUser(server,'identity_contract');
  const doc=await discovery(server,user,info,'owner');
  const setup=await wire(server,`/api/auth/users/${user.id}/totp/setup`,user,'POST');
  expect(setup.response.status).toBe(200);valid('TotpSetupResponse',setup.body);projectedResponse(doc,pathFor('totp/setup'),'post',200,setup.body);
  const uri=new URL(setup.body.otpauth_uri);expect(uri.protocol).toBe('otpauth:');expect(uri.searchParams.get('secret')).toBe(setup.body.secret);
  const verified=await wire(server,`/api/auth/users/${user.id}/totp/verify`,user,'POST',{code:otp(setup.body.secret)});
  expect(verified.response.status).toBe(200);projectedResponse(doc,pathFor('totp/verify'),'post',200,verified.body);
  const status=await ok(server,`/api/auth/users/${user.id}/totp/status`,user);
  projectedResponse(doc,pathFor('totp/status'),'get',200,status);expect(status).toEqual({totp_enabled:true,totp_configured:true});
  // Deterministic fixture reset of replay history permits a distinct login step
  // without waiting for a wall-clock boundary. Replay rejection is covered in F1c.
  database(server,'UPDATE users SET totp_last_used_step=-1 WHERE id=?',[user.id!]);
  const challenge=await ok(server,'/api/auth/login',anonymous,'POST',{username:user.username,password:user.password});valid('LoginMfaChallenge',challenge);
  const completed=await wire(server,'/api/auth/login/totp',anonymous,'POST',{totp_token:challenge.totp_token,code:otp(setup.body.secret),remember_device:true});
  expect(completed.response.status).toBe(200);projectedResponse(doc,'/api/auth/login/totp','post',200,completed.body);
  expect(completed.response.headers.getSetCookie().some(c=>c.startsWith('session=')&&c.includes('HttpOnly'))).toBe(true);
  const cookie=completed.response.headers.getSetCookie().find(c=>c.startsWith('session='))!.split(';')[0]!;
  const disabled=await wire(server,`/api/auth/users/${user.id}/totp/disable`,user,'POST');
  expect(disabled.response.status).toBe(200);projectedResponse(doc,pathFor('totp/disable'),'post',200,disabled.body);
  expect((await wire(server,'/api/auth/verify',{headers:{Cookie:cookie}})).response.status).toBe(401);
  const password=await wire(server,`/api/auth/users/${user.id}/password`,user,'PUT',{old_password:user.password,new_password:'ContractChanged456!'});
  expect(password.response.status).toBe(200);projectedResponse(doc,pathFor('password'),'put',200,password.body);
  expect((await wire(server,'/api/auth/login',anonymous,'POST',{username:user.username,password:user.password})).response.status).toBe(401);
  expect((await wire(server,'/api/auth/login',anonymous,'POST',{username:user.username,password:'ContractChanged456!'})).response.status).toBe(200);
  lintProjections(info);
});

test('legacy logout clears cookies, revokes the actual session and supports its negotiated JSON envelope',async({server,playwright})=>{
  const context=await playwright.request.newContext({baseURL:server.baseURL});
  try {
    expect((await context.post('/api/auth/login',{data:server.credentials.admin})).status()).toBe(200);
    const response=await context.get('/logout',{maxRedirects:0});
    expect(response.status()).toBe(302);expect(response.headers().location).toBe('/login.html?logout=true');expect(await response.text()).toBe('');
    expect(response.headersArray().filter(h=>h.name.toLowerCase()==='set-cookie').every(h=>h.value.includes('Max-Age=0'))).toBe(true);
    expect((await context.get('/api/auth/verify')).status()).toBe(401);
    const json=await context.get('/logout',{headers:{Accept:'application/json'}});expect(json.status()).toBe(200);valid('LogoutResponse',await json.json());
  }finally{await context.dispose();}
});
