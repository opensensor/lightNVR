import test from 'node:test';
import assert from 'node:assert/strict';
import {EventEmitter,once} from 'node:events';
import {PassThrough} from 'node:stream';
import {listAllRecordings,pipeRecordingSource,validateRecordingDurations} from '../../performance/active-recording-utils.mjs';

function recordingCatalog(count){
 const recordings=Array.from({length:count},(_,i)=>({id:i+1,stream:'Load0'})),requests=[];
 return {recordings,requests,async json(route){
  const params=new URL(route,'http://fixture.test').searchParams;
  assert.equal(params.get('sort'),'id');assert.equal(params.get('order'),'asc');
  assert.equal(params.has('offset'),false);
  const page=Number(params.get('page')||1),limit=Math.min(Number(params.get('limit')||20),1000);
  requests.push(page);
  assert.ok(requests.length<=Math.floor(count/1000)+1,'pagination must terminate');
  return {recordings:recordings.slice((page-1)*limit,page*limit)};
 }};
}

test('recording pagination retrieves more than 1000 rows exactly once',async()=>{
 const catalog=recordingCatalog(2505);
 assert.deepEqual(await listAllRecordings(catalog),catalog.recordings);
 assert.deepEqual(catalog.requests,[1,2,3]);
});
test('recording pagination terminates for an exact multiple of the page size',async()=>{
 const catalog=recordingCatalog(2000);
 assert.deepEqual(await listAllRecordings(catalog),catalog.recordings);
 assert.deepEqual(catalog.requests,[1,2,3]);
});
test('recording pagination handles an empty catalog',async()=>{
 const catalog=recordingCatalog(0);
 assert.deepEqual(await listAllRecordings(catalog),[]);
 assert.deepEqual(catalog.requests,[1]);
});
test('repeated pages fail instead of accumulating duplicate footage indefinitely',async()=>{
 const recordings=recordingCatalog(1000).recordings;let requests=0;
 await assert.rejects(listAllRecordings({async json(){requests++;return {recordings};}}),/Repeated recording/);
 assert.equal(requests,2);
});
test('missing recordings cannot silently truncate the catalog',async()=>{
 await assert.rejects(listAllRecordings({async json(){return {};}}),/Missing recordings array/);
});

const row=(stream,duration)=>({stream,probe:{format:{duration}}});

test('full and short tail segments count toward each camera duration',()=>{
 assert.deepEqual(validateRecordingDurations([row('Load0','60'),row('Load0','6'),row('Load1','64')],2,65),{Load0:66,Load1:64});
});
test('a long recording for one camera cannot hide another truncated camera',()=>{
 assert.throws(()=>validateRecordingDurations([row('Load0',120),row('Load1',3)],2,60),/shorter than expected for Load1/);
});
test('missing streams and invalid segment durations fail',()=>{
 assert.throws(()=>validateRecordingDurations([row('Load0',60)],2,60),/Missing finalized/);
 for(const value of [undefined,null,'',0,-1,'N/A',Infinity]){
  assert.throws(()=>validateRecordingDurations([row('Load0',value)],1,10),/invalid recording duration/);
 }
 assert.throws(()=>validateRecordingDurations([row('Other',60)],1,60),/Unexpected recording stream/);
});
function source(){
 const child=new EventEmitter();child.stdout=new PassThrough();child.stderr=new PassThrough();
 child.signals=[];child.kill=signal=>child.signals.push(signal);
 const res=new PassThrough(),failures=[],chunks=[];
 res.on('data',chunk=>chunks.push(chunk));res.on('error',()=>{});
 pipeRecordingSource(child,res,error=>failures.push(error));
 return {child,res,failures,chunks};
}
test('process exit does not truncate the final stdout bytes',async()=>{
 const {child,res,chunks,failures}=source();
 child.stdout.write('first');child.emit('exit',0,null);
 assert.equal(res.writableEnded,false);
 const ended=once(res,'end');child.stdout.end('tail');await ended;
 child.emit('close',0,null);
 assert.equal(Buffer.concat(chunks).toString(),'firsttail');
 assert.equal(failures.length,1); // A looping source should not end on its own.
});
test('intentional client disconnect terminates source without failure',async()=>{
 const {child,res,failures}=source();
 const closed=once(res,'close');res.destroy();await closed;
 child.emit('close',null,'SIGTERM');
 assert.deepEqual(child.signals,['SIGTERM']);assert.deepEqual(failures,[]);
});
test('spawn failure and unexpected nonzero exit are recorded',()=>{
 const first=source();first.child.emit('error',new Error('spawn ENOENT'));
 assert.match(first.failures[0],/ENOENT/);
 const second=source();second.child.stderr.write('decoder failed');second.child.emit('close',1,null);
 assert.match(second.failures[0],/decoder failed/);
});
