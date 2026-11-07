import {execFileSync} from 'node:child_process';
import {copyFileSync,mkdirSync,statSync,writeFileSync} from 'node:fs';
import path from 'node:path';
import {DatabaseSync} from 'node:sqlite';
import type {Server} from '../../permissionCases';

/** Owned, playable day with 300 separated recordings, an interrupted file and another principal's camera. */
export function seedBusyDay(server:Server) {
 if(!path.basename(server.directory).startsWith('lightnvr-contract-'))throw new Error('Owned fixture required');
 const directory=path.join(server.directory,'recordings');mkdirSync(directory,{recursive:true});
 const source=path.join(directory,'busy-source.mp4');
 execFileSync('ffmpeg',['-nostdin','-v','error','-f','lavfi','-i','testsrc2=size=96x64:rate=10','-t','30','-an','-c:v','libx264','-threads','1','-g','10','-pix_fmt','yuv420p','-movflags','+faststart',source],{timeout:20000});
 const start=Date.UTC(2026,10,6,8)/1000,bytes=statSync(source).size;
 const db=new DatabaseSync(path.join(server.directory,'lightnvr.db'));
 try {
  db.exec('PRAGMA busy_timeout=3000; BEGIN IMMEDIATE');
  const insert=db.prepare('INSERT INTO recordings (id,stream_name,camera_uuid,file_path,start_time,end_time,size_bytes,width,height,fps,codec,is_complete,trigger_type,schedule_restricted,playback_issue) VALUES (?,?,?,?,?,?,?,?,?,?,?,?,?,?,?)');
  for(let i=0;i<302;i++) {
   const id=1000+i,stream=i===301?'B':'A',file=path.join(directory,`${id}.mp4`),interrupted=i===300;
   if(interrupted)writeFileSync(file,'owned interrupted media');else copyFileSync(source,file);
   const timestamp=start+(i===301?0:i)*120;
   insert.run(id,stream,server.cameras[stream].camera_uuid,file,timestamp,timestamp+30,interrupted?23:bytes,96,64,10,'h264',1,'continuous',0,interrupted?'invalid_media':'');
  }
  db.exec('COMMIT');
 }finally{db.close();}
 return {start,date:'2026-11-06',count:301,first:1000,interrupted:1300};
}
