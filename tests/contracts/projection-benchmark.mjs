#!/usr/bin/env node
/** Bounded workstation discovery measurements; actual HTTP and SQLite execution. */
import assert from 'node:assert/strict';
import {readFile, writeFile, mkdir, mkdtemp, chmod, copyFile} from 'node:fs/promises';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import path from 'node:path';
import os from 'node:os';
import {performance} from 'node:perf_hooks';
import {startFixture, basic, credentials} from './fixture-server.mjs';

const args=process.argv.slice(2);
function option(name,fallback){const i=args.indexOf(name);return i<0?fallback:args[i+1];}
const binary=path.resolve(option('--binary','build/eng-api-T20a/bin/lightnvr'));
const sizes=option('--sizes','0,16,128,512,1024').split(',').map(Number);
const samples=Number(option('--samples','20'));
const batches=Number(option('--batches','5'));
const instrumentOnly=args.includes('--instrument-only');
const timingOnly=args.includes('--timing-only');
assert(!instrumentOnly||!timingOnly);
assert(sizes.length>0&&sizes.every(n=>Number.isInteger(n)&&n>=0&&n<=1024));
assert(Number.isInteger(samples)&&samples>=5&&samples<=200);
assert(Number.isInteger(batches)&&batches>=1&&batches<=50);
const output=option('--output',null);
const directory=output?path.resolve(output):await mkdtemp(path.join(os.tmpdir(),'lightnvr-projection-benchmark-'));
if(output)await mkdir(directory,{recursive:false,mode:0o700});
const hash=data=>createHash('sha256').update(data).digest('hex');
const shellQuote=value=>`'${value.replaceAll("'", "'\\''")}'`;
const evidencePath=path.resolve(option('--build-manifest',path.join(path.dirname(path.dirname(binary)),'evidence.json')));
const evidence=JSON.parse(await readFile(evidencePath,'utf8'));
assert.equal(evidence.binary_sha256,hash(await readFile(binary)),'Build manifest binary mismatch');
assert.equal(evidence.build_flags.CMAKE_BUILD_TYPE,'Release','Performance requires the declared optimized build');
for(const file of evidence.manifest.filter(file=>/^(?:src|include)\//.test(file.path))) {
 assert.equal(hash(await readFile(file.path)),file.sha256,`Native source changed since build: ${file.path}`);
}
await copyFile(new URL(import.meta.url),path.join(directory,'projection-benchmark.mjs'));
await copyFile(new URL('./fixture-server.mjs',import.meta.url),path.join(directory,'fixture-server.mjs'));
const fixtureSha256=hash(await readFile(new URL('./fixture-server.mjs',import.meta.url)));

/* Generated only into the task's retained output directory. No product hook,
 * SQL rewriting, selector shortcut or credential/SQL text is emitted. */
const traceSource=String.raw`
#define _GNU_SOURCE
#include <sqlite3.h>
#include <dlfcn.h>
#include <pthread.h>
#include <stdint.h>
#include <inttypes.h>
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <unistd.h>
#include <fcntl.h>
#include <sys/syscall.h>
typedef struct entry {sqlite3 *db;uint64_t statements,rows,steps;int snapshot;long tid;struct entry *next;} entry;
static entry *entries;
static pthread_mutex_t lock=PTHREAD_MUTEX_INITIALIZER;
static pthread_once_t initialized=PTHREAD_ONCE_INIT;
static int (*real_open)(const char*,sqlite3**,int,const char*);
static int (*real_close)(sqlite3*),(*real_close_v2)(sqlite3*);
static int (*trace_v2)(sqlite3*,unsigned,int(*)(unsigned,void*,void*,void*),void*);
static int (*stmt_status)(sqlite3_stmt*,int,int);
static void initialize(void){
 real_open=dlsym(RTLD_NEXT,"sqlite3_open_v2");real_close=dlsym(RTLD_NEXT,"sqlite3_close");
 real_close_v2=dlsym(RTLD_NEXT,"sqlite3_close_v2");trace_v2=dlsym(RTLD_NEXT,"sqlite3_trace_v2");
 stmt_status=dlsym(RTLD_NEXT,"sqlite3_stmt_status");
 if(!real_open||!real_close||!real_close_v2||!trace_v2||!stmt_status)_exit(119);
}
static int observed(unsigned kind,void *context,void *statement,void *detail){
 entry *e=context;
 if(kind==SQLITE_TRACE_STMT){e->statements++;
   if(detail&&strstr(detail,"SELECT role,is_active,authorization_mode FROM users WHERE id=?"))e->snapshot=1;
 }else if(kind==SQLITE_TRACE_ROW)e->rows++;
 else if(kind==SQLITE_TRACE_PROFILE)e->steps+=(unsigned)stmt_status(statement,SQLITE_STMTSTATUS_VM_STEP,1);
 return 0;
}
int sqlite3_open_v2(const char *name,sqlite3 **db,int flags,const char *vfs){
 pthread_once(&initialized,initialize);int rc=real_open(name,db,flags,vfs);
 if(rc==SQLITE_OK&&(flags&SQLITE_OPEN_READONLY)){
   entry *e=calloc(1,sizeof(*e));if(!e)_exit(120);e->db=*db;e->tid=syscall(SYS_gettid);
   pthread_mutex_lock(&lock);e->next=entries;entries=e;pthread_mutex_unlock(&lock);
   if(trace_v2(*db,SQLITE_TRACE_STMT|SQLITE_TRACE_ROW|SQLITE_TRACE_PROFILE,observed,e)!=SQLITE_OK)_exit(121);
 }return rc;
}
static void finish(sqlite3 *db){
 pthread_mutex_lock(&lock);entry **cursor=&entries;while(*cursor&&(*cursor)->db!=db)cursor=&(*cursor)->next;
 entry *e=*cursor;if(e)*cursor=e->next;pthread_mutex_unlock(&lock);if(!e)return;
 trace_v2(db,0,NULL,NULL);
 if(e->snapshot){const char *name=getenv("LNVR_PROJECTION_SQL_LOG");int fd=name?open(name,O_WRONLY|O_CREAT|O_APPEND,0600):-1;
   if(fd<0)_exit(122);char line[256];int n=snprintf(line,sizeof(line),
    "{\"pid\":%d,\"thread\":%ld,\"statements\":%" PRIu64 ",\"rows\":%" PRIu64 ",\"vmSteps\":%" PRIu64 "}\n",
    getpid(),e->tid,e->statements,e->rows,e->steps);
   if(write(fd,line,(size_t)n)!=n)_exit(123);close(fd);
 }free(e);
}
int sqlite3_close(sqlite3 *db){pthread_once(&initialized,initialize);finish(db);return real_close(db);}
int sqlite3_close_v2(sqlite3 *db){pthread_once(&initialized,initialize);finish(db);return real_close_v2(db);}
`;
await writeFile(path.join(directory,'sqlite-trace.c'),traceSource);
execFileSync('cc',['-O2','-shared','-fPIC','-Wall','-Wextra',path.join(directory,'sqlite-trace.c'),'-o',path.join(directory,'sqlite-trace.so'),'-ldl','-lpthread'],{stdio:['ignore','ignore','pipe']});
// Include the private implementation only for sizeof. Garbage collection removes
// every product function; the probe does not execute or link a second server.
const sizeSource='#include "src/database/db_authorization_snapshot.c"\n#include "telemetry/stream_metrics.h"\n#include <stdio.h>\nint main(void){printf("%zu %zu %zu\\n",sizeof(fleet_camera_t),sizeof(camera_observations_t),sizeof(stream_metrics_t));return 0;}\n';
await writeFile(path.join(directory,'fleet-size.c'),sizeSource);
execFileSync('cc',['-I.','-Iinclude','-ffunction-sections','-fdata-sections','-Wl,--gc-sections',path.join(directory,'fleet-size.c'),'-o',path.join(directory,'fleet-size')]);
const [fleetCameraBytes,cameraObservationBytes,streamMetricsBytes]=execFileSync(path.join(directory,'fleet-size'),[],{encoding:'utf8'}).trim().split(' ').map(Number);
assert([fleetCameraBytes,cameraObservationBytes,streamMetricsBytes].every(n=>Number.isInteger(n)&&n>0));
const report={formatVersion:1,status:'running',artifacts:directory,evidence,
  harnessSha256:hash(await readFile(new URL(import.meta.url))),fixtureSha256,traceSourceSha256:hash(traceSource),
  environment:{node:process.version,kernel:os.release(),cpus:os.cpus().length,cpu:os.cpus()[0]?.model,loadAtStart:os.loadavg()},
  methodology:{sizes,samples,batches,concurrency:4,warmups:3,sqlSamples:5,configuredMaxStreams:1024,
    requestTimeoutMs:10000,seedTimeoutMs:180000,fixtureCameraBytes:fleetCameraBytes,cameraObservationBytes,streamMetricsBytes,
    sql:'Separate process runs: SQLite trace_v2 on readonly connections recognized by snapshot principal SELECT. Counts actual executed statements, yielded rows and SQLite VM steps through close; excludes normal authentication/writer DB connection work. No SQL text or credentials recorded.',
    timing:'Real HTTP full response-body duration with warmed Basic/token authentication and three discovery warmups. Sequential samples then batches of four concurrent requests. No SQL trace in timing runs.',
    memory:'Linux /proc status RSS and VmHWM. Before/after request phases plus 5ms samples during concurrent batches; sampled peak is a lower bound, VmHWM includes fixture preparation. One fleet inventory plus compact raw/runtime observations requires N*(sizeof(fleet_camera_t)+sizeof(camera_observations_t)), excluding context/JSON/SQLite, stack evaluation copies and transient configured-max-sized runtime metrics allocations.'},cases:[]};
await writeFile(path.join(directory,'result.json'),JSON.stringify(report,null,2)+'\n');

function distribution(values){
 const sorted=[...values].sort((a,b)=>a-b);
 const at=p=>sorted[Math.min(sorted.length-1,Math.max(0,Math.ceil(p*sorted.length)-1))];
 return {n:values.length,min:sorted[0],median:at(.5),p95:at(.95),max:sorted.at(-1),mean:values.reduce((a,b)=>a+b,0)/values.length};
}
async function memory(pid){
 const text=await readFile(`/proc/${pid}/status`,'utf8');const value=name=>Number(text.match(new RegExp(`^${name}:\\s+(\\d+)`,'m'))?.[1]||0);
 return {time:performance.now(),rssKiB:value('VmRSS'),hwmKiB:value('VmHWM'),threads:value('Threads')};
}
async function createUser(server,name){
 const password='ProjectionBench123!';
 const user=await server.json('/api/auth/users',{method:'POST',body:{username:name,password,role:2,is_active:true}});
 return {id:user.id,headers:{Authorization:basic({username:name,password})}};
}
async function setPolicy(server,user,grants){
 const current=await server.json(`/api/authorization/users/${user.id}`);
 await server.json(`/api/authorization/users/${user.id}`,{method:'PUT',body:{mode:'policy',expected_policy_version:current.policy_version,grants}});
}
async function collection(server,name,members){
 const group=await server.json('/api/camera-collections',{method:'POST',body:{name,type:'static',shared:true}});
 await server.json(`/api/camera-collections/${group.uuid}/members`,{method:'PUT',body:{camera_uuids:members}});
 return {type:'collection',collection_uuid:group.uuid};
}
async function prepare(server,size){
 const started=performance.now(),uuids=[];
 await server.json('/api/setup/status',{method:'POST',body:{complete:true}});
 for(let i=0;i<size;i++){
   assert(performance.now()-started<180000,'Fleet seeding exceeded declared180s bound');
   const name=`bench_${String(i).padStart(4,'0')}`;
   await server.json('/api/streams',{method:'POST',body:{name,url:'rtsp://fixture.invalid/disabled',enabled:false,record:false,streaming_enabled:false}});
   const camera=await server.json(`/api/streams/${name}/full`);uuids.push(camera.stream.camera_uuid);
 }
 const role=async(name,actions)=>{
   const current=await server.json('/api/authorization/roles');
   return (await server.json('/api/authorization/roles',{method:'POST',body:{name,description:'Synthetic benchmark',actions,expected_policy_version:current.policy_version}})).role.uuid;
 };
 const read=await role('Benchmark read',['live.view']),configure=await role('Benchmark configure',['camera.configure']);
 const left=await collection(server,'Benchmark left',uuids.filter((_,i)=>i%2===0));
 const right=await collection(server,'Benchmark right',uuids.filter((_,i)=>i%2===1));
 const all=await collection(server,'Benchmark all',uuids);
 const empty=await createUser(server,'benchmark_empty');await setPolicy(server,empty,[]);
 const split=await createUser(server,'benchmark_split');await setPolicy(server,split,[{role_uuid:read,scope:left},{role_uuid:configure,scope:right}]);
 const mixed=await createUser(server,'benchmark_mixed');await setPolicy(server,mixed,[{role_uuid:read,scope:all},{role_uuid:configure,scope:right}]);
 const issued=await server.json('/api/authorization/users/1/tokens',{method:'POST',body:{description:'Benchmark read token',expires_at:Math.floor(Date.now()/1000)+3600,actions:['live.view'],scope:left}});
 return {seedMilliseconds:performance.now()-started,principals:{
   admin:{headers:{Authorization:basic(credentials.admin)},expected:size?4:2,replay:size>0,recordingActions:size>0},empty:{...empty,expected:2},
   split:{...split,expected:size>1?4:size?3:2},mixed:{...mixed,expected:size>1?4:size?3:2},
   adminReadToken:{headers:{Authorization:`Bearer ${issued.secret}`},scoped:true,expected:size?3:2}}};
}
const pilotIds=['getOpenApi','getStreamFull','listStreams','updateStream'];
const identityIds=['createUserToken','getClientConfig','getLoginConfig','listAuthSessions','listTrustedDevices','listUserTokens','login','logout','revokeAuthSession','revokeTrustedDevice','revokeUserToken','verifyIdentity'];
const scopedIdentityIds=['getClientConfig','getLoginConfig','login','logout'];
const recordingCommonIds=['getTimelineSegmentsByIds','listRecordingDetectionLabels','listRecordingTags','listRecordings'];
const recordingReplayIds=['getRecording','getRecordingTags','getRecordingThumbnail','getTimelineSegments','playRecording'];
const recordingJobIds=['downloadRecordingExport','getRecordingDeletionProgress','getRecordingExportStatus'];
const recordingActionIds=['protectRecordings','setRecordingProtection','setRecordingRetention','setRecordingTags','updateRecordingTagsBatch','deleteRecording','deleteRecordings','createRecordingExport','downloadRecording'];
async function discovery(server,principal){
 const start=performance.now();
 const response=await server.request('/api/openapi.json',{principal:'anonymous',headers:principal.headers});
 const bytes=Buffer.from(await response.arrayBuffer());const duration=performance.now()-start;
 assert.equal(response.status,200,bytes.toString());assert.equal(response.headers.get('cache-control'),'private, no-store');
 const document=JSON.parse(bytes.toString());
 const operations=Object.values(document.paths).flatMap(item=>Object.values(item).filter(op=>op?.operationId)).map(op=>op.operationId).sort();
 assert.equal(operations.filter(id=>pilotIds.includes(id)).length,principal.expected,`Unexpected pilot operations ${operations}`);
 assert.deepEqual(operations.filter(id=>identityIds.includes(id)),principal.scoped?scopedIdentityIds:identityIds);
 assert.deepEqual(operations.filter(id=>!pilotIds.includes(id)&&!identityIds.includes(id)&&!recordingJobIds.includes(id)&&!recordingActionIds.includes(id)),[...recordingCommonIds,...(principal.replay?recordingReplayIds:[])].sort());
 assert.deepEqual(operations.filter(id=>recordingJobIds.includes(id)||recordingActionIds.includes(id)),[...recordingJobIds,...(principal.recordingActions?recordingActionIds:[])].sort());
 assert.deepEqual([...document['x-lightnvr-coverage'].covered_operations].sort(),[...pilotIds,...identityIds,...recordingCommonIds,...recordingReplayIds,...recordingJobIds,...recordingActionIds].sort());
 return {duration,bytes:bytes.length,operations};
}
async function traceRows(log){return (await readFile(log,'utf8').catch(()=>'' )).trim().split('\n').filter(Boolean).map(line=>JSON.parse(line));}
async function runSize(size,instrumented){
 const label=`fleet-${size}-${instrumented?'sql':'timing'}`;
 const trace=path.join(directory,label+'.jsonl');let executable=binary;
 if(instrumented){
   executable=path.join(directory,label+'.sh');
   await writeFile(executable,`#!/bin/sh\nexport LD_PRELOAD=${shellQuote(path.join(directory,'sqlite-trace.so'))}\nexport LNVR_PROJECTION_SQL_LOG=${shellQuote(trace)}\nexec ${shellQuote(binary)} "$@"\n`);
   await chmod(executable,0o700);
 }
 const server=await startFixture({binary:executable,maxStreams:1024,seed:false,retain:true,startupTimeoutMs:60000});
 const result={size,instrumented,directory:server.directory,pid:server.pid,cameraStorageMinimumBytes:size*(fleetCameraBytes+cameraObservationBytes),runtimeMetricsAllocationBytes:size?1024*streamMetricsBytes:0,principals:{}};
 report.cases.push(result);
 let failed=true;
 try{
   const prepared=await prepare(server,size);result.seedMilliseconds=prepared.seedMilliseconds;result.baselineMemory=await memory(server.pid);
   for(const [name,principal] of Object.entries(prepared.principals)){
     for(let i=0;i<3;i++)await discovery(server,principal);
     const first=await memory(server.pid),raw=[],sql=[];
     for(let i=0;i<(instrumented?5:samples);i++){
       const before=instrumented?(await traceRows(trace)).length:0;
       const sample=await discovery(server,principal);raw.push(sample);
       if(instrumented){const rows=(await traceRows(trace)).slice(before);assert.equal(rows.length,1,'Expected exactly one instrumented snapshot per request');sql.push(rows[0]);}
     }
     const item={operations:raw[0].operations,responseBytes:raw[0].bytes,serial:distribution(raw.map(x=>x.duration)),serialMilliseconds:raw.map(x=>x.duration),memoryBefore:first};
     if(instrumented){
       item.sql=sql;item.sqlStatements=distribution(sql.map(x=>x.statements));item.sqlRows=distribution(sql.map(x=>x.rows));item.sqlVmSteps=distribution(sql.map(x=>x.vmSteps));
     }else{
       const memorySamples=[];let sampling=false;
       const timer=setInterval(async()=>{if(sampling)return;sampling=true;try{memorySamples.push(await memory(server.pid));}finally{sampling=false;}},5);
       const concurrent=[],batchDurations=[];
       try{for(let batch=0;batch<batches;batch++){
         const start=performance.now();const values=await Promise.all(Array.from({length:4},()=>discovery(server,principal)));
         batchDurations.push(performance.now()-start);concurrent.push(...values.map(x=>x.duration));
       }}finally{clearInterval(timer);while(sampling)await new Promise(resolve=>setTimeout(resolve,1));}
       item.concurrent=distribution(concurrent);item.concurrentMilliseconds=concurrent;item.batchMilliseconds=batchDurations;
       item.memorySamples=memorySamples;item.sampledPeakRssKiB=Math.max(first.rssKiB,...memorySamples.map(x=>x.rssKiB));
     }
     item.memoryAfter=await memory(server.pid);result.principals[name]=item;
     await writeFile(path.join(directory,'result.json'),JSON.stringify(report,null,2)+'\n');
   }
   failed=false;
 }finally{await server.stop({failed});}
}
try{
 for(const size of sizes){if(!instrumentOnly)await runSize(size,false);if(!timingOnly)await runSize(size,true);}
 report.status='passed';report.qualification='Bounded Release-build workstation measurements with disabled synthetic cameras, fixed four-worker HTTP pool and declared samples; not embedded-device, active-media or production SLO qualification.';
}catch(error){report.status='failed';report.error=error.stack||String(error);}
report.environment.loadAtEnd=os.loadavg();
await writeFile(path.join(directory,'result.json'),JSON.stringify(report,null,2)+'\n');
console.log(JSON.stringify({status:report.status,artifacts:directory,error:report.error}));
process.exitCode=report.status==='passed'?0:1;
