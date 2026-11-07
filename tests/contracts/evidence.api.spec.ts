import {test,expect,viewer,ok} from './permissionCases';
import {seedRecordings,seedHistoricalNameReuse} from './fixtures/recordings/seed';
test('detection history resolves captured camera UUIDs before a reused stream name',async({server})=>{
  const fixture=seedRecordings(server);seedHistoricalNameReuse(server,fixture);
  const result=await ok(server,`/api/detection/results/A?start=${fixture.start}&end=${fixture.end}`,viewer);
  expect(result.detections.map((d:any)=>d.label)).toContain('visible-person');
  expect(result.detections.map((d:any)=>d.label)).not.toContain('historical-hidden-label');
  expect(result.detections.map((d:any)=>d.label)).toContain('historical-visible-label');
});

import {admin,wire,valid,createUser,role,policy,token,cameraScope,database,operationIds,lintProjections} from './permissionCases';
import {copyFileSync,mkdirSync,readFileSync,existsSync,writeFileSync} from 'node:fs';
import path from 'node:path';
import {DatabaseSync} from 'node:sqlite';
const evidenceOps=['getEvidenceDetections','listEvidenceModels','searchEvidenceLpr','exportEvidenceLpr','deleteEvidenceLpr','checkEvidenceRecordingFile','deleteEvidenceRecordingFile','countEvidenceProtected','syncEvidenceRecordings','queryEvidenceTimeline','resolveEvidenceSegment','searchEvidence','previewEvidenceRecordings','sampleEvidenceThumbnails','listEvidenceBookmarks','createEvidenceBookmark','getEvidenceBookmark','updateEvidenceBookmark','deleteEvidenceBookmark','getEvidenceSnapshot','getEvidenceThumbnail','getEvidenceManifest','redirectEvidencePlayback'].sort();
function query(server:any,fixture:any){return {camera_uuids:[server.cameras.A.camera_uuid],start_time:fixture.start-1,end_time:fixture.end+1};}
test('investigation queries retain captured identity, filter summaries and reject mixed unauthorized scopes',async({server})=>{
 const fixture=seedRecordings(server);seedHistoricalNameReuse(server,fixture);const body=query(server,fixture);
 for(const [route,schema]of [['timeline','EvidenceTimeline'],['recordings/preview','EvidencePreview'],['thumbnail-samples','EvidenceThumbnails'],['search','EvidenceSearch']]){
  const data=await ok(server,`/api/investigations/${route}`,viewer,'POST',body);valid(schema,data);
  const text=JSON.stringify(data);expect(text).not.toContain('hidden-');expect(text).not.toContain(server.cameras.B.camera_uuid);
  expect((await wire(server,`/api/investigations/${route}`,viewer,'POST',{...body,camera_uuids:[server.cameras.A.camera_uuid,server.cameras.B.camera_uuid]})).response.status).toBe(route==='thumbnail-samples'?400:403);
 }
 const search=await ok(server,'/api/investigations/search',viewer,'POST',body);expect(search.results.map((r:any)=>r.detection.label)).toContain('historical-visible-label');expect(search.page.total).toBe(2);
 const segment=await ok(server,'/api/investigations/segment-at',viewer,'POST',{camera_uuids:body.camera_uuids,timestamp:fixture.start+1});valid('EvidenceSegmentAt',segment);expect(segment.segment.id).not.toBe(401);
 const gap=await ok(server,'/api/investigations/segment-at',viewer,'POST',{camera_uuids:body.camera_uuids,timestamp:fixture.start-100});valid('EvidenceSegmentAt',gap);expect(gap.segment).toBeNull();
 expect((await wire(server,'/api/investigations/search',viewer,'POST',{...body,end_time:body.start_time})).response.status).toBe(400);
});
test('bookmark ownership, replay intersections and optimistic revision enforce durable no-effects',async({server})=>{
 const fixture=seedRecordings(server);const body={...query(server,fixture),title:'Contract evidence bookmark',note:'Synthetic note',cursor_time:fixture.start,primary_camera_uuid:server.cameras.A.camera_uuid,filters:{}};
 const saved=await ok(server,'/api/investigation-bookmarks',viewer,'POST',body);valid('EvidenceBookmark',saved);expect(saved.holds_recordings).toBe(false);const route=`/api/investigation-bookmarks/${saved.uuid}`;
 valid('EvidenceBookmark',await ok(server,route,viewer));valid('EvidenceBookmarks',await ok(server,'/api/investigation-bookmarks',viewer));
 expect((await wire(server,route,admin)).response.status).toBe(404);
 const scoped=await token(server,viewer,cameraScope(server.cameras.A.camera_uuid),['recordings.replay'],undefined,true);valid('EvidenceBookmark',await ok(server,route,scoped));
 const hidden=await token(server,viewer,cameraScope(server.cameras.A.camera_uuid),['live.view']);
 expect((await wire(server,route,hidden)).response.status).toBe(403);expect((await ok(server,'/api/investigation-bookmarks',hidden)).count).toBe(0);
 for(const method of ['PUT','DELETE'])expect((await wire(server,route,hidden,method,{title:'forbidden',revision:saved.revision})).response.status).toBe(403);
 expect((await ok(server,route,viewer)).title).toBe(saved.title);
 const updated=await ok(server,route,viewer,'PUT',{title:'Updated evidence',note:'Updated',revision:saved.revision});valid('EvidenceBookmark',updated);expect(updated.revision).toBe(saved.revision+1);
 expect((await wire(server,route,viewer,'DELETE',{revision:saved.revision})).response.status).toBe(409);expect((await ok(server,route,viewer)).title).toBe(updated.title);
 valid('EvidenceBookmarkDeleted',await ok(server,route,viewer,'DELETE',{revision:updated.revision}));expect((await wire(server,route,viewer)).response.status).toBe(404);
 expect((await ok(server,'/api/recordings/protected',viewer)).protected_count).toBe(0);
});
test('JPEG namespaces, manifest and playback redirect preserve captured media authority and bytes',async({server})=>{
 const fixture=seedRecordings(server);seedHistoricalNameReuse(server,fixture);
 const owner=await createUser(server,'snapshot_owner');const grant=await role(server,'snapshot-owner',['snapshot.create','recordings.replay']);await policy(server,owner,[{role_uuid:grant,scope:cameraScope(server.cameras.A.camera_uuid)}],true);
 const storage=path.join(fixture.directory,'snapshots');for(const name of ['A',server.cameras.A.camera_uuid,server.cameras.B.camera_uuid]){mkdirSync(path.join(storage,name),{recursive:true});copyFileSync(fixture.thumbnail,path.join(storage,name,'fixture.jpg'));}
 for(const name of ['A',server.cameras.A.camera_uuid]){const response=await server.request(`/api/snapshots/${name}/fixture.jpg`,{headers:owner.headers});expect(response.status).toBe(200);expect(response.headers.get('content-type')).toMatch(/^image\/jpeg/);expect(Buffer.from(await response.arrayBuffer())).toEqual(readFileSync(fixture.thumbnail));}
 expect((await server.request(`/api/snapshots/${server.cameras.B.camera_uuid}/fixture.jpg`,{headers:owner.headers})).status).toBe(403);
 // A UUID URL may never fall back to an existing legacy A file.
 copyFileSync(fixture.thumbnail,path.join(storage,'A','only-legacy.jpg'));
 expect((await server.request(`/api/snapshots/${server.cameras.A.camera_uuid}/only-legacy.jpg`,{headers:owner.headers})).status).toBe(404);
 database(server,'UPDATE streams SET name=? WHERE camera_uuid=?',['RenamedA',server.cameras.A.camera_uuid]);
 expect((await server.request(`/api/snapshots/${server.cameras.A.camera_uuid}/fixture.jpg`,{headers:owner.headers})).status).toBe(200);
 expect((await server.request('/api/snapshots/A/fixture.jpg',{headers:owner.headers})).status).toBe(404);
 database(server,'UPDATE streams SET name=? WHERE camera_uuid=?',['A',server.cameras.B.camera_uuid]);
 expect((await server.request('/api/snapshots/A/fixture.jpg',{headers:owner.headers})).status).toBe(403);
 expect((await server.request(`/api/snapshots/${server.cameras.A.camera_uuid}/fixture.jpg`,{headers:owner.headers})).status).toBe(200);
 const manifest=`/api/timeline/manifest?stream=RenamedA&start=${encodeURIComponent(new Date((fixture.start-1)*1000).toISOString())}&end=${encodeURIComponent(new Date((fixture.end+1)*1000).toISOString())}`;
 const response=await server.request(manifest,{headers:viewer.headers});expect(response.status).toBe(200);expect(response.headers.get('content-type')).toMatch(/mpegurl/i);const playlist=await response.text();expect(playlist).toContain('#EXTM3U');expect(playlist).toMatch(/\/api\/recordings\/play\/(101|102|104|402)/);expect(playlist).not.toContain('/401');
 const redirect=await server.request(`/api/timeline/play?stream=RenamedA&start=${fixture.start}`,{headers:viewer.headers,redirect:'manual'});expect(redirect.status).toBe(302);expect(await redirect.text()).toBe('');expect(redirect.headers.get('location')).toMatch(/^\/api\/recordings\/play\/[0-9]+$/);
 const media=await server.request(redirect.headers.get('location')!,{headers:{...viewer.headers,Range:'bytes=0-63'}});expect(media.status).toBe(206);expect(Buffer.from(await media.arrayBuffer())).toEqual(fixture.bytes.subarray(0,64));
 const thumb=await server.request('/api/investigations/thumbnail/101/0',{headers:viewer.headers});expect(thumb.status).toBe(200);expect(thumb.headers.get('content-type')).toMatch(/^image\/jpeg/);const image=Buffer.from(await thumb.arrayBuffer());expect(image.subarray(0,2)).toEqual(Buffer.from([0xff,0xd8]));expect(image.subarray(-2)).toEqual(Buffer.from([0xff,0xd9]));expect(image.length).toBeGreaterThan(100);
 for(const route of ['/api/investigations/thumbnail/401/0',`/api/timeline/play?stream=A&start=${fixture.start}`,manifest.replace('RenamedA','A')])expect((await server.request(route,{headers:viewer.headers,redirect:'manual'})).status).toBe(403);
});
test('recording file lookup and deletion reauthorize captured UUIDs and denied requests leave files and rows intact',async({server})=>{
 const fixture=seedRecordings(server);const hidden=path.join(fixture.directory,'201.mp4'),visible=path.join(fixture.directory,'102.mp4');
 valid('EvidenceFileCheck',await ok(server,`/api/recordings/files/check?path=${encodeURIComponent(visible)}`,viewer));
 for(const [method,route]of [['GET',`/api/recordings/files/check?path=${encodeURIComponent(hidden)}`],['DELETE',`/api/recordings/files?path=${encodeURIComponent(hidden)}`],['DELETE',`/api/recordings/files?path=${encodeURIComponent(visible)}`],['POST','/api/recordings/sync']])expect((await wire(server,route,viewer,method)).response.status).toBe(403);
 expect(existsSync(hidden)).toBe(true);expect(existsSync(visible)).toBe(true);
 const deleted=await ok(server,`/api/recordings/files?path=${encodeURIComponent(visible)}`,admin,'DELETE');valid('EvidenceFileDeleted',deleted);expect(deleted.status).toBe('deleted');expect(existsSync(visible)).toBe(false);
 valid('EvidenceProtectedCount',await ok(server,'/api/recordings/protected',viewer));valid('EvidenceSyncComplete',await ok(server,'/api/recordings/sync',admin,'POST'));
});
test('discovery has every evidence operation and conjunctive same-camera LPR access',async({server},info)=>{
 const doc=await ok(server,'/api/openapi.json');expect(operationIds(doc).filter(id=>evidenceOps.includes(id))).toEqual(evidenceOps);
 valid('EvidenceModels',await ok(server,'/api/detection/models'));expect((await wire(server,'/api/detection/models',viewer)).response.status).toBe(403);
 const scoped=await token(server,admin,cameraScope(server.cameras.A.camera_uuid),['lpr.read']);const projected=operationIds(await ok(server,'/api/openapi.json',scoped));expect(projected).not.toContain('searchEvidenceLpr');expect(projected).not.toContain('exportEvidenceLpr');
 const always=['countEvidenceProtected','searchEvidence','listEvidenceBookmarks','createEvidenceBookmark','getEvidenceBookmark','updateEvidenceBookmark','deleteEvidenceBookmark'];
 const replay=['checkEvidenceRecordingFile','queryEvidenceTimeline','resolveEvidenceSegment','previewEvidenceRecordings','sampleEvidenceThumbnails','getEvidenceThumbnail','getEvidenceManifest','redirectEvidencePlayback'];
 mkdirSync(info.outputPath('projections'),{recursive:true});
 for(const [identity,expected,label]of [[admin,evidenceOps,'admin'],[viewer,[...always,...replay,'getEvidenceDetections','getEvidenceSnapshot'],'viewer'],[scoped,always,'lpr-read-only']]as const){const projection=await ok(server,'/api/openapi.json',identity);expect(operationIds(projection).filter(id=>evidenceOps.includes(id))).toEqual([...expected].sort());writeFileSync(path.join(info.outputPath('projections'),`${label}.json`),JSON.stringify(projection));}
 lintProjections(info);
});

import {startFixture} from './fixture-server.mjs';
import {randomBytes,randomUUID,createCipheriv,createHmac} from 'node:crypto';
test('LPR search and attachment export decrypt owned fixture rows; token intersection and deletion preserve confidentiality',async()=>{
 const key=randomBytes(32),server=await startFixture({lprMasterKeyHex:key.toString('hex')});
 try{
  const now=Date.now(),ids={A:randomUUID(),B:randomUUID()};const db=new DatabaseSync(path.join(server.directory,'lightnvr.db'));
  try{for(const camera of ['A','B'] as const){const plate=camera==='A'?'SYNTHETIC123':'HIDDEN456',nonce=randomBytes(12),uuid=server.cameras[camera].camera_uuid;const cipher=createCipheriv('aes-256-gcm',key,nonce);cipher.setAAD(Buffer.from(`${uuid}|${now}|vendor_api`));const ciphertext=Buffer.concat([cipher.update(plate,'utf8'),cipher.final()]);const derived=createHmac('sha256',key).update('lightnvr:lpr:exact:v1').digest(),blind=createHmac('sha256',derived).update(plate).digest();db.prepare('INSERT INTO lpr_reads(uuid,camera_uuid,stream_name,observed_at_ms,received_at_ms,source,vendor_topic,plate_nonce,plate_ciphertext,plate_tag,plate_exact_hmac,dedupe_hmac) VALUES(?,?,?,?,?,?,?,?,?,?,?,?)').run(ids[camera],uuid,camera,now,now,'vendor_api','',nonce,ciphertext,cipher.getAuthTag(),blind,randomBytes(32));}}finally{db.close();}
  const query={camera_uuid:server.cameras.A.camera_uuid,start_at:now-1,end_at:now+1,match:'exact',plate:'synthetic-123'};
  const scoped=await token(server,admin,cameraScope(server.cameras.A.camera_uuid),['lpr.search','lpr.read','lpr.export']);
  const reads=await ok(server,'/api/lpr/search',scoped,'POST',query);valid('EvidenceLprReads',reads);expect(reads.count).toBe(1);expect(reads.reads[0].plate).toBe('SYNTHETIC123');
  const response=await server.request('/api/lpr/export',{headers:scoped.headers,method:'POST',body:query});expect(response.status).toBe(200);expect(response.headers.get('content-disposition')).toBe('attachment; filename="lightnvr-lpr-export.json"');const bytes=await response.text();expect(bytes).not.toContain('HIDDEN456');const exported=JSON.parse(bytes);valid('EvidenceLprReads',exported);expect(exported).toEqual(reads);
  expect((await wire(server,'/api/lpr/search',scoped,'POST',{...query,camera_uuid:server.cameras.B.camera_uuid})).response.status).toBe(403);
  expect((await wire(server,`/api/lpr/reads/${ids.A}`,scoped,'DELETE')).response.status).toBe(403);expect((await ok(server,'/api/lpr/search',admin,'POST',query)).count).toBe(1);
  const deleted=await ok(server,`/api/lpr/reads/${ids.A}`,admin,'DELETE');valid('EvidenceLprDeleted',deleted);expect((await ok(server,'/api/lpr/search',admin,'POST',query)).count).toBe(0);
  const audit=JSON.stringify(await ok(server,'/api/audit/events'));expect(audit).not.toContain('SYNTHETIC123');expect(audit).not.toContain('synthetic-123');
 }finally{key.fill(0);await server.stop();}
});

test('model discovery accepts global administrators without inventory and camera-scoped configurators',async({server})=>{
 const configure=await createUser(server,'model_configurator');const grant=await role(server,'model-configure',['camera.configure']);await policy(server,configure,[{role_uuid:grant,scope:cameraScope(server.cameras.A.camera_uuid)}]);
 expect(operationIds(await ok(server,'/api/openapi.json',configure))).toContain('listEvidenceModels');valid('EvidenceModels',await ok(server,'/api/detection/models',configure));
 const empty=await startFixture({seed:false});try{expect(operationIds(await ok(empty,'/api/openapi.json'))).toContain('listEvidenceModels');valid('EvidenceModels',await ok(empty,'/api/detection/models'));}finally{await empty.stop();}
});
