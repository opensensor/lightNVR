import {createServer,type Server} from 'node:http';
import type {AddressInfo} from 'node:net';

/** Two owned loopback origins; no credentials or payloads leave this process's
 * servers. The source redirects real HTTP requests to the observing sink. */
export interface RedirectFixture {
  fetch:typeof fetch;
  sourceBodies:string[];
  forwardedBodies:string[];
}
async function listen(server:Server):Promise<string>{
  await new Promise<void>((resolve,reject)=>{server.once('error',reject);server.listen(0,'127.0.0.1',resolve);});
  return `http://127.0.0.1:${(server.address() as AddressInfo).port}`;
}
async function close(server:Server):Promise<void>{
  server.closeAllConnections();
  if(server.listening)await new Promise<void>((resolve,reject)=>server.close(error=>error?reject(error):resolve()));
}
export async function withRedirectFixture<T>(status:307|308,run:(fixture:RedirectFixture)=>Promise<T>):Promise<T>{
  const sourceBodies:string[]=[],forwardedBodies:string[]=[];
  const originalFetch=globalThis.fetch;
  const sink=createServer(async(request,response)=>{
    let body='';for await(const chunk of request)body+=String(chunk);forwardedBodies.push(body);
    response.writeHead(200,{'Content-Type':'application/json','Access-Control-Allow-Origin':'*'});
    response.end('{"success":true,"restart_required":false}');
  });
  let source:Server|undefined;
  try{
    const sinkURL=await listen(sink);
    source=createServer(async(request,response)=>{
      let body='';for await(const chunk of request)body+=String(chunk);sourceBodies.push(body);
      response.writeHead(status,{Location:`${sinkURL}/destination`});response.end();
    });
    const sourceURL=await listen(source);
    const fetchOwned:typeof fetch=(input,options)=>{
      if(typeof input!=='string'||!input.startsWith('/'))throw new Error('Redirect fixture requires an application-relative URL');
      return originalFetch(`${sourceURL}${input}`,options);
    };
    return await run({fetch:fetchOwned,sourceBodies,forwardedBodies});
  }finally{if(source)await close(source);await close(sink);}
}
