import {login,completeTotpLogin,getLoginConfig,logout,getTotpStatus,setupTotp,verifyTotp,disableTotp,changePassword,AuthenticationRequestError} from '../../js/api/authentication.ts';
import {ContractValidationError} from '../../js/api/contractErrors.ts';
const sensitive='do-not-expose-private-payload';
const secret='JBSWY3DPEHPK3PXPJBSWY3DPEHPK3PXP';
const setup={success:true,secret,otpauth_uri:`otpauth://totp/fixture?secret=${secret}`,message:'Scan fixture'};
function respond(body:unknown,status=200){return jest.spyOn(global,'fetch').mockImplementation(async()=>new Response(JSON.stringify(body),{status}));}
beforeEach(()=>{for(const method of ['log','debug','warn','error'] as const)jest.spyOn(console,method).mockImplementation(()=>{});});
afterEach(()=>{for(const method of ['log','debug','warn','error'] as const)expect(console[method]).not.toHaveBeenCalled();jest.restoreAllMocks();});
test('login separates a validated challenge from a complete login',async()=>{
  respond({totp_required:true,totp_token:sensitive});expect(await login({username:'fixture',password:sensitive})).toEqual({kind:'challenge',token:sensitive});
  respond({success:true,redirect:'/index.html',must_change_password:true});expect(await login({username:'fixture',password:sensitive})).toEqual({kind:'authenticated',mustChangePassword:true});
});
test.each([{}, {success:true},{totp_required:true},{success:true,redirect:'/index.html',must_change_password:false,totp_required:true,totp_token:sensitive}, {success:true,redirect:'https://example.invalid',must_change_password:false}])('malformed/ambiguous login cannot authenticate',async body=>{
  const fetch=respond(body);await expect(login({username:'fixture',password:sensitive})).rejects.toBeInstanceOf(ContractValidationError);expect(fetch).toHaveBeenCalledTimes(1);
});
test('MFA and password operations validate every success envelope',async()=>{
  respond({success:true,redirect:'/index.html'});await expect(completeTotpLogin({totp_token:sensitive,code:'123456'})).resolves.toHaveProperty('success',true);
  respond({totp_enabled:false,totp_configured:true});await expect(getTotpStatus(1)).resolves.toHaveProperty('totp_configured',true);
  respond(setup);await expect(setupTotp(1)).resolves.toEqual(setup);
  respond({success:true,message:'Done'});await expect(verifyTotp(1,'123456')).resolves.toHaveProperty('success',true);await expect(disableTotp(1)).resolves.toHaveProperty('success',true);
  respond({success:true,must_change_password:false});await expect(changePassword(1,{old_password:sensitive,new_password:sensitive})).resolves.toHaveProperty('success',true);
  respond({success:true,redirect:'/login.html?logout=true'});await expect(logout()).resolves.toHaveProperty('success',true);
});
test.each([{}, {totp_enabled:'false',totp_configured:true},{totp_enabled:false,totp_configured:false,secret:sensitive}])('unknown MFA status never means disabled',async body=>{
  respond(body);await expect(getTotpStatus(1)).rejects.toBeInstanceOf(ContractValidationError);
});
test.each([{}, {success:false}, {success:true,message:'Done',secret:sensitive}])('malformed mutation responses never acknowledge success or retry',async body=>{
  const fetch=respond(body);
  for(const action of [()=>verifyTotp(1,'123456'),()=>disableTotp(1),()=>changePassword(1,{new_password:sensitive}),()=>completeTotpLogin({totp_token:sensitive,code:'123456'})])
    await expect(action()).rejects.toMatchObject({name:'ContractValidationError',mayHaveApplied:true});
  expect(fetch).toHaveBeenCalledTimes(4);
});
test.each(['https://example.invalid/?secret=', 'otpauth://hotp/fixture?secret=', 'otpauth://totp/fixture?secret=WRONG&ignored='])('enrollment URI must be TOTP and match the returned secret',async prefix=>{
  respond({...setup,otpauth_uri:prefix+secret});await expect(setupTotp(1)).rejects.toBeInstanceOf(ContractValidationError);
});
test.each([0,-1,NaN,1.1,Number.MAX_SAFE_INTEGER+1])('invalid user IDs never send a request: %s',async id=>{
  const fetch=respond({});expect(()=>getTotpStatus(id)).toThrow(ContractValidationError);await expect(setupTotp(id)).rejects.toBeInstanceOf(ContractValidationError);expect(fetch).not.toHaveBeenCalled();
});
test.each([401,409,429,500])('HTTP %s errors preserve status without server payloads',async status=>{
  const fetch=respond({error:sensitive},status);
  await expect(disableTotp(1)).rejects.toEqual(new AuthenticationRequestError('disableTotp',status,status>=500));expect(fetch).toHaveBeenCalledTimes(1);
});
test('bad input, bad JSON and unexpected success status fail safely',async()=>{
  const fetch=respond({});expect(()=>verifyTotp(1,sensitive)).toThrow(ContractValidationError);expect(fetch).not.toHaveBeenCalled();
  fetch.mockResolvedValueOnce(new Response(sensitive));await expect(disableTotp(1)).rejects.toMatchObject({issue:'json',mayHaveApplied:true});
  respond({},202);await expect(disableTotp(1)).rejects.toMatchObject({issue:'status',mayHaveApplied:true});
});
test('public configuration is validated, credentials are same-origin, and requests are private',async()=>{
  const fetch=respond({force_mfa_on_login:false,remember_device_enabled:true,auth_timeout_hours:1,auth_absolute_timeout_hours:2,trusted_device_days:30});
  expect((await getLoginConfig()).remember_device_enabled).toBe(true);
  expect(fetch.mock.calls[0]?.[1]).toMatchObject({credentials:'same-origin',cache:'no-store',redirect:'error'});
  respond({force_mfa_on_login:'false'});await expect(getLoginConfig()).rejects.toBeInstanceOf(ContractValidationError);
});
test('cancellation suppresses late successful responses and exposes no abort reason',async()=>{
  const controller=new AbortController();
  const fetch=respond({success:true,message:'Done'});
  fetch.mockImplementationOnce(async()=>{controller.abort(sensitive);return new Response(JSON.stringify({success:true,message:'Done'}));});
  await expect(disableTotp(1,{signal:controller.signal})).rejects.toMatchObject({name:'AbortError',message:'Request cancelled'});
  fetch.mockClear();await expect(getTotpStatus(1,{signal:controller.signal})).rejects.toMatchObject({name:'AbortError'});expect(fetch).not.toHaveBeenCalled();
});
