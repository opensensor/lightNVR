/** F1 preflight regressions: challenge tokens must never be session credentials. */
import {createHmac} from 'node:crypto';
import path from 'node:path';
import {DatabaseSync} from 'node:sqlite';
import {test,expect,anonymous,ok,wire,createUser,database,type Server,type Identity} from './permissionCases';

const secret='JBSWY3DPEHPK3PXP';
function code(step=Math.floor(Date.now()/30000)) {
  const alphabet='ABCDEFGHIJKLMNOPQRSTUVWXYZ234567';
  let bits='';for(const c of secret) bits+=alphabet.indexOf(c).toString(2).padStart(5,'0');
  const key=Buffer.from(bits.match(/.{8}/g)!.map(byte=>parseInt(byte,2)));
  const counter=Buffer.alloc(8);counter.writeBigUInt64BE(BigInt(step));
  const digest=createHmac('sha1',key).update(counter).digest();
  return ((digest.readUInt32BE(digest[19]!&15)&0x7fffffff)%1000000).toString().padStart(6,'0');
}
function counts(server:Server,user:Identity) {
  const db=new DatabaseSync(path.join(server.directory,'lightnvr.db'));
  try {return db.prepare('SELECT sum(mfa_pending=0) AS sessions,sum(mfa_pending=1) AS challenges FROM sessions WHERE user_id=?').get(user.id!);} finally {db.close();}
}
async function mfaUser(server:Server) {
  const user=await createUser(server,'mfa_boundary');
  database(server,'UPDATE users SET totp_secret=?,totp_enabled=1 WHERE id=?',[secret,user.id!]);
  return user;
}
async function challenge(server:Server,user:Identity) {
  const result=await wire(server,'/api/auth/login',anonymous,'POST',{username:user.username,password:user.password});
  expect(result.response.status).toBe(200);expect(result.body.totp_required).toBe(true);
  expect(result.response.headers.get('set-cookie')).toBeNull();
  return result.body.totp_token as string;
}
async function complete(server:Server,token:string,otp=code(),remember_device=false) {
  return wire(server,'/api/auth/login/totp',anonymous,'POST',{totp_token:token,code:otp,remember_device});
}
function sessionCookie(response:Response) {
  const value=response.headers.getSetCookie().find(value=>value.startsWith('session='));
  expect(value).toBeTruthy();return value!.split(';')[0]!;
}

test('pending MFA tokens cannot authenticate and a completed token cannot be reused',async({server})=>{
  const user=await mfaUser(server), pending=await challenge(server,user);
  const pendingIdentity={headers:{Cookie:`session=${pending}`}};
  for(const route of ['/api/auth/verify','/api/openapi.json','/api/auth/sessions','/api/streams'])
    expect((await wire(server,route,pendingIdentity)).response.status).toBe(401);
  expect((await ok(server,'/api/auth/sessions',user)).sessions).toEqual([]);
  const result=await complete(server,pending);
  expect(result.response.status).toBe(200);expect(result.body).toEqual({success:true,redirect:'/index.html'});
  const authenticated={headers:{Cookie:sessionCookie(result.response)}};
  expect((await wire(server,'/api/auth/verify',authenticated)).response.status).toBe(200);
  expect((await complete(server,pending)).response.status).toBe(401);
  expect(counts(server,user)?.sessions).toBe(1);
  const fullToken=authenticated.headers.Cookie.split('=')[1]!;
  expect((await complete(server,fullToken)).response.status).toBe(401);
  expect((await wire(server,'/api/auth/verify',authenticated)).response.status).toBe(200);
});

test('concurrent MFA completions issue exactly one session and trusted device',async({server})=>{
  const user=await mfaUser(server), pending=await challenge(server,user);
  const responses=await Promise.all([complete(server,pending,code(),true),complete(server,pending,code(),true)]);
  expect(responses.map(r=>r.response.status).sort()).toEqual([200,401]);
  expect(counts(server,user)?.sessions).toBe(1);
  const devices=await ok(server,'/api/auth/trusted-devices',user);
  expect(devices.trusted_devices).toHaveLength(1);
  const success=responses.find(r=>r.response.status===200)!;
  const trusted=success.response.headers.getSetCookie().find(v=>v.startsWith('trusted_device='))!.split(';')[0]!;
  const passwordOnly=await wire(server,'/api/auth/login',{headers:{Cookie:trusted}},'POST',{username:user.username,password:user.password});
  expect(passwordOnly.body.success).toBe(true);expect(passwordOnly.body.totp_required).toBeUndefined();
  await ok(server,`/api/auth/trusted-devices/${devices.trusted_devices[0].id}`,user,'DELETE');
  const revoked=await wire(server,'/api/auth/login',{headers:{Cookie:trusted}},'POST',{username:user.username,password:user.password});
  expect(revoked.body.totp_required).toBe(true);expect(revoked.response.headers.get('set-cookie')).toBeNull();
});

test('five failed MFA checks exhaust a challenge without issuing credentials',async({server})=>{
  const user=await mfaUser(server), pending=await challenge(server,user);
  for(let i=0;i<5;i++) {
    const result=await complete(server,pending,'nototp',true);
    expect(result.response.status).toBe(401);expect(result.response.headers.get('set-cookie')).toBeNull();
  }
  expect((await complete(server,pending)).response.status).toBe(401);
  expect(counts(server,user)?.sessions).toBe(0);
  expect((await ok(server,'/api/auth/trusted-devices',user)).trusted_devices).toEqual([]);
  const fresh=await challenge(server,user);
  expect((await complete(server,fresh)).response.status).toBe(200);
});

test('expired, wrong-context, revoked and source-restricted challenges fail closed',async({server})=>{
  const user=await mfaUser(server), pending=await challenge(server,user);
  const mismatched=await wire(server,'/api/auth/login/totp',{headers:{'User-Agent':'another-client'}},'POST',{totp_token:pending,code:code()});
  expect(mismatched.response.status).toBe(401);
  database(server,'UPDATE sessions SET expires_at=0 WHERE token=?',[pending]);
  expect((await complete(server,pending)).response.status).toBe(401);
  const expiredState=counts(server,user);expect(expiredState?.sessions).toBe(0);
  const restricted=await challenge(server,user);
  database(server,'UPDATE users SET allowed_login_cidrs=? WHERE id=?',['192.0.2.0/24',user.id!]);
  expect((await complete(server,restricted)).response.status).toBe(401);
  database(server,'UPDATE users SET allowed_login_cidrs=NULL WHERE id=?',[user.id!]);
  const revoked=await challenge(server,user);
  await ok(server,`/api/auth/users/${user.id}/password`,user,'PUT',{old_password:user.password,new_password:'ChangedFixture123!'});
  expect((await complete(server,revoked)).response.status).toBe(401);
});

test('legacy form login cannot bypass an MFA account challenge',async({server})=>{
  const user=await mfaUser(server);
  const response=await fetch(`${server.baseURL}/api/auth/login`,{method:'POST',redirect:'manual',headers:{'Content-Type':'application/x-www-form-urlencoded'},body:new URLSearchParams({username:user.username!,password:user.password!})});
  expect(response.status).toBe(302);expect(response.headers.get('location')).toBe('/login.html');
  expect(response.headers.get('set-cookie')).toBeNull();
  expect(counts(server,user)?.sessions??0).toBe(0);
});
