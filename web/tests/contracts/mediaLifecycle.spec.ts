import {createStream,probeStream,deleteStream,refreshStream,controlStreamRecording,getStreamRecording,getStreamZones,saveStreamZones,setStreamRetention,setStreamDetectionEngines,getIceServers,MediaContractError} from '../../js/api/mediaLifecycle.ts';
function respond(body:unknown,status=200){return jest.spyOn(global,'fetch').mockResolvedValue(new Response(JSON.stringify(body),{status,headers:{'Content-Type':'application/json'}}));}
beforeEach(()=>{for(const key of ['log','debug','warn','error'] as const)jest.spyOn(console,key).mockImplementation(()=>{});});
afterEach(()=>jest.restoreAllMocks());
test('creation remains distinct from editing and never retries a mutation',async()=>{
 const spy=respond({success:true,camera_uuid:'fixture',location_uuid:''},201);
 await createStream({name:'New',url:'rtsp://secret:credential@camera.invalid/live',enabled:false},{retries:5,redirect:'follow',credentials:'include'});
 expect(spy).toHaveBeenCalledTimes(1);expect(spy.mock.calls[0]?.[0]).toBe('/api/streams');expect(spy.mock.calls[0]?.[1]).toMatchObject({method:'POST',cache:'no-store',redirect:'error',credentials:'same-origin'});
 for(const key of ['log','debug','warn','error'] as const)expect(console[key]).not.toHaveBeenCalled();
});
test('invalid outbound values never reach transport',async()=>{
 const spy=respond({});
 for(const call of [()=>createStream({name:'No URL'}),()=>saveStreamZones('A',{zones:[{id:'x',name:'x',polygon:[{x:3,y:0}]}]}),()=>setStreamRetention('A',{max_storage_mb:-1}),()=>setStreamDetectionEngines('A',{engines:[{key:'x',type:'motion',threshold:2,interval_seconds:1}]})])await expect(call()).rejects.toBeInstanceOf(MediaContractError);
 expect(spy).not.toHaveBeenCalled();
});
test('acknowledgment status and shape matter; malformed accepted writes may have applied',async()=>{
 respond({success:true,status:'accepted',permanent:true},200);
 await expect(deleteStream('A',true)).rejects.toMatchObject({phase:'response',mayHaveApplied:true,status:200});
 respond({secret:'private-source'},202);
 try{await refreshStream('A');throw new Error('expected rejection');}catch(error){expect(error).toBeInstanceOf(MediaContractError);expect(String(error)).not.toContain('private-source');expect(error).toMatchObject({mayHaveApplied:true});}
});
test('manual start and stop preserve their distinct status contracts',async()=>{
 respond({success:true,stream:'A',action:'start',state:'starting'},202);await expect(controlStreamRecording('A','start')).resolves.toMatchObject({state:'starting'});
 respond({success:true,stream:'A',action:'stop',state:'idle'},200);await expect(controlStreamRecording('A','stop')).resolves.toMatchObject({state:'idle'});
 respond({stream:'A',state:'idle',recording_active:false,capture_method:null,recording_id:null,manual_control_allowed:true,manual_control_reason:null});await expect(getStreamRecording('A')).resolves.toMatchObject({recording_active:false});
});
test('empty zones and disabled ICE are valid; malformed response fields fail closed',async()=>{
 respond({zones:[]});await expect(getStreamZones('A')).resolves.toEqual({zones:[]});
 respond({ice_servers:[]});await expect(getIceServers()).resolves.toEqual({ice_servers:[]});
 respond({ice_servers:[{urls:123}]});await expect(getIceServers()).rejects.toMatchObject({phase:'response',mayHaveApplied:false});
});
test('failed media probing is a typed outcome, not a successful connection',async()=>{
 respond({success:false,message:'Owned source unavailable'});await expect(probeStream({url:'http://127.0.0.1:9/owned',protocol:0})).resolves.toMatchObject({success:false});
 respond({success:true,info:{width:'160',height:120,fps:10,codec:'h264'}});await expect(probeStream({url:'http://127.0.0.1:9/owned',protocol:0})).rejects.toMatchObject({phase:'response'});
});

test.each([307,308] as const)('credential-bearing creation refuses HTTP %s before forwarding its body',async status=>{
 const {withRedirectFixture}=await import('./redirectHarness');
 await withRedirectFixture(status,async fixture=>{
  const spy=jest.spyOn(global,'fetch').mockImplementation(fixture.fetch);
  await expect(createStream({name:'OwnedRedirect',url:'rtsp://owned-user:owned-password@camera.invalid/source'},{redirect:'follow',credentials:'include',retries:4})).rejects.toBeInstanceOf(MediaContractError);
  expect(fixture.sourceBodies).toHaveLength(1);expect(fixture.forwardedBodies).toEqual([]);
  expect(spy.mock.calls[0]?.[1]).toMatchObject({redirect:'error',credentials:'same-origin'});
 });
});

test.each(['text/html','application/jsonp'])('rejects a %s acknowledgment before parsing JSON and closes the body',async contentType=>{
 const response=new Response(JSON.stringify({success:true,camera_uuid:'fixture',location_uuid:''}),{status:201,headers:{'Content-Type':contentType}});
 const json=jest.spyOn(response,'json');
 const fetch=jest.spyOn(global,'fetch').mockResolvedValue(response);
 await expect(createStream({name:'Owned',url:'rtsp://127.0.0.1:9/source'})).rejects.toMatchObject({phase:'response',status:201,mayHaveApplied:true});
 expect(json).not.toHaveBeenCalled();expect(fetch).toHaveBeenCalledTimes(1);expect(response.bodyUsed).toBe(true);
});
