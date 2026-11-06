/** Only isolated owned contract databases; no production credentials. */
import {randomBytes} from 'node:crypto';
import path from 'node:path';
import {DatabaseSync} from 'node:sqlite';
import type {Server} from './permissionCases';

export function seedCredentials(server:Server,owner:number,count=1,activity=Math.floor(Date.now()/1000)) {
  const db=new DatabaseSync(path.join(server.directory,'lightnvr.db'));
  const records:{session:number;sessionToken:string;device:number;deviceToken:string}[]=[];
  try {
    db.exec('PRAGMA busy_timeout=3000');
    for(let i=0;i<count;i++) {
      const sessionToken=randomBytes(16).toString('hex'), deviceToken=randomBytes(16).toString('hex');
      const session=Number(db.prepare('INSERT INTO sessions(user_id,token,created_at,last_activity_at,idle_expires_at,expires_at,ip_address,user_agent) VALUES(?,?,?,?,?,?,?,?)').run(owner,sessionToken,activity,activity,activity+7200,activity+14400,'127.0.0.1','fixture').lastInsertRowid);
      const device=Number(db.prepare('INSERT INTO trusted_devices(user_id,token,created_at,last_used_at,expires_at,ip_address,user_agent) VALUES(?,?,?,?,?,?,?)').run(owner,deviceToken,activity,activity,activity+14400,'127.0.0.1','fixture').lastInsertRowid);
      records.push({session,sessionToken,device,deviceToken});
    }
    return records;
  } finally { db.close(); }
}
export function credentialInventory(server:Server) {
  const db=new DatabaseSync(path.join(server.directory,'lightnvr.db'));
  try {return {sessions:db.prepare('SELECT id,user_id,token,mfa_pending FROM sessions ORDER BY id').all(),devices:db.prepare('SELECT id,user_id,token FROM trusted_devices ORDER BY id').all()};}finally{db.close();}
}
