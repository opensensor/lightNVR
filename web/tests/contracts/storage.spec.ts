import {storageJSON,StorageRequestError} from '../../js/api/storage.ts';
import {ContractValidationError} from '../../js/api/contractErrors.ts';
const uuid='00000000-0000-4000-8000-000000000001';
function respond(value:unknown,status=200){return jest.spyOn(global,'fetch').mockImplementation(async()=>new Response(JSON.stringify(value),{status,headers:{'Content-Type':'application/json'}}));}
beforeEach(()=>{for(const method of ['log','debug','warn','error'] as const)jest.spyOn(console,method).mockImplementation(()=>{});});
afterEach(()=>{for(const method of ['log','debug','warn','error'] as const)expect(console[method]).not.toHaveBeenCalled();jest.restoreAllMocks();});
test('validated empty catalogs preserve zero counts',async()=>{
 for(const [route,value]of [['/api/storage-targets',{targets:[],count:0}],['/api/storage-pools',{pools:[],count:0}],['/api/storage-policies',{policies:[],count:0}],['/api/storage-migrations',{jobs:[],count:0,worker_concurrency:1}]] as const){respond(value);await expect(storageJSON(route)).resolves.toEqual(value);}
});
test.each(['/api/storage-targets','/api/storage-pools','/api/storage-policies','/api/storage-migrations','/api/storage-archive','/api/storage-compliance','/api/storage/health'])('malformed catalogs never reach consumer caches: %s',async route=>{
 respond({secret:'private-wire-value'});await expect(storageJSON(route)).rejects.toBeInstanceOf(ContractValidationError);
});
test('invalid mutations and revisions never reach transport',async()=>{
 const fetch=respond({});for(const [route,options]of [[`/api/storage-targets/${uuid}?revision=0`,{method:'DELETE'}],[`/api/storage-pools/${uuid}?revision=1&revision=2`,{method:'DELETE'}],['/api/storage/cleanup',{method:'POST',body:'{"aggressive":"yes"}'}],['/api/storage-migrations',{method:'POST',body:'{"recording_id":0}'}],['/api/unknown',{method:'POST'}]] as const)await expect(storageJSON(route,options)).rejects.toBeInstanceOf(ContractValidationError);expect(fetch).not.toHaveBeenCalled();
});
test('cleanup acknowledgement is accepted without implying completed cleanup',async()=>{
 const value={success:true,aggressive:false,message:'Cleanup triggered successfully'};respond(value);await expect(storageJSON('/api/storage/cleanup',{method:'POST'})).resolves.toEqual(value);
});
test('failed writes are sanitized, uncertain and never automatically replayed',async()=>{
 const fetch=respond({error:'private-wire-value'},500);await expect(storageJSON('/api/storage/cleanup',{method:'POST',retries:5})).rejects.toEqual(new StorageRequestError('triggerStorageCleanup',500,true));expect(fetch).toHaveBeenCalledTimes(1);
});
test('malformed write acknowledgement preserves mayHaveApplied',async()=>{
 const fetch=respond({success:true});await expect(storageJSON('/api/storage/cleanup',{method:'POST',retries:5})).rejects.toMatchObject({name:'ContractValidationError',mayHaveApplied:true});expect(fetch).toHaveBeenCalledTimes(1);
});
test('cancellation stays cancellation',async()=>{const controller=new AbortController();controller.abort();const fetch=respond({});await expect(storageJSON('/api/storage-targets',{signal:controller.signal})).rejects.toMatchObject({name:'AbortError'});expect(fetch).not.toHaveBeenCalled();});
