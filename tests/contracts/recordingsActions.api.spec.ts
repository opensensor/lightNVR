import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {existsSync,openSync,closeSync,ftruncateSync,readFileSync,statfsSync,unlinkSync,writeFileSync} from 'node:fs';
import path from 'node:path';
import {DatabaseSync} from 'node:sqlite';
import {test,expect,admin,anonymous,viewer,wire,ok,valid,database,createUser,role,policy,token,cameraScope,type Identity,type Server,type Json} from './permissionCases';
import {seedRecordings,seedHistoricalNameReuse,recordingIds as ids,type RecordingFixture} from './fixtures/recordings/seed';

// ZIP jobs intentionally retain their file for a bounded production window.
// Successful isolated tests remove only paths emitted by their own server log;
// failed tests retain those files and logs for diagnosis, never glob shared /tmp.
test.afterEach(async({server},info)=>{
  if(info.status!==info.expectedStatus)return;
  const filename=path.join(server.directory,'server.log');
  const log=existsSync(filename)?readFileSync(filename,'utf8'):'';
  for(const match of log.matchAll(/zip_worker: completed job [0-9a-f-]{36} -> (\/tmp\/lightnvr_zip_[A-Za-z0-9]{6}) /g)) {
    if(existsSync(match[1]))unlinkSync(match[1]);
  }
});

async function actor(server:Server,name:string,actions:string[]):Promise<Identity> {
  const user=await createUser(server,name);
  const permission=await role(server,`${name} permission`,actions);
  await policy(server,user,[{role_uuid:permission,scope:cameraScope(server.cameras.A.camera_uuid)}]);
  return user;
}
function withDb<T>(server:Server,read:(db:DatabaseSync)=>T):T {
  expect(path.basename(server.directory)).toMatch(/^lightnvr-contract-/);
  const db=new DatabaseSync(path.join(server.directory,'lightnvr.db'));
  try {db.exec('PRAGMA busy_timeout=3000');return read(db);} finally {db.close();}
}
function state(server:Server,recordingIds:readonly number[]) {
  return withDb(server,db=>recordingIds.map(id=>{
    const row=db.prepare('SELECT * FROM recordings WHERE id=?').get(id);
    const tags=db.prepare('SELECT tag FROM recording_tags WHERE recording_id=? ORDER BY tag').all(id);
    const file=row?.file_path as string|undefined;
    return {row,tags,exists:!!file&&existsSync(file),hash:file&&existsSync(file)?createHash('sha256').update(readFileSync(file)).digest('hex'):null};
  }));
}
async function jsonResult(server:Server,route:string,identity:Identity,method:string,body:unknown,status:number,schema:string) {
  const result=await wire(server,route,identity,method,body);
  expect(result.response.status,`${method} ${route}: ${JSON.stringify(result.body)}`).toBe(status);
  valid(schema,result.body);
  return result.body;
}
async function deniedUnchanged(server:Server,recordingIds:readonly number[],route:string,identity:Identity,method:string,body?:unknown,status=403) {
  const before=state(server,recordingIds);
  const result=await jsonResult(server,route,identity,method,body,status,'ApiError');
  expect(result).not.toHaveProperty('job_id');expect(result).not.toHaveProperty('token');
  expect(JSON.stringify(result)).not.toContain(server.cameras.B.camera_uuid);
  expect(JSON.stringify(result)).not.toContain(server.directory);
  expect(state(server,recordingIds)).toEqual(before);
}
async function deletionProgress(server:Server,job:string,identity:Identity) {
  const body=await ok(server,`/api/recordings/batch-delete/progress/${job}`,identity);
  valid('RecordingDeletionProgress',body);return body;
}
async function deletionDone(server:Server,job:string,identity:Identity):Promise<Json> {
  let latest:Json={};
  await expect.poll(async()=>{latest=await deletionProgress(server,job,identity);return latest.complete;},
    {timeout:15000,intervals:[50,100,200,500]}).toBe(true);
  return latest;
}
async function exportStatus(server:Server,job:string,identity:Identity) {
  const body=await ok(server,`/api/recordings/batch-download/status/${job}`,identity);
  valid('RecordingExportStatus',body);return body;
}
async function exportDone(server:Server,job:string,identity:Identity):Promise<Json> {
  let latest:Json={};
  await expect.poll(async()=>{latest=await exportStatus(server,job,identity);return ['complete','error'].includes(latest.status);},
    {timeout:20000,intervals:[50,100,200,500]}).toBe(true);
  return latest;
}
async function raw(server:Server,route:string,identity:Identity) {
  return server.request(route,{principal:'anonymous',headers:identity.headers});
}
function verifyZip(bytes:Buffer,filename:string,fixture:RecordingFixture,expectedNames:string[]) {
  writeFileSync(filename,bytes);
  // Python's standard ZIP reader independently checks the central directory,
  // decompression, per-entry CRC, filenames and uncompressed bytes.
  const entries=JSON.parse(execFileSync('python3',['-c',
    'import base64,json,sys,zipfile\nwith zipfile.ZipFile(sys.argv[1]) as z:\n assert z.testzip() is None\n print(json.dumps([{ "name":n,"data":base64.b64encode(z.read(n)).decode()} for n in z.namelist()]))',filename],
    {encoding:'utf8',timeout:10000})) as {name:string;data:string}[];
  expect(entries.map(entry=>entry.name).sort()).toEqual([...expectedNames].sort());
  for(const [index,entry] of entries.entries()) {
    const media=Buffer.from(entry.data,'base64');expect(media).toEqual(fixture.bytes);
    const extracted=path.join(fixture.directory,`zip-verified-${index}.mp4`);writeFileSync(extracted,media);
    execFileSync('ffmpeg',['-nostdin','-hide_banner','-loglevel','error','-i',extracted,'-f','null','-'],{timeout:10000});
  }
}

test('protection, retention and tag writes require evidence.protect and preserve denied state',async({server})=>{
  seedRecordings(server);
  const protector=await actor(server,'recording_protector',['evidence.protect']);
  for(const [route,body] of [
    [`/api/recordings/${ids.playable}/protect`,{protected:true}],
    [`/api/recordings/${ids.playable}/retention`,{retention_days:7}],
    [`/api/recordings/${ids.playable}/tags`,{tags:['case-one']}],
  ] as const) await deniedUnchanged(server,[ids.playable],route,viewer,'PUT',body);
  for(const [suffix,body] of [['protect',{protected:true}],['retention',{retention_days:7}],['tags',{tags:['secret-change']}]] as const)
    await deniedUnchanged(server,[ids.hidden],`/api/recordings/${ids.hidden}/${suffix}`,protector,'PUT',body);
  const protectedBody=await jsonResult(server,`/api/recordings/${ids.playable}/protect`,protector,'PUT',{protected:true},200,'RecordingProtectionResult');
  expect(protectedBody).toMatchObject({id:ids.playable,protected:true});expect(state(server,[ids.playable])[0].row?.protected).toBe(1);
  for(const days of [7,0,-1,-2,7.9]) {
    const result=await jsonResult(server,`/api/recordings/${ids.playable}/retention`,protector,'PUT',{retention_days:days},200,'RecordingRetentionResult');
    const stored=Math.trunc(days);
    expect(result).toMatchObject({id:ids.playable,retention_days:stored});expect(state(server,[ids.playable])[0].row?.retention_override_days).toBe(stored<0?null:stored);
  }
  const tags=await jsonResult(server,`/api/recordings/${ids.playable}/tags`,protector,'PUT',{tags:['case-one','case-two']},200,'RecordingTagsById');
  expect(tags).toMatchObject({id:ids.playable,tags:['case-one','case-two']});
  for(const [suffix,body] of [['protect',{protected:'true'}],['retention',{retention_days:'7'}],['tags',{tags:'case'}]] as const)
    await deniedUnchanged(server,[ids.playable],`/api/recordings/${ids.playable}/${suffix}`,protector,'PUT',body,400);
  await deniedUnchanged(server,[ids.playable],`/api/recordings/${ids.playable}/protect`,anonymous,'PUT',{protected:false},401);
});

test('mixed batch protection and tags reject before any change; authorized batches report processed operations',async({server})=>{
  seedRecordings(server);const protector=await actor(server,'batch_protector',['evidence.protect']);
  for(const [route,body] of [
    ['/api/recordings/batch-protect',{ids:[ids.playable,ids.hidden],protected:true}],
    ['/api/recordings/batch-tags',{ids:[ids.playable,ids.hidden],add:['mixed'],remove:['visible-tag']}],
  ] as const) await deniedUnchanged(server,[ids.playable,ids.hidden],route,protector,'POST',body);
  const protectedBody=await jsonResult(server,'/api/recordings/batch-protect',protector,'POST',{ids:[ids.playable,ids.sibling],protected:true},200,'RecordingBatchProtectionResult');
  expect(protectedBody).toMatchObject({success_count:2,fail_count:0,protected:true});
  expect(state(server,[ids.playable,ids.sibling]).map(value=>value.row?.protected)).toEqual([1,1]);
  const tagged=await jsonResult(server,'/api/recordings/batch-tags',protector,'POST',{ids:[ids.playable,ids.sibling],add:['review'],remove:['visible-tag']},200,'RecordingBatchTagsResult');
  // These counters report successful per-recording SQL operations, including
  // idempotent removal/insertion, rather than the number of changed rows.
  expect(tagged).toMatchObject({recordings_count:2,tags_added:2,tags_removed:2});
  expect(state(server,[ids.playable,ids.sibling]).map(value=>value.tags)).toEqual([[{tag:'review'}],[{tag:'review'}]]);
  const unchanged=state(server,[ids.playable,ids.sibling]);
  const repeated=await jsonResult(server,'/api/recordings/batch-tags',protector,'POST',{ids:[ids.playable,ids.sibling],add:['review'],remove:['visible-tag']},200,'RecordingBatchTagsResult');
  expect(repeated).toMatchObject({recordings_count:2,tags_added:2,tags_removed:2});
  expect(state(server,[ids.playable,ids.sibling])).toEqual(unchanged);
  await deniedUnchanged(server,[ids.playable],'/api/recordings/batch-tags',protector,'POST',{ids:[ids.playable,0],add:['invalid']},400);
});

test('single and mixed deletion preserve protection and capture identity before successful removal',async({server})=>{
  const fixture=seedRecordings(server);const historical=seedHistoricalNameReuse(server,fixture);
  const deleter=await actor(server,'recording_deleter',['recording.delete']);
  const protector=await actor(server,'delete_protector',['evidence.protect']);
  await deniedUnchanged(server,[ids.playable],`/api/recordings/${ids.playable}`,viewer,'DELETE');
  await deniedUnchanged(server,[historical.hidden],`/api/recordings/${historical.hidden}`,deleter,'DELETE');
  await deniedUnchanged(server,[ids.playable,ids.hidden],'/api/recordings/batch-delete',deleter,'POST',{ids:[ids.playable,ids.hidden]});
  await jsonResult(server,`/api/recordings/${ids.playable}/protect`,protector,'PUT',{protected:true},200,'RecordingProtectionResult');
  await deniedUnchanged(server,[ids.playable],`/api/recordings/${ids.playable}`,deleter,'DELETE',undefined,409);
  const batch=await jsonResult(server,'/api/recordings/batch-delete',deleter,'POST',{ids:[ids.playable]},202,'RecordingDeletionJobAccepted');
  const finished=await deletionDone(server,batch.job_id,deleter);expect(finished).toMatchObject({status:'complete',succeeded:0,failed:1,complete:true});
  expect(state(server,[ids.playable])[0]).toMatchObject({exists:true,row:{protected:1}});
  const deletion=await jsonResult(server,`/api/recordings/${ids.sibling}`,deleter,'DELETE',undefined,200,'RecordingDeletionResult');
  expect(deletion.success).toBe(true);expect(state(server,[ids.sibling])[0]).toMatchObject({row:undefined,exists:false});
  expect(existsSync(path.join(fixture.directory,`${ids.sibling}.mp4`))).toBe(false);
  await jsonResult(server,`/api/recordings/${ids.sibling}`,deleter,'DELETE',undefined,404,'ApiError');
});

test('deletion jobs enforce exact creator and token identity and complete with correct effects',async({server})=>{
  const fixture=seedRecordings(server);const owner=await actor(server,'delete_job_owner',['recording.delete']);
  const other=await actor(server,'delete_job_other',['recording.delete']);
  const first=await token(server,owner,cameraScope(server.cameras.A.camera_uuid),['recording.delete']);
  const siblingToken=await token(server,owner,cameraScope(server.cameras.A.camera_uuid),['recording.delete']);
  const accepted=await jsonResult(server,'/api/recordings/batch-delete',first,'POST',{ids:[ids.playable,ids.sibling]},202,'RecordingDeletionJobAccepted');
  const progress=`/api/recordings/batch-delete/progress/${accepted.job_id}`;
  for(const identity of [owner,siblingToken,other,admin])await jsonResult(server,progress,identity,'GET',undefined,404,'ApiError');
  await jsonResult(server,progress,anonymous,'GET',undefined,401,'ApiError');
  const done=await deletionDone(server,accepted.job_id,first);expect(done).toMatchObject({status:'complete',total:2,current:2,succeeded:2,failed:0,pending_deletions:0,complete:true});
  for(const id of [ids.playable,ids.sibling]) {expect(state(server,[id])[0].row).toBeUndefined();expect(existsSync(path.join(fixture.directory,`${id}.mp4`))).toBe(false);}
  expect(state(server,[ids.hidden])[0].exists).toBe(true);
  const basic=await jsonResult(server,'/api/recordings/batch-delete',owner,'POST',{ids:[999999]},202,'RecordingDeletionJobAccepted');
  const basicDone=await deletionDone(server,basic.job_id,owner);expect(basicDone).toMatchObject({status:'complete',succeeded:0,failed:1});
  for(const identity of [first,siblingToken])await jsonResult(server,`/api/recordings/batch-delete/progress/${basic.job_id}`,identity,'GET',undefined,404,'ApiError');
});

test('filtered deletion keeps camera and collection intersection and distinguishes DB failure from no matches',async({server})=>{
  seedRecordings(server);const owner=await actor(server,'filtered_deleter',['recording.delete']);
  const collection=await ok(server,'/api/camera-collections',admin,'POST',{name:'Delete B collection',type:'static',shared:true});
  await ok(server,`/api/camera-collections/${collection.uuid}/members`,admin,'PUT',{camera_uuids:[server.cameras.B.camera_uuid]});
  const before=state(server,[ids.playable,ids.sibling,ids.hidden]);
  const empty=await jsonResult(server,'/api/recordings/batch-delete',owner,'POST',{filter:{stream:'A',collection_uuid:collection.uuid}},202,'RecordingDeletionJobAccepted');
  expect(await deletionDone(server,empty.job_id,owner)).toMatchObject({status:'complete',total:0,succeeded:0,failed:0,complete:true});
  expect(state(server,[ids.playable,ids.sibling,ids.hidden])).toEqual(before);
  // Only this isolated server's schema is renamed. Preflight still resolves
  // camera policy, while the actual asynchronous recording COUNT must fail.
  let renamed=false;
  try {
    database(server,'ALTER TABLE recordings RENAME TO recordings_t24_unavailable');renamed=true;
    const accepted=await jsonResult(server,'/api/recordings/batch-delete',owner,'POST',{filter:{stream:'A'}},202,'RecordingDeletionJobAccepted');
    const failed=await deletionDone(server,accepted.job_id,owner);
    expect(failed).toMatchObject({status:'error',complete:true,succeeded:0,failed:0});
    expect(failed.error_message).toMatch(/Failed to count recordings/);
    expect(failed.status_message).not.toBe('No recordings found');
  } finally {if(renamed)database(server,'ALTER TABLE recordings_t24_unavailable RENAME TO recordings');}
  expect(state(server,[ids.playable,ids.sibling,ids.hidden])).toEqual(before);
  const deleted=await jsonResult(server,'/api/recordings/batch-delete',owner,'POST',{filter:{stream:'A',tag:'visible-tag'}},202,'RecordingDeletionJobAccepted');
  expect(await deletionDone(server,deleted.job_id,owner)).toMatchObject({status:'complete',total:1,succeeded:1,failed:0});
  expect(state(server,[ids.playable])[0].row).toBeUndefined();expect(state(server,[ids.sibling,ids.hidden]).every(value=>value.exists)).toBe(true);
});

test('direct export uses recordings.export independently of replay and preserves real attachment bytes',async({server})=>{
  const fixture=seedRecordings(server);const exporter=await actor(server,'direct_exporter',['recordings.export']);
  const replayOnly=await actor(server,'replay_without_export',['recordings.replay']);
  await deniedUnchanged(server,[ids.playable],`/api/recordings/download/${ids.playable}`,replayOnly,'GET');
  await deniedUnchanged(server,[ids.hidden],`/api/recordings/download/${ids.hidden}`,exporter,'GET');
  const response=await raw(server,`/api/recordings/download/${ids.playable}`,exporter);
  expect(response.status).toBe(200);expect(response.headers.get('content-type')).toBe('video/mp4');expect(response.headers.get('content-disposition')).toMatch(/^attachment; filename="[^"\r\n]+"/);
  expect(Buffer.from(await response.arrayBuffer())).toEqual(fixture.bytes);
  const prepared=await jsonResult(server,`/api/recordings/download/${ids.playable}?prepare=1`,exporter,'GET',undefined,200,'RecordingPlaybackReady');expect(prepared.status).toBe('ready');
  await deniedUnchanged(server,[ids.progress],`/api/recordings/download/${ids.progress}`,exporter,'GET',undefined,409);
});

test('export jobs bind exact owner and token; completed ZIP bytes decode to the original recordings',async({server},info)=>{
  const fixture=seedRecordings(server);const owner=await actor(server,'export_job_owner',['recordings.export']);
  const other=await actor(server,'export_job_other',['recordings.export']);
  const first=await token(server,owner,cameraScope(server.cameras.A.camera_uuid),['recordings.export']);
  const siblingToken=await token(server,owner,cameraScope(server.cameras.A.camera_uuid),['recordings.export']);
  await deniedUnchanged(server,[ids.playable,ids.hidden],'/api/recordings/batch-download',first,'POST',{ids:[ids.playable,ids.hidden]});
  await deniedUnchanged(server,[ids.playable,ids.progress],'/api/recordings/batch-download',first,'POST',{ids:[ids.progress,ids.playable]},409);
  await deniedUnchanged(server,[ids.playable],'/api/recordings/batch-download',first,'POST',{ids:[999999,ids.playable]},404);
  const accepted=await jsonResult(server,'/api/recordings/batch-download',first,'POST',{ids:[ids.playable,ids.sibling],filename:'../evidence.zip'},202,'RecordingExportAccepted');
  for(const identity of [owner,siblingToken,other,admin])for(const part of ['status','result'])
    await jsonResult(server,`/api/recordings/batch-download/${part}/${accepted.token}`,identity,'GET',undefined,404,'ApiError');
  for(const part of ['status','result'])await jsonResult(server,`/api/recordings/batch-download/${part}/${accepted.token}`,anonymous,'GET',undefined,401,'ApiError');
  expect(await exportDone(server,accepted.token,first)).toMatchObject({status:'complete',current:2,total:2,error:''});
  const result=await raw(server,`/api/recordings/batch-download/result/${accepted.token}`,first);expect(result.status).toBe(200);expect(result.headers.get('content-type')).toBe('application/zip');expect(result.headers.get('content-disposition')).toBe('attachment; filename="evidence.zip"');
  const bytes=Buffer.from(await result.arrayBuffer());verifyZip(bytes,info.outputPath('verified-recordings.zip'),fixture,[`${ids.playable}.mp4`,`${ids.sibling}.mp4`]);
  const repeated=await raw(server,`/api/recordings/batch-download/result/${accepted.token}`,first);expect(repeated.status).toBe(200);expect(Buffer.from(await repeated.arrayBuffer())).toEqual(bytes);
  const basic=await jsonResult(server,'/api/recordings/batch-download',owner,'POST',{ids:[ids.playable]},202,'RecordingExportAccepted');
  expect((await exportDone(server,basic.token,owner)).status).toBe('complete');
  for(const identity of [first,siblingToken])for(const part of ['status','result'])await jsonResult(server,`/api/recordings/batch-download/${part}/${basic.token}`,identity,'GET',undefined,404,'ApiError');
});

test('real missing-source export reaches error and result409 without a partial archive',async({server})=>{
  const fixture=seedRecordings(server);const exporter=await actor(server,'failed_exporter',['recordings.export']);
  unlinkSync(path.join(fixture.directory,`${ids.sibling}.mp4`));
  const before=state(server,[ids.playable,ids.sibling]);
  const accepted=await jsonResult(server,'/api/recordings/batch-download',exporter,'POST',{ids:[ids.playable,ids.sibling]},202,'RecordingExportAccepted');
  const failed=await exportDone(server,accepted.token,exporter);expect(failed).toMatchObject({status:'error',current:2,total:2});expect(failed.error).toMatch(/Export incomplete/);
  const result=await jsonResult(server,`/api/recordings/batch-download/result/${accepted.token}`,exporter,'GET',undefined,409,'ApiError');expect(result.error).toBe('Job is error');
  expect(state(server,[ids.playable,ids.sibling])).toEqual(before);
});

test('a bounded regular export source exposes running and not_ready before completing',async({server})=>{
  const filesystem=statfsSync('/tmp');
  const freeBytes=filesystem.bavail*filesystem.bsize;
  expect(freeBytes,'256MiB export fixture requires at least2GiB free space').toBeGreaterThanOrEqual(2*1024*1024*1024);
  const fixture=seedRecordings(server);const exporter=await actor(server,'running_exporter',['recordings.export']);
  // Real bounded work (256 MiB sparse suffix), rather than a native test hook
  // or an arbitrary sleep, keeps CRC/archive generation observable over HTTP.
  const source=path.join(fixture.directory,`${ids.playable}.mp4`);
  const fd=openSync(source,'r+');try {ftruncateSync(fd,256*1024*1024);} finally {closeSync(fd);}
  database(server,'UPDATE recordings SET size_bytes=? WHERE id=?',[256*1024*1024,ids.playable]);
  const accepted=await jsonResult(server,'/api/recordings/batch-download',exporter,'POST',{ids:[ids.playable]},202,'RecordingExportAccepted');
  await expect.poll(async()=>(await exportStatus(server,accepted.token,exporter)).status,{timeout:5000,intervals:[20,50,100]}).toBe('running');
  const result=await jsonResult(server,`/api/recordings/batch-download/result/${accepted.token}`,exporter,'GET',undefined,409,'ApiError');expect(result.error).toBe('Job is not_ready');
  expect(await exportDone(server,accepted.token,exporter)).toMatchObject({status:'complete',current:1,total:1,error:''});
});
