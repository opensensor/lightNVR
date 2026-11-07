import {getSystemSettings,getSystemInformation,getSystemStatus,getHostHealth,getHostHealthIncidents,getSystemLogs,getGo2rtcEffectiveConfiguration,getGo2rtcOverrideDiagnostics,getWorkspaces,getSetupStatus,getStreamHealth,updateSystemSettings,updateWorkspaces,updateSetupStatus,validateGo2rtcConfiguration,clearSystemLogs,createSystemBackup,restartSystem,shutdownSystem,SystemRequestError} from '../../js/api/system.ts';
import {ContractValidationError} from '../../js/api/contractErrors.ts';
const privateValue='fixture-secret-must-not-leak';
function respond(body:unknown,status=200){return jest.spyOn(global,'fetch').mockImplementation(async()=>new Response(JSON.stringify(body),{status,headers:{'Content-Type':'application/json'}}));}
beforeEach(()=>{for(const method of ['log','debug','warn','error'] as const)jest.spyOn(console,method).mockImplementation(()=>{});});
afterEach(()=>{for(const method of ['log','debug','warn','error'] as const)expect(console[method]).not.toHaveBeenCalled();jest.restoreAllMocks();});
test('system reads reject unvalidated partial data rather than presenting healthy defaults',async()=>{
 for(const call of [getSystemSettings,getSystemInformation,getSystemStatus,getHostHealth,getHostHealthIncidents,getSystemLogs,getGo2rtcEffectiveConfiguration,getGo2rtcOverrideDiagnostics,getWorkspaces,getSetupStatus,getStreamHealth]){
  respond({message:privateValue});await expect(call()).rejects.toBeInstanceOf(ContractValidationError);
 }
});
test('incident history accepts stored observations without live freshness and still rejects incomplete values',async()=>{
 const observation={metric:'host.memory.available_ratio',resource:'host',scope:'host',capability:'available',unit:'ratio',value:0.1};
 const incident={incident_id:'11111111-1111-4111-8111-111111111111',condition:'memory.available_low',subject:'host',scope:'host',state:'closed',severity:'warning',first_observed_at_ms:1000,last_observed_at_ms:2000,closed_at_ms:2000,reconciliation:'reconciled',revision:1,observation};
 const body={schema_version:1,count:1,incidents:[incident],next_cursor:null};
 respond(body);
 await expect(getHostHealthIncidents({limit:20,include_closed:true})).resolves.toEqual(body);
 for(const field of Object.keys(observation)){
  const incomplete={...observation} as Record<string,unknown>;
  delete incomplete[field];
  respond({...body,incidents:[{...incident,observation:incomplete}]});
  await expect(getHostHealthIncidents()).rejects.toBeInstanceOf(ContractValidationError);
 }
});
test('zero-valued settings preserve intent and malformed acknowledgments may already have applied',async()=>{
 const fetch=respond({success:true,restart_required:false});
 await expect(updateSystemSettings({default_detection_threshold:0,max_storage_size:0},{retries:5})).resolves.toEqual({success:true,restart_required:false});
 expect(JSON.parse(fetch.mock.calls[0]?.[1]?.body as string)).toEqual({default_detection_threshold:0,max_storage_size:0});
 respond({success:true});await expect(updateSystemSettings({mqtt_password:privateValue})).rejects.toMatchObject({mayHaveApplied:true,phase:'response'});
});
test('invalid settings, workspaces and YAML never reach the transport',()=>{
 const fetch=respond({});
 expect(()=>updateSystemSettings({api_detection_format:'invalid',default_detection_threshold:0})).toThrow(ContractValidationError);
 expect(()=>updateWorkspaces({workspaces:{'live.navigator':'false'}})).toThrow(ContractValidationError);
 expect(()=>updateWorkspaces({workspaces:{}})).toThrow(ContractValidationError);
 expect(()=>validateGo2rtcConfiguration({override:'x'.repeat(65536)})).toThrow(ContractValidationError);
 expect(fetch).not.toHaveBeenCalled();
});
test('administrative writes never retry failures or expose secret-bearing HTTP bodies',async()=>{
 for(const call of [()=>updateSystemSettings({turn_password:privateValue},{retries:4}),()=>updateWorkspaces({workspaces:{investigation:false}},{retries:4}),()=>updateSetupStatus({complete:false},{retries:4}),()=>clearSystemLogs({retries:4}),()=>createSystemBackup({retries:4}),()=>restartSystem({retries:4}),()=>shutdownSystem({retries:4})]){
  const fetch=respond({error:privateValue},503);fetch.mockClear();
  try{await call();throw new Error('expected rejection');}catch(error){expect(error).toBeInstanceOf(SystemRequestError);expect(String(error)).not.toContain(privateValue);expect(error).toMatchObject({mayHaveApplied:true});}
  expect(fetch).toHaveBeenCalledTimes(1);
 }
});
test('bootstrap completion requires success and system status requires the native state',async()=>{
 respond({success:false});await expect(updateSetupStatus({complete:true})).rejects.toMatchObject({mayHaveApplied:true});
 respond({status:'broken',message:privateValue});await expect(getSystemStatus()).rejects.toBeInstanceOf(ContractValidationError);
 respond({success:true});await expect(updateSetupStatus({complete:true})).resolves.toEqual({success:true});
});
test('request headers are preserved; log cursor cannot change the route',async()=>{
 const fetch=respond({logs:[],file:'fixture',level:'info'});
 await getSystemLogs({last_ts:'?secret='+privateValue},{headers:new Headers({'X-API-Key':privateValue})});
 expect(new Headers(fetch.mock.calls[0]?.[1]?.headers).get('X-API-Key')).toBe(privateValue);
 expect(String(fetch.mock.calls[0]?.[0])).toBe('/api/system/logs?last_ts=%3Fsecret%3D'+privateValue);
 expect(fetch.mock.calls[0]?.[1]?.cache).toBe('no-store');
});
test('malformed JSON and abort stay sanitized',async()=>{
 jest.spyOn(global,'fetch').mockResolvedValue(new Response(privateValue));await expect(getSystemStatus()).rejects.toMatchObject({issue:'json'});
 const controller=new AbortController();controller.abort(privateValue);const fetch=respond({});fetch.mockClear();
 await expect(getSystemSettings({signal:controller.signal})).rejects.toMatchObject({name:'AbortError'});expect(fetch).not.toHaveBeenCalled();
});

import {withRedirectFixture} from './redirectHarness';
test.each([307,308] as const)('HTTP %i cannot forward settings credentials or retry the mutation',async status=>{
 await withRedirectFixture(status,async fixture=>{
  const fetch=jest.spyOn(global,'fetch').mockImplementation(fixture.fetch);
  const failure=await updateSystemSettings({mqtt_password:privateValue},{redirect:'follow',credentials:'include',retries:7}).then(()=>null,error=>error);
  expect(fixture.sourceBodies).toHaveLength(1);expect(JSON.parse(fixture.sourceBodies[0]!).mqtt_password).toBe(privateValue);
  expect(fixture.forwardedBodies).toHaveLength(0);expect(failure).toMatchObject({name:'SystemRequestError',mayHaveApplied:true});expect(fetch).toHaveBeenCalledTimes(1);
  expect(fetch.mock.calls[0]?.[1]).toMatchObject({redirect:'error',credentials:'same-origin'});
 });
});
test.each(['text/html','text/plain',''])('a %s acknowledgment cannot confirm a settings write',async contentType=>{
 jest.spyOn(global,'fetch').mockResolvedValue(new Response('{"success":true,"restart_required":false}',{status:200,headers:{'Content-Type':contentType}}));
 await expect(updateSystemSettings({mqtt_password:privateValue})).rejects.toMatchObject({phase:'response',issue:'json',mayHaveApplied:true});
});
