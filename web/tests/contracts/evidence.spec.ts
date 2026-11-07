import {evidenceJSON,EvidenceRequestError} from '../../js/api/evidence.ts';
import {ContractValidationError} from '../../js/api/contractErrors.ts';
const camera='00000000-0000-4000-8000-000000000001';
const input={camera_uuids:[camera],start_time:1,end_time:3};
function response(body:unknown,status=200,headers:Record<string,string>={}){return new Response(JSON.stringify(body),{status,headers:{'Content-Type':'application/json',...headers}});}
function respond(body:unknown,status=200){return jest.spyOn(global,'fetch').mockResolvedValue(response(body,status));}
beforeEach(()=>{for(const name of ['log','debug','warn','error'] as const)jest.spyOn(console,name).mockImplementation(()=>{});});
afterEach(()=>{for(const name of ['log','debug','warn','error'] as const)expect(console[name]).not.toHaveBeenCalled();jest.restoreAllMocks();});
test('query results use canonical route-specific guards and reject hidden extra data',async()=>{
 respond({segment:null});expect(await evidenceJSON('/api/investigations/segment-at',{method:'POST',body:JSON.stringify({camera_uuids:[camera],timestamp:2})})).toEqual({segment:null});
 for(const invalid of [{segment:null,password:'PRIVATE'},{segment:{id:1}},[]]){respond(invalid);await expect(evidenceJSON('/api/investigations/segment-at',{method:'POST',body:JSON.stringify({camera_uuids:[camera],timestamp:2})})).rejects.toBeInstanceOf(ContractValidationError);}
});
test('unknown routes and invalid query bounds are rejected before sending credentials',async()=>{
 const fetch=respond({});for(const [url,options]of [['https://other.invalid/api/investigation-bookmarks',{}],['/api/investigations/search',{method:'POST',body:'{}'}],['/api/investigations/thumbnail-samples',{method:'POST',body:JSON.stringify({...input,sample_count:1000})}] ]as const)await expect(evidenceJSON(url,options)).rejects.toBeInstanceOf(Error);expect(fetch).not.toHaveBeenCalled();
});
test('query POST failures are sanitized but never classified as applied mutations',async()=>{
 const fetch=respond({error:'PRIVATE'},500);await expect(evidenceJSON('/api/investigations/timeline',{method:'POST',body:JSON.stringify(input)})).rejects.toEqual(new EvidenceRequestError('queryEvidenceTimeline',500,false));expect(fetch).toHaveBeenCalledTimes(1);
});
test('versioned bookmark deletes cannot acknowledge malformed success and never retry',async()=>{
 const fetch=respond({success:'PRIVATE'});await expect(evidenceJSON(`/api/investigation-bookmarks/${camera}`,{method:'DELETE',body:'{"revision":1}',retries:4})).rejects.toMatchObject({phase:'response',mayHaveApplied:true});expect(fetch).toHaveBeenCalledTimes(1);expect(fetch.mock.calls[0]?.[1]).toMatchObject({credentials:'same-origin',cache:'no-store',redirect:'error'});
});
test('revision is required before sending a bookmark write',async()=>{
 const fetch=respond({success:true});await expect(evidenceJSON(`/api/investigation-bookmarks/${camera}`,{method:'DELETE',body:'{}'})).rejects.toMatchObject({phase:'request',mayHaveApplied:false});expect(fetch).not.toHaveBeenCalled();
});
test('file deletion accepts native pending response and server errors remain ambiguous',async()=>{
 respond({success:true,existed:false,status:'deletion_pending'},202);expect(await evidenceJSON('/api/recordings/files?path=private',{method:'DELETE'})).toMatchObject({status:'deletion_pending'});
 const fetch=respond({error:'PRIVATE'},503);await expect(evidenceJSON('/api/recordings/files?path=private',{method:'DELETE',retries:5})).rejects.toEqual(new EvidenceRequestError('deleteEvidenceRecordingFile',503,true));expect(fetch).toHaveBeenCalledTimes(2);
});
test('LPR export requires actual JSON attachment metadata and canonical data',async()=>{
 const options={method:'POST',body:JSON.stringify({camera_uuid:camera,start_at:1,end_at:2})};
 respond({reads:[],count:0});await expect(evidenceJSON('/api/lpr/export',options)).rejects.toBeInstanceOf(ContractValidationError);
 jest.spyOn(global,'fetch').mockResolvedValue(response({reads:[],count:0},200,{'Content-Disposition':'attachment; filename="lightnvr-lpr-export.json"'}));expect(await evidenceJSON('/api/lpr/export',options)).toEqual({reads:[],count:0});
});
test('schema failures and malformed JSON preserve sanitized operation errors',async()=>{
 for(const value of [response({detections:[],timestamp:'time',stream_status:'invalid'}),new Response('PRIVATE',{headers:{'Content-Type':'application/json'}})]){jest.spyOn(global,'fetch').mockResolvedValue(value);await expect(evidenceJSON('/api/detection/results/A')).rejects.toMatchObject({name:'ContractValidationError',mayHaveApplied:false});}
});
