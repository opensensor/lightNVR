import {listAuthSessions,listTrustedDevices,revokeAuthSession,revokeTrustedDevice,IdentityRequestError} from '../../js/api/identity.ts';
import {ContractValidationError} from '../../js/api/contractErrors.ts';
const session={id:1,created_at:1,last_activity_at:2,idle_expires_at:3,expires_at:4,ip_address:'127.0.0.1',user_agent:'fixture',current:true};
const device={id:2,created_at:1,last_used_at:2,expires_at:4,ip_address:'127.0.0.1',user_agent:'fixture',current:false};
const secret='private-fixture-credential';
function respond(body:unknown,status=200){return jest.spyOn(global,'fetch').mockImplementation(async()=>new Response(JSON.stringify(body),{status,headers:{'Content-Type':'application/json'}}));}
beforeEach(()=>{for(const name of ['log','debug','warn','error'] as const)jest.spyOn(console,name).mockImplementation(()=>{});});
afterEach(()=>jest.restoreAllMocks());
function noLogs(){for(const name of ['log','debug','warn','error'] as const)expect(console[name]).not.toHaveBeenCalled();}

test('typed reads accept real empty and populated response shapes without logging credential metadata',async()=>{
  respond({sessions:[session]});expect(await listAuthSessions()).toEqual({sessions:[session]});
  respond({trusted_devices:[device]});expect(await listTrustedDevices()).toEqual({trusted_devices:[device]});
  respond({sessions:[]});expect(await listAuthSessions()).toEqual({sessions:[]});noLogs();
});
test.each([{}, {sessions:null},{sessions:[{...session,id:1.5}]},{sessions:[{...session,current:'yes'}]},{sessions:[{...session,token:secret}]},{sessions:Array(33).fill(session)}])('rejects malformed/private session lists',async body=>{
  respond(body);await expect(listAuthSessions()).rejects.toBeInstanceOf(ContractValidationError);noLogs();
});
test('device lists require complete redacted records',async()=>{
  respond({trusted_devices:[{...device,token:secret}]});await expect(listTrustedDevices()).rejects.toBeInstanceOf(ContractValidationError);
  respond({trusted_devices:[{id:2}]});await expect(listTrustedDevices()).rejects.toBeInstanceOf(ContractValidationError);noLogs();
});
test.each([{}, {success:false},{success:true,token:secret}])('invalid revoke acknowledgment cannot succeed or trigger a retry',async body=>{
  const fetch=respond(body);await expect(revokeAuthSession(1)).rejects.toMatchObject({name:'ContractValidationError',mayHaveApplied:true});expect(fetch).toHaveBeenCalledTimes(1);noLogs();
});
test('successful revokes use DELETE with validated IDs',async()=>{
  const fetch=respond({success:true});expect(await revokeAuthSession(1)).toEqual({success:true});expect(await revokeTrustedDevice(2)).toEqual({success:true});
  expect(fetch.mock.calls[0]?.[0]).toBe('/api/auth/sessions/1');expect(fetch.mock.calls[1]?.[0]).toBe('/api/auth/trusted-devices/2');
  expect(fetch.mock.calls.every(call=>call[1]?.method==='DELETE')).toBe(true);noLogs();
});
test.each([0,-1,1.5,NaN,Infinity,Number.MAX_SAFE_INTEGER+1])('invalid numeric ID %s never sends a request',async id=>{
  const fetch=respond({success:true});await expect(revokeAuthSession(id)).rejects.toBeInstanceOf(ContractValidationError);await expect(revokeTrustedDevice(id)).rejects.toBeInstanceOf(ContractValidationError);expect(fetch).not.toHaveBeenCalled();noLogs();
});
test('HTTP failure redacts server errors and does not automatically retry a revoke',async()=>{
  const fetch=respond({error:secret},500);await expect(revokeTrustedDevice(2,{retries:3})).rejects.toEqual(new IdentityRequestError('revokeTrustedDevice',500,true));expect(fetch).toHaveBeenCalledTimes(1);noLogs();
});
test('malformed JSON and unexpected success status cannot acknowledge revocation',async()=>{
  jest.spyOn(global,'fetch').mockResolvedValue(new Response(secret,{status:200}));await expect(revokeAuthSession(1)).rejects.toMatchObject({name:'ContractValidationError',issue:'json',mayHaveApplied:true});
  respond({success:true},202);await expect(revokeAuthSession(1)).rejects.toMatchObject({name:'ContractValidationError',issue:'status',mayHaveApplied:true});noLogs();
});
test('an aborted read remains a cancellation with no sensitive error contents',async()=>{
  const controller=new AbortController();controller.abort(secret);
  const fetch=respond({sessions:[]});await expect(listAuthSessions({signal:controller.signal})).rejects.toMatchObject({name:'AbortError'});expect(fetch).not.toHaveBeenCalled();noLogs();
});
