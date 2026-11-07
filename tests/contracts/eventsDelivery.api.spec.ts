import path from 'node:path';
import {DatabaseSync} from 'node:sqlite';
import {startMqttSink} from './mqtt-sink';
import {test,expect,ok,admin,viewer,wire,valid} from './permissionCases';
const type='io.lightnvr.detection.object.v1';
test('managed event delivery emits real MQTT bytes, retries unacknowledged work and suppresses denied motion',async({server})=>{
 test.skip(process.env.LIGHTNVR_CONTRACT_MQTT!=='1','Requires the explicitly MQTT-enabled contract build (enabled in CI).');
 const sink=await startMqttSink();
 const records=()=>sink.publications.map(p=>({topic:p.topic,qos:p.qos,body:JSON.parse(p.payload.toString())})).filter(p=>p.body.type===type);
 const outbox=()=>{const db=new DatabaseSync(path.join(server.directory,'lightnvr.db'));try{return db.prepare("SELECT state,attempt_count FROM event_outbox WHERE event_type=? ORDER BY id DESC LIMIT 1").get(type);}finally{db.close();}};
 try {
  const destination=await ok(server,'/api/event-destinations',admin,'POST',{name:'Owned MQTT sink',broker:{host:'127.0.0.1',port:sink.port,client_id:'owned-contract-sink',topic_template:'fixture/{type}/{subject_id}',qos:1,keepalive_seconds:5},tls:{mode:'disabled'}});valid('EventDestination',destination);
  const route=await ok(server,'/api/event-routes',admin,'POST',{name:'Owned detection route',destination:destination.key,event_types:[type],camera_scope:{type:'selector',selector:{version:1,expression:{op:'camera_uuid',values:[server.cameras.A.camera_uuid]}}},suppression:{cooldown_seconds:60}});valid('EventRoute',route);
  sink.acknowledge(false);
  const result=await ok(server,'/api/motion/trigger',admin,'POST',{stream:'A',action:'start',objects:[{label:'contract-person',confidence:0.9}]});valid('MotionTriggered',result);
  await ok(server,'/api/motion/trigger',admin,'POST',{stream:'A',action:'start',label:'cooldown-duplicate'});
  await expect.poll(()=>records().length,{timeout:20000}).toBeGreaterThan(0);
  const first=records()[0];expect(first.qos).toBe(1);expect(first.topic).toContain(server.cameras.A.camera_uuid);expect(first.body.specversion).toBe('1.0');expect(JSON.stringify(first.body)).toContain('contract-person');expect(JSON.stringify(first.body)).not.toContain(server.cameras.B.camera_uuid);
  expect(outbox()?.state).not.toBe('delivered');
  expect(JSON.stringify(records())).not.toContain('cooldown-duplicate');
  sink.acknowledge(true);sink.disconnect();
  await expect.poll(()=>outbox()?.state,{timeout:45000}).toBe('delivered');
  expect(Number(outbox()?.attempt_count)).toBeGreaterThanOrEqual(2);
  expect(records().filter(r=>r.body.id===first.body.id).length).toBeGreaterThanOrEqual(2);
  expect(new Set(records().map(record=>record.body.id)).size).toBe(1);
  const before=records().length;
  expect((await wire(server,'/api/motion/trigger',viewer,'POST',{stream:'A',action:'start',label:'must-not-publish'})).response.status).toBe(403);
  await ok(server,'/api/motion/trigger',admin,'POST',{stream:'B',action:'start',label:'excluded-camera'});
  await ok(server,`/api/event-routes/${route.uuid}`,admin,'PUT',{revision:route.revision,enabled:false});
  await ok(server,'/api/motion/trigger',admin,'POST',{stream:'A',action:'start',label:'paused-route'});
  // The queue is inspected after real request completion; wait one worker poll
  // for accidental deliveries rather than asserting only the acknowledgment.
  await new Promise(resolve=>setTimeout(resolve,1500));
  expect(records().length).toBe(before);expect(JSON.stringify(records())).not.toContain('must-not-publish');
 }finally{await sink.stop();}
});
