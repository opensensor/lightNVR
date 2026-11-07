#!/usr/bin/env node
/** Bounded synthetic active-recording measurements on an owned loopback fixture. */
import {startFixture} from '../contracts/fixture-server.mjs';
import {trackedDiffSha256} from './git-provenance.mjs';
import {createServer} from 'node:http';
import {spawn,execFileSync} from 'node:child_process';
import {mkdir,readFile,readdir,writeFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import path from 'node:path';
import os from 'node:os';
const args=process.argv.slice(2),option=(name,fallback)=>{const at=args.indexOf(name);return at<0?fallback:args[at+1];};
const binary=path.resolve(option('--binary',process.env.LIGHTNVR_CONTRACT_BIN || 'build/eng-api-T2/bin/lightnvr'));
const directory=path.resolve(option('--output','build/active-recording'));
const count=Number(option('--streams','8')),seconds=Number(option('--seconds','60'));
if(!Number.isInteger(count)||count<1||count>32||!Number.isInteger(seconds)||seconds<10||seconds>3600)throw new Error('streams 1–32; seconds 10–3600');
await mkdir(directory,{recursive:false});
const git=(...argv)=>execFileSync('git',argv,{encoding:'utf8'}).trim();
const hash=value=>createHash('sha256').update(value).digest('hex');
const report={revision:git('rev-parse','HEAD'),trackedDiffSha256:await trackedDiffSha256(),binarySha256:hash(await readFile(binary)),
 cpu:os.cpus()[0]?.model,cores:os.cpus().length,ramBytes:os.totalmem(),os:os.release(),
 workload:{streams:count,durationSeconds:seconds,codec:'H.264',width:1920,height:1080,fps:15,gop:15,source:'owned loopback HTTP MPEG-TS',viewers:0},
 limitation:'Workstation synthetic recording only. Source generation shares the host. No physical cameras, active live viewers, remote storage, sustainable-capacity or power-loss claim.',samples:[],recordings:[],status:'running'};
const source=path.join(directory,'source.ts');
execFileSync('ffmpeg',['-nostdin','-hide_banner','-loglevel','error','-f','lavfi','-i','testsrc2=size=1920x1080:rate=15','-t','5','-an','-c:v','libx264','-threads','2','-preset','ultrafast','-pix_fmt','yuv420p','-g','15','-f','mpegts',source],{timeout:30000});
report.sourceSha256=hash(await readFile(source));
report.harnessSha256=hash(await readFile(new URL(import.meta.url)));
report.nativeSourceSha256=hash(await readFile('src/video/mp4_segment_recorder.c'));
const buildDirectory=path.dirname(path.dirname(binary));
for(const [key,relative] of [['cmakeCache','CMakeCache.txt'],['compileFlags','CMakeFiles/lightnvr.dir/flags.make']]) {
 try { const receipt=await readFile(path.join(buildDirectory,relative),'utf8');
  report[key+'Sha256']=hash(receipt);
  if(key==='cmakeCache')report.buildType=receipt.match(/^CMAKE_BUILD_TYPE:STRING=(.*)$/m)?.[1];
  if(key==='compileFlags')report.compilerFlags=receipt.match(/^C_FLAGS = (.*)$/m)?.[1];
 } catch { report[key+'Unavailable']=true; }
}
const children=new Set(),childExits=[];
const http=createServer((req,res)=>{
 res.writeHead(200,{'Content-Type':'video/mp2t'});
 const child=spawn('ffmpeg',['-nostdin','-hide_banner','-loglevel','error','-re','-stream_loop','-1','-i',source,'-c','copy','-f','mpegts','pipe:1'],{stdio:['ignore','pipe','pipe']});
 children.add(child);childExits.push(new Promise(resolve=>child.once('close',()=>{children.delete(child);resolve();})));
 child.stdout.pipe(res);let error='';child.stderr.on('data',chunk=>{error+=chunk;});
 child.once('error',error=>res.destroy(error));
 child.once('exit',code=>{if(code)report.sourceError=error;});
 res.on('close',()=>child.kill('SIGTERM'));
});
await new Promise(resolve=>http.listen(0,'127.0.0.1',resolve));
let server;
try { server=await startFixture({binary,maxStreams:count,seed:false,retain:true}); }
catch(error) { http.closeAllConnections();await new Promise(resolve=>http.close(resolve));throw error; }
report.fixtureDirectory=server.directory;
const save=()=>writeFile(path.join(directory,'results.json'),JSON.stringify(report,null,2)+'\n');
try {
 await server.json('/api/setup/status',{method:'POST',body:{complete:true}});
 for(let i=0;i<count;i++){
  const name=`Load${i}`;
  await server.json('/api/streams',{method:'POST',body:{name,url:`http://127.0.0.1:${http.address().port}/${i}.ts`,enabled:false,streaming_enabled:false,record:false,segment_duration:60}});
  await server.json(`/api/streams/${name}/recording`,{method:'POST',body:{action:'start'}});
 }
 const started=performance.now();
 while(performance.now()-started<seconds*1000){
  const requestStart=performance.now();
  const rows=await server.json('/api/recordings');
  const latencyMs=performance.now()-requestStart;
  const state=await readFile(`/proc/${server.pid}/status`,'utf8');
  const stat=await readFile(`/proc/${server.pid}/stat`,'utf8');
  const fields=stat.slice(stat.lastIndexOf(')')+2).split(' ');
  report.samples.push({elapsedMs:performance.now()-started,listLatencyMs:latencyMs,listed:rows.recordings.length,
   rssKiB:Number(state.match(/^VmRSS:\s+(\d+)/m)?.[1]),peakRssKiB:Number(state.match(/^VmHWM:\s+(\d+)/m)?.[1]),
   descriptors:(await readdir(`/proc/${server.pid}/fd`)).length,cpuTicks:Number(fields[11])+Number(fields[12])});
  await save();await new Promise(resolve=>setTimeout(resolve,1000));
 }
 for(let i=0;i<count;i++)await server.json(`/api/streams/Load${i}/recording`,{method:'POST',body:{action:'stop'}});
 const rows=(await server.json('/api/recordings?limit=1000')).recordings;
 const durationToleranceSeconds=5;
 const minExpectedDurationSeconds=Math.max(0,seconds-durationToleranceSeconds);
 for(const row of rows){
  const detail=await server.json(`/api/recordings/${row.id}`);
  const filename=detail.file_path||detail.recording?.file_path;
  if(!filename?.startsWith(server.directory+'/'))throw new Error('Media outside owned fixture');
  const probe=JSON.parse(execFileSync('ffprobe',['-v','error','-show_entries','format=duration,size:stream=codec_name,width,height','-of','json',filename],{encoding:'utf8',timeout:30000}));
  const durationSeconds=Number(probe?.format?.duration);
  if(!Number.isFinite(durationSeconds))throw new Error(`Missing recording duration for ${filename}`);
  if(durationSeconds<minExpectedDurationSeconds)throw new Error(`Recording shorter than expected for ${row.stream}: ${durationSeconds}s < ${minExpectedDurationSeconds}s`);
  execFileSync('ffmpeg',['-nostdin','-hide_banner','-loglevel','error','-xerror','-i',filename,'-f','null','-'],{timeout:60000});
  report.recordings.push({id:row.id,stream:row.stream,probe});
 }
 if(new Set(report.recordings.map(row=>row.stream)).size!==count)throw new Error('Missing finalized camera footage');
 if(report.sourceError)throw new Error(`Source process failed: ${report.sourceError}`);
 report.status='passed';
}catch(error){report.status='failed';report.error=String(error);process.exitCode=1;}
finally{
 try{await server.stop({failed:report.status!=='passed'});}catch(error){report.status='failed';report.cleanupError=String(error);process.exitCode=1;}
 http.closeAllConnections();await new Promise(resolve=>http.close(resolve));
 for(const child of children)child.kill('SIGTERM');
 const killTimer=setTimeout(()=>{for(const child of children)child.kill('SIGKILL');},2000);
 await Promise.all(childExits);clearTimeout(killTimer);
 await save();
}
console.log(JSON.stringify({status:report.status,output:directory,error:report.error}));
