import {fleetWriteJSON,FleetWriteError} from '../../js/api/fleetWrites.ts';
import {ContractValidationError} from '../../js/api/contractErrors.ts';
const uuid='00000000-0000-4000-8000-000000000001';
const location={uuid,name:'Fixture',parent_uuid:null,type:'site',sort_order:0,is_system:false,metadata:{},child_count:0,camera_count:0,created_at:0,updated_at:0};
function respond(body:unknown,status=200){return jest.spyOn(global,'fetch').mockImplementation(async()=>new Response(JSON.stringify(body),{status,headers:{'Content-Type':'application/json'}}));}
beforeEach(()=>{for(const method of ['log','debug','warn','error'] as const)jest.spyOn(console,method).mockImplementation(()=>{});});
afterEach(()=>{for(const method of ['log','debug','warn','error'] as const)expect(console[method]).not.toHaveBeenCalled();jest.restoreAllMocks();});
test('validated create retains complete native response and never retries',async()=>{
 const fetch=respond(location,201);await expect(fleetWriteJSON('/api/locations',{method:'POST',body:JSON.stringify({name:'Fixture'}),retries:9})).resolves.toEqual(location);expect(fetch).toHaveBeenCalledTimes(1);
});
test.each([{}, {...location,secret:'private-wire-value'},{...location,camera_count:'0'}])('malformed write responses cannot be reported as success',async body=>{
 const fetch=respond(body,201);await expect(fleetWriteJSON('/api/locations',{method:'POST',body:JSON.stringify({name:'Fixture'}),retries:9})).rejects.toMatchObject({name:'ContractValidationError',mayHaveApplied:true});expect(fetch).toHaveBeenCalledTimes(1);
});
test.each([['/api/locations',{method:'POST',body:'{}'}],['/api/locations/invalid',{method:'DELETE'}],['/api/fleet/views/'+uuid,{method:'DELETE',body:'{"revision":0}'}],['/api/unknown',{method:'POST'}]])('invalid write input fails before transport',async(url,options)=>{
 const fetch=respond({});await expect(fleetWriteJSON(url as string,options as any)).rejects.toBeInstanceOf(ContractValidationError);expect(fetch).not.toHaveBeenCalled();
});
test('HTTP failures are sanitized and no mutation is replayed',async()=>{
 const fetch=respond({error:'private-wire-value'},500);await expect(fleetWriteJSON('/api/camera-favorites/'+uuid,{method:'PUT',retries:8})).rejects.toEqual(new FleetWriteError('addCameraFavorite',500));expect(fetch).toHaveBeenCalledTimes(1);
});
test('plan and layout envelopes fail closed instead of populating query caches',async()=>{
 for(const route of ['/api/live/plans','/api/live/layouts','/api/camera-favorites']){
  respond({plans:[{name:'private-wire-value'}],layouts:[],favorites:[]});await expect(fleetWriteJSON(route)).rejects.toBeInstanceOf(ContractValidationError);
 }
});
test('aborted workspace fetch remains cancellation',async()=>{
 const controller=new AbortController();controller.abort();const fetch=respond({layouts:[],count:0,can_share:false,can_modify:false});await expect(fleetWriteJSON('/api/live/layouts',{signal:controller.signal})).rejects.toMatchObject({name:'AbortError'});expect(fetch).not.toHaveBeenCalled();
});
test('background upload only accepts an owned bounded image blob',async()=>{
 const fetch=respond({});await expect(fleetWriteJSON('/api/live/plans/'+uuid+'/background',{method:'PUT',headers:{'Content-Type':'text/plain'},body:new Blob(['bad'])})).rejects.toBeInstanceOf(ContractValidationError);expect(fetch).not.toHaveBeenCalled();
});
