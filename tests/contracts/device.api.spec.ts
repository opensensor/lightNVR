import {test,expect,admin,wire,ok,database, type Server} from './permissionCases';
import {startDevice} from './device-fixture';
async function bind(server:Server,port:number){database(server,'UPDATE streams SET url=?, onvif_port=?, ptz_enabled=1, is_onvif=1, onvif_profile=? WHERE name=?',[`rtsp://127.0.0.1:${port}/owned`,port,'Profile_1','A']);}
test('invalid device payloads produce no SOAP or persisted effects',async({server})=>{
 const device=await startDevice();try{
  await bind(server,device.port);
  for(const [route,body] of [['ptz/move',{pan:'fast'}],['ptz/move',[]],['ptz/absolute',{pan:0.1,zoom:'bad'}],['imaging/settings',{brightness:42,contrast:'bad'}],['daynight',{mode:'day',force_persistence:'false'}]] as const){
    const before=device.requests.length;const result=await wire(server,`/api/streams/A/${route}`,admin,route.startsWith('ptz')?'POST':'PUT',body);
    expect(result.response.status,JSON.stringify({route,body,actual:result.body})).toBe(400);
    expect(device.requests.slice(before)).toEqual([]);
  }
 }finally{await device.close();}
});
test('owned SOAP confirms PTZ and imaging request wiring',async({server})=>{
 const device=await startDevice();try{
  await bind(server,device.port);
  expect((await ok(server,'/api/streams/A/ptz/move',admin,'POST',{pan:0.25,tilt:0,zoom:0})).success).toBe(true);
  expect(device.requests.at(-1)?.action).toBe('ContinuousMove');expect(device.requests.at(-1)?.body).toContain('x="0.25"');
  const imaging=await ok(server,'/api/streams/A/imaging/settings');expect(imaging.settings.brightness).toBe(43);
  expect(device.requests.find(r=>r.action==='GetProfiles')?.path).toBe('/onvif/media_service');
  expect((await ok(server,'/api/streams/A/daynight')).mode).toBe('auto');
 }finally{await device.close();}
});

import {viewer,anonymous,createUser,role,policy,token,cameraScope,operationIds,valid,projectedResponse,type Identity} from './permissionCases';
const cameraReads=['getPtzCapabilities','getPtzPresets','getImagingSettings','getImagingOptions','getDaynight'];
const ptzWrites=['movePtz','stopPtz','absolutePtz','relativePtz','homePtz','setPtzHome','gotoPtzPreset','setPtzPreset'];
const imagingWrites=['setImagingSettings','setDaynight'];
const onvifOps=['getOnvifDiscoveryStatus','getOnvifDevices','discoverOnvifDevices','getOnvifProfiles','addOnvifDevice','claimOnvifDevice','testOnvifDevice'];
const allDeviceOps=[...cameraReads,...ptzWrites,...imagingWrites,...onvifOps];
test('device discovery and real denial agree for Basic, session, legacy key and scoped token principals',async({server})=>{
 const device=await startDevice();try{
  await bind(server,device.port);
  const operator=await createUser(server,'device_operator');const roleId=await role(server,'device_role',['live.view','ptz.control','camera.configure']);
  await policy(server,operator,[{role_uuid:roleId,scope:cameraScope(server.cameras.A.camera_uuid)}]);
  const empty=await createUser(server,'device_empty');await policy(server,empty,[]);
  const cameraToken=await token(server,admin,cameraScope(server.cameras.A.camera_uuid),['live.view','ptz.control','camera.configure']);
  const globalToken=await token(server,admin,{type:'all'},['live.view','ptz.control','camera.configure']);
  const login=await wire(server,'/api/auth/login',anonymous,'POST',server.credentials.admin);const session={headers:{Cookie:login.response.headers.getSetCookie().find(c=>c.startsWith('session='))!.split(';')[0]!}};
  // Independent legacy-key seed; issuance behavior is covered by identity contracts.
  database(server,'UPDATE users SET api_key=? WHERE id=1',['owned-device-legacy-key']);const legacy={headers:{'X-API-Key':'owned-device-legacy-key'}};
  for(const [identity,expected] of [[admin,allDeviceOps],[session,allDeviceOps],[legacy,allDeviceOps],[viewer,cameraReads],[operator,[...cameraReads,...ptzWrites,...imagingWrites]],[cameraToken,[...cameraReads,...ptzWrites,...imagingWrites]],[globalToken,allDeviceOps],[empty,[]]] as [Identity,string[]][]){
   const doc=await ok(server,'/api/openapi.json',identity);expect(operationIds(doc).filter(id=>allDeviceOps.includes(id)).sort()).toEqual([...expected].sort());
  }
  for(const identity of [anonymous,viewer,empty]){
   for(const [suffix,method,body] of [['ptz/move','POST',{pan:0.1}],['ptz/stop','POST',undefined],['ptz/absolute','POST',{zoom:0.2}],['ptz/relative','POST',{pan:0.1}],['ptz/home','POST',undefined],['ptz/set-home','POST',undefined],['ptz/goto-preset','POST',{token:'preset-one'}],['ptz/preset','PUT',{name:'new'}],['imaging/settings','PUT',{brightness:40}],['daynight','PUT',{mode:'day'}]] as const){
    const before=device.requests.length;const result=await wire(server,`/api/streams/A/${suffix}`,identity,method,body);expect(result.response.status).toBe(identity===anonymous?401:403);expect(device.requests.slice(before)).toEqual([]);
   }
  }
  for(const identity of [operator,cameraToken,empty,anonymous]){
   for(const [route,method,body] of [['/api/onvif/discovery/status','GET',undefined],['/api/onvif/devices','GET',undefined],['/api/onvif/discovery/discover','POST',{network:'127.0.0.1/32'}],['/api/onvif/device/test','POST',{url:device.base+'/onvif/device_service'}],['/api/onvif/device/add','POST',{device_url:device.base+'/onvif/device_service',profile_token:'Profile_1',stream_name:'Owned add'}],['/api/onvif/device/claim','POST',{inventory_uuid:'00000000-0000-4000-8000-000000000001',stream_name:'A'}],['/api/onvif/device/profiles','GET',undefined]] as const){
    const before=device.requests.length;const result=await wire(server,route,identity,method,body);expect(result.response.status).toBe(identity===anonymous?401:403);expect(device.requests.slice(before)).toEqual([]);
   }
  }
  for(const identity of [operator,cameraToken]){const before=device.requests.length;expect((await wire(server,'/api/streams/B/ptz/move',identity,'POST',{pan:0.2})).response.status).toBe(403);expect(device.requests.slice(before)).toEqual([]);}
 }finally{await device.close();}
});

test('all owned PTZ/imaging responses match canonical and projected contracts including faults',async({server})=>{
 const device=await startDevice();try{
  await bind(server,device.port);const doc=await ok(server,'/api/openapi.json');
  for(const [suffix,method,body,schema,soap] of [
   ['ptz/capabilities','GET',undefined,'PtzCapabilities',null],['ptz/presets','GET',undefined,'PtzPresets','GetPresets'],
   ['ptz/move','POST',{pan:0.25,tilt:0,zoom:0},'DeviceCommandResult','ContinuousMove'],['ptz/stop','POST',undefined,'DeviceCommandResult','Stop'],
   ['ptz/absolute','POST',{zoom:0.2},'DeviceCommandResult','AbsoluteMove'],['ptz/relative','POST',{pan:-0.15},'DeviceCommandResult','RelativeMove'],['ptz/home','POST',undefined,'DeviceCommandResult','GotoHomePosition'],['ptz/set-home','POST',undefined,'DeviceCommandResult','SetHomePosition'],['ptz/goto-preset','POST',{token:'preset-one'},'DeviceCommandResult','GotoPreset'],['ptz/preset','PUT',{name:'Owned & safe'},'PtzPresetResult','SetPreset'],
   ['imaging/settings','GET',undefined,'ImagingSettingsResponse','GetImagingSettings'],['imaging/options','GET',undefined,'ImagingOptionsResponse','GetOptions'],['imaging/settings','PUT',{brightness:0,wdr:{mode:'ON',level:12},force_persistence:false},'DeviceCommandResult','SetImagingSettings'],['daynight','GET',undefined,'DaynightResponse','GetOptions'],['daynight','PUT',{mode:'night'},'DaynightUpdated','SetImagingSettings']
  ] as const){const result=await wire(server,`/api/streams/A/${suffix}`,admin,method,body);expect(result.response.status,JSON.stringify(result.body)).toBe(200);valid(schema,result.body);projectedResponse(doc,`/api/streams/{name}/${suffix}`,method.toLowerCase(),200,result.body);if(soap)expect(device.requests.at(-1)?.action).toBe(soap);}
  const caps=await ok(server,'/api/streams/A/ptz/capabilities');expect(caps.onvif_queried).toBe(true);expect(caps.pan_range).toEqual({min:-2,max:2});expect(caps.has_continuous_move).toBe(true);
  expect(device.requests.find(r=>r.action==='SetPreset')?.body).toContain('Owned &amp; safe');
  expect(device.requests.find(r=>r.action==='SetImagingSettings'&&r.body.includes('<tt:Brightness>0'))?.body).toContain('false');
  device.setFault(true);
  for(const suffix of ['imaging/settings','imaging/options','daynight','ptz/presets']){const result=await wire(server,`/api/streams/A/${suffix}`);expect(result.response.status).toBe(502);valid('ApiError',result.body);}
  const fallback=await ok(server,'/api/streams/A/ptz/capabilities');expect(fallback.onvif_queried).toBe(false);valid('PtzCapabilities',fallback);
  const fault=await wire(server,'/api/streams/A/ptz/move',admin,'POST',{pan:0.1});expect(fault.response.status).toBe(500);valid('ApiError',fault.body);
 }finally{await device.close();}
});
test('owned ONVIF headers, empty inventory, profiles, connection test, and claims preserve wire semantics',async({server})=>{
 const device=await startDevice();try{
  const doc=await ok(server,'/api/openapi.json');
  for(const [route,schema] of [['/api/onvif/discovery/status','OnvifDiscoveryStatus'],['/api/onvif/devices','OnvifDevicesResponse']] as const){const result=await wire(server,route);expect(result.response.status).toBe(200);valid(schema,result.body);projectedResponse(doc,route,'get',200,result.body);}
  const identity={headers:{...admin.headers,'X-Device-URL':device.base+'/onvif/device_service','X-Username':'owned-user','X-Password':'owned-secret'}};
  const profiles=await wire(server,'/api/onvif/device/profiles',identity);expect(profiles.response.status).toBe(200);valid('OnvifProfilesResponse',profiles.body);expect(profiles.body.profiles[0].video_source_token).toBe('VideoSource_1');expect(profiles.body.profiles[0].stream_uri).toBe('');projectedResponse(doc,'/api/onvif/device/profiles','get',200,profiles.body);
  const tested=await wire(server,'/api/onvif/device/test',admin,'POST',{url:device.base+'/onvif/device_service'});expect(tested.response.status).toBe(200);valid('DeviceCommandResult',tested.body);
  device.setEmpty(true);const empty=await wire(server,'/api/onvif/device/profiles',identity);expect(empty.response.status).toBe(200);expect(empty.body.profiles).toEqual([]);valid('OnvifProfilesResponse',empty.body);
  expect((await wire(server,'/api/onvif/device/test',admin,'POST',{url:device.base+'/onvif/device_service'})).response.status).toBe(500);
  for(const [route,body] of [['/api/onvif/discovery/discover',{network:4}],['/api/onvif/device/test',{url:device.base,username:false}],['/api/onvif/device/add',{device_url:device.base,profile_token:'x',stream_name:'A',password:4}],['/api/onvif/device/claim',{inventory_uuid:'bad',stream_name:'A'}]] as const){const before=device.requests.length;expect((await wire(server,route,admin,'POST',body)).response.status).toBe(400);expect(device.requests.slice(before)).toEqual([]);}
 }finally{await device.close();}
});

test('owned add and inventory claim bind returned camera identity without denied changes',async({server})=>{
 const device=await startDevice();try{
  device.setStreamUri('rtsp://127.0.0.1:9/owned-added');
  const body={device_url:device.base+'/onvif/device_service',profile_token:'Profile_1',stream_name:'Owned added',username:'owned',password:'owned-secret'};
  const before=await ok(server,'/api/streams');
  expect((await wire(server,'/api/onvif/device/add',viewer,'POST',body)).response.status).toBe(403);expect(await ok(server,'/api/streams')).toEqual(before);expect(device.requests).toEqual([]);
  const added=await wire(server,'/api/onvif/device/add',admin,'POST',body);expect(added.response.status,JSON.stringify(added.body)).toBe(200);valid('OnvifAddResult',added.body);expect(added.body.stream_name).toBe('Owned added');
  const uuid=(await ok(server,'/api/streams/Owned%20added/full')).stream.camera_uuid;
  const inventory='ab000000-0000-4000-8000-000000000001';
  database(server,'INSERT INTO onvif_discovery_inventory(uuid,endpoint,device_service,first_seen_at,last_seen_at) VALUES(?,?,?,?,?)',[inventory,'urn:uuid:owned',device.base+'/onvif/device_service',1,2]);
  const prior=await ok(server,'/api/onvif/devices');valid('OnvifDevicesResponse',prior);
  expect((await wire(server,'/api/onvif/device/claim',viewer,'POST',{inventory_uuid:inventory,stream_name:'Owned added'})).response.status).toBe(403);expect(await ok(server,'/api/onvif/devices')).toEqual(prior);
  const claim=await wire(server,'/api/onvif/device/claim',admin,'POST',{inventory_uuid:inventory,stream_name:'Owned added'});expect(claim.response.status).toBe(200);valid('DeviceSuccess',claim.body);
  const row=(await ok(server,'/api/onvif/devices')).devices.find((row:{inventory_uuid:string})=>row.inventory_uuid===inventory);expect(row.claim_state).toBe('claimed');expect(row.claimed_camera_uuid).toBe(uuid);
  expect((await wire(server,'/api/onvif/device/claim',admin,'POST',{inventory_uuid:inventory,stream_name:'Missing'})).response.status).toBe(404);expect((await ok(server,'/api/onvif/devices')).devices[0].claimed_camera_uuid).toBe(uuid);
 }finally{await device.close();}
});
