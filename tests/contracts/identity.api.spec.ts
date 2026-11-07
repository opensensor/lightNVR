import {readFileSync} from 'node:fs';
import {DatabaseSync} from 'node:sqlite';
import path from 'node:path';
import {test,expect,admin,anonymous,viewer,wire,ok,token,createUser,role,policy,cameraScope,discovery,nonRecordingOperationIds as operationIds,identityOps,scopedIdentityOps,allOps,privateResponse,lintProjections,type Server,type Identity, withoutLoginTime} from './permissionCases';
import {validateLoginRequest,validateLoginResponse,validateLoginConfig,validateAuthVerifyResponse,validateLogoutResponse,validateClientConfig,validateApiTokenCreateRequest,validateApiTokenMetadata,validateApiTokenList,validateApiTokenCreated,validateSuccessResponse} from '../../web/generated/api/validators';

// Inspect only our isolated fixture. No credential rows/values are emitted.
function sessionCount(server:Server):number {
  expect(path.basename(server.directory)).toMatch(/^lightnvr-contract-/);
  const db=new DatabaseSync(path.join(server.directory,'lightnvr.db'));
  try {return Number(db.prepare('SELECT count(*) AS count FROM sessions').get()!.count);} finally {db.close();}
}
function noSecrets(value:unknown):void {
  if(Array.isArray(value)){for(const item of value)noSecrets(item);return;}
  if(!value || typeof value!=='object')return;
  for(const [key,child] of Object.entries(value)) {
    expect(['secret','api_key','password','password_hash','token_hash','totp_secret','totp_token']).not.toContain(key);
    noSecrets(child);
  }
}
const creation=(server:Server)=>({description:'Identity contract token',expires_at:Math.floor(Date.now()/1000)+3600,actions:['live.view'],scope:cameraScope(server.cameras.A.camera_uuid)});
async function inventory(server:Server,owner=admin) {
  const body=await ok(server,`/api/authorization/users/${owner.id}/tokens`,owner);
  expect(validateApiTokenList(body)).toBe(true);noSecrets(body);return body;
}

test('public login configuration and logout do not make client-config or discovery public',async({server},info)=>{
  const config=await wire(server,'/api/auth/login/config',anonymous);
  expect(config.response.status).toBe(200);expect(validateLoginConfig(config.body)).toBe(true);noSecrets(config.body);
  for(const route of ['/api/client-config','/api/auth/verify'])expect((await wire(server,route,anonymous)).response.status).toBe(401);
  await discovery(server,anonymous,info,'anonymous',401);
  const redirect=await server.request('/api/auth/logout',{principal:'anonymous',method:'POST',redirect:'manual'});
  expect(redirect.status).toBe(302);expect(redirect.headers.get('location')).toBe('/login.html?logout=true');expect(await redirect.text()).toBe('');
  const logout=await wire(server,'/api/auth/logout',{headers:{Accept:'application/json'}},'POST');
  expect(logout.response.status).toBe(200);expect(validateLogoutResponse(logout.body)).toBe(true);
  expect(logout.response.headers.get('set-cookie')).toContain('Max-Age=0');
  const requested=await wire(server,'/api/auth/logout',{headers:{'X-Requested-With':'XMLHttpRequest'}},'POST');
  expect(requested.response.status).toBe(200);expect(validateLogoutResponse(requested.body)).toBe(true);
});

test('JSON and form login, identity verification and logout follow actual session lifecycle',async({server,playwright},info)=>{
  const context=await playwright.request.newContext({baseURL:server.baseURL});
  try {
    const before=sessionCount(server);
    for(const request of [{username:'admin'},{username:'admin',password:'invalid-contract-password'}]) {
      const response=await context.post('/api/auth/login',{data:request});
      expect(response.status()).toBe('password' in request?401:400);
      expect(response.headers()['set-cookie']).toBeUndefined();expect(sessionCount(server)).toBe(before);
    }
    expect(validateLoginRequest(server.credentials.admin)).toBe(true);
    const login=await context.post('/api/auth/login',{data:server.credentials.admin});
    expect(login.status()).toBe(200);expect(validateLoginResponse(await login.json())).toBe(true);
    expect(login.headers()['set-cookie']).toContain('HttpOnly');expect(sessionCount(server)).toBe(before+1);
    const state=await context.storageState();const identity={recordingReplay:true,recordingActions:admin.recordingActions,headers:{Cookie:state.cookies.map(c=>`${c.name}=${c.value}`).join('; ')}};
    const verified=await wire(server,'/api/auth/verify',identity);
    expect(verified.response.status).toBe(200);expect(validateAuthVerifyResponse(verified.body)).toBe(true);noSecrets(verified.body);
    expect(verified.body.id).toBe(1);
    const doc=await discovery(server,identity,info,'session');expect(operationIds(doc)).toEqual([...allOps,...identityOps].sort());
    const logout=await context.post('/api/auth/logout',{headers:{Accept:'application/json'}});
    expect(logout.status()).toBe(200);expect(validateLogoutResponse(await logout.json())).toBe(true);expect(sessionCount(server)).toBe(before);
    expect((await context.get('/api/auth/verify')).status()).toBe(401);
    const form=await context.post('/api/auth/login',{form:{...server.credentials.admin},maxRedirects:0});
    expect(form.status()).toBe(302);expect(form.headers().location).toBe('/index.html');
    expect((await context.get('/api/auth/verify')).status()).toBe(200);
  } finally {await context.dispose();}
  lintProjections(info);
});

test('Basic and legacy keys retain identity/self-service while scoped tokens cannot manage or rewrite policy',async({server},info)=>{
  const legacy=await ok(server,'/api/auth/users/1/api-key',admin,'POST');
  const scoped=await token(server);const policyBefore=await ok(server,'/api/authorization/users/1');
  for(const [label,identity] of [['basic',admin],['legacy-header',{recordingReplay:true,recordingActions:admin.recordingActions,headers:{'X-API-Key':legacy.api_key}}],['legacy-bearer',{recordingReplay:true,recordingActions:admin.recordingActions,headers:{Authorization:`Bearer ${legacy.api_key}`}}]] as const) {
    const verified=await wire(server,'/api/auth/verify',identity);expect(verified.response.status).toBe(200);
    expect(validateAuthVerifyResponse(verified.body)).toBe(true);noSecrets(verified.body);
    const list=await wire(server,'/api/authorization/users/1/tokens',identity);expect(list.response.status).toBe(200);expect(validateApiTokenList(list.body)).toBe(true);noSecrets(list.body);
    const doc=await discovery(server,identity,info,label);expect(operationIds(doc)).toEqual([...allOps,...identityOps].sort());
    const security=doc.paths['/api/auth/verify'].get.security;expect(security).toEqual([{sessionCookie:[]},{basicAuth:[]},{legacyApiKeyHeader:[]},{legacyBearer:[]}]);
  }
  const scopedDoc=await discovery(server,scoped,info,'scoped');
  expect(operationIds(scopedDoc).filter(id=>identityOps.includes(id))).toEqual(scopedIdentityOps);
  expect(scopedDoc.components.schemas).not.toHaveProperty('ApiTokenCreated');
  expect((await wire(server,'/api/auth/verify',scoped)).response.status).toBe(401);
  for(const identity of [admin,viewer,scoped]) {
    const config=await wire(server,'/api/client-config',identity);expect(config.response.status).toBe(200);expect(validateClientConfig(config.body)).toBe(true);noSecrets(config.body);
    expect(validateClientConfig({...config.body,storage_path:'/private/path'})).toBe(false);
  }
  // Token usage may update last_used_at. Prime it before taking the durable snapshot.
  const before=await inventory(server);
  for(const [route,method,body] of [
    ['/api/authorization/users/1/tokens','GET',undefined],
    ['/api/authorization/users/1/tokens','POST',creation(server)],
    [`/api/authorization/users/1/tokens/${scoped.tokenId}`,'DELETE',undefined],
    ['/api/authorization/users/1','PUT',{mode:'legacy',expected_policy_version:policyBefore.policy_version,grants:[]}],
  ] as const)expect((await wire(server,route,scoped,method,body)).response.status).toBe(403);
  expect(await inventory(server)).toEqual(before);expect(withoutLoginTime(await ok(server,'/api/authorization/users/1'))).toEqual(withoutLoginTime(policyBefore));
  // Even a scoped token containing users.manage cannot bypass the explicit guard.
  const manager=await token(server,admin,{type:'all'},['users.manage']);
  const managerBefore=await inventory(server);
  expect((await wire(server,'/api/authorization/users/1',manager,'PUT',{mode:'legacy',expected_policy_version:policyBefore.policy_version,grants:[]})).response.status).toBe(403);
  expect((await wire(server,'/api/authorization/users/1/tokens',manager,'POST',creation(server))).response.status).toBe(403);
  expect((await inventory(server)).tokens.map((row:{uuid:string;revoked_at:number|null})=>[row.uuid,row.revoked_at])).toEqual(managerBefore.tokens.map((row:{uuid:string;revoked_at:number|null})=>[row.uuid,row.revoked_at]));
  expect(withoutLoginTime(await ok(server,'/api/authorization/users/1'))).toEqual(withoutLoginTime(policyBefore));lintProjections(info);
});

test('self-service token issuance is bounded by owner/requester authority and invalid writes have no effects',async({server},info)=>{
  const route=`/api/authorization/users/${viewer.id}/tokens`;const request=creation(server);
  expect(validateApiTokenCreateRequest(request)).toBe(true);
  const policyBefore=await ok(server,`/api/authorization/users/${viewer.id}`);
  const before=await inventory(server,viewer);
  for(const [identity,body] of [
    [viewer,{...request,scope:{type:'all'}}],
    [viewer,{...request,scope:cameraScope(server.cameras.B.camera_uuid)}],
    [viewer,{...request,actions:['camera.configure']}],
    [admin,{...request,actions:['camera.configure']}],
  ] as const) {
    expect((await wire(server,route,identity,'POST',body)).response.status).toBe(403);
    expect(await inventory(server,viewer)).toEqual(before);expect(withoutLoginTime(await ok(server,`/api/authorization/users/${viewer.id}`))).toEqual(withoutLoginTime(policyBefore));
  }
  for(const body of [{...request,description:''},{...request,actions:[]},{...request,actions:['unknown.action']},{...request,expires_at:1},{...request,expires_at:Math.floor(Date.now()/1000)+367*86400},{...request,scope:{type:'invalid'}}]) {
    expect((await wire(server,route,viewer,'POST',body)).response.status).toBe(400);
    expect(await inventory(server,viewer)).toEqual(before);expect(withoutLoginTime(await ok(server,`/api/authorization/users/${viewer.id}`))).toEqual(withoutLoginTime(policyBefore));
  }
  expect((await wire(server,'/api/authorization/users/1/tokens',viewer)).response.status).toBe(403);
  expect((await wire(server,`${route}/not-a-uuid`,viewer,'DELETE')).response.status).toBe(400);
  expect(await inventory(server,viewer)).toEqual(before);
  const issued=await wire(server,route,viewer,'POST',request);expect(issued.response.status).toBe(201);expect(validateApiTokenCreated(issued.body)).toBe(true);
  expect(validateApiTokenMetadata(issued.body.token)).toBe(true);noSecrets(issued.body.token);
  const credential:Identity={id:viewer.id,tokenId:issued.body.token.uuid,headers:{Authorization:`Bearer ${issued.body.secret}`}};
  expect((await wire(server,'/api/streams/A/full',credential)).response.status).toBe(200);
  expect((await wire(server,'/api/streams/B/full',credential)).response.status).toBe(403);
  const listed=await inventory(server,viewer);expect(listed.count).toBe(before.count+1);
  expect(JSON.stringify(listed)).not.toContain(issued.body.secret);
  const doc=await discovery(server,viewer,info,'owner');expect(JSON.stringify(doc)).not.toContain(issued.body.secret);
  expect(validateApiTokenList({...listed,secret:issued.body.secret})).toBe(false);
  expect(validateApiTokenMetadata({...issued.body.token,secret:issued.body.secret})).toBe(false);
  expect(validateApiTokenMetadata({...issued.body.token,token_hash:'forbidden-hash'})).toBe(false);
  const removed=await wire(server,`${route}/${issued.body.token.uuid}`,viewer,'DELETE');expect(removed.response.status).toBe(200);expect(validateSuccessResponse(removed.body)).toBe(true);
  expect((await wire(server,'/api/streams/A/full',credential)).response.status).toBe(401);
  const deniedDiscovery=await wire(server,'/api/openapi.json',credential);expect(deniedDiscovery.response.status).toBe(401);privateResponse(deniedDiscovery.response);
  expect(withoutLoginTime(await ok(server,`/api/authorization/users/${viewer.id}`))).toEqual(withoutLoginTime(policyBefore));lintProjections(info);
});

test('synthetic identity variants remain explicit without synthetic discovery access',async({server},info)=>{
  await ok(server,'/api/settings',admin,'POST',{demo_mode:true});
  const demo=await wire(server,'/api/auth/verify',anonymous);expect(demo.response.status).toBe(200);expect(validateAuthVerifyResponse(demo.body)).toBe(true);expect(demo.body.authenticated).toBe(false);
  await discovery(server,anonymous,info,'demo',403);
  await ok(server,'/api/settings',admin,'POST',{web_auth_enabled:false});
  const disabled=await wire(server,'/api/auth/verify',anonymous);expect(disabled.response.status).toBe(200);expect(validateAuthVerifyResponse(disabled.body)).toBe(true);expect(disabled.body.auth_enabled).toBe(false);
  await discovery(server,anonymous,info,'auth-disabled',403);
});

test('canonical identity guard negatives reject malformed data and contain no credential examples',()=>{
  const canonical=JSON.parse(readFileSync('docs/api/openapi.json','utf8'));
  for(const name of ['LoginRequest','LoginResponse','LoginConfig','AuthVerifyResponse','LogoutResponse','ClientConfig','ApiTokenCreateRequest','ApiTokenMetadata','ApiTokenList','ApiTokenCreated','SuccessResponse']) {
    expect(canonical['x-lightnvr-validators']).toContain(name);
    expect(JSON.stringify(canonical.components.schemas[name])).not.toMatch(/"examples?"\s*:/);
  }
  expect(validateLoginResponse({success:'true',redirect:'/index.html',must_change_password:false})).toBe(false);
  expect(validateLoginResponse({totp_required:true,totp_token:'ephemeral-fixture-challenge'})).toBe(true);
  expect(validateAuthVerifyResponse({authenticated:true})).toBe(false);
  expect(validateApiTokenCreated({secret:'',token:{}})).toBe(false);
  expect(validateApiTokenList({user_id:1,count:0,tokens:[],api_key:'forbidden'})).toBe(false);
  expect(validateSuccessResponse({success:true,totp_token:'forbidden'})).toBe(false);
});


test('users.manage cannot delegate token scope beyond the requester even for a broader owner',async({server})=>{
  const manager=await createUser(server,'identity_manager');
  const manage=await role(server,'Identity token manager',['users.manage']);
  const read=await role(server,'Identity bounded read',['live.view']);
  await policy(server,manager,[{role_uuid:manage,scope:{type:'all'}},{role_uuid:read,scope:cameraScope(server.cameras.A.camera_uuid)}]);
  const policyBefore=await ok(server,'/api/authorization/users/1');
  const before=await inventory(server);
  for(const scope of [{type:'all'},cameraScope(server.cameras.B.camera_uuid)]) {
    expect((await wire(server,'/api/authorization/users/1/tokens',manager,'POST',{...creation(server),scope})).response.status).toBe(403);
    expect(await inventory(server)).toEqual(before);expect(withoutLoginTime(await ok(server,'/api/authorization/users/1'))).toEqual(withoutLoginTime(policyBefore));
  }
  const issued=await wire(server,'/api/authorization/users/1/tokens',manager,'POST',creation(server));
  expect(issued.response.status).toBe(201);expect(validateApiTokenCreated(issued.body)).toBe(true);
  const revoked=await wire(server,`/api/authorization/users/1/tokens/${issued.body.token.uuid}`,manager,'DELETE');
  expect(revoked.response.status).toBe(200);expect(validateSuccessResponse(revoked.body)).toBe(true);
  expect(withoutLoginTime(await ok(server,'/api/authorization/users/1'))).toEqual(withoutLoginTime(policyBefore));
});
