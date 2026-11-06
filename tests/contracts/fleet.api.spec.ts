/** Fleet disclosure oracle: independently authored scope/ownership expectations. */
import {test,expect,admin,viewer,anonymous,wire,ok,createUser,role,policy,token,cameraScope,discovery,operationIds,fleetOps,fleetCommonOps,projectedResponse,valid,lintProjections,type Server,type Identity} from './permissionCases';
const all={version:1,expression:{op:'all'}};
async function organization(server:Server) {
  const root=await ok(server,'/api/locations',admin,'POST',{name:'Fixture site'});
  const a=await ok(server,'/api/locations',admin,'POST',{name:'Visible wing',parent_uuid:root.uuid});
  const b=await ok(server,'/api/locations',admin,'POST',{name:'Hidden wing',parent_uuid:root.uuid});
  const shared=await ok(server,'/api/camera-tags',admin,'POST',{label:'Shared fixture tag'});
  const privateTag=await ok(server,'/api/camera-tags',admin,'POST',{label:'Hidden fixture tag'});
  for(const [name,location,tags] of [['A',a,[shared.uuid]],['B',b,[shared.uuid,privateTag.uuid]]] as const) {
    await ok(server,`/api/cameras/${server.cameras[name].camera_uuid}/location`,admin,'PUT',{location_uuid:location.uuid});
    await ok(server,`/api/cameras/${server.cameras[name].camera_uuid}/tags`,admin,'PUT',{tag_uuids:tags});
  }
  return {root,a,b,shared,privateTag};
}
async function projection(server:Server,identity:Identity,info:Parameters<typeof discovery>[2],label:string,hasCamera:boolean) {
  const doc=await discovery(server,identity,info,label);
  expect(operationIds(doc).filter(id=>fleetOps.includes(id))).toEqual([...fleetCommonOps,...(hasCamera?['getCameraTagAssignments']:[])].sort());
  return doc;
}

test('fleet rows, facets, counts and per-camera configuration remain scoped for principals and tokens',async({server},info)=>{
  const org=await organization(server),read=await role(server,'Fleet read',['live.view']),configure=await role(server,'Fleet configure',['camera.configure']);
  const split=await createUser(server,'fleet_split');
  await policy(server,split,[{role_uuid:read,scope:cameraScope(server.cameras.A.camera_uuid)},{role_uuid:configure,scope:cameraScope(server.cameras.B.camera_uuid)}]);
  const empty=await createUser(server,'fleet_empty');await policy(server,empty,[]);
  const scoped=await token(server);
  for(const [label,identity,names,canConfigure] of [['admin',admin,['A','B'],true],['viewer',viewer,['A'],false],['split',split,['A'],false],['token',scoped,['A'],false],['empty',empty,[],false]] as const) {
    const doc=await projection(server,identity,info,label,names.length>0);
    const query=await wire(server,'/api/fleet/cameras/query',identity,'POST',{selector:all,facets:true,explain:true,page_size:1});
    expect(query.response.status).toBe(200);projectedResponse(doc,'/api/fleet/cameras/query','post',200,query.body);
    expect(query.body.total).toBe(names.length);expect(query.body.total_pages).toBe(names.length);
    expect(query.body.cameras.map((row:any)=>row.name)).toEqual(names.slice(0,1));
    expect(query.body.cameras.every((row:any)=>row.can_configure===canConfigure)).toBe(true);
    expect(query.body.facets.tags.find((row:any)=>row.uuid===org.shared.uuid)?.count??0).toBe(names.length);
    expect(query.body.facets.enabled.reduce((sum:number,row:any)=>sum+row.count,0)).toBe(names.length);
    for(const row of query.body.cameras)expect(row).not.toHaveProperty('url');
    const second=await ok(server,'/api/fleet/cameras/query',identity,'POST',{page:2,page_size:1,facets:false});valid('FleetQueryResponse',second);
    expect(second.cameras.map((row:any)=>row.name)).toEqual(names.slice(1));expect(second).not.toHaveProperty('facets');
    const impossible=await ok(server,'/api/fleet/selectors/preview',identity,'POST',{selector:{version:1,expression:{op:'and',children:[{op:'tag_any',uuids:[org.privateTag.uuid]},{op:'location_subtree',uuid:org.a.uuid}]}}});
    projectedResponse(doc,'/api/fleet/selectors/preview','post',200,impossible);expect(impossible.total).toBe(0);expect(impossible.cameras).toEqual([]);
  }
  expect((await wire(server,'/api/fleet/cameras/query',anonymous,'POST',{})).response.status).toBe(401);
  for(const body of [{page:0},{page_size:201},{selector:{version:9,expression:{op:'all'}}}])expect((await wire(server,'/api/fleet/cameras/query',viewer,'POST',body)).response.status).toBe(400);
  expect((await wire(server,'/api/fleet/selectors/preview',viewer,'POST',{page_size:51})).response.status).toBe(400);
  lintProjections(info);
});

test('organization trees and camera tag assignments never expose hidden-camera counts',async({server},info)=>{
  const org=await organization(server),doc=await projection(server,viewer,info,'viewer',true);
  const locations=await ok(server,'/api/locations',viewer);projectedResponse(doc,'/api/locations','get',200,locations);
  expect(locations.locations.map((row:any)=>row.uuid).sort()).toEqual([org.root.uuid,org.a.uuid].sort());
  expect(locations.locations.find((row:any)=>row.uuid===org.root.uuid).child_count).toBe(1);
  const location=await ok(server,`/api/locations/${org.a.uuid}`,viewer);projectedResponse(doc,'/api/locations/{uuid}','get',200,location);expect(location.camera_count).toBe(1);
  expect((await wire(server,`/api/locations/${org.b.uuid}`,viewer)).response.status).toBe(403);
  const tags=await ok(server,'/api/camera-tags',viewer);projectedResponse(doc,'/api/camera-tags','get',200,tags);expect(tags.count).toBe(1);expect(tags.tags[0].camera_count).toBe(1);
  const tag=await ok(server,`/api/camera-tags/${org.shared.uuid}`,viewer);projectedResponse(doc,'/api/camera-tags/{uuid}','get',200,tag);expect(tag.camera_count).toBe(1);
  expect((await wire(server,`/api/camera-tags/${org.privateTag.uuid}`,viewer)).response.status).toBe(403);
  for(const identity of [viewer,await token(server)]) {
    const assigned=await ok(server,`/api/cameras/${server.cameras.A.camera_uuid}/tags`,identity);projectedResponse(doc,'/api/cameras/{uuid}/tags','get',200,assigned);
    expect(assigned.tags[0].camera_count).toBe(1);
    expect((await wire(server,`/api/cameras/${server.cameras.B.camera_uuid}/tags`,identity)).response.status).toBe(403);
  }
  expect((await ok(server,`/api/cameras/${server.cameras.A.camera_uuid}/tags`,admin)).tags[0].camera_count).toBe(2);
  const editor=await token(server,admin,cameraScope(server.cameras.A.camera_uuid),['live.view','camera.configure']);
  const edited=await ok(server,`/api/cameras/${server.cameras.A.camera_uuid}/tags`,editor,'PUT',{tag_uuids:[org.shared.uuid]});
  expect(edited.tags[0].camera_count).toBe(1);

  lintProjections(info);
});

test('collection reads scope members, redact shared selectors and hide private or empty foreign collections',async({server},info)=>{
  const doc=await projection(server,viewer,info,'viewer',true);
  const shared=await ok(server,'/api/camera-collections',admin,'POST',{name:'Shared',type:'static',shared:true});
  await ok(server,`/api/camera-collections/${shared.uuid}/members`,admin,'PUT',{camera_uuids:[server.cameras.A.camera_uuid,server.cameras.B.camera_uuid]});
  const hidden=await ok(server,'/api/camera-collections',admin,'POST',{name:'Private',type:'static',shared:false});
  const smart=await ok(server,'/api/camera-collections',admin,'POST',{name:'Smart',type:'smart',shared:true,selector:all});
  const list=await ok(server,'/api/camera-collections',viewer);projectedResponse(doc,'/api/camera-collections','get',200,list);
  expect(list.collections.map((row:any)=>row.uuid).sort()).toEqual([shared.uuid,smart.uuid].sort());
  for(const item of list.collections)expect(item.effective_count).toBe(1);
  const detail=await ok(server,`/api/camera-collections/${smart.uuid}`,viewer);projectedResponse(doc,'/api/camera-collections/{uuid}','get',200,detail);expect(detail.selector).toBeNull();expect(detail.selector_redacted).toBe(true);
  expect((await ok(server,`/api/camera-collections/${smart.uuid}`,admin)).selector).toEqual(all);
  const members=await ok(server,`/api/camera-collections/${shared.uuid}/members`,viewer);projectedResponse(doc,'/api/camera-collections/{uuid}/members','get',200,members);expect(members.camera_uuids).toEqual([server.cameras.A.camera_uuid]);
  const preview=await ok(server,`/api/camera-collections/${shared.uuid}/preview`,viewer,'POST');projectedResponse(doc,'/api/camera-collections/{uuid}/preview','post',200,preview);expect(preview.matched_count).toBe(1);expect(preview.sample.map((row:any)=>row.name)).toEqual(['A']);
  expect((await wire(server,`/api/camera-collections/${smart.uuid}/members`,viewer)).response.status).toBe(409);
  for(const suffix of ['', '/members','/preview'])expect((await wire(server,`/api/camera-collections/${hidden.uuid}${suffix}`,viewer,suffix==='/preview'?'POST':'GET')).response.status).toBe(404);
  await ok(server,`/api/camera-collections/${shared.uuid}/members`,admin,'PUT',{camera_uuids:[server.cameras.B.camera_uuid]});
  expect((await wire(server,`/api/camera-collections/${shared.uuid}`,viewer)).response.status).toBe(404);
  expect((await ok(server,'/api/fleet/cameras/query',viewer,'POST',{collection_uuid:shared.uuid})).cameras).toEqual([]);
  lintProjections(info);
});

test('saved views retain ownership/sharing and querying a stale saved selector never expands access',async({server},info)=>{
  const doc=await projection(server,viewer,info,'viewer',true);
  const mine=await ok(server,'/api/fleet/views',viewer,'POST',{name:'Mine',selector:all});
  const shared=await ok(server,'/api/fleet/views',admin,'POST',{name:'Shared view',is_shared:true,selector:{version:1,expression:{op:'camera_uuid',values:[server.cameras.B.camera_uuid]}}});
  const hidden=await ok(server,'/api/fleet/views',admin,'POST',{name:'Private view',selector:all});
  const list=await ok(server,'/api/fleet/views',viewer);projectedResponse(doc,'/api/fleet/views','get',200,list);expect(list.can_share).toBe(false);
  expect(list.views.map((row:any)=>row.uuid).sort()).toEqual([mine.uuid,shared.uuid].sort());
  const detail=await ok(server,`/api/fleet/views/${shared.uuid}`,viewer);projectedResponse(doc,'/api/fleet/views/{uuid}','get',200,detail);expect(detail.owned).toBe(false);
  expect((await wire(server,`/api/fleet/views/${hidden.uuid}`,viewer)).response.status).toBe(404);
  expect((await ok(server,'/api/fleet/cameras/query',viewer,'POST',{selector:detail.selector})).total).toBe(0);
  const stale=await ok(server,'/api/fleet/cameras/query',viewer,'POST',{selector:{version:1,expression:{op:'camera_uuid',values:['00000000-0000-4000-8000-000000000099']}}});valid('FleetQueryResponse',stale);expect(stale.total).toBe(0);
  lintProjections(info);
});
