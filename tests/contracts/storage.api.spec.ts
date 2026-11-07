import {mkdir,readFile,access,readdir,writeFile} from 'node:fs/promises';
import path from 'node:path';
import {createHash} from 'node:crypto';
import {DatabaseSync} from 'node:sqlite';
import {seedRecordings,recordingIds} from './fixtures/recordings/seed';
/** Owned local storage fixture; independent policy and filesystem assertions. */
import {test,expect,admin,viewer,wire,ok,token,operationIds,valid,projectedResponse,database,createUser,role,policy,type Server,type Identity} from './permissionCases';
const operations=['getStorageHealth','triggerStorageCleanup','getStorageArchive','retryStorageArchive','applyStoragePolicy','listStorageTargets','createStorageTarget','probeStorageTarget','getStorageTarget','updateStorageTarget','deleteStorageTarget','listStorageMigrations','createStorageMigration','cancelStorageMigration','retryStorageMigration','getStorageMigration','listStoragePools','createStoragePool','getStoragePool','updateStoragePool','deleteStoragePool','getStorageCompliance','listStoragePolicies','createStoragePolicy','previewStoragePolicy','getStoragePolicy','updateStoragePolicy','deleteStoragePolicy'];
test('all storage routes are explicitly covered in administrator discovery',async({server})=>{
 expect(operationIds(await ok(server,'/api/openapi.json')).filter(id=>operations.includes(id))).toEqual([...operations].sort());
});
test('invalid cleanup body fails before scheduling cleanup',async({server})=>{
 for(const body of [[],{aggressive:'yes'},false])expect((await wire(server,'/api/storage/cleanup',admin,'POST',body)).response.status).toBe(400);
 const trailing=await fetch(server.baseURL+'/api/storage/cleanup',{method:'POST',headers:{...admin.headers,'Content-Type':'application/json'},body:'{"aggressive":true}junk'});expect(trailing.status).toBe(400);
});

async function accepted(server:Server,route:string,method:string,body:unknown,schema:string,identity:Identity=admin,status=200){
 const result=await wire(server,route,identity,method,body);expect(result.response.status,JSON.stringify(result.body)).toBe(status);valid(schema,result.body);
 const projected=await ok(server,'/api/openapi.json',identity);const normalized=route.split('?')[0]!.replace(/[0-9a-f]{8}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{4}-[0-9a-f]{12}/gi,'{uuid}');projectedResponse(projected,normalized,method.toLowerCase(),status,result.body);return result.body;
}
async function target(server:Server,name:string,extra:Record<string,unknown>={}){
 const root=path.join(server.directory,name);await mkdir(root);return accepted(server,'/api/storage-targets','POST',{name,root_path:root,...extra},'StorageTarget',admin,201);
}
test('storage configuration preserves revisions, references and policy preview/apply semantics',async({server})=>{
 const t=await target(server,'owned-target');
 const list=await accepted(server,'/api/storage-targets','GET',undefined,'StorageTargetsResponse');expect(list.targets).toHaveLength(2);
 await accepted(server,`/api/storage-targets/${t.uuid}`,'GET',undefined,'StorageTarget');
 await accepted(server,`/api/storage-targets/${t.uuid}/probe`,'POST',undefined,'StorageTarget');
 const updated=await accepted(server,`/api/storage-targets/${t.uuid}`,'PUT',{revision:t.revision,reserve_bytes:0,archive_budget_bytes:0,migration_bandwidth_bps:0},'StorageTarget');expect(updated.reserve_bytes).toBe(0);
 expect((await wire(server,`/api/storage-targets/${t.uuid}`,admin,'PUT',{revision:t.revision,name:'Stale'})).response.status).toBe(409);
 expect((await wire(server,`/api/storage-targets/${t.uuid}?revision=${updated.revision}`,viewer,'DELETE')).response.status).toBe(403);
 const pool=await accepted(server,'/api/storage-pools','POST',{name:'Owned pool',members:[{target_uuid:t.uuid,weight:1}]},'StoragePool',admin,201);
 await accepted(server,'/api/storage-pools','GET',undefined,'StoragePoolsResponse');await accepted(server,`/api/storage-pools/${pool.uuid}`,'GET',undefined,'StoragePool');
 const poolUpdated=await accepted(server,`/api/storage-pools/${pool.uuid}`,'PUT',{revision:pool.revision,strategy:'priority'},'StoragePool');
 const payload={name:'Owned policy',primary_pool_uuid:pool.uuid,selector:{version:1,expression:{op:'all'}},priority:0,minimum_retention_days:0,desired_retention_days:0,maximum_retention_days:0};
 const disabled=await accepted(server,'/api/storage-policies/preview','POST',{...payload,enabled:false},'StoragePolicyPreview');expect(disabled.sample.every((row:any)=>row.effective_policy===null)).toBe(true);
 const p=await accepted(server,'/api/storage-policies','POST',payload,'StoragePolicy',admin,201);
 await accepted(server,'/api/storage-policies','GET',undefined,'StoragePoliciesResponse');await accepted(server,`/api/storage-policies/${p.uuid}`,'GET',undefined,'StoragePolicy');
 const preview=await accepted(server,'/api/storage-policies/preview','POST',{...payload,name:'Draft'},'StoragePolicyPreview');expect(preview.matched_camera_count).toBe(2);expect(preview.conflicts).toHaveLength(1);
 const changed=await accepted(server,`/api/storage-policies/${p.uuid}`,'PUT',{revision:p.revision,name:'Updated policy'},'StoragePolicy');
 expect((await wire(server,`/api/storage-pools/${pool.uuid}?revision=${poolUpdated.revision}`,admin,'DELETE')).response.status).toBe(409);
 expect((await wire(server,`/api/storage-targets/${t.uuid}?revision=${updated.revision}`,admin,'DELETE')).response.status).toBe(409);
 seedRecordings(server);
 database(server,'UPDATE recordings SET storage_policy_uuid=?,storage_policy_version=? WHERE id=101',[p.uuid,p.revision]);
 const dry=await accepted(server,`/api/storage-policies/${p.uuid}/apply`,'POST',{revision:changed.revision,preview:true},'StoragePolicyApplyResponse');expect(dry.preview).toBe(true);expect(dry.recordings).toBe(1);expect(dbRead(server,'SELECT storage_policy_version FROM recordings WHERE id=101')[0]?.storage_policy_version).toBe(p.revision);
 const applied=await accepted(server,`/api/storage-policies/${p.uuid}/apply`,'POST',{revision:changed.revision,preview:false},'StoragePolicyApplyResponse');expect(applied.recordings).toBe(1);expect(dbRead(server,'SELECT storage_policy_version FROM recordings WHERE id=101')[0]?.storage_policy_version).toBe(changed.revision);
 expect((await wire(server,`/api/storage-policies/${p.uuid}/apply`,admin,'POST',{revision:p.revision,preview:false})).response.status).toBe(409);
 await accepted(server,'/api/storage-compliance','GET',undefined,'StorageComplianceResponse');await accepted(server,'/api/storage/health','GET',undefined,'StorageHealthResponse');
 database(server,'UPDATE recordings SET storage_policy_uuid=NULL,storage_policy_version=0 WHERE id=101');
 await accepted(server,`/api/storage-policies/${p.uuid}?revision=${changed.revision}`,'DELETE',undefined,'FleetMutationAck');
 await accepted(server,`/api/storage-pools/${pool.uuid}?revision=${poolUpdated.revision}`,'DELETE',undefined,'FleetMutationAck');
 await accepted(server,`/api/storage-targets/${t.uuid}?revision=${updated.revision}`,'DELETE',undefined,'FleetMutationAck');
});
test('global storage authority is independent of camera scope and denied mutations have no jobs/files',async({server})=>{
 const scoped=await token(server),configure=await token(server,admin,{type:'all'},['storage.configure']);
 for(const identity of [viewer,scoped]){
  expect(operationIds(await ok(server,'/api/openapi.json',identity)).filter(id=>operations.includes(id))).toEqual([]);
  expect((await wire(server,'/api/storage-targets',identity)).response.status).toBe(403);
  expect((await wire(server,'/api/storage-targets',identity,'POST',{name:'Denied',root_path:path.join(server.directory,'denied')})).response.status).toBe(403);
  expect((await wire(server,'/api/storage/cleanup',identity,'POST',{aggressive:true})).response.status).toBe(403);
  expect((await wire(server,'/api/storage-migrations',identity,'POST',{recording_id:999,destination_target_uuid:'00000000-0000-4000-8000-000000000001'})).response.status).toBe(403);
 }
 expect(operationIds(await ok(server,'/api/openapi.json',configure)).filter(id=>operations.includes(id))).toEqual([...operations].sort());
 expect((await ok(server,'/api/storage-targets')).targets).toHaveLength(1);expect((await ok(server,'/api/storage-migrations')).jobs).toEqual([]);expect(await readdir(server.directory)).not.toContain('denied');
 await accepted(server,'/api/storage-archive','GET',undefined,'StorageArchiveResponse',configure);
 // An inert disabled S3 configuration exercises reference/secret separation without contacting any provider.
 const s3=await accepted(server,'/api/storage-targets','POST',{name:'Disabled archive',target_type:'s3',root_path:'s3://contract-bucket/owned',endpoint:'https://127.0.0.1:1',region:'fixture',bucket:'contract-bucket',credential_ref:'fixture-credentials',enabled:false},'StorageTarget',configure,201);
 expect(s3.credential_ref).toBe('fixture-credentials');expect(s3.archive_only).toBe(true);expect(s3.health.capacity_known).toBe(false);expect(s3).not.toHaveProperty('secret_access_key');expect(s3).not.toHaveProperty('access_key_id');
 await accepted(server,`/api/storage-targets/${s3.uuid}?revision=${s3.revision}`,'DELETE',undefined,'FleetMutationAck',configure);
});
function dbRead(server:Server,sql:string){expect(path.basename(server.directory)).toMatch(/^lightnvr-contract-/);const db=new DatabaseSync(path.join(server.directory,'lightnvr.db'));try{return db.prepare(sql).all();}finally{db.close();}}
async function terminal(server:Server,uuid:string,expected='completed'){
 await expect.poll(async()=>{const job=await ok(server,`/api/storage-migrations/${uuid}`);valid('StorageMigration',job);return job.state;},{timeout:15000}).toBe(expected);
 return accepted(server,`/api/storage-migrations/${uuid}`,'GET',undefined,'StorageMigration');
}
test('local migrations publish verified bytes and preserve catalog and protection across copy/move/cancel/retry',async({server})=>{
 const media=seedRecordings(server),initial=(await ok(server,'/api/storage-targets')).targets[0];
 database(server,"UPDATE recordings SET storage_target_uuid=?,object_key=CAST(id AS TEXT)||'.mp4'",[initial.uuid]);
 database(server,'UPDATE recordings SET protected=1 WHERE id=?',[recordingIds.playable]);
 const dest=await target(server,'migration-destination');
 const copy=await accepted(server,'/api/storage-migrations','POST',{recording_id:recordingIds.playable,destination_target_uuid:dest.uuid,operation:'copy'},'StorageMigration',admin,202);
 const copied=await terminal(server,copy.uuid);expect(copied.checksum).toBe(createHash('sha256').update(media.bytes).digest('hex'));expect(copied.bytes_copied).toBe(media.bytes.length);
 expect(await readFile(path.join(dest.root_path,copied.destination_object_key))).toEqual(media.bytes);expect(await readFile(path.join(media.directory,'101.mp4'))).toEqual(media.bytes);
 expect(dbRead(server,'SELECT protected FROM recordings WHERE id=101')[0]?.protected).toBe(1);expect(dbRead(server,'SELECT * FROM storage_recording_copies WHERE recording_id=101')).toHaveLength(1);
 expect((await wire(server,'/api/storage-migrations',viewer,'POST',{recording_id:recordingIds.sibling,destination_target_uuid:dest.uuid})).response.status).toBe(403);
 const moved=await accepted(server,'/api/storage-migrations','POST',{recording_id:recordingIds.sibling,destination_target_uuid:dest.uuid,operation:'move'},'StorageMigration',admin,202);
 const complete=await terminal(server,moved.uuid);expect(await readFile(path.join(dest.root_path,complete.destination_object_key))).toEqual(media.bytes);
 await expect.poll(async()=>{try{await access(path.join(media.directory,'102.mp4'));return false;}catch{return true;}}).toBe(true);
 expect(dbRead(server,'SELECT storage_target_uuid FROM recordings WHERE id=102')[0]?.storage_target_uuid).toBe(dest.uuid);
 const now=new Date(),minute=now.getUTCHours()*60+now.getUTCMinutes();
 const deferred=await target(server,'windowed-target',{archival_window_start_minute:(minute+60)%1440,archival_window_end_minute:(minute+61)%1440});
 const queued=await accepted(server,'/api/storage-migrations','POST',{recording_id:recordingIds.hidden,destination_target_uuid:deferred.uuid},'StorageMigration',admin,202);
 const manager=await createUser(server,'storage_other_owner');const roleId=await role(server,'Storage manager',['storage.configure']);await policy(server,manager,[{role_uuid:roleId,scope:{type:'all'}}]);
 const cancelled=await accepted(server,`/api/storage-migrations/${queued.uuid}/cancel`,'POST',undefined,'StorageMigration',manager);expect(cancelled.state).toBe('cancelled');expect(await readdir(deferred.root_path)).toEqual([]);
 const retried=await accepted(server,`/api/storage-migrations/${queued.uuid}/retry`,'POST',undefined,'StorageMigration',manager);expect(retried.state).toBe('queued');
 await accepted(server,`/api/storage-migrations/${queued.uuid}/cancel`,'POST',undefined,'StorageMigration',manager);
 await accepted(server,'/api/storage-migrations','GET',undefined,'StorageMigrationsResponse',manager);
 // Acknowledgement is distinct from cleanup outcome; protected source bytes survive.
 const cleanupStarted=Math.floor(Date.now()/1000);await accepted(server,'/api/storage/cleanup','POST',{aggressive:false},'StorageCleanupResponse');
 await expect.poll(async()=>(await ok(server,'/api/storage/health')).last_cleanup_time,{timeout:15000}).toBeGreaterThanOrEqual(cleanupStarted);expect(await readFile(path.join(media.directory,'101.mp4'))).toEqual(media.bytes);
});
test('archive retry acknowledges scheduling, then owned deletion ledger reaches completion',async({server})=>{
 const media=seedRecordings(server),uuid='11111111-1111-4111-8111-111111111111',file=path.join(media.directory,'owned-archive-delete.bin');await writeFile(file,'owned deletion payload');
 // Fault fixture: a prior deletion was deferred. No supported API can create this clock state.
 database(server,"INSERT INTO storage_deletions(uuid,recording_id,reason) VALUES(?,?,'contract fault')",[uuid,recordingIds.orphan]);
 database(server,"INSERT INTO storage_deletion_objects(deletion_uuid,file_path,state,next_attempt_at) VALUES(?,?,'pending',9999999999)",[uuid,file]);
 database(server,'UPDATE recordings SET deletion_pending=1 WHERE id=?',[recordingIds.orphan]);
 const archive=await accepted(server,'/api/storage-archive','GET',undefined,'StorageArchiveResponse');expect(archive.pending_deletions.map((row:any)=>row.uuid)).toContain(uuid);
 expect((await wire(server,'/api/storage-archive/retry',viewer,'POST',{deletion_uuid:uuid})).response.status).toBe(403);expect(await readFile(file,'utf8')).toBe('owned deletion payload');
 await accepted(server,'/api/storage-archive/retry','POST',{deletion_uuid:uuid},'StorageArchiveRetryResponse',admin,202);
 await expect.poll(async()=>{try{await access(file);return false;}catch{return true;}},{timeout:15000}).toBe(true);
 await expect.poll(async()=>(await ok(server,'/api/storage-archive')).pending_deletions.length,{timeout:15000}).toBe(0);
});
