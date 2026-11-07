import {administrationJSON,exportAuditEvents,auditCsvHeader,AdministrationRequestError} from '../../js/api/administration.ts';
import {ContractValidationError} from '../../js/api/contractErrors.ts';
const privateValue='DO-NOT-LOG-PRIVATE-PAYLOAD';
const user={id:3,username:'fixture',email:'',role:2,role_name:'viewer',created_at:1,updated_at:1,last_login:0,is_active:true,password_change_locked:false,must_change_password:false,totp_enabled:false,authorization_mode:'policy',allowed_login_cidrs:null};
function respond(body:unknown,status=200){return jest.spyOn(global,'fetch').mockResolvedValue(new Response(JSON.stringify(body),{status,headers:{'Content-Type':'application/json'}}));}
beforeEach(()=>{for(const method of ['log','debug','warn','error'] as const)jest.spyOn(console,method).mockImplementation(()=>{});});
afterEach(()=>{for(const method of ['log','debug','warn','error'] as const)expect(console[method]).not.toHaveBeenCalled();jest.restoreAllMocks();});
test('closed route map validates users and rejects credential-bearing records without mutating input',async()=>{
  const body={users:[user]};respond(body);expect(await administrationJSON('/api/auth/users')).toEqual(body);
  for(const malformed of [{},{users:null},{users:[{...user,id:'3'}]},{users:[{...user,api_key:privateValue}]},{users:[{...user,password_hash:privateValue}]}]){
    respond(malformed);await expect(administrationJSON('/api/auth/users')).rejects.toBeInstanceOf(ContractValidationError);
  }
  expect(body).toEqual({users:[user]});
});
test('writes validate requests before sending credentials and never retry failed acknowledgments',async()=>{
  const fetch=respond({success:false,error:privateValue});
  await expect(administrationJSON('/api/auth/users/3/password-lock',{method:'PUT',body:'{"locked":"yes"}'})).rejects.toMatchObject({phase:'request',mayHaveApplied:false});
  expect(fetch).not.toHaveBeenCalled();
  await expect(administrationJSON('/api/auth/users/3',{method:'DELETE',retries:4})).rejects.toMatchObject({phase:'response',mayHaveApplied:true});
  expect(fetch).toHaveBeenCalledTimes(1);
  expect(fetch.mock.calls[0]?.[1]).toMatchObject({cache:'no-store',credentials:'same-origin',redirect:'error'});
});
test('HTTP errors cannot leak messages and ambiguous mutations are identified',async()=>{
  const fetch=respond({error:privateValue},500);
  await expect(administrationJSON('/api/auth/users/3/api-key',{method:'POST',retries:3})).rejects.toEqual(new AdministrationRequestError('createLegacyApiKey',500,true));
  expect(fetch).toHaveBeenCalledTimes(1);
});
test('legacy key accepts only the one-time exact response and no list secret',async()=>{
  respond({success:true,api_key:privateValue});expect(await administrationJSON('/api/auth/users/3/api-key',{method:'POST'})).toEqual({success:true,api_key:privateValue});
  respond({success:true,api_key:privateValue,totp_secret:privateValue});await expect(administrationJSON('/api/auth/users/3/api-key',{method:'POST'})).rejects.toMatchObject({mayHaveApplied:true});
});
test('malformed JSON/status/content type cannot acknowledge writes',async()=>{
  for(const response of [new Response(privateValue,{status:200,headers:{'Content-Type':'application/json'}}),new Response(JSON.stringify(user),{status:201,headers:{'Content-Type':'application/json'}}),new Response(JSON.stringify(user),{status:200,headers:{'Content-Type':'text/plain'}})]){
    jest.spyOn(global,'fetch').mockResolvedValue(response);
    await expect(administrationJSON('/api/auth/users/3',{method:'PUT',body:'{"email":"test"}'})).rejects.toMatchObject({mayHaveApplied:true});
  }
});
test('versioned writes reject missing revision and unsupported route/unsafe ID before fetch',async()=>{
  const fetch=respond({});
  for(const [route,options] of [['/api/authorization/roles',{method:'POST',body:'{"name":"role","actions":["users.manage"]}'}],['/api/auth/users/9007199254740992',{}],['https://foreign.invalid/api/auth/users',{}],['/api/auth/users/3/uncovered',{method:'DELETE'}]] as const){await expect(administrationJSON(route,options)).rejects.toBeInstanceOf(Error);}
  expect(fetch).not.toHaveBeenCalled();
});
test('CSV export requires attachment metadata and actual CSV header bytes',async()=>{
  jest.spyOn(global,'fetch').mockResolvedValue(new Response(auditCsvHeader,{headers:{'Content-Type':'text/csv; charset=utf-8','Content-Disposition':'attachment; filename=lightnvr-audit.csv','X-Total-Count':'0'}}));
  const blob=await exportAuditEvents('page=1');expect(blob.type).toBe('text/csv;charset=utf-8');
  respond({error:privateValue});await expect(exportAuditEvents('')).rejects.toBeInstanceOf(ContractValidationError);
  jest.spyOn(global,'fetch').mockResolvedValue(new Response(privateValue,{headers:{'Content-Type':'text/csv','Content-Disposition':'attachment; filename=lightnvr-audit.csv','X-Total-Count':'0'}}));
  await expect(exportAuditEvents('')).rejects.toMatchObject({schema:'AuditCsv'});
});
test('aborted operation stays cancellation and sends no request',async()=>{
  const controller=new AbortController();controller.abort();const fetch=respond({});
  await expect(administrationJSON('/api/auth/users',{signal:controller.signal})).rejects.toMatchObject({name:'AbortError'});expect(fetch).not.toHaveBeenCalled();
});

test('composite user save explicitly reports partial lock failure without retrying creation',async()=>{
  const {saveAdminUser}=await import('../../js/api/administration.ts');
  const fetch=jest.spyOn(global,'fetch').mockResolvedValueOnce(new Response(JSON.stringify(user),{headers:{'Content-Type':'application/json'}})).mockResolvedValueOnce(new Response('{"error":"private"}',{status:500,headers:{'Content-Type':'application/json'}}));
  await expect(saveAdminUser('/api/auth/users',{username:'fixture',password:'ContractPassword123!',password_change_locked:true},{method:'POST',retries:4})).rejects.toThrow('User details were saved, but the password lock was not confirmed.');
  expect(fetch).toHaveBeenCalledTimes(2);expect(fetch.mock.calls.map(call=>call[0])).toEqual(['/api/auth/users','/api/auth/users/3/password-lock']);
  expect(JSON.parse(fetch.mock.calls[0]![1]!.body as string)).not.toHaveProperty('password_change_locked');
});


test('a mismatched user response cannot redirect the follow-up password lock to another account',async()=>{
  const {saveAdminUser}=await import('../../js/api/administration.ts');const fetch=respond({...user,id:4});
  await expect(saveAdminUser('/api/auth/users/3',{email:'fixture@example.test',password_change_locked:true},{method:'PUT'})).rejects.toMatchObject({phase:'response',mayHaveApplied:true});
  expect(fetch).toHaveBeenCalledTimes(1);
});
