import {mkdirSync,writeFileSync} from 'node:fs';
import path from 'node:path';
/** F3: independently authored admin policies and native wire assertions. */
import {test, expect, admin, viewer, anonymous, wire, ok, createUser, token, role, policy, database, valid, operationIds, lintProjections} from './permissionCases';

test('scoped management tokens cannot issue an unrestricted legacy key', async ({server}) => {
  const manager = await token(server, admin, {type:'all'}, ['users.manage']);
  const user = await createUser(server, 'key_boundary');
  const issued = await ok(server, `/api/auth/users/${user.id}/api-key`, user, 'POST');
  const before = await wire(server, '/api/auth/verify', {headers:{'X-API-Key':issued.api_key}});
  expect(before.response.status).toBe(200);
  expect((await wire(server, `/api/auth/users/${user.id}/api-key`, manager, 'POST')).response.status).toBe(403);
  expect((await wire(server, '/api/auth/verify', {headers:{'X-API-Key':issued.api_key}})).response.status).toBe(200);
});

const selfOps=['updateAdminUser','createLegacyApiKey'];
const managementOps=['listAdminUsers','getAdminUser','deleteAdminUser','setUserPasswordLock','clearUserLoginLockout','listAuthorizationActions','simulateAuthorization'];
const nonScopedOps=['createAdminUser','listAuthorizationRoles','createAuthorizationRole','updateAuthorizationRole','deleteAuthorizationRole','getUserAuthorizationPolicy','setUserAuthorizationPolicy'];
const auditOps=['listAuditEvents','exportAuditEvents','getAuditSettings','updateAuditSettings'];
const domainOps=[...selfOps,...managementOps,...nonScopedOps,...auditOps];
const selected=(doc:Record<string,any>)=>operationIds(doc).filter(id=>domainOps.includes(id));
// last_login has one-second resolution and changes on every authenticated login,
// so no-side-effect comparisons ignore it wherever it appears.
const withoutLoginTime=(value:unknown)=>JSON.parse(JSON.stringify(value,(key,item)=>key==='last_login'?0:item));
function noSecrets(body:unknown){
  if(Array.isArray(body)){body.forEach(noSecrets);return;}
  if(!body||typeof body!=='object')return;
  for(const [key,value] of Object.entries(body)){
    expect(['secret','password','password_hash','api_key','totp_secret','token_hash']).not.toContain(key);noSecrets(value);
  }
}

test('F3 discovery independently matches identity class, owner/token intersections and revocation',async({server},info)=>{
  const manager=await createUser(server,'delegated_manager');
  const managementRole=await role(server,'Scoped manager',['users.manage']);
  await policy(server,manager,[{role_uuid:managementRole,scope:{type:'all'}}]);
  const scoped=await token(server,manager,{type:'all'},['users.manage']);
  const auditor=await token(server,admin,{type:'all'},['system.admin']);
  for(const [identity,expected,label] of [[admin,domainOps,'admin'],[viewer,selfOps,'viewer'],[manager,[...selfOps,...managementOps,...nonScopedOps],'manager'],[scoped,['updateAdminUser',...managementOps],'scoped'],[auditor,auditOps,'auditor']] as const){
    // This domain deliberately authors its own allow-list rather than deriving it from schema metadata.
    const doc=await ok(server,'/api/openapi.json',identity);
    mkdirSync(info.outputPath('projections'),{recursive:true});writeFileSync(path.join(info.outputPath('projections'),`${label}.json`),JSON.stringify(doc));
    expect(selected(doc)).toEqual([...expected].sort());expect(JSON.stringify(doc)).not.toContain('ContractMatrix123!');
    expect(doc['x-lightnvr-coverage'].covered_operations.filter((id:string)=>domainOps.includes(id)).sort()).toEqual(domainOps.sort());
  }
  lintProjections(info);
  const before=await ok(server,`/api/authorization/users/${manager.id}`);
  expect((await wire(server,`/api/authorization/users/${manager.id}`,manager,'PUT',{mode:'policy',expected_policy_version:before.policy_version,grants:[]})).response.status).toBe(409);
  expect(await ok(server,`/api/authorization/users/${manager.id}`)).toEqual(before);
  await policy(server,manager,[]);
  expect(selected(await ok(server,'/api/openapi.json',manager))).toEqual([...selfOps].sort());
  expect(selected(await ok(server,'/api/openapi.json',scoped))).toEqual([]);
  expect((await wire(server,'/api/auth/users',scoped)).response.status).toBe(403);
  await ok(server,`/api/authorization/users/${manager.id}/tokens/${scoped.tokenId}`,admin,'DELETE');
  expect((await wire(server,'/api/openapi.json',scoped)).response.status).toBe(401);
});

test('user administration validates all emitted fields and keeps denied writes durable-no-op',async({server})=>{
  const user=await createUser(server,'admin_contract_user');
  const route=`/api/auth/users/${user.id}`;
  valid('AdminUsers',await ok(server,'/api/auth/users'));valid('AdminUser',await ok(server,route));
  noSecrets(await ok(server,'/api/auth/users'));
  const original=await ok(server,route);
  for(const [identity,method,body,status] of [[viewer,'PUT',{email:'forbidden@example.test'},403],[user,'PUT',{is_active:false},403],[admin,'PUT',{role:0},400],[admin,'PUT',{allowed_login_cidrs:'invalid'},400],[viewer,'DELETE',undefined,403]] as const){
    expect((await wire(server,route,identity,method,body)).response.status).toBe(status);expect({...await ok(server,route),last_login:0}).toEqual({...original,last_login:0});
  }
  const updated=await ok(server,route,user,'PUT',{email:'own@example.test'});valid('AdminUser',updated);expect(updated.email).toBe('own@example.test');
  const locked=await ok(server,`${route}/password-lock`,admin,'PUT',{locked:true});valid('AdminUser',locked);expect(locked.password_change_locked).toBe(true);
  expect((await wire(server,route,admin,'PUT',{password:'ChangedContract123!'})).response.status).toBe(403);
  expect((await wire(server,`${route}/password-lock`,admin,'PUT',{locked:'yes'})).response.status).toBe(400);
  expect((await ok(server,route)).password_change_locked).toBe(true);
  valid('LoginLockoutCleared',await ok(server,`${route}/login-lockout/clear`,admin,'POST'));
  valid('AdminUser',await ok(server,`${route}/password-lock`,admin,'PUT',{locked:false}));
  const key=await ok(server,`${route}/api-key`,user,'POST');valid('LegacyApiKeyCreated',key);
  const legacy={headers:{'X-API-Key':key.api_key}};
  valid('AdminUser',await ok(server,route,legacy,'PUT',{email:'legacy@example.test'}));
  expect((await wire(server,'/api/auth/users/1',admin,'DELETE')).response.status).toBe(403);
  valid('AdminUserDeleted',await ok(server,route,admin,'DELETE'));
  expect((await wire(server,route)).response.status).toBe(404);
  expect((await wire(server,'/api/auth/verify',legacy)).response.status).toBe(401);
});

test('roles and policy enforce revisions, delegation and self-lockout with no grant side effects',async({server})=>{
  valid('AdminActionCatalog',await ok(server,'/api/authorization/actions'));
  const catalog=await ok(server,'/api/authorization/roles');valid('AdminRoles',catalog);
  const created=await ok(server,'/api/authorization/roles',admin,'POST',{name:'F3 role',actions:['live.view'],expected_policy_version:catalog.policy_version});valid('AdminRoleWritten',created);
  const route=`/api/authorization/roles/${created.role.uuid}`;
  const body={name:'F3 role updated',description:'updated',actions:['live.view','recordings.replay'],expected_policy_version:created.policy_version};
  const changed=await ok(server,route,admin,'PUT',body);valid('AdminRoleWritten',changed);
  expect((await wire(server,route,admin,'PUT',body)).response.status).toBe(409);
  expect((await ok(server,'/api/authorization/roles')).roles.find((r:any)=>r.uuid===created.role.uuid)).toEqual(changed.role);
  const subject=await createUser(server,'policy_subject');
  const state=await ok(server,`/api/authorization/users/${subject.id}`);valid('AdminUserPolicy',state);
  const grants=[{role_uuid:created.role.uuid,scope:{type:'all'}}];
  const changedPolicy=await ok(server,`/api/authorization/users/${subject.id}`,admin,'PUT',{mode:'policy',expected_policy_version:state.policy_version,grants});valid('AdminUserPolicy',changedPolicy);
  const simulation=await ok(server,'/api/authorization/simulate',admin,'POST',{user_id:subject.id,action:'live.view',camera_uuid:server.cameras.A.camera_uuid});valid('AdminSimulation',simulation);expect(simulation.allowed).toBe(true);noSecrets(simulation);
  const global=await ok(server,'/api/authorization/simulate',admin,'POST',{user_id:subject.id,action:'users.manage'});valid('AdminSimulation',global);expect(global.allowed).toBe(false);expect(global.resource).toBeNull();
  expect((await wire(server,'/api/authorization/simulate',admin,'POST',{user_id:subject.id,action:'live.view'})).response.status).toBe(400);
  expect((await wire(server,route,admin,'DELETE',{expected_policy_version:changedPolicy.policy_version})).response.status).toBe(409);
  const manager=await createUser(server,'narrow_manager');const manage=await role(server,'Narrow management',['users.manage']);await policy(server,manager,[{role_uuid:manage,scope:{type:'all'}}]);
  const before=await ok(server,`/api/authorization/users/${subject.id}`);
  expect((await wire(server,`/api/authorization/users/${subject.id}`,manager,'PUT',{mode:'policy',expected_policy_version:before.policy_version,grants})).response.status).toBe(403);
  expect(withoutLoginTime(await ok(server,`/api/authorization/users/${subject.id}`))).toEqual(withoutLoginTime(before));
  const list=await ok(server,'/api/auth/users');
  expect((await wire(server,'/api/auth/users',manager,'POST',{username:'escalated_user',password:'ContractEscalate123!',role:0})).response.status).toBe(403);
  expect(await ok(server,'/api/auth/users')).toEqual(list);
  const scoped=await token(server,admin,{type:'all'},['users.manage']);
  for(const [route,method,body] of [['/api/authorization/roles','GET',undefined],['/api/authorization/roles','POST',{name:'forbidden',actions:['users.manage'],expected_policy_version:before.policy_version}],[`/api/authorization/users/${subject.id}`,'GET',undefined],['/api/auth/users','POST',{username:'token_user',password:'ContractToken123!'}]] as const)expect((await wire(server,route,scoped,method,body)).response.status).toBe(403);
  await policy(server,subject,[]);const current=await ok(server,'/api/authorization/roles');
  valid('AdminRoleDeleted',await ok(server,route,admin,'DELETE',{expected_policy_version:current.policy_version}));
});

test('audit reads, settings, filters and actual paginated CSV preserve permission and formula boundaries',async({server})=>{
  const settings=await ok(server,'/api/audit/settings');valid('AuditSettings',settings);
  for(const body of [{retention_days:0},{retention_days:3651},{retention_days:30,allowed_decision_modes:{'users.manage':'invalid'}}]){
    expect((await wire(server,'/api/audit/settings',admin,'PUT',body)).response.status).toBe(400);expect(await ok(server,'/api/audit/settings')).toEqual(settings);
  }
  expect((await wire(server,'/api/audit/settings',viewer,'PUT',{retention_days:30})).response.status).toBe(403);
  expect(await ok(server,'/api/audit/settings')).toEqual(settings);
  const updated=await ok(server,'/api/audit/settings',admin,'PUT',{retention_days:30,allowed_decision_modes:{'users.manage':'summarize'}});valid('AuditSettings',updated);expect(updated).toHaveProperty('pruned_events');
  database(server,`INSERT INTO audit_events(uuid,occurred_at,request_id,principal_user_id,principal_username,auth_method,action,outcome,remote_address,details_json) VALUES(?,?,?,?,?,?,?,?,?,?)`,['00000000-0000-4000-8000-000000000123',Math.floor(Date.now()/1000),'csv-fixture',1,' =SUM(1,2)"\nrow','basic','csv.fixture','success','127.0.0.1','{"event_type":"csv.fixture"}']);
  const query='request_id=csv-fixture&page_size=1';const page=await ok(server,`/api/audit/events?${query}`);valid('AuditEventsPage',page);expect(page.count).toBe(1);noSecrets(page);
  const response=await server.request(`/api/audit/events/export?${query}`);expect(response.status).toBe(200);expect(response.headers.get('content-type')).toBe('text/csv; charset=utf-8');expect(response.headers.get('content-disposition')).toBe('attachment; filename=lightnvr-audit.csv');expect(response.headers.get('x-total-count')).toBe('1');
  const bytes=await response.text();expect(bytes).toContain('uuid,occurred_at,request_id,principal_user_id,principal_username,');expect(bytes).toContain('"\' =SUM(1,2)""\nrow"');expect(bytes).toContain('"csv-fixture"');
  const empty=await server.request(`/api/audit/events/export?${query}&page=2`);expect((await empty.text()).split('\n')).toHaveLength(2);
  for(const route of ['/api/audit/events','/api/audit/events/export','/api/audit/settings'])expect((await wire(server,route,viewer)).response.status).toBe(403);
  for(const query of ['page=0','page_size=1001','since=2&until=1','since=invalid'])expect((await wire(server,`/api/audit/events?${query}`)).response.status).toBe(400);
});

test('all twenty direct routes deny unauthorized principals before changing administrative state',async({server})=>{
  const target=await createUser(server,'denied_matrix');const roleId=await role(server,'Denied matrix role',['live.view']);
  const userRoute=`/api/auth/users/${target.id}`,policyRoute=`/api/authorization/users/${target.id}`,roleRoute=`/api/authorization/roles/${roleId}`;
  const before={user:await ok(server,userRoute),policy:await ok(server,policyRoute),roles:await ok(server,'/api/authorization/roles'),settings:await ok(server,'/api/audit/settings')};
  const roleBody={name:'Denied change',actions:['live.view'],expected_policy_version:before.roles.policy_version};
  const routes:[string,string,unknown?][]=[
    ['/api/auth/users','GET'],['/api/auth/users','POST',{username:'denied_creation',password:'DeniedCreation123!'}],[userRoute,'GET'],[userRoute,'PUT',{email:'denied@example.test'}],[userRoute,'DELETE'],[`${userRoute}/api-key`,'POST'],[`${userRoute}/password-lock`,'PUT',{locked:true}],[`${userRoute}/login-lockout/clear`,'POST'],
    ['/api/authorization/actions','GET'],['/api/authorization/simulate','POST',{user_id:target.id,action:'users.manage'}],['/api/authorization/roles','GET'],['/api/authorization/roles','POST',roleBody],[roleRoute,'PUT',roleBody],[roleRoute,'DELETE',{expected_policy_version:before.roles.policy_version}],[policyRoute,'GET'],[policyRoute,'PUT',{mode:'policy',expected_policy_version:before.policy.policy_version,grants:[]}],
    ['/api/audit/events','GET'],['/api/audit/events/export','GET'],['/api/audit/settings','GET'],['/api/audit/settings','PUT',{retention_days:1}],
  ];
  expect(routes).toHaveLength(20);
  for(const [identity,status] of [[anonymous,401],[viewer,403]] as const)for(const [route,method,body] of routes){
    const denied=await wire(server,route,identity,method,body);expect(denied.response.status,`${status}: ${method} ${route}`).toBe(status);valid('ApiError',denied.body);
  }
  expect(await ok(server,userRoute)).toEqual(before.user);expect(await ok(server,policyRoute)).toEqual(before.policy);expect(await ok(server,'/api/authorization/roles')).toEqual(before.roles);expect(await ok(server,'/api/audit/settings')).toEqual(before.settings);
});
