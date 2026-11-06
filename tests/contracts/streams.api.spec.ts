import {test,expect,accessCases,identityFor,discovery,pilotOperationIds as operationIds,wire,ok,admin,valid,projectedResponse,lintProjections} from './permissionCases';

for(const scenario of accessCases) test(`${scenario.id}: actual Streams access and conditional contract agree`,async({server},info)=>{
  const identity=await identityFor(server,scenario);
  const doc=await discovery(server,identity,info,scenario.id);
  expect(operationIds(doc)).toEqual(scenario.ops);
  expect(Boolean(doc.components.schemas.StreamFullPrivilegedResponse)).toBe(scenario.fullPrivileged);
  const list=await wire(server,'/api/streams',identity);
  expect(list.response.status).toBe(200);valid('StreamCompatibilityList',list.body);
  expect((list.body as unknown as {name:string}[]).map(row=>row.name).sort()).toEqual(scenario.read);
  projectedResponse(doc,'/api/streams','get',200,list.body);
  for(const [surface,schema] of [['admin','StreamSummaryPage'],['live','LiveStreamSummaryPage'],['lpr','LprStreamSummaryPage']]) {
    const result=await wire(server,`/api/streams?summary=true&surface=${surface}&include_admin_url=true`,identity);
    expect(result.response.status).toBe(200);valid(schema,result.body);
    const expected=surface==='lpr' ? scenario.id==='admin'?['A','B']:[] : scenario.read;
    expect(result.body.streams.map((row:{name:string})=>row.name).sort()).toEqual(expected);
    expect(result.body.total).toBe(expected.length);
    projectedResponse(doc,'/api/streams','get',200,result.body);
  }
  for(const name of ['A','B']) {
    const result=await wire(server,`/api/streams/${name}/full`,identity);
    if(scenario.read.includes(name)) {
      expect(result.response.status).toBe(200);valid('StreamFullResponse',result.body);
      expect(result.body.stream.admin_url).toBe(scenario.configure.includes(name)?`http://127.0.0.1:9/admin-${name.toLowerCase()}`:'');
      expect(result.body.stream.can_control_privacy).toBe(scenario.configure.includes(name));
      projectedResponse(doc,'/api/streams/{name}/full','get',200,result.body);
    } else {expect(result.response.status).toBe(403);valid('ApiError',result.body);}
    const before=await ok(server,`/api/streams/${name}/full`);
    const changed=await wire(server,`/api/streams/${name}`,identity,'PUT',{set_privacy_mode:true});
    if(scenario.configure.includes(name)) {
      expect(changed.response.status).toBe(200);valid('StreamPrivacyUpdated',changed.body);
      expect((await ok(server,`/api/streams/${name}/full`)).stream.privacy_mode).toBe(true);
      projectedResponse(doc,'/api/streams/{name}','put',200,changed.body);
    } else {
      expect(changed.response.status).toBe(403);valid('ApiError',changed.body);
      expect(await ok(server,`/api/streams/${name}/full`)).toEqual(before);
    }
  }
  lintProjections(info);
});

test('invalid edits preserve durable settings; queued updates preserve zero and explicit clears',async({server},info)=>{
  const doc=await discovery(server,admin,info,'admin');
  for(const body of [{set_privacy_mode:true,enabled:true},{enable_disabled:true,enabled:false},{playback_transport:'invalid'},{retention_days:366},{eptz_config:'not-json'}]) {
    const before=await ok(server,'/api/streams/A/full');
    const result=await wire(server,'/api/streams/A',admin,'PUT',body);
    expect(result.response.status).toBe(400);valid('ApiError',result.body);
    expect(await ok(server,'/api/streams/A/full')).toEqual(before);
  }
  const result=await wire(server,'/api/streams/A',admin,'PUT',{retention_days:0,detection_retention_days:0,detection_threshold:0,admin_url:null});
  expect(result.response.status).toBe(202);valid('StreamUpdateAccepted',result.body);
  projectedResponse(doc,'/api/streams/{name}','put',202,result.body);
  await expect.poll(async()=> (await ok(server,'/api/streams/A/full')).stream.admin_url).toBe('');
  const stream=(await ok(server,'/api/streams/A/full')).stream;
  expect(stream.retention_days).toBe(0);expect(stream.detection_retention_days).toBe(0);expect(stream.detection_threshold).toBe(0);
  expect((await ok(server,'/api/streams/B/full')).stream.admin_url).toBe('http://127.0.0.1:9/admin-b');
  lintProjections(info);
});
