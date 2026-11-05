import {execFileSync} from 'node:child_process';
import {copyFileSync,mkdirSync,readFileSync,writeFileSync} from 'node:fs';
import path from 'node:path';
import {DatabaseSync} from 'node:sqlite';
import type {Server} from '../../permissionCases';

export const recordingIds={playable:101,sibling:102,interrupted:103,progress:104,hidden:201,orphan:301} as const;
export interface RecordingFixture {directory:string;source:string;thumbnail:string;bytes:Buffer;start:number;end:number;probe:unknown}
/** Real synthetic media plus controlled fixture-only database state. Never changes auth. */
export function seedRecordings(server:Server):RecordingFixture {
  if(!/^lightnvr-contract-/.test(path.basename(server.directory)))throw new Error('Recording seed requires an isolated contract fixture');
  const directory=path.join(server.directory,'recordings');mkdirSync(directory,{recursive:true});
  const source=path.join(directory,'contract-source.mp4');
  execFileSync('ffmpeg',['-nostdin','-hide_banner','-loglevel','error','-f','lavfi','-i','testsrc2=size=96x64:rate=10','-t','2','-an','-c:v','libx264','-threads','1','-pix_fmt','yuv420p','-movflags','+faststart','-y',source],{timeout:20000});
  const probe=JSON.parse(execFileSync('ffprobe',['-v','error','-show_entries','format=duration,size:stream=codec_name,codec_type,width,height','-of','json',source],{encoding:'utf8',timeout:10000}));
  execFileSync('ffmpeg',['-nostdin','-hide_banner','-loglevel','error','-i',source,'-f','null','-'],{timeout:10000});
  const thumbdir=path.join(directory,'thumbnails');mkdirSync(thumbdir,{recursive:true});
  const thumbnail=path.join(thumbdir,`${recordingIds.playable}_0.jpg`);
  execFileSync('ffmpeg',['-nostdin','-hide_banner','-loglevel','error','-i',source,'-frames:v','1','-threads','1','-y',thumbnail],{timeout:10000});
  const bytes=readFileSync(source);const start=Math.floor(Date.now()/1000)-20;const end=start+2;
  const db=new DatabaseSync(path.join(server.directory,'lightnvr.db'));
  try {
    db.exec('PRAGMA busy_timeout=3000; PRAGMA foreign_keys=ON; BEGIN IMMEDIATE');
    const insert=db.prepare('INSERT INTO recordings (id,stream_name,camera_uuid,file_path,start_time,end_time,size_bytes,width,height,fps,codec,is_complete,trigger_type,schedule_restricted,playback_issue) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)');
    for(const [key,id] of Object.entries(recordingIds)) {
      const stream=key==='hidden'?'B':key==='orphan'?'Deleted-camera':'A';
      const uuid=key==='orphan'?'00000000-0000-4000-8000-000000000099':server.cameras[stream].camera_uuid;
      const file=path.join(directory,`${id}.mp4`);copyFileSync(source,file);
      if(key==='interrupted')writeFileSync(file,'synthetic interrupted media: missing moov');
      insert.run(id,stream,uuid,file,start,key==='progress'?0:end,key==='interrupted'?40:bytes.length,96,64,10,'h264',key==='progress'?0:1,'continuous',0,key==='interrupted'?'invalid_media':'');
    }
    db.prepare('INSERT INTO recording_tags(recording_id,tag) VALUES(?,?)').run(recordingIds.playable,'visible-tag');
    db.prepare('INSERT INTO recording_tags(recording_id,tag) VALUES(?,?)').run(recordingIds.hidden,'hidden-tag');
    const detection=db.prepare('INSERT INTO detections(stream_name,camera_uuid,timestamp,label,confidence,x,y,width,height,recording_id) VALUES(?,?,?,?,0.9,0,0,1,1,?)');
    detection.run('A',server.cameras.A.camera_uuid,start+1,'visible-person',recordingIds.playable);
    detection.run('B',server.cameras.B.camera_uuid,start+1,'hidden-vehicle',recordingIds.hidden);
    db.exec('COMMIT');
  } finally {db.close();}
  return {directory,source,thumbnail,bytes,start,end,probe};
}
/** Model a historical camera name now reused by another camera, without changing identity. */
export function seedHistoricalNameReuse(server:Server,fixture:RecordingFixture):{hidden:number;visible:number} {
  const db=new DatabaseSync(path.join(server.directory,'lightnvr.db'));
  try {
    const insert=db.prepare('INSERT INTO recordings(id,stream_name,camera_uuid,file_path,start_time,end_time,size_bytes,is_complete,trigger_type) VALUES(?,?,?,?,?,?,?,1,?)');
    insert.run(401,'A',server.cameras.B.camera_uuid,fixture.source,fixture.start,fixture.end,fixture.bytes.length,'continuous');
    insert.run(402,'B',server.cameras.A.camera_uuid,fixture.source,fixture.start,fixture.end,fixture.bytes.length,'continuous');
    db.prepare('INSERT INTO recording_tags(recording_id,tag) VALUES(401,?)').run('historical-hidden-tag');
    db.prepare('INSERT INTO recording_tags(recording_id,tag) VALUES(402,?)').run('historical-visible-tag');
    const det=db.prepare('INSERT INTO detections(stream_name,camera_uuid,timestamp,label,confidence,recording_id) VALUES(?,?,?,?,0.9,?)');
    det.run('A',server.cameras.B.camera_uuid,fixture.start+1,'historical-hidden-label',401);
    det.run('B',server.cameras.A.camera_uuid,fixture.start+1,'historical-visible-label',402);
  } finally {db.close();}
  return {hidden:401,visible:402};
}

/** Legacy null/empty capture UUIDs keep name-based authority; missing names fail404. */
export function seedLegacyCaptureRows(server:Server,fixture:RecordingFixture):void {
  const db=new DatabaseSync(path.join(server.directory,'lightnvr.db'));
  try {
    const insert=db.prepare('INSERT INTO recordings(id,stream_name,camera_uuid,file_path,start_time,end_time,size_bytes,is_complete) VALUES(?,?,?,?,?,?,?,1)');
    insert.run(501,'A',null,fixture.source,fixture.start,fixture.end,fixture.bytes.length);
    insert.run(502,'A','',fixture.source,fixture.start,fixture.end,fixture.bytes.length);
    insert.run(503,'Deleted-camera',null,fixture.source,fixture.start,fixture.end,fixture.bytes.length);
  } finally {db.close();}
}
