/** Real component/browser lifecycle; only settings, telemetry and the player API are fixtures. */
const path = require('node:path');
const {build} = require('esbuild');
const {chromium} = require('../../../node_modules/playwright');

let bundle;
beforeAll(async () => {
  const result = await build({
    stdin: {resolveDir: path.resolve(__dirname, '../..'), loader: 'jsx', contents: `
      import {h,render} from 'preact';
      import {QueryClient,QueryClientProvider} from './js/query-client.ts';
      import {HLSVideoCell} from './js/components/preact/HLSVideoCell.tsx';
      window.players=[];window.requests=[];window.failures=[];window.errors=[];
      window.recoveries=[];window.settingsReads=[];window.nativeSources=[];window.cleanups=0;
      window.addEventListener('error',event=>{if(event instanceof ErrorEvent){window.errors.push(event.message);event.preventDefault();}});
      const originalTimeout=window.setTimeout;
      window.setTimeout=(callback,delay,...args)=>{
        if(window.captureRecovery && (delay===1000||delay===2000)) {
          window.recoveries.push(()=>callback(...args));return -window.recoveries.length;
        }
        return originalTimeout(callback,delay,...args);
      };
      Object.defineProperty(HTMLMediaElement.prototype,'src',{
        configurable:true,get(){return this.fixtureSrc||'';},
        set(value){this.fixtureSrc=value;window.nativeSources.push(value);}
      });
      HTMLMediaElement.prototype.canPlayType=()=> 'probably';
      HTMLMediaElement.prototype.play=()=>Promise.resolve();
      HTMLMediaElement.prototype.pause=()=>{};
      HTMLMediaElement.prototype.load=()=>{};
      window.fetch=async(input,options={})=>{
        window.requests.push({url:String(input),signal:options.signal});
        if(window.config.deferManifest) return new Promise((resolve,reject)=>{
          options.signal.addEventListener('abort',()=>{window.fetchAborted=true;reject(new DOMException('Aborted','AbortError'));},{once:true});
        });
        return new Response('#EXTM3U',{status:200});
      };
      const client=new QueryClient({defaultOptions:{queries:{retry:false}}});
      window.mount=()=>render(h(QueryClientProvider,{client},h(HLSVideoCell,{
        stream:{name:'fixture camera',status:'Running',go2rtc_hls_available:true},
        streamId:'fixture',showControls:false,showLabels:false,globalShowDetections:false,
        onToggleFullscreen:()=>{},onTransportFailure:error=>window.failures.push(error)
      })),document.getElementById('root'));
      window.unmount=()=>render(null,document.getElementById('root'));
      window.emitFatal=(type,count=1)=>{
        window.captureRecovery=true;
        try {for(let i=0;i<count;i++) window.players[0].emit('error',{fatal:true,type,details:'fixture fatal'});}
        catch(error){window.errors.push(error.name+': '+error.message);}
        finally{window.captureRecovery=false;}
      };
      window.mount();
    `},
    bundle:true,write:false,format:'iife',jsx:'automatic',jsxImportSource:'preact',
    alias:{react:'preact/compat','react-dom':'preact/compat'},
    plugins:[{name:'fixture',setup(builder){
      builder.onResolve({filter:/^hls\.js$/},args=>({path:args.path,namespace:'fixture'}));
      builder.onResolve({filter:/(?:settings-utils|player-telemetry|i18n)\.ts$/},args=>{
        if(args.path.endsWith('settings-utils.ts')&&!args.importer.endsWith('HLSVideoCell.tsx')) return;
        return {path:args.path,namespace:'fixture'};
      });
      builder.onLoad({filter:/.*/,namespace:'fixture'},args=>({loader:'js',contents:
        args.path==='hls.js' ? `export default class Hls {
          static Events={ERROR:'error',MANIFEST_PARSED:'manifest',FRAG_BUFFERED:'fragment'};
          static ErrorTypes={NETWORK_ERROR:'network',MEDIA_ERROR:'media'};
          static isSupported(){return !window.config.nativePlayer;}
          constructor(){this.handlers={};this.sources=[];this.destroyed=0;this.starts=0;this.mediaRecoveries=0;window.players.push(this);}
          on(name,callback){this.handlers[name]=callback;}
          emit(name,data){this.handlers[name]?.(name,data);}
          loadSource(url){this.sources.push(url);}
          attachMedia(video){this.video=video;}
          destroy(){this.destroyed++;}
          startLoad(){this.starts++;}
          recoverMediaError(){this.mediaRecoveries++;}
        }` : args.path.endsWith('settings-utils.ts') ? `
          export const isForceNativeHls=async()=>{window.settingsReads.push('forceNative');return !!window.config.forceNative;};
          export const isGo2rtcEnabled=async()=>{
            window.settingsReads.push('enabled');
            if(window.config.deferSettings) return new Promise(resolve=>window.resolveSettings=resolve);
            return !!window.config.enabled;
          };
          export const isGo2rtcAvailable=async()=>true;
          export const getGo2rtcBaseUrl=async()=>'/go2rtc';
        ` : args.path.endsWith('player-telemetry.ts') ? `
          export const createPlayerTelemetry=()=>({recordFirstFrame(){},recordRebufferStart(){},recordRebufferEnd(){},destroy(){window.cleanups++;}});
        ` : 'export const t=key=>key;export const useI18n=()=>({t});'
      }));
    }}],
  });
  bundle=result.outputFiles[0].text;
});

async function openPlayer(config={}) {
  const browser=await chromium.launch({headless:true});
  const page=await browser.newPage();
  page.setDefaultTimeout(5000);
  await page.route('http://fixture.invalid/**',route=>route.fulfill({
    contentType:'text/html',body:'<style>svg{width:20px;height:20px}</style><div id="root"></div>',
  }));
  await page.goto('http://fixture.invalid/');
  await page.evaluate(value=>{window.config=value;},config);
  await page.addScriptTag({content:bundle});
  await page.waitForFunction(()=>window.config.deferSettings ? !!window.resolveSettings :
    window.config.deferManifest ? window.requests.length>0 :
    window.config.nativePlayer ? window.nativeSources.length>0 : window.players.length>0);
  return {browser,page};
}

it.each(['exhausted','unrecoverable','native'])(
  '%s recovery falls back when go2rtc was only auto-detected',async branch=>{
    const {browser,page}=await openPlayer({enabled:false,nativePlayer:branch==='native'});
    try {
      if(branch==='native') await page.locator('video').dispatchEvent('error');
      else await page.evaluate(value=>window.emitFatal(value==='exhausted'?'media':'other',value==='exhausted'?4:1),branch);
      expect(await page.evaluate(()=>window.errors)).toEqual([]);
      await page.waitForFunction(()=>window.requests.some(r=>r.url==='/hls/fixture%20camera/index.m3u8'));
      expect(await page.evaluate(()=>window.failures)).toEqual([]);
      if(branch!=='native') expect(await page.evaluate(()=>window.players[0].destroyed)).toBe(1);
    } finally {await browser.close();}
  },
);

it.each(['exhausted','unrecoverable','native'])(
  '%s recovery reports failure without fallback when go2rtc is enabled',async branch=>{
    const {browser,page}=await openPlayer({enabled:true,nativePlayer:branch==='native'});
    try {
      if(branch==='native') await page.locator('video').dispatchEvent('error');
      else await page.evaluate(value=>window.emitFatal(value==='exhausted'?'media':'other',value==='exhausted'?4:1),branch);
      expect(await page.evaluate(()=>window.errors)).toEqual([]);
      await page.waitForFunction(()=>window.failures.length>0);
      expect(await page.evaluate(()=>window.requests.map(r=>r.url))).toEqual(['/go2rtc/api/stream.m3u8?src=fixture%20camera&mp4=flac']);
      expect(await page.evaluate(()=>window.failures[0])).toBe(branch==='unrecoverable'?'fixture fatal':'live.go2rtcHlsFailedRetry');
    } finally {await browser.close();}
  },
);

it('retries the go2rtc master manifest and prevents pending recovery after unmount',async()=>{
  const {browser,page}=await openPlayer({enabled:true});
  try {
    await page.evaluate(()=>{window.emitFatal('network');window.recoveries.shift()();});
    expect(await page.evaluate(()=>({starts:window.players[0].starts,sources:window.players[0].sources})))
      .toEqual({starts:1,sources:Array(2).fill('/go2rtc/api/stream.m3u8?src=fixture%20camera&mp4=flac')});
    await page.evaluate(()=>{window.emitFatal('network');window.emitFatal('media');window.unmount();});
    // Preact schedules passive effect cleanup after paint; observe its completion.
    await page.waitForFunction(()=>window.cleanups>0);
    await page.evaluate(()=>{window.recoveries.splice(0).forEach(fn=>fn());
      window.players[0].emit('error',{fatal:true,type:'other'});});
    expect(await page.evaluate(()=>({starts:window.players[0].starts,media:window.players[0].mediaRecoveries,
      destroyed:window.players[0].destroyed,errors:window.errors})))
      .toEqual({starts:1,media:0,destroyed:1,errors:[]});
  } finally {await browser.close();}
});

it('does not initialize after unmount while runtime settings are unresolved',async()=>{
  const {browser,page}=await openPlayer({enabled:true,deferSettings:true});
  try {
    await page.evaluate(()=>window.unmount());
    await page.waitForFunction(()=>window.cleanups>0);
    await page.evaluate(async()=>{window.resolveSettings(true);await Promise.resolve();await Promise.resolve();});
    expect(await page.evaluate(()=>({players:window.players.length,requests:window.requests.length,errors:window.errors})))
      .toEqual({players:0,requests:0,errors:[]});
  } finally {await browser.close();}
});

it('aborts an in-flight manifest preflight on unmount without creating a player',async()=>{
  const {browser,page}=await openPlayer({enabled:true,deferManifest:true});
  try {
    await page.evaluate(()=>window.unmount());
    await page.waitForFunction(()=>window.cleanups>0&&window.fetchAborted);
    expect(await page.evaluate(()=>({players:window.players.length,aborted:window.requests[0].signal.aborted,errors:window.errors})))
      .toEqual({players:0,aborted:true,errors:[]});
  } finally {await browser.close();}
});

it('force-native preference bypasses go2rtc settings and uses native recovery',async()=>{
  const {browser,page}=await openPlayer({enabled:true,forceNative:true});
  try {
    await page.waitForFunction(()=>window.players.some(p=>!p.destroyed));
    expect(await page.evaluate(()=>window.requests.every(r=>r.url==='/hls/fixture%20camera/index.m3u8'))).toBe(true);
    expect(await page.evaluate(()=>window.settingsReads.includes('enabled'))).toBe(false);
    await page.evaluate(()=>window.players.find(p=>!p.destroyed).emit('error',{fatal:true,type:'other',details:'native fatal'}));
    await page.waitForFunction(()=>window.failures.includes('native fatal'));
    expect(await page.evaluate(()=>window.errors)).toEqual([]);
  } finally {await browser.close();}
});
