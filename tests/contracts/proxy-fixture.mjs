/** Owned loopback proxy for deployment-prefix tests. No host-wide process cleanup. */
import http from 'node:http';
import {mkdtemp,writeFile,rm} from 'node:fs/promises';
import {tmpdir} from 'node:os';
import path from 'node:path';
export async function startProxy(target, {prefix='/nvr/'}={}) {
  const upstream=new URL(target);
  if(upstream.protocol!=='http:' || !['127.0.0.1','localhost','[::1]'].includes(upstream.hostname)) throw new Error('Fixture proxy requires loopback HTTP upstream');
  if(!/^\/[a-zA-Z0-9_-]+\/$/.test(prefix)) throw new Error('Fixture prefix must be one directory');
  const directory=await mkdtemp(path.join(tmpdir(),'lightnvr-contract-proxy-'));
  const requests=[],sockets=new Set(),pending=new Set();let stopped=false;
  const server=http.createServer((request,response)=>{
    const url=request.url||'/';
    const entry={method:request.method||'GET',url,status:0};requests.push(entry);
    if(!url.startsWith(prefix)){entry.status=404;response.writeHead(404);response.end();return;}
    const forwarded=http.request({hostname:upstream.hostname,port:upstream.port,method:request.method,path:url.slice(prefix.length-1),headers:{...request.headers,host:upstream.host}},incoming=>{
      const headers={...incoming.headers};
      if(headers.location?.startsWith('/')) headers.location=prefix.slice(0,-1)+headers.location;
      if(headers['set-cookie']) headers['set-cookie']=headers['set-cookie'].map(cookie=>cookie.replace(/Path=\//gi,`Path=${prefix}`));
      entry.status=incoming.statusCode||502;response.writeHead(entry.status,headers);incoming.pipe(response);
    });
    pending.add(forwarded);forwarded.on('close',()=>pending.delete(forwarded));
    forwarded.on('error',()=>{entry.status=502;if(!response.headersSent)response.writeHead(502,{'content-type':'application/json'});response.end('{"error":"Fixture upstream unavailable"}');});
    request.pipe(forwarded);
    response.on('close',()=>{if(!response.writableFinished)forwarded.destroy();});
  });
  server.on('connection',socket=>{sockets.add(socket);socket.on('close',()=>sockets.delete(socket));});
  try {await new Promise((resolve,reject)=>{server.once('error',reject);server.listen(0,'127.0.0.1',resolve);});}
  catch(error){await rm(directory,{recursive:true,force:true});throw error;}
  return {
    directory,requests,baseURL:`http://127.0.0.1:${server.address().port}${prefix.slice(0,-1)}`,
    async stop({failed=false}={}) {
      if(stopped)return;stopped=true;
      for(const request of pending)request.destroy();for(const socket of sockets)socket.destroy();
      await new Promise((resolve,reject)=>server.close(error=>error?reject(error):resolve()));
      if(failed){await writeFile(path.join(directory,'requests.json'),JSON.stringify(requests,null,2));console.error(`Contract proxy retained: ${directory}`);}
      else await rm(directory,{recursive:true,force:true});
    },
  };
}
