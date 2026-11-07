/** Owned media and durable database effects; expected authority is independent of discovery. */
import {test,expect,admin,anonymous,viewer,wire,ok,valid,token,createUser,policy,cameraScope,operationIds,database,type Identity} from './permissionCases';
import {createServer} from 'node:http';
import {execFileSync} from 'node:child_process';
import {readFileSync} from 'node:fs';
import path from 'node:path';
const ids=['createStream','probeStream','getStream','deleteStream','refreshStream','getStreamRecording','controlStreamRecording','getStreamZones','saveStreamZones','deleteStreamZones','getStreamRetention','setStreamRetention','getStreamDetectionEngines','setStreamDetectionEngines','getIceServers'];
const readIds=['getStream','getStreamRecording','getStreamZones','getIceServers'];
const writeIds=['deleteStream','refreshStream','controlStreamRecording','saveStreamZones','deleteStreamZones','getStreamRetention','setStreamRetention','getStreamDetectionEngines','setStreamDetectionEngines'];
const zone={id:'entrance',name:'Entrance',enabled:true,color:'#224466',polygon:[{x:0,y:0},{x:1,y:0},{x:1,y:1}],filter_classes:'person',min_confidence:0.5};
const engine={key:'owned-motion',type:'motion',model_path:'motion',enabled:true,threshold:0.5,interval_seconds:2,sort_order:1,config:{}};
async function assertError(result:Awaited<ReturnType<typeof wire>>,status:number){expect(result.response.status,JSON.stringify(result.body)).toBe(status);valid('ApiError',result.body);}

test('stream configuration rejects malformed values before durable effects @api',async({server})=>{
 const retained=await ok(server,'/api/streams/A/retention');valid('StreamRetention',retained);
 for(const body of [{max_storage_mb:-1},{max_storage_mb:1e30},{max_storage_mb:1.5},{max_storage_mb:'5'},{retention_days:1.5},{detection_retention_days:'3'},[]]){
  await assertError(await wire(server,'/api/streams/A/retention',admin,'PUT',body),400);
  expect(await ok(server,'/api/streams/A/retention')).toEqual(retained);
 }
 const saved=await ok(server,'/api/streams/A/retention',admin,'PUT',{retention_days:0,detection_retention_days:-1,max_storage_mb:0});valid('StreamRetention',saved);expect(saved).toMatchObject({retention_days:0,detection_retention_days:-1,max_storage_mb:0});
});

test('zone replacement is atomic for invalid shapes and typed options @api',async({server})=>{
 valid('StreamZonesSaved',await ok(server,'/api/streams/A/zones',admin,'POST',{zones:[zone]}));
 const before=await ok(server,'/api/streams/A/zones');valid('StreamZones',before);
 for(const bad of [{...zone,id:''},{...zone,id:'a'.repeat(64)},{...zone,enabled:'yes'},{...zone,color:'invalid'},{...zone,min_confidence:1.5},{...zone,min_confidence:'0.5'},{...zone,polygon:[{x:0,y:0}]},{...zone,id:'staged'}]){
  await assertError(await wire(server,'/api/streams/A/zones',admin,'POST',{zones:[{...zone,id:'staged'},bad]}),400);
  expect(await ok(server,'/api/streams/A/zones')).toEqual(before);
 }
 valid('StreamZonesDeleted',await ok(server,'/api/streams/A/zones',admin,'DELETE'));
 expect((await ok(server,'/api/streams/A/zones')).zones).toEqual([]);
});

test('engine replacement rejects lossy keys, oversized integers and invalid ranges @api',async({server})=>{
 valid('StreamDetectionEngines',await ok(server,'/api/streams/A/detection-engines',admin,'PUT',{engines:[engine]}));
 const before=await ok(server,'/api/streams/A/detection-engines');valid('StreamDetectionEngines',before);expect(before.engines.find((e:any)=>e.key===engine.key)).toMatchObject(engine);
 for(const bad of [{...engine,key:'a'.repeat(64)},{...engine,sort_order:1e30},{...engine,threshold:1.1},{...engine,interval_seconds:0},{...engine,key:'legacy-primary'},{...engine,model_path:'a'.repeat(1024)}]){
  await assertError(await wire(server,'/api/streams/A/detection-engines',admin,'PUT',{engines:[bad]}),400);
  expect(await ok(server,'/api/streams/A/detection-engines')).toEqual(before);
 }
});

test('principal matrix preserves Basic, session, legacy key and scoped camera intersections @api',async({server})=>{
 const configure=await token(server,admin,cameraScope(server.cameras.A.camera_uuid),['camera.configure']);
 const live=await token(server);
 const empty=await createUser(server,'no_media');await policy(server,empty,[]);
 const legacy=await createUser(server,'legacy_media');database(server,'UPDATE users SET api_key=? WHERE id=?',['owned-legacy-media-key',legacy.id!]);
 const key:Identity={id:legacy.id,headers:{'X-API-Key':'owned-legacy-media-key'}};
 const login=await server.request('/api/auth/login',{method:'POST',principal:'anonymous',body:server.credentials.admin});expect(login.status).toBe(200);
 const session:Identity={id:1,headers:{Cookie:login.headers.getSetCookie().map(v=>v.split(';')[0]).join('; ')}};
 for(const [identity,expected] of [[admin,ids],[session,ids],[viewer,readIds],[live,readIds],[configure,writeIds],[empty,[]],[key,readIds]] as [Identity,string[]][]){
  const doc=await ok(server,'/api/openapi.json',identity);expect(operationIds(doc).filter(id=>ids.includes(id))).toEqual([...expected].sort());
 }
 for(const identity of [viewer,live,empty,anonymous]){
  const status=identity===anonymous?401:403;
  for(const [route,method,body] of [['/api/streams','POST',{name:'must-not-create',url:'rtsp://127.0.0.1:9/x'}],['/api/streams/test','POST',{url:'http://127.0.0.1:9/x',protocol:0}],['/api/streams/A','DELETE',undefined],['/api/streams/A/refresh','POST',undefined],['/api/streams/A/zones','POST',{zones:[zone]}],['/api/streams/A/zones','DELETE',undefined],['/api/streams/A/retention','PUT',{retention_days:8}],['/api/streams/A/detection-engines','PUT',{engines:[engine]}],['/api/streams/A/recording','POST',{action:'start'}]] as [string,string,unknown][]){await assertError(await wire(server,route,identity,method,body),status);}
 }
 for(const [route,method,body] of [['/api/streams/B/retention','PUT',{retention_days:8}],['/api/streams/B/zones','POST',{zones:[zone]}],['/api/streams/B/detection-engines','PUT',{engines:[engine]}],['/api/streams','POST',{name:'scope-escape',url:'rtsp://127.0.0.1:9/x'}]] as [string,string,unknown][]){await assertError(await wire(server,route,configure,method,body),403);}
 for(const identity of [viewer,live,key]){
  valid('StreamSingleResponse',await ok(server,'/api/streams/A',identity));
  expect(await ok(server,'/api/streams/A',identity)).toMatchObject({onvif_password:'',onvif_username:'',can_control_privacy:false});
  valid('StreamRecordingState',await ok(server,'/api/streams/A/recording',identity));
  valid('IceServersResponse',await ok(server,'/api/ice-servers',identity));
 }
 await assertError(await wire(server,'/api/ice-servers',empty),403);
 expect((await ok(server,'/api/streams')).map((s:any)=>s.name).sort()).toEqual(['A','B']);
 expect((await ok(server,'/api/streams/A/zones')).zones).toEqual([]);
 expect((await ok(server,'/api/streams/A/recording')).recording_active).toBe(false);
 expect((await ok(server,'/api/streams/A/retention')).retention_days).toBe(-1);
});

test('create, probe, refresh and deletion expose actual lifecycle outcomes @api',async({server})=>{
 const media=path.join(server.directory,'owned.ts');
 execFileSync('ffmpeg',['-hide_banner','-loglevel','error','-f','lavfi','-i','testsrc=size=160x120:rate=10','-t','1','-c:v','libx264','-preset','ultrafast','-pix_fmt','yuv420p','-f','mpegts',media]);
 const bytes=readFileSync(media);let reads=0;
 const source=createServer((req,res)=>{reads++;res.writeHead(200,{'Content-Type':'video/mp2t','Content-Length':bytes.length});res.end(bytes);});
 await new Promise<void>(resolve=>source.listen(0,'127.0.0.1',resolve));
 try{
  const address=source.address();if(!address||typeof address==='string')throw new Error('Owned fixture did not bind');
  const url=`http://127.0.0.1:${address.port}/owned.ts`;
  await assertError(await wire(server,'/api/streams/test',viewer,'POST',{url,protocol:0}),403);expect(reads).toBe(0);
  const probe=await ok(server,'/api/streams/test',admin,'POST',{url,protocol:0});valid('StreamProbeResponse',probe);expect(probe).toEqual({success:true,info:{width:160,height:120,fps:10,codec:'h264'}});expect(reads).toBeGreaterThan(0);
  const fail=await ok(server,'/api/streams/test',admin,'POST',{url:`${server.directory}/absent`,protocol:0});valid('StreamProbeResponse',fail);expect(fail.success).toBe(false);
  const created=await wire(server,'/api/streams',admin,'POST',{name:'Owned',url,enabled:false,streaming_enabled:false,record:false});expect(created.response.status).toBe(201);valid('StreamCreated',created.body);
  valid('StreamSingleResponse',await ok(server,'/api/streams/Owned'));
  const refresh=await wire(server,'/api/streams/Owned/refresh',admin,'POST');expect(refresh.response.status).toBe(202);valid('StreamRefreshAccepted',refresh.body);
  await expect.poll(async()=> (await ok(server,'/api/streams/Owned')).camera_uuid).toBe(created.body.camera_uuid);
  const gone=await wire(server,'/api/streams/Owned?permanent=true',admin,'DELETE');expect(gone.response.status).toBe(202);valid('StreamDeleteAccepted',gone.body);expect(gone.body.permanent).toBe(true);
  await expect.poll(async()=> (await server.request('/api/streams/Owned')).status).toBe(404);
  const second=await ok(server,'/api/streams',admin,'POST',{name:'Owned',url,enabled:false,streaming_enabled:false,record:false});expect(second.camera_uuid).not.toBe(created.body.camera_uuid);
  await assertError(await wire(server,'/api/streams/Owned/recording',admin,'POST',{action:'bad'}),400);
  await assertError(await wire(server,'/api/streams/Owned/recording',admin,'POST',{action:'stop'}),409);
 }finally{source.closeAllConnections();await new Promise<void>(resolve=>source.close(()=>resolve()));}
});

test('manual recording accepts, records owned media, stops, and exposes finalized bytes @api',async({server})=>{
 const media=path.join(server.directory,'recording-source.ts');
 execFileSync('ffmpeg',['-hide_banner','-loglevel','error','-f','lavfi','-i','testsrc=size=160x120:rate=10','-t','60','-c:v','libx264','-preset','ultrafast','-pix_fmt','yuv420p','-g','10','-f','mpegts',media]);
 const bytes=readFileSync(media);let reads=0;
 const source=createServer((req,res)=>{reads++;res.writeHead(200,{'Content-Type':'video/mp2t'});let offset=0;const chunk=188*20;const timer=setInterval(()=>{res.write(bytes.subarray(offset,offset+chunk));offset+=chunk;if(offset>=bytes.length){clearInterval(timer);res.end();}},20);res.on('close',()=>clearInterval(timer));});
 await new Promise<void>(resolve=>source.listen(0,'127.0.0.1',resolve));
 try{
  const address=source.address();if(!address||typeof address==='string')throw new Error('Owned fixture did not bind');
  await ok(server,'/api/streams',admin,'POST',{name:'Recorder',url:`http://127.0.0.1:${address.port}/owned.ts`,enabled:false,streaming_enabled:false,record:false,segment_duration:60});
  await assertError(await wire(server,'/api/streams/Recorder/recording',viewer,'POST',{action:'start'}),403);expect(reads).toBe(0);
  const accepted=await wire(server,'/api/streams/Recorder/recording',admin,'POST',{action:'start'});expect(accepted.response.status).toBe(202);valid('StreamRecordingChanged',accepted.body);expect(accepted.body.state).toBe('starting');
  await expect.poll(async()=>{const state=await ok(server,'/api/streams/Recorder/recording');valid('StreamRecordingState',state);return state.recording_id;},{timeout:15000}).toBeGreaterThan(0);
  const state=await ok(server,'/api/streams/Recorder/recording');expect(state).toMatchObject({recording_active:true,capture_method:'manual'});
  await assertError(await wire(server,'/api/streams/Recorder/recording',admin,'POST',{action:'start'}),409);
  // Let the owned input advance beyond initialization; inspect the finalized MP4 below.
  await new Promise(resolve=>setTimeout(resolve,600));
  const stopped=await wire(server,'/api/streams/Recorder/recording',admin,'POST',{action:'stop'});expect(stopped.response.status).toBe(200);valid('StreamRecordingChanged',stopped.body);
  expect((await ok(server,'/api/streams/Recorder/recording')).recording_active).toBe(false);
  const recordings=await ok(server,'/api/recordings');const rows=recordings.recordings;expect(rows.length).toBeGreaterThan(0);
  const row=rows.find((r:any)=>r.stream==='Recorder'||r.stream_name==='Recorder');expect(row).toBeDefined();
  const detail=await ok(server,`/api/recordings/${row.id}`);const filename=detail.file_path||detail.recording?.file_path;
  expect(typeof filename).toBe('string');expect(filename.startsWith(server.directory+'/')).toBe(true);
  const probe=JSON.parse(execFileSync('ffprobe',['-v','error','-show_streams','-count_packets','-of','json',filename],{encoding:'utf8'}));
  expect(probe.streams.find((s:any)=>s.codec_type==='video')).toMatchObject({codec_name:'h264',width:160,height:120});
  expect(Number(probe.streams.find((s:any)=>s.codec_type==='video').nb_read_packets)).toBeGreaterThan(0);
 }finally{source.closeAllConnections();await new Promise<void>(resolve=>source.close(()=>resolve()));}
});
