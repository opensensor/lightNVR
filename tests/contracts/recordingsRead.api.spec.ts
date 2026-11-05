import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {readFileSync,writeFileSync,unlinkSync,existsSync} from 'node:fs';
import path from 'node:path';
import {DatabaseSync} from 'node:sqlite';
import {test,expect,admin,anonymous,viewer,wire,ok,database,createUser,role,policy,token,cameraScope,discovery,operationIds,recordingCommonOps,recordingReplayOps,lintProjections,type Identity,type Server} from './permissionCases';
import {seedRecordings,seedHistoricalNameReuse,seedLegacyCaptureRows,recordingIds as ids,type RecordingFixture} from './fixtures/recordings/seed';
import {validateRecordingsPage,validateRecordingDetail,validateRecordingTags,validateRecordingTagsById,validateRecordingDetectionLabels,validateRecordingTimeline,validateRecordingTimelineByIds,validateRecordingPlaybackReady,validateRecordingPlaybackPreparing,validateApiError} from '../../web/generated/api/validators';
const replayIds=[ids.playable,ids.sibling,ids.interrupted,ids.progress];
const sortedIds=(rows:{id:number}[])=>rows.map(row=>row.id).sort((a,b)=>a-b);
const timelineQuery=(fixture:RecordingFixture)=>`start=${encodeURIComponent(new Date((fixture.start-5)*1000).toISOString())}&end=${encodeURIComponent(new Date(Date.now()+10000).toISOString())}`;
function safeDenied(body:unknown,server:Server,fixture:RecordingFixture) {
  expect(validateApiError(body)).toBe(true);
  const text=JSON.stringify(body);
  for(const forbidden of [server.cameras.B.camera_uuid,fixture.directory,'hidden-tag','hidden-vehicle','historical-hidden'])expect(text).not.toContain(forbidden);
}
async function page(server:Server,identity=viewer,query='limit=all') {
  const body=await ok(server,`/api/recordings?${query}`,identity);expect(validateRecordingsPage(body)).toBe(true);return body;
}
async function media(server:Server,id:number,identity:Identity=viewer,headers:Record<string,string>={},suffix='') {
  return server.request(`/api/recordings/play/${id}${suffix}`,{principal:'anonymous',headers:{...identity.headers,...headers}});
}

test('recording rows, totals, facets and collection filters share replay scope and empty results',async({server},info)=>{
  const fixture=seedRecordings(server);
  const result=await page(server);expect(sortedIds(result.recordings)).toEqual(replayIds);expect(result.pagination).toEqual({page:1,pages:1,total:4,limit:4});expect(validateRecordingsPage({...result,pagination:{...result.pagination,total:'4'}})).toBe(false);
  const paginated=await page(server,viewer,'limit=2&page=2&sort=id&order=asc');expect(sortedIds(paginated.recordings)).toEqual([ids.interrupted,ids.progress]);expect(paginated.pagination).toEqual({page:2,pages:2,total:4,limit:2});
  const tags=await ok(server,'/api/recordings/tags',viewer);expect(validateRecordingTags(tags)).toBe(true);expect(tags.tags).toEqual(['visible-tag']);
  const labels=await ok(server,'/api/recordings/detection-labels',viewer);expect(validateRecordingDetectionLabels(labels)).toBe(true);expect(labels.labels).toEqual(['visible-person']);
  expect((await page(server,viewer,'limit=all&tag=hidden-tag')).recordings).toEqual([]);
  expect((await page(server,viewer,'limit=all&detection_label=hidden-vehicle')).recordings).toEqual([]);
  const forbidden=await wire(server,'/api/recordings?stream=A,B',viewer);expect(forbidden.response.status).toBe(403);safeDenied(forbidden.body,server,fixture);
  const collection=await ok(server,'/api/camera-collections',admin,'POST',{name:'Recording read fixture',type:'static',shared:true});
  await ok(server,`/api/camera-collections/${collection.uuid}/members`,admin,'PUT',{camera_uuids:[server.cameras.A.camera_uuid,server.cameras.B.camera_uuid]});
  expect(sortedIds((await page(server,viewer,`limit=all&collection_uuid=${collection.uuid}`)).recordings)).toEqual(replayIds);
  const noReplay=await token(server); // live.view only, despite broad owner authority
  const empty=await page(server,noReplay);expect(empty.recordings).toEqual([]);expect(empty.pagination.total).toBe(0);
  expect((await ok(server,'/api/recordings/tags',noReplay)).tags).toEqual([]);expect((await ok(server,'/api/recordings/detection-labels',noReplay)).labels).toEqual([]);
  const denied=await wire(server,`/api/recordings/${ids.playable}`,noReplay);expect(denied.response.status).toBe(403);safeDenied(denied.body,server,fixture);
  const emptyDoc=await discovery(server,noReplay,info,'live-only');expect(operationIds(emptyDoc).filter(id=>[...recordingCommonOps,...recordingReplayOps].includes(id))).toEqual(recordingCommonOps);
  const replay=await token(server,admin,cameraScope(server.cameras.A.camera_uuid),['recordings.replay'],undefined,true);
  expect(sortedIds((await page(server,replay)).recordings)).toEqual(replayIds);
  const doc=await discovery(server,replay,info,'replay-A');expect(operationIds(doc).filter(id=>[...recordingCommonOps,...recordingReplayOps].includes(id))).toEqual([...recordingCommonOps,...recordingReplayOps].sort());
  lintProjections(info);
});

test('detail distinguishes finalized, active, interrupted, missing-source and deleted-camera recordings',async({server})=>{
  const fixture=seedRecordings(server);
  const detail=await ok(server,`/api/recordings/${ids.playable}`,viewer);expect(validateRecordingDetail(detail)).toBe(true);expect(detail.id).toBe(ids.playable);expect(detail.camera_uuid).toBe(server.cameras.A.camera_uuid);expect(detail.in_progress).toBe(false);expect(detail.playback_issue).toBe('');expect(detail).not.toHaveProperty('tags');expect(validateRecordingDetail({...detail,id:'101'})).toBe(false);expect(validateRecordingDetail({...detail,playback_issue:null})).toBe(false);
  const tags=await ok(server,`/api/recordings/${ids.playable}/tags`,viewer);expect(validateRecordingTagsById(tags)).toBe(true);expect(tags).toEqual({id:ids.playable,tags:['visible-tag']});
  const active=await ok(server,`/api/recordings/${ids.progress}`,viewer);expect(validateRecordingDetail(active)).toBe(true);expect(active).toMatchObject({in_progress:true,end_time:null,end_time_unix:null});
  const interrupted=await ok(server,`/api/recordings/${ids.interrupted}`,viewer);expect(validateRecordingDetail(interrupted)).toBe(true);expect(interrupted).toMatchObject({in_progress:false,playback_issue:'invalid_media'});
  for(const [id,status] of [[ids.progress,409],[ids.interrupted,422]] as const) {
    const response=await media(server,id);expect(response.status).toBe(status);const body=await response.json();safeDenied(body,server,fixture);
    if(status===422)expect(body.error).toMatch(/interrupted recording.*Download.*recovery/);
  }
  for(const route of [`/api/recordings/${ids.orphan}`,`/api/recordings/${ids.orphan}/tags`,`/api/recordings/play/${ids.orphan}`]) {
    const denied=await wire(server,route,admin);expect(denied.response.status).toBe(404);safeDenied(denied.body,server,fixture);
  }
  seedLegacyCaptureRows(server,fixture);
  for(const id of [501,502]) {const legacy=await ok(server,`/api/recordings/${id}`,viewer);expect(validateRecordingDetail(legacy)).toBe(true);expect(legacy.camera_uuid).toBeNull();}
  expect((await wire(server,'/api/recordings/503',viewer)).response.status).toBe(404);
  // Missing source cannot turn a denied camera into a file existence oracle.
  unlinkSync(path.join(fixture.directory,`${ids.hidden}.mp4`));
  const hidden=await media(server,ids.hidden);expect(hidden.status).toBe(403);safeDenied(await hidden.json(),server,fixture);
  const missing=await media(server,ids.hidden,admin);expect(missing.status).toBe(404);expect(validateApiError(await missing.json())).toBe(true);
  for(const route of ['/api/recordings','/api/recordings/tags','/api/recordings/detection-labels',`/api/recordings/${ids.playable}`])expect((await wire(server,route,anonymous)).response.status).toBe(401);
});

test('timelines filter requested IDs, counts and ranges without revealing hidden capture identities',async({server})=>{
  const fixture=seedRecordings(server);const query=timelineQuery(fixture);
  for(const camera of ['stream=A',`camera_uuid=${server.cameras.A.camera_uuid}&stream=B`]) {
    const timeline=await ok(server,`/api/timeline/segments?${camera}&${query}`,viewer);expect(validateRecordingTimeline(timeline)).toBe(true);expect(sortedIds(timeline.segments)).toEqual(replayIds);expect(timeline.segment_count).toBe(4);expect(timeline.stream).toBe('A');
    expect(timeline.segments.every((row:{camera_uuid:string})=>row.camera_uuid===server.cameras.A.camera_uuid)).toBe(true);
    const interrupted=timeline.segments.find((row:{id:number})=>row.id===ids.interrupted);expect(interrupted.in_progress).toBe(false);expect(interrupted).not.toHaveProperty('playback_issue');
  }
  const selected=await ok(server,`/api/timeline/segments-by-ids?ids=${ids.hidden},${ids.playable},${ids.orphan},999999`,viewer);
  expect(validateRecordingTimelineByIds(selected)).toBe(true);expect(sortedIds(selected.segments)).toEqual([ids.playable]);expect(selected.segment_count).toBe(1);expect(selected.multi_stream).toBe(true);
  const empty=await ok(server,`/api/timeline/segments-by-ids?ids=${ids.hidden},${ids.orphan}`,viewer);expect(validateRecordingTimelineByIds(empty)).toBe(true);expect(empty).toMatchObject({segments:[],segment_count:0,start_time:'',end_time:''});
  const duplicate=await ok(server,`/api/timeline/segments-by-ids?ids=${ids.playable},${ids.playable}`,viewer);expect(duplicate.segment_count).toBe(2);
  for(const [route,status] of [[`/api/timeline/segments?stream=B&${query}`,403],['/api/timeline/segments?camera_uuid=invalid',400],['/api/timeline/segments-by-ids?ids=invalid,0',400]] as const) {
    const denied=await wire(server,route,viewer);expect(denied.response.status).toBe(status);safeDenied(denied.body,server,fixture);
  }
});

test('real playable bytes support prefix, suffix, open and clamped ranges; invalid ranges match current protocol',async({server})=>{
  const fixture=seedRecordings(server);expect(fixture.probe).toMatchObject({streams:[{codec_name:'h264',codec_type:'video',width:96,height:64}]});
  const full=await media(server,ids.playable);expect(full.status).toBe(200);expect(full.headers.get('content-type')).toBe('video/mp4');expect(full.headers.get('accept-ranges')).toBe('bytes, bytes');expect(Number(full.headers.get('content-length'))).toBe(fixture.bytes.length);expect(Buffer.from(await full.arrayBuffer())).toEqual(fixture.bytes);
  const size=fixture.bytes.length;const cases:[string,number,number][]=[['bytes=0-127',0,127],['bytes=128-',128,size-1],['bytes=-64',size-64,size-1],[`bytes=${size-4}-${size+100}`,size-4,size-1],[`bytes=-${size+100}`,0,size-1],['bytes=0',0,size-1],['bytes=0-3,8-9',0,3]];
  const fragments:Buffer[]=[];
  for(const [range,start,end] of cases) {
    const response=await media(server,ids.playable,viewer,{Range:range});expect(response.status).toBe(206);expect(response.headers.get('content-range')).toBe(`bytes ${start}-${end}/${size}`);expect(Number(response.headers.get('content-length'))).toBe(end-start+1);
    const bytes=Buffer.from(await response.arrayBuffer());expect(bytes).toEqual(fixture.bytes.subarray(start,end+1));if(fragments.length<2)fragments.push(bytes);
  }
  const reconstructed=path.join(fixture.directory,'reconstructed.mp4');writeFileSync(reconstructed,Buffer.concat(fragments));expect(readFileSync(reconstructed)).toEqual(fixture.bytes);
  const probe=JSON.parse(execFileSync('ffprobe',['-v','error','-show_entries','format=duration','-of','json',reconstructed],{encoding:'utf8'}));expect(Number(probe.format.duration)).toBeGreaterThan(1);
  execFileSync('ffmpeg',['-nostdin','-hide_banner','-loglevel','error','-i',reconstructed,'-f','null','-'],{timeout:10000});
  for(const range of [`bytes=${size}-`,'bytes=5-1','items=0-1','bytes=-0']) {
    const response=await media(server,ids.playable,viewer,{Range:range});expect(response.status).toBe(416);expect(response.headers.get('content-range')).toBeNull();expect(validateApiError(await response.json())).toBe(true);
  }
});

test('playback preparation and thumbnails retain their own JSON/media and permission boundaries',async({server})=>{
  const fixture=seedRecordings(server);
  for(const suffix of ['?prepare=1&transcode=0','?prepare=1&transcode=1','?prepare=1','?prepare=1&archive=1&transcode=0']) {
    const ready=await media(server,ids.playable,viewer,{},suffix);expect(ready.status).toBe(200);expect(ready.headers.get('content-type')).toMatch(/^application\/json/);expect(validateRecordingPlaybackReady(await ready.json())).toBe(true);
  }
  for(const [id,status] of [[ids.progress,409],[ids.interrupted,422],[ids.hidden,403]] as const) {const response=await media(server,id,viewer,{},'?prepare=1');expect(response.status).toBe(status);safeDenied(await response.json(),server,fixture);}
  await ok(server,'/api/settings',admin,'POST',{generate_thumbnails:true,thumbnails_per_recording:3});
  const thumbnail=await server.request(`/api/recordings/thumbnail/${ids.playable}/0`,{principal:'viewer'});expect(thumbnail.status).toBe(200);expect(thumbnail.headers.get('content-type')).toBe('image/jpeg');expect(Buffer.from(await thumbnail.arrayBuffer())).toEqual(readFileSync(fixture.thumbnail));
  for(const [route,identity,status] of [[`/api/recordings/thumbnail/${ids.hidden}/0`,viewer,403],[`/api/recordings/thumbnail/${ids.playable}/3`,viewer,400],[`/api/recordings/thumbnail/${ids.playable}/0`,anonymous,401]] as const) {const result=await wire(server,route,identity);expect(result.response.status).toBe(status);safeDenied(result.body,server,fixture);}
  await ok(server,'/api/settings',admin,'POST',{thumbnails_per_recording:1});expect((await wire(server,`/api/recordings/thumbnail/${ids.playable}/1`,viewer)).response.status).toBe(403);
  expect(validateRecordingPlaybackPreparing({status:'preparing',source:'archive',error:'Preparing archived recording'})).toBe(true);
  expect(validateRecordingPlaybackReady({status:'preparing'})).toBe(false);expect(validateRecordingPlaybackPreparing({status:'ready'})).toBe(false);
});

test('durable recording IDs survive sorting, repeated polling and supported sibling deletion',async({server})=>{
  seedRecordings(server);
  const original=await page(server,viewer,'limit=all&sort=id&order=asc');
  for(const order of ['desc','asc']) {
    const next=await page(server,viewer,`limit=all&sort=id&order=${order}`);expect(sortedIds(next.recordings)).toEqual(sortedIds(original.recordings));
    const detail=await ok(server,`/api/recordings/${ids.playable}`,viewer);expect(detail.id).toBe(ids.playable);
  }
  const deletion=await wire(server,`/api/recordings/${ids.sibling}`,admin,'DELETE');expect(deletion.response.ok).toBe(true);
  await expect.poll(async()=>sortedIds((await page(server)).recordings)).toEqual([ids.playable,ids.interrupted,ids.progress]);
  const remaining=await page(server);expect(remaining.pagination.total).toBe(3);expect((await ok(server,`/api/recordings/${ids.playable}`,viewer)).id).toBe(ids.playable);
  expect((await wire(server,`/api/recordings/${ids.sibling}`,viewer)).response.status).toBe(404);
});

test('historical capture UUID wins over reused names in rows, counts, facets, detail labels and timelines',async({server})=>{
  const fixture=seedRecordings(server);const historical=seedHistoricalNameReuse(server,fixture);
  const result=await page(server);expect(sortedIds(result.recordings)).toEqual([...replayIds,historical.visible]);expect(result.pagination.total).toBe(5);
  const serialized=JSON.stringify(result);for(const forbidden of [server.cameras.B.camera_uuid,'historical-hidden-label','hidden-vehicle'])expect(serialized).not.toContain(forbidden);
  expect(sortedIds((await page(server,viewer,'limit=all&stream=A')).recordings)).toEqual([...replayIds,historical.visible]);
  const tags=await ok(server,'/api/recordings/tags',viewer);expect(tags.tags).toContain('historical-visible-tag');expect(tags.tags).not.toContain('historical-hidden-tag');
  const labels=await ok(server,'/api/recordings/detection-labels',viewer);expect(labels.labels).toContain('historical-visible-label');expect(labels.labels).not.toContain('historical-hidden-label');
  for(const id of [ids.playable,historical.visible]) {const detail=await ok(server,`/api/recordings/${id}`,viewer);expect(validateRecordingDetail(detail)).toBe(true);expect(JSON.stringify(detail)).not.toContain('historical-hidden-label');}
  for(const selector of ['stream=A',`camera_uuid=${server.cameras.A.camera_uuid}`]) {
    const timeline=await ok(server,`/api/timeline/segments?${selector}&${timelineQuery(fixture)}`,viewer);expect(sortedIds(timeline.segments)).toEqual([...replayIds,historical.visible]);expect(timeline.segment_count).toBe(5);
  }
  const direct=await wire(server,`/api/recordings/${historical.hidden}`,viewer);expect(direct.response.status).toBe(403);safeDenied(direct.body,server,fixture);
  const byIds=await ok(server,`/api/timeline/segments-by-ids?ids=${historical.hidden},${historical.visible}`,viewer);expect(sortedIds(byIds.segments)).toEqual([historical.visible]);
  // Uncovered legacy media routes must not provide an alternate identity bypass.
  const hiddenStart=fixture.start-7200;
  database(server,'UPDATE recordings SET start_time=?,end_time=? WHERE id=?',[hiddenStart,hiddenStart+2,historical.hidden]);
  const hiddenWindow=`start=${encodeURIComponent(new Date(hiddenStart*1000).toISOString())}&end=${encodeURIComponent(new Date((hiddenStart+2)*1000).toISOString())}`;
  const manifest=await wire(server,`/api/timeline/manifest?stream=A&${hiddenWindow}`,viewer);expect(manifest.response.status).toBe(404);safeDenied(manifest.body,server,fixture);
  const alternate=await server.request(`/api/timeline/play?stream=A&start=${hiddenStart}`,{principal:'viewer',redirect:'manual'});expect(alternate.status).toBe(404);safeDenied(await alternate.json(),server,fixture);

});

test('related mutation boundaries reject historical and mixed unauthorized recording sets without claiming action coverage',async({server})=>{
  const fixture=seedRecordings(server);const historical=seedHistoricalNameReuse(server,fixture);
  const actor=await createUser(server,'historical_writer');const actions=await role(server,'Capture-bound fixture writes',['evidence.protect','recording.delete','recordings.export']);await policy(server,actor,[{role_uuid:actions,scope:cameraScope(server.cameras.A.camera_uuid)}]);
  const db=new DatabaseSync(path.join(server.directory,'lightnvr.db'));
  const state=()=>db.prepare('SELECT id,camera_uuid,protected,file_path FROM recordings WHERE id=?').get(historical.hidden);
  try {
    const before=state();const hash=createHash('sha256').update(readFileSync(fixture.source)).digest('hex');
    for(const [route,method,body] of [[`/api/recordings/${historical.hidden}/protect`,'PUT',{protected:true}],[`/api/recordings/${historical.hidden}`,'DELETE',undefined],[`/api/recordings/download/${historical.hidden}`,'GET',undefined]] as const) {
      const denied=await wire(server,route,actor,method,body);expect(denied.response.status).toBe(403);safeDenied(denied.body,server,fixture);expect(state()).toEqual(before);expect(existsSync(fixture.source)).toBe(true);expect(createHash('sha256').update(readFileSync(fixture.source)).digest('hex')).toBe(hash);
    }
    const batchIds=[ids.playable,ids.hidden];
    const batchRows=batchIds.map(id=>db.prepare('SELECT id,camera_uuid,protected,file_path FROM recordings WHERE id=?').get(id));
    const batchHashes=batchIds.map(id=>createHash('sha256').update(readFileSync(path.join(fixture.directory,`${id}.mp4`))).digest('hex'));
    const denied=await wire(server,'/api/recordings/batch-delete',actor,'POST',{ids:batchIds});
    expect(denied.response.status).toBe(403);safeDenied(denied.body,server,fixture);expect(denied.body).not.toHaveProperty('job_id');
    expect(batchIds.map(id=>db.prepare('SELECT id,camera_uuid,protected,file_path FROM recordings WHERE id=?').get(id))).toEqual(batchRows);
    expect(batchIds.map(id=>createHash('sha256').update(readFileSync(path.join(fixture.directory,`${id}.mp4`))).digest('hex'))).toEqual(batchHashes);
  } finally {db.close();}
});


test('warmed counts track current-name legacy fallback immediately after an isolated camera rename',async({server})=>{
  const fixture=seedRecordings(server);seedLegacyCaptureRows(server,fixture);
  const before=await page(server,viewer,'limit=20&sort=id');expect(before.pagination.total).toBe(6);expect(sortedIds(before.recordings)).toEqual([...replayIds,501,502]);
  // There is no supported camera-rename API; exercise a controlled DB transition
  // without incrementing the recording mutation generation or waiting out cache.
  database(server,'UPDATE streams SET name=? WHERE camera_uuid=?',['Renamed-A',server.cameras.A.camera_uuid]);
  const after=await page(server,viewer,'limit=20&sort=id');expect(after.pagination.total).toBe(4);expect(sortedIds(after.recordings)).toEqual(replayIds);
  expect((await ok(server,`/api/recordings/${ids.playable}`,viewer)).camera_uuid).toBe(server.cameras.A.camera_uuid);
  for(const id of [501,502])expect((await wire(server,`/api/recordings/${id}`,viewer)).response.status).toBe(404);
});
