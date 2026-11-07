import test from 'node:test';
import assert from 'node:assert/strict';
import {EventEmitter,once} from 'node:events';
import {PassThrough} from 'node:stream';
import {pipeRecordingSource,validateRecordingDurations} from '../../performance/active-recording-utils.mjs';

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
