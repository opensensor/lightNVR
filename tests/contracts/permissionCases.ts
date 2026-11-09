/** Independent policy scenarios. Expected access is authored here, never derived from schemas. */
import {test as base, expect, type TestInfo} from '@playwright/test';
import {readFileSync, mkdirSync, writeFileSync} from 'node:fs';
import {execFileSync} from 'node:child_process';
import path from 'node:path';
import {DatabaseSync, type SQLInputValue} from 'node:sqlite';
import Ajv2020 from 'ajv/dist/2020.js';
import addFormats from 'ajv-formats';
import {startFixture, basic, cameraScope, type ContractFixture} from './fixture-server.mjs';
export {expect, cameraScope};
export type Server = ContractFixture;
export type Json = Record<string, any>; // Test inspection only; all wire bodies additionally pass runtime schemas.
export type RecordingAction = 'protect' | 'delete' | 'export';
export type Identity = {headers: Record<string,string>; id?: number; username?: string; password?: string; tokenId?: string; identityManagement?: boolean; recordingReplay?: boolean; recordingActions?: readonly RecordingAction[]};
export const admin: Identity = {headers:{Authorization:basic({username:'admin',password:'ContractAdmin123!'})},id:1,recordingReplay:true,recordingActions:['protect','delete','export']};
export const anonymous: Identity = {headers:{}};
export const viewer: Identity = {headers:{Authorization:basic({username:'contract_viewer',password:'ContractViewer123!'})},id:2,recordingReplay:true,recordingActions:['export']};
export const test = base.extend<{server: Server}>({
  server: async ({}, use, info) => {
    const server = await startFixture();
    try { await use(server); }
    finally {
      const failed=info.status!==info.expectedStatus;
      if(failed) await info.attach('native-process-log',{path:server.logPath,contentType:'text/plain'});
      await server.stop({failed});
    }
  },
});
const canonical: Json = JSON.parse(readFileSync('docs/api/openapi.json','utf8'));
const ajv = new Ajv2020({strict:false,allErrors:true}); addFormats(ajv);
ajv.addSchema({$id:'urn:lightnvr:canonical',components:canonical.components});
/** Every helper request authenticates, which updates last_login at one-second
 * resolution, so before/after "no side effect" comparisons of user data ignore it. */
export function withoutLoginTime<T>(value: T): T {
  return JSON.parse(JSON.stringify(value, (key, item) => key === 'last_login' ? 0 : item));
}
export function valid(name: string, body: unknown) {
  const validator=ajv.compile({$ref:`urn:lightnvr:canonical#/components/schemas/${name}`});
  expect(validator(body), `${name}: ${JSON.stringify(validator.errors)}`).toBe(true);
}
export async function wire(server: Server, route: string, identity: Identity=admin, method='GET', body?: unknown) {
  const response=await server.request(route,{principal:'anonymous',headers:identity.headers,method,body});
  expect(response.headers.get('content-type')).toMatch(/^application\/json(?:;|$)/);
  const value: Json = await response.json();
  return {response,body:value};
}
export async function ok(server: Server, route: string, identity: Identity=admin, method='GET', body?:unknown): Promise<Json> {
  const result=await wire(server,route,identity,method,body);
  expect(result.response.ok, `${method} ${route}: ${result.response.status} ${JSON.stringify(result.body)}`).toBe(true);
  return result.body;
}
export async function createUser(server: Server, name: string, extra: Json={}): Promise<Identity> {
  const password='ContractMatrix123!';
  const user=await ok(server,'/api/auth/users',admin,'POST',{...extra,username:name,password,role:2,is_active:true});
  return {id:user.id,username:name,password,recordingReplay:true,recordingActions:['export'],headers:{Authorization:basic({username:name,password})}};
}
export async function role(server: Server, name: string, actions: string[]) {
  const current=await ok(server,'/api/authorization/roles');
  const result=await ok(server,'/api/authorization/roles',admin,'POST',{name,description:'Synthetic contract fixture',actions,expected_policy_version:current.policy_version});
  return result.role.uuid as string;
}
export async function policy(server: Server, user: Identity, grants: {role_uuid:string;scope:unknown}[], recordingReplay=false,recordingActions:readonly RecordingAction[]=[]) {
  const current=await ok(server,`/api/authorization/users/${user.id}`);
  await ok(server,`/api/authorization/users/${user.id}`,admin,'PUT',{mode:'policy',expected_policy_version:current.policy_version,grants});
  user.recordingReplay=recordingReplay;user.recordingActions=recordingActions;
}
export async function token(server: Server, owner: Identity=admin, scope: unknown=cameraScope(server.cameras.A.camera_uuid), actions=['live.view'], expiresAt=Math.floor(Date.now()/1000)+3600,recordingReplay=false,recordingActions:readonly RecordingAction[]=[]): Promise<Identity> {
  const issued=await ok(server,`/api/authorization/users/${owner.id}/tokens`,admin,'POST',{description:'Synthetic matrix token',expires_at:expiresAt,actions,scope});
  return {id:owner.id,tokenId:issued.token.uuid,recordingReplay,recordingActions,headers:{Authorization:`Bearer ${issued.secret}`}};
}
/** Only isolated fixture databases; fault/clock/password flags unavailable via supported APIs. */
export function database(server: Server, sql: string, values: SQLInputValue[]=[]): void {
  expect(path.basename(server.directory)).toMatch(/^lightnvr-contract-/);
  const db=new DatabaseSync(path.join(server.directory,'lightnvr.db'));
  try {
    db.exec('PRAGMA busy_timeout=3000');
    const statement=db.prepare(sql);
    // run() also reads lastInsertRowid; large-ID fixtures need exact BigInt results.
    statement.setReadBigInts(true);
    statement.run(...values);
  } finally {db.close();}
}
export function operationIds(doc: Json): string[] {
  return Object.values(doc.paths).flatMap((item:any)=>Object.values(item).filter((op:any)=>op && typeof op.operationId==='string').map((op:any)=>op.operationId)).sort();
}
// Independently authored coverage expectations: new operations are asserted too,
// while pilot operation assertions retain their original camera-permission scope.
export const identityManagementOps=['changePassword','disableTotp','getTotpStatus','setupTotp','verifyTotp'];
export const identityOps=[...identityManagementOps,'completeTotpLogin','legacyLogout','createUserToken','getClientConfig','getLoginConfig','listAuthSessions','listTrustedDevices','listUserTokens','login','logout','revokeAuthSession','revokeTrustedDevice','revokeUserToken','verifyIdentity'].sort();
export const scopedIdentityOps=['completeTotpLogin','getClientConfig','getLoginConfig','legacyLogout','login','logout'];
export const fleetCommonOps=['queryFleetCameras','previewFleetSelector','listFleetViews','getFleetView','listCameraCollections','getCameraCollection','getCameraCollectionMembers','previewCameraCollection','listFleetLocations','getFleetLocation','listFleetTags','getFleetTag'];
export const fleetOps=[...fleetCommonOps,'getCameraTagAssignments'];
export const recordingCommonOps=['getTimelineSegmentsByIds','listRecordingDetectionLabels','listRecordingTags','listRecordings'];
export const recordingReplayOps=['getRecording','getRecordingTags','getRecordingThumbnail','getTimelineSegments','playRecording'];
export const recordingJobOps=['downloadRecordingExport','getRecordingDeletionProgress','getRecordingExportStatus'];
export const recordingProtectOps=['protectRecordings','setRecordingProtection','setRecordingRetention','setRecordingTags','updateRecordingTagsBatch'];
export const recordingDeleteOps=['deleteRecording','deleteRecordings'];
export const recordingExportOps=['createRecordingExport','downloadRecording'];
export const recordingActionOps=[...recordingJobOps,...recordingProtectOps,...recordingDeleteOps,...recordingExportOps];
/** Explicit fixture oracle, independent of token action payloads and canonical metadata. */
export function expectedRecordingActionOps(identity:Identity):string[]{
  const groups=identity.recordingActions??[];
  return [...recordingJobOps,...(groups.includes('protect')?recordingProtectOps:[]),...(groups.includes('delete')?recordingDeleteOps:[]),...(groups.includes('export')?recordingExportOps:[])].sort();
}
export function expectedRecordingOps(identity:Identity):string[]{return [...recordingCommonOps,...(identity.recordingReplay?recordingReplayOps:[])].sort();}
export function nonRecordingOperationIds(doc:Json):string[]{return operationIds(doc).filter(id=>identityOps.includes(id)||allOps.includes(id));}
export function pilotOperationIds(doc: Json): string[] { return operationIds(doc).filter(id=>allOps.includes(id)); }
export const readOps=['getOpenApi','getStreamFull','listStreams'];
export const allOps=['getOpenApi','getStreamFull','listStreams','updateStream'];
export const emptyOps=['getOpenApi','listStreams'];
export function privateResponse(response: Response) {
  expect(response.headers.get('cache-control')).toBe('private, no-store');
  expect(response.headers.get('etag')).toBeNull();
}
export async function discovery(server:Server, identity:Identity, info:TestInfo, label:string, status=200):Promise<Json> {
  const {response,body}=await wire(server,'/api/openapi.json',identity);
  privateResponse(response); expect(response.status).toBe(status);
  if(status!==200){valid('ApiError',body);expect(body).not.toHaveProperty('paths');return body;}
  valid('OpenApiDocument',body);
  const ids=operationIds(body);
  expect(nonRecordingOperationIds(body).filter(id=>!allOps.includes(id))).toEqual(identity.tokenId ? [...scopedIdentityOps,...(identity.identityManagement?identityManagementOps:[])].sort() : identityOps);
  expect(ids.filter(id=>recordingCommonOps.includes(id)||recordingReplayOps.includes(id))).toEqual(expectedRecordingOps(identity));
  expect(ids.filter(id=>recordingActionOps.includes(id))).toEqual(expectedRecordingActionOps(identity));
  expect(body['x-lightnvr-coverage'].covered_operations.filter((id:string)=>[...allOps,...identityOps,...fleetOps,...recordingCommonOps,...recordingReplayOps,...recordingActionOps].includes(id)).sort()).toEqual([...allOps,...identityOps,...fleetOps,...recordingCommonOps,...recordingReplayOps,...recordingActionOps].sort());
  expect(body['x-lightnvr-coverage'].partial).toBe(true);
  expect(body['x-lightnvr-coverage'].canonical_revision).toMatch(/^[a-f0-9]{64}$/);
  expect(body.servers).toEqual([{url:'../',description:canonical.servers[0].description}]);
  const serialized=JSON.stringify(body);
  for(const forbidden of ['x-lightnvr-authorization','x-lightnvr-visibility','x-lightnvr-validators',server.cameras.A.camera_uuid,server.cameras.B.camera_uuid,'fixture-camera-secret','127.0.0.1:9','contract_viewer']) expect(serialized).not.toContain(forbidden);
  const dir=info.outputPath('projections');mkdirSync(dir,{recursive:true});writeFileSync(path.join(dir,`${label}.json`),JSON.stringify(body));
  return body;
}
export function lintProjections(info:TestInfo) {
  const output=execFileSync(process.execPath,['scripts/contracts/lint.mjs','--projections',info.outputPath('projections')],{encoding:'utf8',timeout:45000});
  writeFileSync(info.outputPath('projection-lint.log'),output);
}
export function projectedResponse(doc:Json, route:string, method:string, status:number, body:unknown) {
  const schema=doc.paths[route][method].responses[String(status)].content['application/json'].schema;
  const validator=new Ajv2020({strict:false,allErrors:true});addFormats(validator);
  const check=validator.compile({...schema,components:doc.components});
  expect(check(body),JSON.stringify(check.errors)).toBe(true);
}
export interface AccessCase {id:string;read:string[];configure:string[];ops:string[];fullPrivileged:boolean}
export const accessCases:AccessCase[]=[
  {id:'admin',read:['A','B'],configure:['A','B'],ops:allOps,fullPrivileged:true},
  {id:'viewer_a',read:['A'],configure:[],ops:readOps,fullPrivileged:false},
  {id:'configure_a_only',read:[],configure:['A'],ops:['getOpenApi','listStreams','updateStream'],fullPrivileged:false},
  {id:'mixed_scope',read:['A','B'],configure:['A'],ops:allOps,fullPrivileged:true},
  {id:'split_actions',read:['A'],configure:['B'],ops:allOps,fullPrivileged:false},
  {id:'empty_scope',read:[],configure:[],ops:emptyOps,fullPrivileged:false},
  {id:'admin_read_token_a',read:['A'],configure:[],ops:readOps,fullPrivileged:false},
];
export async function identityFor(server:Server, scenario:AccessCase):Promise<Identity> {
  if(scenario.id==='admin')return admin;
  if(scenario.id==='viewer_a')return viewer;
  if(scenario.id==='admin_read_token_a')return token(server);
  const identity=await createUser(server,scenario.id);
  const readRole=await role(server,'Read', ['live.view']);
  const configureRole=await role(server,'Configure',['camera.configure']);
  const grants=[...scenario.read.map(name=>({role_uuid:readRole,scope:cameraScope(server.cameras[name].camera_uuid)})),...scenario.configure.map(name=>({role_uuid:configureRole,scope:cameraScope(server.cameras[name].camera_uuid)}))];
  if(scenario.id==='empty_scope')grants.push({role_uuid:readRole,scope:cameraScope('00000000-0000-4000-8000-000000000099')});
  await policy(server,identity,grants);return identity;
}
