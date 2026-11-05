import {test,expect,admin,anonymous,viewer,wire,ok,createUser,role,policy,token,cameraScope,database,discovery,pilotOperationIds as operationIds,allOps,readOps,emptyOps,lintProjections,privateResponse,valid,type Identity} from './permissionCases';

const configureOps=['getOpenApi','listStreams','updateStream'];
test('anonymous: discovery is private JSON 401 without canonical fallback',async({server},info)=>{
  await discovery(server,anonymous,info,'anonymous',401);
  expect((await wire(server,'/api/streams',anonymous)).response.status).toBe(401);
});

test('owner_two_tokens: disjoint cookie-free tokens cannot borrow owner or sibling authority',async({server},info)=>{
  const a=await token(server),b=await token(server,admin,cameraScope(server.cameras.B.camera_uuid));
  let firstDocument: unknown;
  for(const [label,identity,allowed,denied] of [['A',a,'A','B'],['B',b,'B','A'],['A-again',a,'A','B']] as const) {
    const doc=await discovery(server,identity,info,label);expect(operationIds(doc)).toEqual(readOps);
    if(firstDocument) expect(doc).toEqual(firstDocument); else firstDocument=doc;
    expect(doc.components.schemas).not.toHaveProperty('StreamFullPrivilegedResponse');
    expect((await ok(server,'/api/streams',identity) as unknown as {name:string}[]).map(row=>row.name)).toEqual([allowed]);
    expect((await wire(server,`/api/streams/${denied}/full`,identity)).response.status).toBe(403);
    expect((await wire(server,`/api/streams/${allowed}`,identity,'PUT',{set_privacy_mode:true})).response.status).toBe(403);
  }
  expect(operationIds(await discovery(server,admin,info,'owner'))).toEqual(allOps);
  lintProjections(info);
});

test('expired_token: expiry rejects next discovery and direct API request',async({server},info)=>{
  const identity=await token(server,admin,cameraScope(server.cameras.A.camera_uuid),['live.view'],Math.floor(Date.now()/1000)+5);
  expect(operationIds(await discovery(server,identity,info,'before'))).toEqual(readOps);
  // Exercise actual time expiry without bypassing SQLite's issuance constraints.
  await expect.poll(async()=> (await wire(server,'/api/openapi.json',identity)).response.status,{timeout:8000}).toBe(401);
  await discovery(server,identity,info,'expired',401);
  expect((await wire(server,'/api/streams',identity)).response.status).toBe(401);lintProjections(info);
});

test('revoked_token: supported revocation invalidates the next request',async({server},info)=>{
  const identity=await token(server);await discovery(server,identity,info,'before');
  await ok(server,`/api/authorization/users/1/tokens/${identity.tokenId}`,admin,'DELETE');
  await discovery(server,identity,info,'revoked',401);
  expect((await wire(server,'/api/streams',identity)).response.status).toBe(401);lintProjections(info);
});

test('inactive_owner: deactivation invalidates both Basic identity and its token',async({server},info)=>{
  const owner=await createUser(server,'inactive_owner');const identity=await token(server,owner,{type:'all'});
  await discovery(server,owner,info,'basic-before');await discovery(server,identity,info,'token-before');
  await ok(server,`/api/auth/users/${owner.id}`,admin,'PUT',{is_active:false});
  for(const [label,principal] of [['basic',owner],['token',identity]] as const) {
    await discovery(server,principal,info,label,401);expect((await wire(server,'/api/streams',principal)).response.status).toBe(401);
  }
  lintProjections(info);
});

test('demo_anonymous: synthetic viewer can use legacy API but cannot download personalized discovery',async({server},info)=>{
  await ok(server,'/api/settings',admin,'POST',{demo_mode:true});
  await discovery(server,anonymous,info,'demo',403);
  const list=await wire(server,'/api/streams',anonymous);expect(list.response.status).toBe(200);valid('StreamCompatibilityList',list.body);
});

test('auth_disabled: synthetic admin cannot download personalized discovery',async({server},info)=>{
  await ok(server,'/api/settings',admin,'POST',{web_auth_enabled:false});
  await discovery(server,anonymous,info,'auth-disabled',403);
  const list=await wire(server,'/api/streams',anonymous);expect(list.response.status).toBe(200);valid('StreamCompatibilityList',list.body);
});

test('authenticated_demo: real demo identity retains actual operation permissions',async({server},info)=>{
  await ok(server,'/api/settings',admin,'POST',{demo_mode:true});
  const doc=await discovery(server,admin,info,'authenticated-demo');expect(operationIds(doc)).toEqual(allOps);
  const update=await wire(server,'/api/streams/A',admin,'PUT',{set_privacy_mode:true});
  expect(update.response.status).toBe(200);valid('StreamPrivacyUpdated',update.body);
  expect((await ok(server,'/api/streams/A/full')).stream.privacy_mode).toBe(true);lintProjections(info);
});

test('health_selector: raw read and runtime-sensitive fields retain distinct observations',async({server},info)=>{
  const identity=await createUser(server,'health_selector');
  const read=await role(server,'Unknown health reader',['live.view']);
  const configure=await role(server,'Down health configurer',['camera.configure']);
  const healthScope=(health:string)=>({type:'selector',selector:{version:1,expression:{op:'and',children:[{op:'camera_uuid',values:[server.cameras.A.camera_uuid]},{op:'health',values:[health]}]}}});
  await policy(server,identity,[{role_uuid:read,scope:healthScope('unknown')},{role_uuid:configure,scope:healthScope('down')}]);
  // Start an owned recorder against the fixture's closed loopback port. Its
  // failed connection supplies real DOWN runtime metrics; raw inventory remains
  // UNKNOWN. No external camera or network endpoint is contacted.
  await ok(server,'/api/streams/A',admin,'PUT',{enabled:true,streaming_enabled:true,enable_disabled:true,record:true});
  await expect.poll(async()=> {
    const result=await ok(server,'/api/openapi.json',identity);
    return Boolean(result.components.schemas.StreamFullPrivilegedResponse);
  },{timeout:15000}).toBe(true);
  const doc=await discovery(server,identity,info,'health');expect(operationIds(doc)).toEqual(readOps);
  expect(doc.components.schemas).toHaveProperty('StreamFullPrivilegedResponse');
  const full=await wire(server,'/api/streams/A/full',identity);expect(full.response.status).toBe(200);
  valid('StreamFullResponse',full.body);expect(full.body.stream.admin_url).toBe('http://127.0.0.1:9/admin-a');
  const list=await ok(server,'/api/streams',identity);expect(list).toEqual([]);
  const before=await ok(server,'/api/streams/A/full');
  expect((await wire(server,'/api/streams/A',identity,'PUT',{set_privacy_mode:true})).response.status).toBe(403);
  expect(await ok(server,'/api/streams/A/full')).toEqual(before);lintProjections(info);
});

test('cookie_with_token: existing session precedence is explicit and isolated from token-only requests',async({server,playwright},info)=>{
  const identity=await token(server);
  const session=await playwright.request.newContext({baseURL:server.baseURL});
  try {
    expect((await session.post('/api/auth/login',{data:server.credentials.admin})).status()).toBe(200);
    const cookies=await session.storageState();expect(cookies.cookies.length).toBeGreaterThan(0);
    const mixed:Identity={recordingReplay:true,recordingActions:admin.recordingActions,headers:{...identity.headers,Cookie:cookies.cookies.map(c=>`${c.name}=${c.value}`).join('; ')}};
    expect(operationIds(await discovery(server,mixed,info,'admin-cookie-plus-read-token'))).toEqual(allOps);
    expect((await wire(server,'/api/streams/B/full',mixed)).response.status).toBe(200);
    expect(operationIds(await discovery(server,identity,info,'read-token-alone'))).toEqual(readOps);
    expect((await wire(server,'/api/streams/B/full',identity)).response.status).toBe(403);
    expect((await session.post('/api/auth/logout')).status()).toBe(200);
    expect(operationIds(await discovery(server,{...mixed,tokenId:identity.tokenId,recordingReplay:false,recordingActions:[]},info,'stale-cookie-falls-through-token'))).toEqual(readOps);
  } finally {await session.dispose();}
  lintProjections(info);
});

test('password_change_required: Basic/session gate blocks discovery while scoped token and legacy key keep exemption',async({server,playwright},info)=>{
  const owner=await createUser(server,'pending_password');
  const key=await ok(server,`/api/auth/users/${owner.id}/api-key`,admin,'POST');
  const identity=await token(server,owner,{type:'all'});
  const session=await playwright.request.newContext({baseURL:server.baseURL});
  try {
    expect((await session.post('/api/auth/login',{data:{username:owner.username,password:owner.password}})).status()).toBe(200);
    const {cookies}=await session.storageState();
    database(server,'UPDATE users SET must_change_password=1 WHERE id=?',[owner.id!]);
    await discovery(server,owner,info,'basic-pending',401);
    await discovery(server,{headers:{Cookie:cookies.map(c=>`${c.name}=${c.value}`).join('; ')}},info,'session-pending',401);
    expect(operationIds(await discovery(server,identity,info,'token-exempt'))).toEqual(readOps);
    expect(operationIds(await discovery(server,{recordingReplay:true,recordingActions:['export'],headers:{'X-API-Key':key.api_key}},info,'key-exempt'))).toEqual(readOps);
    expect((await wire(server,'/api/streams',owner)).response.status).toBe(401);
  } finally {await session.dispose();}
  lintProjections(info);
});

test('cidr_rejected: effective loopback peer cannot use a documentation-only CIDR allowlist',async({server},info)=>{
  const identity=await createUser(server,'cidr_rejected',{allowed_login_cidrs:'192.0.2.0/24'});
  await discovery(server,identity,info,'cidr',401);
  const spoofed={headers:{...identity.headers,'X-Forwarded-For':'192.0.2.1'}};
  await discovery(server,spoofed,info,'untrusted-forwarded-cidr',401);
  expect((await wire(server,'/api/streams',identity)).response.status).toBe(401);
});

test('snapshot_failure: failed policy read returns generic private JSON 500 and recovers without fallback',async({server},info)=>{
  await discovery(server,viewer,info,'before');
  database(server,'ALTER TABLE authz_grants RENAME TO contract_unavailable_grants');
  try {
    const body=await discovery(server,viewer,info,'failure',500);
    expect(body).toEqual({error:'API discovery is unavailable'});
  } finally {database(server,'ALTER TABLE contract_unavailable_grants RENAME TO authz_grants');}
  expect(operationIds(await discovery(server,viewer,info,'after'))).toEqual(readOps);lintProjections(info);
});

test('concurrent_policy_change: each document is wholly one policy observation and next request sees replacement',async({server},info)=>{
  const identity=await createUser(server,'concurrent_policy');
  const read=await role(server,'Concurrent read',['live.view']),configure=await role(server,'Concurrent configure',['camera.configure']);
  const grants=[[{role_uuid:read,scope:cameraScope(server.cameras.A.camera_uuid)}],[{role_uuid:configure,scope:cameraScope(server.cameras.B.camera_uuid)}]];
  await policy(server,identity,grants[0]);
  expect(operationIds(await discovery(server,identity,info,'before'))).toEqual(readOps);
  for(let i=0;i<12;i++) {
    const [doc]=await Promise.all([discovery(server,identity,info,`race-${i}`),policy(server,identity,grants[(i+1)%2])]);
    expect([readOps.join(','),configureOps.join(',')]).toContain(operationIds(doc).join(','));
    expect(doc.components.schemas).not.toHaveProperty('StreamFullPrivilegedResponse');
  }
  await policy(server,identity,grants[1]);
  expect(operationIds(await discovery(server,identity,info,'after'))).toEqual(configureOps);
  expect(await ok(server,'/api/streams',identity)).toEqual([]);
  expect((await wire(server,'/api/streams/A/full',identity)).response.status).toBe(403);lintProjections(info);
});

test('membership_change: collection and tag edits change next discovery and direct camera visibility',async({server},info)=>{
  const identity=await createUser(server,'membership');const read=await role(server,'Membership read',['live.view']);
  const collection=await ok(server,'/api/camera-collections',admin,'POST',{name:'Contract collection',type:'static',shared:true});
  await policy(server,identity,[{role_uuid:read,scope:{type:'collection',collection_uuid:collection.uuid}}]);
  expect(operationIds(await discovery(server,identity,info,'empty-collection'))).toEqual(emptyOps);
  for(const name of ['A','B']) {
    await ok(server,`/api/camera-collections/${collection.uuid}/members`,admin,'PUT',{camera_uuids:[server.cameras[name].camera_uuid]});
    expect(operationIds(await discovery(server,identity,info,`collection-${name}`))).toEqual(readOps);
    expect((await ok(server,'/api/streams',identity) as unknown as {name:string}[]).map(row=>row.name)).toEqual([name]);
    expect((await wire(server,`/api/streams/${name==='A'?'B':'A'}/full`,identity)).response.status).toBe(403);
  }
  await ok(server,`/api/camera-collections/${collection.uuid}/members`,admin,'PUT',{camera_uuids:[]});
  expect(operationIds(await discovery(server,identity,info,'removed-members'))).toEqual(emptyOps);
  const tag=await ok(server,'/api/camera-tags',admin,'POST',{label:'Contract tag'});
  await policy(server,identity,[{role_uuid:read,scope:{type:'selector',selector:{version:1,expression:{op:'tag_any',uuids:[tag.uuid]}}}}]);
  expect(operationIds(await discovery(server,identity,info,'empty-tag'))).toEqual(emptyOps);
  await ok(server,`/api/cameras/${server.cameras.A.camera_uuid}/tags`,admin,'PUT',{tag_uuids:[tag.uuid]});
  expect(operationIds(await discovery(server,identity,info,'tag-A'))).toEqual(readOps);
  expect((await ok(server,'/api/streams',identity) as unknown as {name:string}[]).map(row=>row.name)).toEqual(['A']);
  await ok(server,`/api/cameras/${server.cameras.A.camera_uuid}/tags`,admin,'PUT',{tag_uuids:[]});
  expect(operationIds(await discovery(server,identity,info,'removed-tag'))).toEqual(emptyOps);lintProjections(info);
});

test('recording action discovery separates protection, deletion, export and exact camera scope',async({server},info)=>{
  const scopes=[
    {name:'protect',action:'evidence.protect',groups:['protect'] as const},
    {name:'delete',action:'recording.delete',groups:['delete'] as const},
    {name:'export',action:'recordings.export',groups:['export'] as const},
  ];
  for(const scenario of scopes) {
    const identity=await createUser(server,`docs_recording_${scenario.name}`);
    const permission=await role(server,`Recording ${scenario.name}`,[scenario.action]);
    await policy(server,identity,[{role_uuid:permission,scope:cameraScope(server.cameras.A.camera_uuid)}],false,scenario.groups);
    // No live/replay permission is implied by mutation or export permission.
    expect(operationIds(await discovery(server,identity,info,`${scenario.name}-basic`))).toEqual(emptyOps);
    const scoped=await token(server,admin,cameraScope(server.cameras.A.camera_uuid),[scenario.action],Math.floor(Date.now()/1000)+3600,false,scenario.groups);
    expect(operationIds(await discovery(server,scoped,info,`${scenario.name}-scoped`))).toEqual(emptyOps);
    const absent=await token(server,admin,cameraScope('00000000-0000-4000-8000-000000000099'),[scenario.action]);
    expect(operationIds(await discovery(server,absent,info,`${scenario.name}-nonmatching`))).toEqual(emptyOps);
  }
  lintProjections(info);
});
