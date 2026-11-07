/** Owned production-build measurements, not a physical phone or live-camera qualification. */
import {chromium, type Browser} from 'playwright';
import {startFixture} from '../contracts/fixture-server.mjs';
import {seedRecordings} from '../contracts/fixtures/recordings/seed.ts';
import {mkdir,readFile,readdir,writeFile} from 'node:fs/promises';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import os from 'node:os';
import path from 'node:path';
const args=process.argv.slice(2);
const option=(name:string,fallback:string)=>{const at=args.indexOf(name);return at<0?fallback:args[at+1]!;};
const binary=path.resolve(option('--binary','build/mobile-review/bin/lightnvr'));
const output=path.resolve(option('--output','build/browser-startup'));
const samples=Number(option('--samples','3'));
if(!Number.isInteger(samples)||samples<1||samples>20)throw new Error('samples must be 1–20');
await mkdir(output,{recursive:false});
const hash=(value:string|Buffer)=>createHash('sha256').update(value).digest('hex');
const git=(...argv:string[])=>execFileSync('git',argv,{encoding:'utf8'}).trim();
const report:any={revision:git('rev-parse','HEAD'),trackedDiffSha256:hash(git('diff','HEAD')),
 binarySha256:hash(await readFile(binary)),binary,go2rtc:git('ls-tree','HEAD','go2rtc'),
 machine:{cpu:os.cpus()[0]?.model,cores:os.cpus().length,ramBytes:os.totalmem(),os:os.release(),arch:os.arch(),node:process.version},
 methodology:'Production assets and native loopback fixture. Authenticated cold and warm HTTP-cache navigations; no request interception. Three samples by default. Phone profile is Chromium touch emulation with 4x CPU throttling, not phone hardware. First-frame observations use a generated 96x64 H.264 recording, not a live camera. Browser script/task duration covers the whole page, not validator-only CPU attribution.',
 limitations:['No live transport negotiation or physical-device/network timing','No active-camera capacity claim','Loopback storage, native host and browser share this workstation'],samples:[]};
const assetHashes:Record<string,string>={};
async function hashTree(directory:string) {
 for(const entry of await readdir(directory,{withFileTypes:true})) {
  const file=path.join(directory,entry.name);
  if(entry.isDirectory())await hashTree(file);else assetHashes[file]=hash(await readFile(file));
 }
}
await hashTree('web/dist');
report.assets=assetHashes;
report.harnessSha256=hash(await readFile(new URL(import.meta.url)));
report.status='running';
let failed=false;
const server=await startFixture({binary,retain:true});
let browser:Browser|undefined;
report.fixtureDirectory=server.directory;
try {
 browser=await chromium.launch({headless:true});
 report.browser=browser.version();
 seedRecordings(server);
 const pages=[['index.html','#live-transport-options'],['recordings.html?view=grid','.recording-card'],['settings.html','#settings-page'],['api-docs.html','#api-reference[aria-busy="false"] .swagger-ui']] as const;
 for(const profile of ['desktop','phone-emulated'])for(let sample=0;sample<samples;sample++)for(const [url,selector] of pages){
  const context=await server.browserContext(browser);
  const page=await context.newPage();page.setDefaultTimeout(20000);
  const cdp=await context.newCDPSession(page);
  await cdp.send('Performance.enable');
  if(profile==='phone-emulated'){
   await page.setViewportSize({width:390,height:844});
   await cdp.send('Emulation.setCPUThrottlingRate',{rate:4});
   await cdp.send('Emulation.setTouchEmulationEnabled',{enabled:true});
  } else await page.setViewportSize({width:1440,height:900});
  try{
   await cdp.send('Network.clearBrowserCache');
   for(const cache of ['cold','warm']){
    await page.goto(`${server.baseURL}/${url}`,{waitUntil:'domcontentloaded'});
    await page.locator(url === 'index.html' && profile === 'phone-emulated' ? '.live-mobile-options' : selector).first().waitFor({state:'visible'});
    const controlsMs=await page.evaluate(()=>performance.now());
    let firstFrameMs:number|null=null;
    if(url.startsWith('recordings')){
     await page.locator('.recording-card').getByRole('button',{name:'Play',exact:true}).filter({visible:true}).first().click();
     const video=page.locator('#video-preview-modal video');
     await page.waitForFunction(()=>{const v=document.querySelector('#video-preview-modal video') as HTMLVideoElement|null;return v && v.readyState>=2;});
     firstFrameMs=await video.evaluate(async node=>{
      const player=node as HTMLVideoElement;
      const decoded=new Promise<number>((resolve,reject)=>{
       const timer=setTimeout(()=>reject(new Error('No decoded frame in 15 seconds')),15000);
       player.requestVideoFrameCallback(()=>{clearTimeout(timer);resolve(performance.now());});
      });
      await player.play();return decoded;
     });
    }
    const resources=await page.evaluate(()=>performance.getEntriesByType('resource').map(raw=>{
     const e=raw as PerformanceResourceTiming;return {path:new URL(e.name).pathname,transferBytes:e.transferSize,decodedBytes:e.decodedBodySize,durationMs:e.duration};
    }));
    const after=await cdp.send('Performance.getMetrics');
    // Chromium resets these document counters on navigation. Subtracting the
    // previous page would produce invalid negative warm-load durations.
    const metric=(name:string)=>after.metrics.find(m=>m.name===name)?.value||0;
    report.samples.push({profile,sample,cache,page:url,controlsMs,firstFrameMs,
     scriptMs:1000*metric('ScriptDuration'),
     taskMs:1000*metric('TaskDuration'),
     heapBytes:metric('JSHeapUsedSize'),resources});
    await writeFile(path.join(output,'results.json'),JSON.stringify(report,null,2)+'\n');
   }
  }finally{await context.close();}
 }
 report.status='passed';
}catch(error){failed=true;report.status='failed';report.error=String(error);throw error;}
finally{
 await browser?.close();
 try { await server.stop({failed}); }
 catch(error) { report.status='failed';report.cleanupError=String(error);process.exitCode=1; }
 await writeFile(path.join(output,'results.json'),JSON.stringify(report,null,2)+'\n');
}
console.log(`Measured ${report.samples.length} navigations: ${output}/results.json`);
