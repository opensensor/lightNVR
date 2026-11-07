/** Validate footage across segment boundaries, including the short final segment. */
export function validateRecordingDurations(recordings, streamCount, seconds, toleranceSeconds=5) {
 const totals=new Map(Array.from({length:streamCount},(_,i)=>[`Load${i}`,0]));
 for(const row of recordings){
  if(!totals.has(row.stream))throw new Error(`Unexpected recording stream: ${row.stream}`);
  const duration=Number(row.probe?.format?.duration);
  if(!Number.isFinite(duration)||duration<=0)throw new Error(`Missing or invalid recording duration for ${row.stream}`);
  totals.set(row.stream,totals.get(row.stream)+duration);
 }
 const minimum=Math.max(0,seconds-toleranceSeconds);
 for(const [stream,duration] of totals){
  if(duration===0)throw new Error(`Missing finalized camera footage for ${stream}`);
  if(duration<minimum)throw new Error(`Recording shorter than expected for ${stream}: ${duration}s < ${minimum}s`);
 }
 return Object.fromEntries(totals);
}

/** Pipe until stdout ends; process exit can precede the final buffered bytes. */
export function pipeRecordingSource(child,res,onFailure) {
 let stderr='',clientStopped=false;
 child.stderr.on('data',chunk=>{stderr=(stderr+chunk).slice(-16384);});
 child.once('error',error=>{onFailure(String(error));res.destroy(error);});
 child.once('close',(code,signal)=>{
  if(!clientStopped)onFailure(`Source exited unexpectedly (code ${code}, signal ${signal}): ${stderr}`);
 });
 res.once('close',()=>{
  if(!res.writableEnded){clientStopped=true;child.kill('SIGTERM');}
 });
 child.stdout.pipe(res);
}
