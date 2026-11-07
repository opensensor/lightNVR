import sharp from 'sharp';
import {readdir} from 'node:fs/promises';
import path from 'node:path';
/** Independent native mutation oracle; no expected permissions derived from OpenAPI. */
import {test,expect,admin,viewer,anonymous,wire,ok,token,operationIds,valid,createUser,policy,projectedResponse,type Server,type Identity} from './permissionCases';
const all={version:1,expression:{op:'all'}};
const operations=['createFleetLocation','updateFleetLocation','deleteFleetLocation','setCameraLocation','createFleetTag','updateFleetTag','deleteFleetTag','mergeFleetTag','setCameraTags','createCameraCollection','updateCameraCollection','deleteCameraCollection','setCameraCollectionMembers','createFleetView','updateFleetView','deleteFleetView','listLiveLayouts','createLiveLayout','updateLiveLayout','deleteLiveLayout','listCameraFavorites','addCameraFavorite','deleteCameraFavorite','listFloorPlans','createFloorPlan','getFloorPlan','updateFloorPlan','deleteFloorPlan','getFloorPlanBackground','setFloorPlanBackground','deleteFloorPlanBackground'];
async function accepted(server:Server,route:string,method:string,body:unknown,schema:string,identity:Identity=admin){const value=await ok(server,route,identity,method,body);valid(schema,value);const doc=await ok(server,'/api/openapi.json',identity);const normalized=route.replace(/[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}/gi,'{uuid}');projectedResponse(doc,normalized,method.toLowerCase(),method==='POST'&&!route.endsWith('/merge')?201:200,value);return value;}
test('fleet write discovery has the complete reviewed operation inventory',async({server})=>{
  const doc=await ok(server,'/api/openapi.json');
  expect(operationIds(doc).filter(id=>operations.includes(id))).toEqual([...operations].sort());
});
test('organization and collection mutations validate native responses and denied writes have no effects',async({server})=>{
  const camera=server.cameras.A.camera_uuid;
  const location=await accepted(server,'/api/locations','POST',{name:'Contract wing'},'FleetLocation');
  await accepted(server,`/api/locations/${location.uuid}`,'PUT',{name:'Contract wing updated'},'FleetLocation');
  await accepted(server,`/api/cameras/${camera}/location`,'PUT',{location_uuid:location.uuid},'CameraLocationAssignment');
  expect((await wire(server,`/api/locations/${location.uuid}`,viewer,'PUT',{name:'Denied'})).response.status).toBe(403);
  expect((await ok(server,`/api/locations/${location.uuid}`)).name).toBe('Contract wing updated');
  expect((await wire(server,`/api/locations/${location.uuid}`,admin,'DELETE')).response.status).toBe(409);
  const tag=await accepted(server,'/api/camera-tags','POST',{label:'Contract source'},'FleetTag');
  const target=await accepted(server,'/api/camera-tags','POST',{label:'Contract target'},'FleetTag');
  await accepted(server,`/api/camera-tags/${tag.uuid}`,'PUT',{color:'#00aaFF'},'FleetTag');
  await accepted(server,`/api/cameras/${camera}/tags`,'PUT',{tag_uuids:[tag.uuid]},'CameraTagAssignmentsResponse');
  expect((await wire(server,`/api/cameras/${camera}/tags`,viewer,'PUT',{tag_uuids:[]})).response.status).toBe(403);
  expect((await ok(server,`/api/cameras/${camera}/tags`)).tags[0].uuid).toBe(tag.uuid);
  expect((await wire(server,`/api/cameras/${camera}/tags`,admin,'PUT',{tag_uuids:[target.uuid,'00000000-0000-4000-8000-000000000099']})).response.status).toBe(404);
  expect((await ok(server,`/api/cameras/${camera}/tags`)).tags.map((row:any)=>row.uuid)).toEqual([tag.uuid]);
  await accepted(server,`/api/camera-tags/${tag.uuid}/merge`,'POST',{target_uuid:target.uuid},'FleetTagMergeResult');
  expect((await ok(server,`/api/cameras/${camera}/tags`)).tags[0].uuid).toBe(target.uuid);
  await accepted(server,`/api/camera-tags/${target.uuid}`,'DELETE',undefined,'FleetMutationAck');
  expect((await ok(server,`/api/cameras/${camera}/tags`)).tags).toEqual([]);
  const collection=await accepted(server,'/api/camera-collections','POST',{name:'Contract set',type:'static'},'CameraCollection');
  await accepted(server,`/api/camera-collections/${collection.uuid}/members`,'PUT',{camera_uuids:[camera,server.cameras.B.camera_uuid]},'CameraCollectionMembersResponse');
  expect((await ok(server,`/api/camera-collections/${collection.uuid}/members`,viewer)).camera_uuids).toEqual([camera]);
  expect((await wire(server,`/api/camera-collections/${collection.uuid}/members`,viewer,'PUT',{camera_uuids:[]})).response.status).toBe(403);
  expect((await ok(server,`/api/camera-collections/${collection.uuid}/members`)).camera_uuids).toHaveLength(2);
  await accepted(server,`/api/camera-collections/${collection.uuid}`,'PUT',{type:'smart',selector:all},'CameraCollection');
  expect((await wire(server,`/api/camera-collections/${collection.uuid}/members`,admin,'PUT',{camera_uuids:[]})).response.status).toBe(409);
  await accepted(server,`/api/camera-collections/${collection.uuid}`,'DELETE',undefined,'FleetMutationAck');
  const spare=await accepted(server,'/api/locations','POST',{name:'Deletable'},'FleetLocation');
  await accepted(server,`/api/locations/${spare.uuid}`,'DELETE',undefined,'FleetMutationAck');
});
test('views and layouts preserve sharing, ownership, optimistic revisions and favorites isolation',async({server})=>{
  for(const [path,schema,payload] of [
    ['/api/fleet/views','FleetSavedView',{name:'Shared view',selector:all,is_shared:true}],
    ['/api/live/layouts','LiveLayout',{name:'Shared layout',availability:'all',columns:2,rows:1,camera_slots:[{camera_uuid:server.cameras.A.camera_uuid},{camera_uuid:server.cameras.B.camera_uuid}],is_shared:true}],
  ] as const){
    const created=await accepted(server,path,'POST',payload,schema);
    const route=`${path}/${created.uuid}`;
    expect((await wire(server,route,viewer,'PUT',{...payload,revision:created.revision})).response.status).toBe(403);
    const changed=await accepted(server,route,'PUT',{...payload,name:'Updated',revision:created.revision},schema);
    expect(changed.revision).toBe(created.revision+1);
    expect((await wire(server,route,admin,'DELETE',{revision:created.revision})).response.status).toBe(409);
    await accepted(server,route,'DELETE',{revision:changed.revision},'FleetMutationAck');
  }
  const scoped=await token(server);
  await accepted(server,'/api/live/layouts','GET',undefined,'LiveLayoutsResponse',viewer);
  expect((await wire(server,'/api/live/layouts',scoped,'POST',{name:'Denied'})).response.status).toBe(403);
  const route=`/api/camera-favorites/${server.cameras.A.camera_uuid}`;
  await accepted(server,route,'PUT',undefined,'CameraFavorite',viewer);
  await accepted(server,'/api/camera-favorites','GET',undefined,'CameraFavoritesResponse',viewer);
  expect((await ok(server,'/api/camera-favorites',admin)).favorites).toEqual([]);
  expect((await ok(server,'/api/camera-favorites',scoped)).favorites).toEqual([]);
  expect((await wire(server,route,scoped,'DELETE')).response.status).toBe(403);
  await accepted(server,route,'DELETE',undefined,'CameraFavoriteRemoval',viewer);
});
test('floor plans enforce revisions and scope and preserve actual background bytes',async({server})=>{
  const payload={name:'Contract plan',canvas_width:1200,canvas_height:800,cameras:[{camera_uuid:server.cameras.A.camera_uuid,x:.2,y:.3,rotation:0,fov:65},{camera_uuid:server.cameras.B.camera_uuid,x:.8,y:.7,rotation:0,fov:65}]};
  const plan=await accepted(server,'/api/live/plans','POST',payload,'FloorPlan');
  const route=`/api/live/plans/${plan.uuid}`;
  await accepted(server,'/api/live/plans','GET',undefined,'FloorPlansResponse',viewer);
  const visible=await accepted(server,route,'GET',undefined,'FloorPlan',viewer);
  expect(visible.cameras.map((row:any)=>row.camera_uuid)).toEqual([server.cameras.A.camera_uuid]);
  expect(visible.partially_authorized).toBe(true);
  expect((await wire(server,route,viewer,'PUT',{...payload,revision:plan.revision})).response.status).toBe(403);
  const changed=await accepted(server,route,'PUT',{...payload,name:'Revised',revision:plan.revision},'FloorPlan');
  expect((await wire(server,route,admin,'DELETE',{revision:plan.revision})).response.status).toBe(409);
  // A valid 1x1 PNG, checked as bytes rather than through a JSON acknowledgement.
  const png=Buffer.from('iVBORw0KGgoAAAANSUhEUgAAAAEAAAABCAQAAAC1HAwCAAAAC0lEQVR42mP8/x8AAwMCAO+jWZkAAAAASUVORK5CYII=','base64');
  const upload=await fetch(server.baseURL+route+'/background',{method:'PUT',headers:{...admin.headers,'Content-Type':'image/png'},body:png});
  expect(upload.status).toBe(200);const uploaded=await upload.json();valid('FloorPlan',uploaded);
  const download=await fetch(server.baseURL+route+'/background',{headers:viewer.headers});
  expect(download.headers.get('content-type')).toContain('image/png');expect(Buffer.from(await download.arrayBuffer())).toEqual(png);
  const denied=await fetch(server.baseURL+route+'/background',{method:'PUT',headers:{...viewer.headers,'Content-Type':'image/png'},body:png});expect(denied.status).toBe(403);
  const invalid=await fetch(server.baseURL+route+'/background',{method:'PUT',headers:{...admin.headers,'Content-Type':'image/png'},body:Buffer.from('invalid')});expect(invalid.status).toBe(400);
  expect(Buffer.from(await (await fetch(server.baseURL+route+'/background',{headers:admin.headers})).arrayBuffer())).toEqual(png);
  const jpeg=await sharp({create:{width:2,height:2,channels:3,background:'#204060'}}).jpeg().toBuffer();
  const replaced=await fetch(server.baseURL+route+'/background',{method:'PUT',headers:{...admin.headers,'Content-Type':'image/jpeg'},body:jpeg});expect(replaced.status).toBe(200);valid('FloorPlan',await replaced.json());
  const jpegDownload=await fetch(server.baseURL+route+'/background',{headers:viewer.headers});expect(jpegDownload.headers.get('content-type')).toContain('image/jpeg');expect(Buffer.from(await jpegDownload.arrayBuffer())).toEqual(jpeg);
  const imageDir=path.join(server.directory,'recordings','floor_plans');expect(await readdir(imageDir)).toEqual([plan.uuid+'.jpg']);
  const cleared=await accepted(server,route+'/background','DELETE',undefined,'FloorPlan');expect(cleared.background_mime).toBeNull();expect(await readdir(imageDir)).toEqual([]);
  expect((await fetch(server.baseURL+route+'/background',{headers:admin.headers})).status).toBe(404);
  await accepted(server,route,'DELETE',{revision:cleared.revision},'FloorPlanDeletion');
  expect((await wire(server,route)).response.status).toBe(404);
});

// Explicit permission expectations, independent of the canonical authorization metadata.
test('projection and writes agree for Basic, browser sessions, scoped tokens and empty policies',async({server})=>{
  const read=['listLiveLayouts','listCameraFavorites','listFloorPlans','getFloorPlan','getFloorPlanBackground'];
  const views=['createFleetView','updateFleetView','deleteFleetView'];
  const personal=[...views,'createLiveLayout','updateLiveLayout','deleteLiveLayout','deleteCameraFavorite'];
  const scoped=await token(server);
  const empty=await createUser(server,'empty_workspace');await policy(server,empty,[]);
  const login=await wire(server,'/api/auth/login',anonymous,'POST',{username:'contract_viewer',password:'ContractViewer123!'});
  expect(login.response.status).toBe(200);
  const cookie=login.response.headers.get('set-cookie')!.split(';')[0]!;
  const session:Identity={id:2,headers:{Cookie:cookie}};
  for(const [identity,expected] of [[admin,operations],[viewer,[...read,...personal,'addCameraFavorite']],[session,[...read,...personal,'addCameraFavorite']],[scoped,[...read,...views]],[empty,[...read,...personal]]] as const){
    const doc=await ok(server,'/api/openapi.json',identity);
    expect(operationIds(doc).filter(id=>operations.includes(id))).toEqual([...expected].sort());
  }
  const mine=await accepted(server,'/api/fleet/views','POST',{name:'Session owned',selector:all},'FleetSavedView',session);
  expect((await wire(server,`/api/fleet/views/${mine.uuid}`,admin,'DELETE',{revision:mine.revision})).response.status).toBe(404);
  expect((await ok(server,`/api/fleet/views/${mine.uuid}`,session)).name).toBe('Session owned');
  await accepted(server,`/api/fleet/views/${mine.uuid}`,'DELETE',{revision:mine.revision},'FleetMutationAck',session);
  const globalToken=await token(server,admin,{type:'all'},['live.view','camera.configure']);
  const plan=await accepted(server,'/api/live/plans','POST',{name:'Protected plan'},'FloorPlan');
  expect((await wire(server,`/api/live/plans/${plan.uuid}`,globalToken,'DELETE',{revision:plan.revision})).response.status).toBe(403);
  expect((await ok(server,`/api/live/plans/${plan.uuid}`)).revision).toBe(plan.revision);
  expect(operationIds(await ok(server,'/api/openapi.json',globalToken))).not.toContain('deleteFloorPlan');
});
