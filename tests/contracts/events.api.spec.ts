/** Independent event authority and durable mutation checks against owned servers. */
import {test,expect,admin,viewer,anonymous,ok,wire,token,role,policy,createUser,valid,operationIds} from './permissionCases';
const type='io.lightnvr.detection.object.v1';
const ops=['getEventCatalog','listEventDestinations','createEventDestination','getEventDestination','updateEventDestination','deleteEventDestination','listEventRoutes','createEventRoute','getEventRoute','updateEventRoute','deleteEventRoute','previewEventRoute','triggerMotion'].sort();

test('event discovery follows global events.configure independently of camera configuration',async({server})=>{
 const observed=async(identity:typeof admin)=>operationIds(await ok(server,'/api/openapi.json',identity)).filter(id=>ops.includes(id));
 expect(await observed(admin)).toEqual(ops);expect(await observed(viewer)).toEqual([]);expect(await observed(await token(server))).toEqual([]);
 const events=await token(server,admin,{type:'all'},['events.configure']);
 expect(await observed(events)).toEqual(ops.filter(id=>id!=='triggerMotion'));
 const configure=await token(server,admin,{type:'all'},['camera.configure']);expect(await observed(configure)).toEqual(['triggerMotion']);
 const manager=await createUser(server,'events_manager');await policy(server,manager,[{role_uuid:await role(server,'Event operator',['events.configure']),scope:{type:'all'}}]);
 expect(await observed(manager)).toEqual(ops.filter(id=>id!=='triggerMotion'));
 for(const identity of [admin,events,manager]){
  valid('EventCatalog',await ok(server,'/api/events/catalog',identity));
  valid('EventDestinations',await ok(server,'/api/event-destinations',identity));
  valid('EventRoutes',await ok(server,'/api/event-routes',identity));
 }
 await policy(server,manager,[]);expect((await wire(server,'/api/events/catalog',manager)).response.status).toBe(403);
});

test('destination and route lifecycle preserve secrets, revisions and references',async({server})=>{
 const created=await wire(server,'/api/event-destinations',admin,'POST',{name:'Fixture destination',enabled:false,broker:{host:'127.0.0.1',port:18883},authentication:{username:'fixture',password:'synthetic-event-secret'},tls:{mode:'disabled'}});
 expect(created.response.status).toBe(201);valid('EventDestination',created.body);const destination=created.body;
 expect(destination.authentication).toEqual({username:'fixture',password_configured:true});expect(JSON.stringify(destination)).not.toContain('synthetic-event-secret');
 valid('EventDestination',await ok(server,`/api/event-destinations/${destination.uuid}`));
 const updated=await ok(server,`/api/event-destinations/${destination.uuid}`,admin,'PUT',{revision:destination.revision,description:'updated'});valid('EventDestination',updated);expect(updated.authentication.password_configured).toBe(true);expect(updated.revision).toBe(destination.revision+1);
 expect((await wire(server,`/api/event-destinations/${destination.uuid}`,admin,'PUT',{revision:destination.revision,name:'stale'})).response.status).toBe(409);
 const routeBody={name:'Fixture route',destination:destination.key,event_types:[type],camera_scope:{type:'selector',selector:{version:1,expression:{op:'camera_uuid',values:[server.cameras.A.camera_uuid]}}},predicate:{version:1,detection:{labels_any:['person'],min_confidence:0}},suppression:{cooldown_seconds:0}};
 const preview=await ok(server,'/api/event-routes/preview',admin,'POST',routeBody);valid('EventRoutePreview',preview);expect(preview.would_publish).toBe(false);expect(preview.camera_sample.map((camera:{name:string})=>camera.name)).toEqual(['A']);
 expect((await ok(server,'/api/event-routes')).routes).toEqual([]);
 const createdRoute=await wire(server,'/api/event-routes',admin,'POST',routeBody);expect(createdRoute.response.status).toBe(201);valid('EventRoute',createdRoute.body);const route=createdRoute.body;
 valid('EventRoute',await ok(server,`/api/event-routes/${route.uuid}`));valid('EventRoutes',await ok(server,'/api/event-routes'));valid('EventDestinations',await ok(server,'/api/event-destinations'));
 expect((await wire(server,`/api/event-destinations/${destination.uuid}?revision=${updated.revision}`,admin,'DELETE')).response.status).toBe(409);
 const next=await ok(server,`/api/event-routes/${route.uuid}`,admin,'PUT',{revision:route.revision,enabled:false});valid('EventRoute',next);expect(next.enabled).toBe(false);
 expect((await wire(server,`/api/event-routes/${route.uuid}?revision=${route.revision}`,admin,'DELETE')).response.status).toBe(409);
 valid('EventMutationAck',await ok(server,`/api/event-routes/${route.uuid}?revision=${next.revision}`,admin,'DELETE'));
 const cleared=await ok(server,`/api/event-destinations/${destination.uuid}`,admin,'PUT',{revision:updated.revision,authentication:{password:null}});valid('EventDestination',cleared);expect(cleared.authentication.password_configured).toBe(false);
 valid('EventMutationAck',await ok(server,`/api/event-destinations/${destination.uuid}?revision=${cleared.revision}`,admin,'DELETE'));
 expect((await wire(server,`/api/event-routes/${route.uuid}`)).response.status).toBe(404);expect((await wire(server,`/api/event-destinations/${destination.uuid}`)).response.status).toBe(404);
});

test('denied and malformed event writes have no durable catalog or route effects',async({server})=>{
 const destination=await ok(server,'/api/event-destinations',admin,'POST',{name:'Immutable destination',enabled:false,broker:{host:'127.0.0.1'},tls:{mode:'disabled'}});
 const route=await ok(server,'/api/event-routes',admin,'POST',{name:'Immutable route',event_types:[type]});
 const before=[await ok(server,'/api/event-destinations'),await ok(server,'/api/event-routes')];
 const requests=[['POST','/api/event-destinations',{name:'denied',broker:{host:'127.0.0.1'}}],['PUT',`/api/event-destinations/${destination.uuid}`,{revision:destination.revision,enabled:true}],['DELETE',`/api/event-destinations/${destination.uuid}?revision=${destination.revision}`,undefined],['POST','/api/event-routes',{name:'denied',event_types:[type]}],['PUT',`/api/event-routes/${route.uuid}`,{revision:route.revision,enabled:false}],['DELETE',`/api/event-routes/${route.uuid}?revision=${route.revision}`,undefined],['POST','/api/event-routes/preview',{name:'denied',event_types:[type]}]] as const;
 for(const identity of [anonymous,viewer,await token(server)])for(const [method,url,body] of requests){const result=await wire(server,url,identity,method,body);expect(result.response.status).toBe(identity===anonymous?401:403);valid('ApiError',result.body);}
 for(const body of [{revision:destination.revision,broker:{port:0}},{revision:destination.revision,authentication:{password:'secret',username:''}},{revision:destination.revision,tls:{mode:'mutual',ca_file:'',cert_file:'',key_file:''}},{revision:destination.revision,unknown:true}])expect((await wire(server,`/api/event-destinations/${destination.uuid}`,admin,'PUT',body)).response.status).toBe(400);
 for(const body of [{event_types:[type,type]},{event_types:['unknown.fixture']},{camera_scope:{type:'collection',collection_uuid:'00000000-0000-4000-8000-000000000001'}},{suppression:{cooldown_seconds:-1}},{schedule:{version:1,timezone:'../etc',windows:[]}},{predicate:{version:1,health:{condition_codes_any:['unknown.fixture']}}}])expect((await wire(server,`/api/event-routes/${route.uuid}`,admin,'PUT',{revision:route.revision,...body})).response.status).toBe(400);
 // Collection references are not an event selector feature: existing private
 // and shared collections must both be rejected rather than widening scope.
 for(const shared of [false,true]){
  const collection=await ok(server,'/api/camera-collections',admin,'POST',{name:shared?'Shared event fixture':'Private event fixture',type:'static',shared});
  const invalid={name:'Collection scope',event_types:[type],camera_scope:{type:'collection',collection_uuid:collection.uuid}};
  expect((await wire(server,'/api/event-routes/preview',admin,'POST',invalid)).response.status).toBe(400);
  expect((await wire(server,'/api/event-routes',admin,'POST',invalid)).response.status).toBe(400);
 }
 expect([await ok(server,'/api/event-destinations'),await ok(server,'/api/event-routes')]).toEqual(before);
});

test('motion acknowledgments distinguish annotations from recording and honor camera authority',async({server})=>{
 for(const identity of [anonymous,viewer,await token(server)])expect((await wire(server,'/api/motion/trigger',identity,'POST',{stream:'A',action:'start',label:'denied'})).response.status).toBe(identity===anonymous?401:403);
 for(const body of [{stream:'A',action:'pulse',duration_ms:0},{stream:'A',action:'pulse',duration_ms:1.5},{stream:'A',action:'invalid'},{stream:'A',action:'start',objects:[{label:'person',confidence:2}]}])expect((await wire(server,'/api/motion/trigger',admin,'POST',body)).response.status).toBe(400);
 const configure=await token(server,admin,{type:'selector',selector:{version:1,expression:{op:'camera_uuid',values:[server.cameras.A.camera_uuid]}}},['camera.configure']);
 expect((await wire(server,'/api/motion/trigger',configure,'POST',{stream:'B',action:'start'})).response.status).toBe(403);
 for(const action of ['start','stop','pulse']){
  const result=await wire(server,'/api/motion/trigger',configure,'POST',{stream:'A',action,duration_ms:10,objects:['person'],tags:['fixture']});
  expect(result.response.status).toBe(202);valid('MotionTriggered',result.body);expect(result.body.recording_triggered).toBe(false);expect(result.body.tags_applied).toBe(0);expect(result.body.detections_stored).toBe(action==='stop'?0:1);
 }
});
