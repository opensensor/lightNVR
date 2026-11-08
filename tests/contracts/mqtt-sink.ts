/** Minimal owned MQTT 3.1.1 sink: observes real PUBLISH bytes, controls PUBACK. */
import net from 'node:net';
/** port 0 = ephemeral; pass a reserved port to bring a "down" broker up later. */
export async function startMqttSink(port = 0) {
 const sockets=new Set<net.Socket>();
 const publications:{topic:string;payload:Buffer;qos:number}[]=[];
 let acknowledge=true;
 const server=net.createServer(socket=>{
  sockets.add(socket);socket.on('close',()=>sockets.delete(socket));socket.on('error',()=>{});
  let buffered=Buffer.alloc(0);
  socket.on('data',chunk=>{
   buffered=Buffer.concat([buffered,chunk]);
   while(buffered.length>=2){
    let remaining=0,multiplier=1,end=1;
    for(;end<buffered.length&&end<=4;end++){
     const byte=buffered[end];remaining+=(byte&127)*multiplier;multiplier*=128;
     if(!(byte&128)){end++;break;}
    }
    if(remaining>1024*1024||end>5){socket.destroy();return;}
    if(end>buffered.length||buffered[end-1]&128||buffered.length<end+remaining)return;
    const header=buffered[0],body=buffered.subarray(end,end+remaining);buffered=buffered.subarray(end+remaining);
    switch(header>>4){
     case 1:socket.write(Buffer.from([0x20,2,0,0]));break;
     case 3:{
      if(body.length<2){socket.destroy();return;}
      const length=body.readUInt16BE(0),qos=(header>>1)&3;
      if(body.length<2+length+(qos?2:0)){socket.destroy();return;}
      const topic=body.subarray(2,2+length).toString();
      const payload=Buffer.from(body.subarray(2+length+(qos?2:0)));
      publications.push({topic,payload,qos});
      if(qos===1&&acknowledge)socket.write(Buffer.from([0x40,2,body[2+length],body[3+length]]));
      break;
     }
     case 12:socket.write(Buffer.from([0xd0,0]));break;
     case 14:socket.end();break;
     default:socket.destroy();
    }
   }
  });
 });
 await new Promise<void>((resolve,reject)=>{server.once('error',reject);server.listen(port,'127.0.0.1',resolve);});
 return {port:(server.address() as net.AddressInfo).port,publications,
  acknowledge(value:boolean){acknowledge=value;},
  disconnect(){for(const socket of sockets)socket.destroy();},
  async stop(){for(const socket of sockets)socket.destroy();await new Promise<void>(resolve=>server.close(()=>resolve()));},
 };
}
