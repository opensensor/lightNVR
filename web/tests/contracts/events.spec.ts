import {eventJSON,EventRequestError} from '../../js/api/events.ts';
import {ContractValidationError} from '../../js/api/contractErrors.ts';
const uuid='00000000-0000-4000-8000-000000000001',secret='synthetic-secret-never-expose';
const destination={uuid,key:`mqtt:${uuid}`,name:'Fixture',description:'',enabled:false,type:'mqtt',managed:true,broker:{host:'127.0.0.1',port:1883,client_id:'fixture',topic_template:'events/{type}/{subject_id}',status_topic_template:'',keepalive_seconds:60,qos:1},authentication:{username:'fixture',password_configured:true},tls:{mode:'disabled',ca_file:'',cert_file:'',key_file:''},revision:1,created_at:0,updated_at:0};
const body={name:'Fixture',broker:{host:'127.0.0.1'},authentication:{username:'fixture',password:secret}};
function respond(value:unknown,status=200,contentType='application/json'){return jest.spyOn(global,'fetch').mockImplementation(async()=>new Response(JSON.stringify(value),{status,headers:{'Content-Type':contentType}}));}
beforeEach(()=>{for(const method of ['log','debug','warn','error'] as const)jest.spyOn(console,method).mockImplementation(()=>{});});
afterEach(()=>{for(const method of ['log','debug','warn','error'] as const)expect(console[method]).not.toHaveBeenCalled();jest.restoreAllMocks();});
test('destination issuance sends credentials only in the authorized write and validates redacted response',async()=>{
 const fetch=respond(destination,201);await expect(eventJSON('/api/event-destinations',{method:'POST',body:JSON.stringify(body)})).resolves.toEqual(destination);
 expect(JSON.parse(fetch.mock.calls[0]?.[1]?.body as string)).toEqual(body);expect(fetch.mock.calls[0]?.[1]?.cache).toBe('no-store');
 respond({...destination,authentication:{...destination.authentication,password:secret}},201);
 await expect(eventJSON('/api/event-destinations',{method:'POST',body:JSON.stringify(body)})).rejects.toMatchObject({mayHaveApplied:true});
});
test.each([{}, {...destination,revision:'1'},{...destination,broker:null},{...destination,password:secret}])('malformed destination cannot reach an editor',async value=>{
 respond(value);await expect(eventJSON(`/api/event-destinations/${uuid}`)).rejects.toBeInstanceOf(ContractValidationError);
});
test('invalid routes, revisions and preview intent fail before transport',async()=>{
 const fetch=respond({});
 for(const url of ['/api/other',`https://invalid.example/api/event-destinations/${uuid}`,`/api/event-destinations/${uuid}/../other`])await expect(eventJSON(url)).rejects.toBeInstanceOf(ContractValidationError);
 for(const suffix of ['', '?revision=0','?revision=1&revision=2','?revision=9007199254740992'])await expect(eventJSON(`/api/event-routes/${uuid}${suffix}`,{method:'DELETE'})).rejects.toBeInstanceOf(ContractValidationError);
 await expect(eventJSON('/api/event-routes/preview',{method:'POST',body:JSON.stringify({name:'Preview',event_types:['io.lightnvr.detection.object.v1'],revision:1})})).rejects.toMatchObject({phase:'request'});
 expect(fetch).not.toHaveBeenCalled();
});
test('writes never retry secret-bearing failures',async()=>{
 const fetch=respond({error:secret},503);
 try{await eventJSON('/api/event-destinations',{method:'POST',body:JSON.stringify(body),retries:6});throw new Error('expected rejection');}
 catch(error){expect(error).toBeInstanceOf(EventRequestError);expect(String(error)).not.toContain(secret);expect(error).toMatchObject({mayHaveApplied:true,status:503});}
 expect(fetch).toHaveBeenCalledTimes(1);
});
test('wrong content type and status reject, and canceled requests do not leak the reason',async()=>{
 respond(destination,200,'text/html');await expect(eventJSON(`/api/event-destinations/${uuid}`)).rejects.toMatchObject({issue:'json'});
 respond(destination,202);await expect(eventJSON(`/api/event-destinations/${uuid}`)).rejects.toMatchObject({issue:'status'});
 const controller=new AbortController();controller.abort(secret);const fetch=respond({});fetch.mockClear();
 await expect(eventJSON('/api/events/catalog',{signal:controller.signal})).rejects.toMatchObject({name:'AbortError'});expect(fetch).not.toHaveBeenCalled();
});

import {withRedirectFixture} from './redirectHarness';
test.each([307,308] as const)('HTTP %i cannot forward broker credentials or retry the mutation',async status=>{
 await withRedirectFixture(status,async fixture=>{
  const fetch=jest.spyOn(global,'fetch').mockImplementation(fixture.fetch);
  const failure=await eventJSON('/api/event-destinations',{method:'POST',body:JSON.stringify(body),redirect:'follow',credentials:'include',retries:7}).then(()=>null,error=>error);
  expect(fixture.sourceBodies).toHaveLength(1);expect(JSON.parse(fixture.sourceBodies[0]!).authentication.password).toBe(secret);
  expect(fixture.forwardedBodies).toHaveLength(0);expect(failure).toMatchObject({name:'EventRequestError',mayHaveApplied:true});expect(fetch).toHaveBeenCalledTimes(1);
  expect(fetch.mock.calls[0]?.[1]).toMatchObject({redirect:'error',credentials:'same-origin'});
 });
});
