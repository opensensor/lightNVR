/** Independent system/bootstrap/workspace contract and durable-effect checks. */
import {readFileSync, mkdtempSync, rmSync, readdirSync, statSync} from 'node:fs';
import {tmpdir} from 'node:os';
import path from 'node:path';
import {startFixture} from './fixture-server.mjs';
import {test,expect,admin,viewer,anonymous,wire,ok,token,role,createUser,policy,valid,database,operationIds} from './permissionCases';

const protectedReads=[
 ['/api/settings','SystemSettings'],['/api/system','SystemInformation'],['/api/system/info','SystemInformation'],
 ['/api/system/status','SystemStatus'],['/api/system/health','HostHealth'],['/api/system/health/incidents','HostHealthIncidents'],
 ['/api/system/logs','SystemLogs'],['/api/system/go2rtc/effective-config','Go2rtcEffectiveConfiguration'],
 ['/api/system/go2rtc/override-status','Go2rtcOverrideDiagnostics'],
] as const;
const protectedWrites=['/api/settings','/api/settings/go2rtc/validate','/api/system/logs/clear','/api/system/backup','/api/system/restart','/api/system/shutdown'];

const systemOps = ['getSystemSettings','updateSystemSettings','validateGo2rtcConfiguration','getSystemInformationLegacy','getSystemInformation','getSystemStatus','getHostHealth','getHostHealthIncidents','getSystemLogs','clearSystemLogs','createSystemBackup','restartSystem','shutdownSystem','getGo2rtcEffectiveConfiguration','getGo2rtcOverrideDiagnostics','getWorkspaces','updateWorkspaces','getSetupStatus','updateSetupStatus','getStreamHealth','getHlsHealth','getMetrics','recordPlayerTelemetry'].sort();
test('system discovery is independently constrained for Basic, session, key and scoped credentials',async({server})=>{
 const observed = async(identity:typeof admin) => operationIds(await ok(server,'/api/openapi.json',identity)).filter(id=>systemOps.includes(id));
 expect(await observed(admin)).toEqual(systemOps);
 const login=await server.request('/api/auth/login',{principal:'anonymous',method:'POST',body:server.credentials.admin});
 expect(login.status).toBe(200);
 const session={headers:{Cookie:login.headers.get('set-cookie')!.split(';')[0]!}};
 expect(await observed(session)).toEqual(systemOps);
 const issued=await ok(server,'/api/auth/users/1/api-key',admin,'POST');
 expect(await observed({headers:{'X-API-Key':issued.api_key}})).toEqual(systemOps);
 const common=['getHlsHealth','getSetupStatus','getStreamHealth','getWorkspaces','recordPlayerTelemetry'];
 expect(await observed(viewer)).toEqual([...common,'updateWorkspaces'].sort());
 expect(await observed(await token(server))).toEqual(common.sort());
 const operator=await token(server,admin,{type:'all'},['system.admin']);
 expect(await observed(operator)).toEqual(systemOps.filter(id=>!['updateWorkspaces','recordPlayerTelemetry'].includes(id)));
 valid('SystemActionAccepted',await ok(server,'/api/system/logs/clear',admin,'POST'));
});

test('system reads require effective system authority and have precise response contracts',async({server})=>{
 const manager=await createUser(server,'system_operator');
 await policy(server,manager,[{role_uuid:await role(server,'System operator',['system.admin']),scope:{type:'all'}}]);
 const scoped=await token(server,admin,{type:'all'},['system.admin']);
 for(const identity of [admin,manager,scoped])for(const [route,schema] of protectedReads)valid(schema,await ok(server,route,identity));
 for(const identity of [anonymous,viewer,await token(server)])for(const [route] of protectedReads){
  const denied=await wire(server,route,identity);expect(denied.response.status).toBe(identity===anonymous?401:403);valid('ApiError',denied.body);
 }
 const metrics=await server.request('/api/metrics');expect(metrics.status).toBe(200);expect(metrics.headers.get('content-type')).toContain('text/plain');
 expect(await metrics.text()).toContain('# HELP');
 await policy(server,manager,[]);
 expect((await wire(server,'/api/system/status',manager)).response.status).toBe(403);
});

test('persisted incident history matches the contract without live observation freshness',async({server})=>{
 const id='11111111-1111-4111-8111-111111111111';
 database(server,`INSERT INTO system_health_incidents
  (uuid,condition_code,subject,scope,state,severity,first_seen_at_ms,last_seen_at_ms,
   closed_at_ms,last_observation_json,reconciliation_state,boot_id,run_id)
  VALUES ('${id}','memory.available_low','host','host','closed','warning',1000,2000,2000,
   '{"metric":"host.memory.available_ratio","value":0.1,"unit":"ratio","capability":"available"}',
   'reconciled','boot-fixture','22222222-2222-4222-8222-222222222222')`);
 try {
  const result=await ok(server,'/api/system/health/incidents?limit=20&include_closed=true');
  valid('HostHealthIncidents',result);
  expect(result.incidents.find((item:{incident_id:string})=>item.incident_id===id)?.observation).toEqual({
   metric:'host.memory.available_ratio',resource:'host',scope:'host',value:0.1,unit:'ratio',capability:'available',
  });
 } finally {database(server,`DELETE FROM system_health_incidents WHERE uuid='${id}'`);}
});

test('denied system mutations do not change config or stop the isolated process',async({server})=>{
 const config=path.join(server.directory,'lightnvr.ini'),before=readFileSync(config);
 for(const identity of [anonymous,viewer,await token(server)])for(const route of protectedWrites){
  const denied=await wire(server,route,identity,'POST',{default_detection_threshold:0});
  expect(denied.response.status).toBe(identity===anonymous?401:403);
 }
 expect(readFileSync(config)).toEqual(before);
 valid('SystemStatus',await ok(server,'/api/system/status'));
});

test('settings preserve zero, mask credentials, and reject invalid formats before changing state',async({server})=>{
 const config=path.join(server.directory,'lightnvr.ini'),before=readFileSync(config);
 const denied=await wire(server,'/api/settings',admin,'POST',{api_detection_format:'unsupported-fixture',default_detection_threshold:0});
 expect(denied.response.status).toBe(400);expect(readFileSync(config)).toEqual(before);
 const result=await ok(server,'/api/settings',admin,'POST',{default_detection_threshold:0,mqtt_password:'fixture-mqtt-secret',turn_password:'fixture-turn-secret'});
 valid('SystemSettingsSaved',result);expect(result.success).toBe(true);
 const settings=await ok(server,'/api/settings');valid('SystemSettings',settings);
 expect(settings.default_detection_threshold).toBe(0);expect(settings.mqtt_password).toBe('********');expect(settings.turn_password).toBe('********');
 expect(JSON.stringify(settings)).not.toContain('fixture-mqtt-secret');expect(JSON.stringify(settings)).not.toContain('fixture-turn-secret');
 const yaml=await ok(server,'/api/settings/go2rtc/validate',admin,'POST',{override:'streams: {}\n'});valid('Go2rtcConfigurationValidation',yaml);
});

test('workspace preferences belong to the interactive owner and do not grant camera access',async({server})=>{
 const before=await ok(server,'/api/ui/workspaces');valid('WorkspacesResponse',before);
 const updated=await ok(server,'/api/ui/workspaces',viewer,'PUT',{workspaces:{'live.navigator':false,investigation:false}});valid('WorkspacesResponse',updated);
 expect(updated.workspaces.every((w:{visible:boolean})=>!w.visible)).toBe(true);
 expect(await ok(server,'/api/ui/workspaces')).toEqual(before);
 const scoped=await token(server);const view=await ok(server,'/api/ui/workspaces',scoped);valid('WorkspacesResponse',view);expect(view.configurable).toBe(false);
 expect((await wire(server,'/api/ui/workspaces',scoped,'PUT',{workspaces:{'live.navigator':false}})).response.status).toBe(403);
 for(const workspaces of [{'live.navigator':'false'},{unknown:true},{}])expect((await wire(server,'/api/ui/workspaces',viewer,'PUT',{workspaces})).response.status).toBe(400);
 expect((await wire(server,`/api/streams/${server.cameras.B.name}/full`,viewer)).response.status).toBe(403);
});

test('public health variants hide unauthorized cameras and completed bootstrap rejects reset',async({server})=>{
 for(const [route,schema] of [['/api/health','StreamHealth'],['/api/health?liveness=true','ProcessLiveness'],['/api/health?history_only=true','StreamHealthHistory'],['/api/health/hls','HlsHealth'],['/api/setup/status','SetupStatus']] as const){
  const value=await ok(server,route,anonymous);valid(schema,value);
  if(value.streams_detail)expect(value.streams_detail).toEqual([]);
 }
 const status=await ok(server,'/api/setup/status',anonymous);expect(status.complete).toBe(true);
 expect((await wire(server,'/api/setup/status',anonymous,'POST',{complete:false})).response.status).toBe(401);
 expect((await wire(server,'/api/setup/status',viewer,'POST',{complete:false})).response.status).toBe(403);
 expect(await ok(server,'/api/setup/status',anonymous)).toEqual(status);
 const result=await server.request('/api/telemetry/player',{principal:'viewer',method:'POST',body:{stream_name:'A',transport:'hls',ttff_ms:12}});
 expect(result.status).toBe(204);expect(await result.text()).toBe('');
 expect((await server.request('/api/telemetry/player',{principal:'viewer',method:'POST',body:{stream_name:'B',transport:'hls',ttff_ms:12}})).status).toBe(403);
});

test('backup creates parseable private bytes only in an owned web root',async()=>{
 const webRoot=mkdtempSync(path.join(tmpdir(),'lightnvr-system-contract-'));
 const server=await startFixture({webRoot});
 try {
  const result=await ok(server,'/api/system/backup',admin,'POST');valid('SystemBackupCreated',result);
  expect(result.backupUrl).toBe(`/backups/${result.filename}`);
  expect(path.basename(result.filename)).toBe(result.filename);
  const file=path.join(webRoot,'backups',result.filename),bytes=readFileSync(file);
  expect(JSON.parse(bytes.toString())).toEqual(expect.any(Object));
  expect(statSync(file).mode&0o777).toBe(0o600);expect(readdirSync(path.dirname(file))).toEqual([result.filename]);
 } finally {await server.stop();rmSync(webRoot,{recursive:true,force:true});}
});


test('setup discovery follows the current bootstrap state and global policy',async({server})=>{
 const ids = async(identity:typeof admin) => operationIds(await ok(server,'/api/openapi.json',identity));
 expect(await ids(admin)).toContain('updateSetupStatus');
 expect(await ids(viewer)).not.toContain('updateSetupStatus');
 valid('SetupUpdated',await ok(server,'/api/setup/status',admin,'POST',{complete:false}));
 expect(await ids(viewer)).toContain('updateSetupStatus');
 valid('SetupUpdated',await ok(server,'/api/setup/status',anonymous,'POST',{complete:true}));
 expect(await ids(viewer)).not.toContain('updateSetupStatus');
 database(server,"UPDATE system_settings SET value='1' WHERE key='setup_complete'");
});

test('restart and shutdown acknowledgments are followed by actual owned-process outcomes',async()=>{
 const server=await startFixture();
 try {
  const before=readFileSync(server.logPath,'utf8');
  valid('SystemActionAccepted',await ok(server,'/api/system/restart',admin,'POST'));
  await expect.poll(()=>readFileSync(server.logPath,'utf8').slice(before.length),{timeout:30000}).toMatch(/Restart requested in (native|container) mode/);
  const after=readFileSync(server.logPath,'utf8').slice(before.length);
  if(after.includes('native mode')){
   await expect.poll(async()=>{try{return (await server.request('/api/system/status')).status;}catch{return 0;}},{timeout:30000}).toBe(200);
   valid('SystemActionAccepted',await ok(server,'/api/system/shutdown',admin,'POST'));
  }
  await expect.poll(()=>{try{process.kill(server.pid,0);return true;}catch{return false;}},{timeout:30000}).toBe(false);
 } finally {await server.stop();}
});
