/** F1c security regressions, separate from canonical contract expansion. */
import {createHmac} from 'node:crypto';
import {DatabaseSync} from 'node:sqlite';
import path from 'node:path';
import {test,expect,admin,anonymous,ok,wire,createUser,database,token,role,policy,type Server,type Identity} from './permissionCases';
import {basic} from './fixture-server.mjs';

const secret='JBSWY3DPEHPK3PXP';
function otp(step=Math.floor(Date.now()/30000),keySecret=secret) {
  const alphabet='ABCDEFGHIJKLMNOPQRSTUVWXYZ234567';
  const bits=[...keySecret].map(c=>alphabet.indexOf(c).toString(2).padStart(5,'0')).join('');
  const key=Buffer.from(bits.match(/.{8}/g)!.map(b=>parseInt(b,2)));
  const counter=Buffer.alloc(8);counter.writeBigUInt64BE(BigInt(step));
  const digest=createHmac('sha1',key).update(counter).digest();
  return ((digest.readUInt32BE(digest[19]!&15)&0x7fffffff)%1000000).toString().padStart(6,'0');
}
function wrongOtp() {
  const step=Math.floor(Date.now()/30000), valid=new Set([-2,-1,0,1,2].map(i=>otp(step+i)));
  return Array.from({length:10},(_,i)=>i.toString().padStart(6,'0')).find(c=>!valid.has(c))!;
}
function read(server:Server,sql:string,id:number) {
  const db=new DatabaseSync(path.join(server.directory,'lightnvr.db'));
  try {return db.prepare(sql).get(id)!;} finally {db.close();}
}
async function mfaUser(server:Server,name:string) {
  const user=await createUser(server,name);
  database(server,'UPDATE users SET totp_secret=?,totp_enabled=1 WHERE id=?',[secret,user.id!]);
  return user;
}
async function challenge(server:Server,user:Identity,cookie?:string) {
  return wire(server,'/api/auth/login',cookie?{headers:{Cookie:cookie}}:anonymous,'POST',
    {username:user.username,password:user.password});
}
async function complete(server:Server,token:string,code=otp(),remember_device=false) {
  return wire(server,'/api/auth/login/totp',anonymous,'POST',{totp_token:token,code,remember_device});
}
function cookie(response:Response,name:string) {
  const value=response.headers.getSetCookie().find(c=>c.startsWith(`${name}=`));
  expect(value).toBeTruthy();return value!.split(';')[0]!;
}

test('setup cannot replace an enabled MFA secret',async({server})=>{
  const user=await mfaUser(server,'active_mfa_setup');
  const before=read(server,'SELECT totp_secret,totp_enabled FROM users WHERE id=?',user.id!);
  expect((await wire(server,`/api/auth/users/${user.id}/totp/setup`,user,'POST')).response.status).toBe(409);
  expect(read(server,'SELECT totp_secret,totp_enabled FROM users WHERE id=?',user.id!)).toEqual(before);
});

test('malformed TOTP user IDs cannot mutate a numeric-prefix account',async({server})=>{
  const user=await createUser(server,'totp_path');
  const result=await wire(server,`/api/auth/users/${user.id}junk/totp/setup`,user,'POST');
  expect(result.response.status).toBe(400);
  expect(read(server,'SELECT totp_secret FROM users WHERE id=?',user.id!).totp_secret).toBeNull();
});

test('a TOTP step cannot authenticate two separate challenges',async({server})=>{
  const user=await mfaUser(server,'otp_replay');
  const first=await challenge(server,user),second=await challenge(server,user),code=otp();
  expect((await complete(server,first.body.totp_token,code)).response.status).toBe(200);
  const replay=await complete(server,second.body.totp_token,code,true);
  expect(replay.response.status).toBe(401);
  expect(replay.response.headers.get('set-cookie')).toBeNull();
  expect(read(server,'SELECT count(*) AS total FROM sessions WHERE user_id=? AND mfa_pending=0',user.id!).total).toBe(1);
  expect((await ok(server,'/api/auth/trusted-devices',user)).trusted_devices).toEqual([]);
});

test('rotating challenges cannot bypass the account OTP attempt window',async({server})=>{
  const user=await mfaUser(server,'otp_limit');
  for(let i=0;i<5;i++) {
    const pending=await challenge(server,user);
    expect((await complete(server,pending.body.totp_token,wrongOtp())).response.status).toBe(401);
  }
  const pending=await challenge(server,user);
  const result=await complete(server,pending.body.totp_token,otp(),true);
  expect(result.response.status).toBe(429);
  expect(result.response.headers.get('set-cookie')).toBeNull();
  expect(read(server,'SELECT count(*) AS total FROM sessions WHERE user_id=? AND mfa_pending=0',user.id!).total).toBe(0);
});

test('password changes revoke sessions, challenges and remembered devices',async({server})=>{
  const user=await mfaUser(server,'password_revoke');
  const pending=await challenge(server,user),signedIn=await complete(server,pending.body.totp_token,otp(),true);
  expect(signedIn.response.status).toBe(200);
  const session=cookie(signedIn.response,'session'),trusted=cookie(signedIn.response,'trusted_device');
  const outstanding=await challenge(server,user);
  const next='UpdatedCredential123!';
  await ok(server,`/api/auth/users/${user.id}/password`,user,'PUT',{old_password:user.password,new_password:next});
  expect((await wire(server,'/api/auth/verify',{headers:{Cookie:session}})).response.status).toBe(401);
  expect((await wire(server,'/api/auth/verify',user)).response.status).toBe(401);
  expect((await complete(server,outstanding.body.totp_token)).response.status).toBe(401);
  const updated={...user,password:next,headers:{Authorization:basic({username:user.username!,password:next})}};
  expect((await ok(server,'/api/auth/trusted-devices',updated)).trusted_devices).toEqual([]);
  const remembered=await challenge(server,updated,trusted);
  expect(remembered.body.totp_required).toBe(true);
  expect(remembered.response.headers.get('set-cookie')).toBeNull();
});

test('credential revocation failure rolls back a password change',async({server})=>{
  const user=await createUser(server,'password_rollback');
  const login=await challenge(server,user),session=cookie(login.response,'session');
  const before=read(server,'SELECT password_hash,salt FROM users WHERE id=?',user.id!);
  database(server,`CREATE TRIGGER fail_credential_revoke BEFORE DELETE ON sessions WHEN OLD.user_id=${user.id} BEGIN SELECT RAISE(ABORT,'fixture revocation failure'); END`);
  const result=await wire(server,`/api/auth/users/${user.id}/password`,user,'PUT',
    {old_password:user.password,new_password:'ChangedButRolledBack123!'});
  expect(result.response.status).toBe(500);
  expect(read(server,'SELECT password_hash,salt FROM users WHERE id=?',user.id!)).toEqual(before);
  expect((await wire(server,'/api/auth/verify',user)).response.status).toBe(200);
  expect((await wire(server,'/api/auth/verify',{headers:{Cookie:session}})).response.status).toBe(200);
});

async function inline(server:Server,user:Identity,code:string) {
  return wire(server,'/api/auth/login',anonymous,'POST',{
    username:user.username,password:user.password,totp_code:code,remember_device:true});
}

test('inline login shares challenge replay protection and account throttling',async({server})=>{
  const user=await mfaUser(server,'inline_replay'),pending=await challenge(server,user),code=otp();
  expect((await inline(server,user,code)).response.status).toBe(200);
  expect((await complete(server,pending.body.totp_token,code)).response.status).toBe(401);
  expect((await inline(server,user,code)).response.status).toBe(401);
  for(let i=0;i<3;i++)expect((await inline(server,user,wrongOtp())).response.status).toBe(401);
  const limited=await complete(server,pending.body.totp_token,otp());
  expect(limited.response.status).toBe(429);expect(limited.response.headers.get('set-cookie')).toBeNull();
  expect((await inline(server,user,otp())).response.status).toBe(429);
});

test('enrollment expires and enabling MFA preserves only the initiating owner session',async({server})=>{
  const user=await createUser(server,'enrollment');
  const owner={headers:{Cookie:cookie((await challenge(server,user)).response,'session')}};
  const other={headers:{Cookie:cookie((await challenge(server,user)).response,'session')}};
  const setup=await wire(server,`/api/auth/users/${user.id}/totp/setup`,owner,'POST');
  expect(setup.response.status).toBe(200);expect(setup.response.headers.get('cache-control')).toContain('no-store');
  database(server,'UPDATE users SET totp_setup_expires_at=0 WHERE id=?',[user.id!]);
  expect((await wire(server,`/api/auth/users/${user.id}/totp/verify`,owner,'POST',{code:otp(undefined,setup.body.secret)})).response.status).toBe(409);
  expect(read(server,'SELECT totp_enabled FROM users WHERE id=?',user.id!).totp_enabled).toBe(0);
  const retry=await ok(server,`/api/auth/users/${user.id}/totp/setup`,owner,'POST');
  const code=otp(undefined,retry.secret);
  await ok(server,`/api/auth/users/${user.id}/totp/verify`,owner,'POST',{code});
  expect((await wire(server,'/api/auth/verify',owner)).response.status).toBe(200);
  expect((await wire(server,'/api/auth/verify',other)).response.status).toBe(401);
  const status=await ok(server,`/api/auth/users/${user.id}/totp/status`,owner);
  expect(status).toEqual({totp_enabled:true,totp_configured:true});
  const pending=await challenge(server,user);
  expect((await complete(server,pending.body.totp_token,code)).response.status).toBe(401);
  expect((await wire(server,`/api/auth/users/${user.id}/totp/setup`,owner,'POST')).response.status).toBe(409);
});

test('disabling MFA revokes other sessions, challenges and devices but retains owner cookie',async({server})=>{
  const user=await mfaUser(server,'disable_revoke');
  const login=await complete(server,(await challenge(server,user)).body.totp_token,otp(),true);
  const owner={headers:{Cookie:cookie(login.response,'session')}};
  const remembered=cookie(login.response,'trusted_device');
  const second=await challenge(server,user,remembered);
  const other={headers:{Cookie:cookie(second.response,'session')}};
  const pending=await challenge(server,user);
  const result=await wire(server,`/api/auth/users/${user.id}/totp/disable`,owner,'POST');
  expect(result.response.status).toBe(200);expect(result.response.headers.get('set-cookie')).toContain('trusted_device=');
  expect((await wire(server,'/api/auth/verify',owner)).response.status).toBe(200);
  expect((await wire(server,'/api/auth/verify',other)).response.status).toBe(401);
  expect((await complete(server,pending.body.totp_token)).response.status).toBe(401);
  expect((await ok(server,'/api/auth/trusted-devices',user)).trusted_devices).toEqual([]);
  expect(read(server,'SELECT totp_secret,totp_enabled FROM users WHERE id=?',user.id!)).toEqual({totp_secret:null,totp_enabled:0});
  expect((await challenge(server,user)).response.headers.get('set-cookie')).toContain('session=');
});

test('management preserves Basic and legacy keys and enforces effective scoped permissions',async({server})=>{
  const owner=await createUser(server,'management_owner'),other=await createUser(server,'management_other');
  const legacy=await ok(server,`/api/auth/users/${owner.id}/api-key`,admin,'POST');
  const key={headers:{'X-API-Key':legacy.api_key}};
  const limited=await token(server,owner,{type:'all'},['live.view']);
  const manager=await token(server,admin,{type:'all'},['users.manage']);
  for(const identity of [owner,key]) {
    await ok(server,`/api/auth/users/${owner.id}/totp/setup`,identity,'POST');
    expect((await ok(server,`/api/auth/users/${owner.id}/totp/status`,identity)).totp_configured).toBe(true);
  }
  const before=read(server,'SELECT totp_secret,totp_enabled,password_hash,salt FROM users WHERE id=?',owner.id!);
  for(const [identity,status] of [[anonymous,401],[other,403],[limited,403]] as const) {
    for(const [suffix,method,body] of [
      ['totp/setup','POST',undefined],['totp/verify','POST',{code:wrongOtp()}],
      ['totp/disable','POST',undefined],['totp/status','GET',undefined],
      ['password','PUT',{new_password:'DeniedMutation123!'}],
    ] as const) expect((await wire(server,`/api/auth/users/${owner.id}/${suffix}`,identity,method,body)).response.status).toBe(status);
  }
  expect(read(server,'SELECT totp_secret,totp_enabled,password_hash,salt FROM users WHERE id=?',owner.id!)).toEqual(before);
  await ok(server,`/api/auth/users/${owner.id}/totp/disable`,manager,'POST');
  // Effective users.manage works without the legacy administrator role.
  const delegated=await createUser(server,'delegated_manager');
  const managerRole=await role(server,'Identity manager',['users.manage']);
  await policy(server,delegated,[{role_uuid:managerRole,scope:{type:'all'}}]);
  await ok(server,`/api/auth/users/${owner.id}/totp/setup`,delegated,'POST');
  const delegatedToken=await token(server,delegated,{type:'all'},['users.manage']);
  await ok(server,`/api/auth/users/${owner.id}/totp/setup`,delegatedToken,'POST');
  // A token loses management when its owner's effective grants no longer include it.
  await policy(server,delegated,[]);
  expect((await wire(server,`/api/auth/users/${owner.id}/totp/disable`,delegatedToken,'POST')).response.status).toBe(403);
  expect((await wire(server,`/api/auth/users/${owner.id}/totp/disable`,delegated,'POST')).response.status).toBe(403);
  expect((await wire(server,'/api/auth/users/9223372036854775807/totp/status',admin)).response.status).toBe(404);
  // Key authentication also retains password self-service with old-password proof.
  await ok(server,`/api/auth/users/${owner.id}/password`,key,'PUT',{old_password:owner.password,new_password:'KeyCompatible123!'});
  expect((await wire(server,'/api/auth/verify',key)).response.status).toBe(200);
});

test('force-MFA inline login enforces the same persisted OTP state',async({server})=>{
  const user=await mfaUser(server,'force_mfa');
  await ok(server,'/api/settings',admin,'POST',{force_mfa_on_login:true});
  expect((await challenge(server,user)).response.status).toBe(401);
  const code=otp(),login=await inline(server,user,code);
  expect(login.response.status).toBe(200);
  expect((await inline(server,user,code)).response.status).toBe(401);
  await ok(server,'/api/settings',admin,'POST',{force_mfa_on_login:false});
  expect((await complete(server,(await challenge(server,user)).body.totp_token,code)).response.status).toBe(401);
});
