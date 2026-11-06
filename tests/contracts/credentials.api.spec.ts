/** Independent self-service oracle; these four handlers never accept scoped tokens. */
import {seedCredentials as seed,credentialInventory as inventory} from './credentialFixture';
import {test,expect,admin,anonymous,wire,ok,createUser,token,policy,database,discovery,lintProjections,valid,type Server,type Identity} from './permissionCases';

const routes=[['/api/auth/sessions','sessions','AuthSessionsResponse'],['/api/auth/trusted-devices','trusted_devices','TrustedDevicesResponse']] as const;
function privateResponse(response:Response) {
  const directives=response.headers.get('cache-control')!.split(',').map(value=>value.trim());
  expect(directives).toEqual(expect.arrayContaining(['private','no-store']));
  expect(directives).not.toContain('public');expect(response.headers.get('etag')).toBeNull();
}
function credential(row:ReturnType<typeof seed>[number],route:string) {return route.endsWith('sessions')?row.session:row.device;}

test('self-service credential discovery and handlers agree for Basic, legacy keys, cookies and scoped tokens',async({server},info)=>{
  const owner=await createUser(server,'credential_owner'),other=await createUser(server,'credential_other');
  const mine=seed(server,owner.id!)[0]!,theirs=seed(server,other.id!)[0]!;
  const legacy=await ok(server,`/api/auth/users/${owner.id}/api-key`,admin,'POST');
  await policy(server,owner,[]); // No administrative/camera grants are needed for own credentials.
  const identities:Identity[]=[owner,{id:owner.id,headers:{Cookie:`session=${mine.sessionToken}`}},{id:owner.id,headers:{'X-API-Key':legacy.api_key}},{id:owner.id,headers:{Authorization:`Bearer ${legacy.api_key}`}}];
  for(const [index,identity] of identities.entries()) {
    const doc=await discovery(server,identity,info,`owner-${index}`);
    for(const [route,key,schema] of routes) {
      expect(doc.paths[route].get.security).toEqual([{sessionCookie:[]},{basicAuth:[]},{legacyApiKeyHeader:[]},{legacyBearer:[]}]);
      const result=await wire(server,route,identity);expect(result.response.status).toBe(200);privateResponse(result.response);valid(schema,result.body);
      expect(result.body[key].map((row:{id:number})=>row.id)).toEqual([credential(mine,route)]);
      expect(result.body[key].some((row:{id:number})=>row.id===credential(theirs,route))).toBe(false);
      for(const secret of [mine.sessionToken,mine.deviceToken,legacy.api_key])expect(JSON.stringify(result.body)).not.toContain(secret);
    }
  }
  for(const actions of [['live.view'],['users.manage']]) {
    const scoped=await token(server,admin,{type:'all'},actions);
    scoped.identityManagement = actions[0] === 'users.manage';
    const doc=await discovery(server,scoped,info,`token-${actions[0]}`);
    for(const [route] of routes) {
      expect(doc.paths).not.toHaveProperty(route);expect(doc.paths).not.toHaveProperty(`${route}/{id}`);
      const before=inventory(server);
      for(const method of ['GET','DELETE'])expect((await wire(server,method==='GET'?route:`${route}/${credential(mine,route)}`,scoped,method)).response.status).toBe(401);
      expect(inventory(server)).toEqual(before);
    }
    for(const name of ['AuthSession','TrustedDevice','AuthSessionsResponse','TrustedDevicesResponse','CredentialRevoked'])expect(doc.components.schemas).not.toHaveProperty(name);
  }
  const mixed={headers:{Cookie:`session=${mine.sessionToken}`,Authorization:`Bearer ${(await token(server)).headers.Authorization!.split(' ')[1]}`}};
  expect((await wire(server,'/api/auth/sessions',mixed)).body.sessions[0].id).toBe(mine.session);
  lintProjections(info);
});

test('malformed IDs cannot revoke a numeric-prefix record or clear cookies',async({server})=>{
  const owner=await createUser(server,'strict_ids'),row=seed(server,owner.id!)[0]!;
  for(const [route] of routes) {
    const id=credential(row,route);
    for(const invalid of [`${id}junk`,`${id}/extra`,`${id}/`,`0${id}`,`+${id}`,`-${id}`,'0','9007199254740992','9223372036854775808',`${id}%00suffix`,`${id}%20`]) {
      const before=inventory(server);
      const result=await wire(server,`${route}/${invalid}`,owner,'DELETE');
      expect(result.response.status,invalid).toBe(invalid.includes('/')?404:400);
      if(!invalid.includes('/'))privateResponse(result.response);valid('ApiError',result.body);
      expect(result.response.headers.get('set-cookie')).toBeNull();expect(inventory(server)).toEqual(before);
    }
    expect((await wire(server,`${route}/9007199254740991`,owner,'DELETE')).response.status).toBe(404);
  }
});

test('revocation is owner-scoped, single-winner, and independent of role names',async({server})=>{
  const owner=await createUser(server,'revoke_owner'),other=await createUser(server,'revoke_other');
  const row=seed(server,owner.id!)[0]!,kept=seed(server,other.id!)[0]!;
  for(const [route] of routes) {
    for(const denied of [other,admin,anonymous]) {
      const before=inventory(server),result=await wire(server,`${route}/${credential(row,route)}`,denied,'DELETE');
      expect(result.response.status).toBe(denied===anonymous?401:404);expect(result.response.headers.get('set-cookie')).toBeNull();expect(inventory(server)).toEqual(before);
    }
    const responses=await Promise.all([wire(server,`${route}/${credential(row,route)}`,owner,'DELETE'),wire(server,`${route}/${credential(row,route)}`,owner,'DELETE')]);
    expect(responses.map(r=>r.response.status).sort()).toEqual([200,404]);
    valid('CredentialRevoked',responses.find(r=>r.response.status===200)!.body);
  }
  expect((await wire(server,'/api/auth/verify',{headers:{Cookie:`session=${row.sessionToken}`}})).response.status).toBe(401);
  expect((await wire(server,'/api/auth/verify',{headers:{Cookie:`session=${kept.sessionToken}`}})).response.status).toBe(200);
  expect((await ok(server,'/api/auth/trusted-devices',other)).trusted_devices.map((r:{id:number})=>r.id)).toEqual([kept.device]);
});

test('capped lists exclude pending/expired rows while older current credentials still revoke and clear cookies',async({server})=>{
  const owner=await createUser(server,'many_credentials'),now=Math.floor(Date.now()/1000);
  const old=seed(server,owner.id!,1,now-60)[0]!,newer=seed(server,owner.id!,35,now+3600);
  const expired=seed(server,owner.id!)[0]!,pending=seed(server,owner.id!)[0]!;
  database(server,'UPDATE sessions SET expires_at=0 WHERE id=?',[expired.session]);
  database(server,'UPDATE trusted_devices SET expires_at=0 WHERE id=?',[expired.device]);
  database(server,'UPDATE sessions SET mfa_pending=1 WHERE id=?',[pending.session]);
  const identity={headers:{Cookie:`session=${old.sessionToken}; trusted_device=${old.deviceToken}`}};
  for(const [route,key,schema] of routes) {
    const result=await wire(server,route,identity);expect(result.response.status).toBe(200);valid(schema,result.body);
    expect(result.body[key]).toHaveLength(32);expect(result.body[key].map((r:{id:number})=>r.id)).toEqual(newer.slice(-32).reverse().map(row=>credential(row,route)));
    expect(result.body[key].every((r:{current:boolean})=>!r.current)).toBe(true);
  }
  expect((await wire(server,`/api/auth/sessions/${pending.session}`,owner,'DELETE')).response.status).toBe(404);
  // Remove the device first to keep the session available for the second request.
  for(const [route,cookie] of [['/api/auth/trusted-devices','trusted_device'],['/api/auth/sessions','session']]) {
    const result=await wire(server,`${route}/${credential(old,route!)}`,identity,'DELETE');
    expect(result.response.status).toBe(200);valid('CredentialRevoked',result.body);expect(result.response.headers.get('set-cookie')).toContain(`${cookie}=;`);expect(result.response.headers.get('set-cookie')).toContain('Max-Age=0');
  }
  expect((await wire(server,'/api/auth/verify',identity)).response.status).toBe(401);
  for(const [route] of routes)expect((await wire(server,`${route}/${credential(expired,route)}`,owner,'DELETE')).response.status).toBe(200);
});

test('database revocation failures do not clear cookies or report success',async({server})=>{
  const owner=await createUser(server,'fault_credentials'),row=seed(server,owner.id!)[0]!;
  const identity={headers:{Cookie:`session=${row.sessionToken}; trusted_device=${row.deviceToken}`}};
  for(const [route] of routes) {
    const table=route.endsWith('sessions')?'sessions':'trusted_devices';
    database(server,`CREATE TRIGGER fail_revoke BEFORE DELETE ON ${table} BEGIN SELECT RAISE(ABORT,'fixture'); END`);
    const before=inventory(server),result=await wire(server,`${route}/${credential(row,route)}`,identity,'DELETE');
    database(server,'DROP TRIGGER fail_revoke');
    expect(result.response.status).toBe(500);valid('ApiError',result.body);expect(result.response.headers.get('set-cookie')).toBeNull();expect(inventory(server)).toEqual(before);
  }
});

test('password-change, inactive and CIDR gates deny credential self-service without mutation',async({server})=>{
  const owner=await createUser(server,'gated_credentials'),row=seed(server,owner.id!)[0]!;
  for(const [column,value,reset] of [['must_change_password',1,0],['is_active',0,1],['allowed_login_cidrs','192.0.2.0/24',null]] as const) {
    database(server,`UPDATE users SET ${column}=? WHERE id=?`,[value,owner.id!]);
    const before=inventory(server);
    for(const identity of [owner,{headers:{Cookie:`session=${row.sessionToken}`}}])for(const [route] of routes) {
      expect((await wire(server,route,identity)).response.status).toBe(401);
      expect((await wire(server,`${route}/${credential(row,route)}`,identity,'DELETE')).response.status).toBe(401);
    }
    expect(inventory(server)).toEqual(before);
    database(server,`UPDATE users SET ${column}=? WHERE id=?`,[reset,owner.id!]);
  }
});


test('synthetic disabled-auth principal cannot list or revoke personal credentials',async({server})=>{
  const row=seed(server,1)[0]!;
  await ok(server,'/api/settings',admin,'POST',{web_auth_enabled:false});
  const before=inventory(server);
  for(const [route] of routes) {
    expect((await wire(server,route,anonymous)).response.status).toBe(403);
    expect((await wire(server,`${route}/${credential(row,route)}`,anonymous,'DELETE')).response.status).toBe(403);
  }
  expect(inventory(server)).toEqual(before);
});
