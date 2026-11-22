/** Real component/browser lifecycle, with a simulated harness for settings/telemetry/player APIs plus fetch, media-element methods, selected timers, and i18n (including recovery timing). */
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
      window.fullscreenToggles=0;
      window.recoveries=[];window.settingsReads=[];window.nativeSources=[];window.cleanups=0;
      window.addEventListener('error',event=>{if(event instanceof ErrorEvent){window.errors.push(event.message);event.preventDefault();}});
      window.addEventListener('unhandledrejection',event=>{
        const reason=event.reason;
        window.errors.push(reason instanceof Error ? reason.message : String(reason));
        event.preventDefault();
      });
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
        if(options.method==='PUT'&&window.config.deferPrivacy) return new Promise(resolve=>{
          window.resolvePrivacyRequest=()=>{window.privacyResolved=true;resolve(new Response('',{status:200}));};
        });
        if(window.config.deferManifest) return new Promise((resolve,reject)=>{
          options.signal.addEventListener('abort',()=>{window.fetchAborted=true;reject(new DOMException('Aborted','AbortError'));},{once:true});
        });
        return new Response('#EXTM3U',{status:200});
      };
      const client=new QueryClient({defaultOptions:{queries:{retry:false}}});
      window.mount=()=>render(h(QueryClientProvider,{client},h(HLSVideoCell,{
        stream:{name:window.config.streamName||'fixture camera',status:'Running',go2rtc_hls_available:true,
          privacy_mode:!!window.config.privacyMode},
        streamId:window.config.streamId||'fixture',showControls:!!window.config.showControls,showLabels:false,globalShowDetections:false,
        onToggleFullscreen:()=>{window.fullscreenToggles++;},onTransportFailure:error=>window.failures.push(error)
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
          static ErrorDetails={MANIFEST_LOAD_ERROR:'manifestLoadError',MANIFEST_LOAD_TIMEOUT:'manifestLoadTimeOut'};
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
          export const isForceNativeHls=async()=>{window.settingsReads.push('forceNative');if(window.config.rejectForceNative)throw new Error('settings unavailable');return !!window.config.forceNative;};
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
  let browser;
  let page;
  try {
    browser=await chromium.launch({headless:true});
    page=await browser.newPage();
    page.setDefaultTimeout(5000);
    await page.route('http://fixture.invalid/**',route=>route.fulfill({
      contentType:'text/html',body:'<style>svg{width:20px;height:20px}</style><div id="root"></div>',
    }));
    await page.goto('http://fixture.invalid/');
    await page.evaluate(value=>{window.config=value;},config);
    await page.addScriptTag({content:bundle});
    await page.waitForFunction(()=>window.config.expectInitFailure ? window.failures.length>0 :
      window.config.privacyMode ?
      !!document.querySelector('button[aria-label="live.resumeStream"]') :
      window.config.deferSettings ? !!window.resolveSettings :
      window.config.deferManifest ? window.requests.length>0 :
      window.config.nativePlayer ? window.nativeSources.length>0 : window.players.length>0);
    return {browser,page};
  } catch (error) {
    if (browser) await browser.close().catch(() => {});
    throw error;
  }
}

it('reports asynchronous initialization failures and stops loading',async()=>{
  const {browser,page}=await openPlayer({rejectForceNative:true,expectInitFailure:true});
  try {
    expect(await page.evaluate(()=>window.failures)).toContain('settings unavailable');
    expect(await page.locator('.retry-button').isVisible()).toBe(true);
    expect(await page.evaluate(()=>window.errors)).toEqual([]);
  } finally {await browser.close();}
});

it('keeps audio enabled when an older play request rejects',async()=>{
  const {browser,page}=await openPlayer({enabled:true,showControls:true});
  try {
    await page.evaluate(()=>window.players[0].emit('manifest'));
    const audio=page.locator('.audio-toggle-btn');
    await audio.waitFor({state:'visible'});
    await page.waitForFunction(()=>!document.querySelector('.audio-toggle-btn')?.disabled);
    await page.evaluate(()=>{
      window.audioRejects=[];
      HTMLMediaElement.prototype.play=()=>new Promise((_resolve,reject)=>window.audioRejects.push(reject));
    });
    await audio.click();
    await audio.click();
    await audio.click();
    await page.waitForFunction(()=>window.audioRejects.length>=2);
    await page.evaluate(()=>window.audioRejects[0](new Error('older request rejected')));
    await page.evaluate(async()=>{
      await Promise.resolve();
      await Promise.resolve();
    });
    await page.waitForFunction(()=>document.querySelector('.audio-toggle-btn')?.getAttribute('aria-pressed')==='true');
    await page.waitForFunction(()=>!!document.querySelector('video') && !document.querySelector('video').muted);
    expect(await audio.getAttribute('aria-pressed')).toBe('true');
    expect(await page.locator('video').evaluate(video=>video.muted)).toBe(false);
    expect(await page.evaluate(()=>window.errors)).toEqual([]);
  } finally {await browser.close();}
});

it('focuses the resume control when privacy mode starts active',async()=>{
  const {browser,page}=await openPlayer({privacyMode:true});
  try {
    await page.waitForFunction(()=>document.activeElement?.getAttribute('aria-label')==='live.resumeStream');
    expect(await page.evaluate(()=>document.activeElement?.getAttribute('aria-label'))).toBe('live.resumeStream');
    expect(await page.locator('video').evaluate(video=>video.parentElement.inert)).toBe(true);
    await page.locator('.video-cell').evaluate(cell=>cell.click());
    expect(await page.evaluate(()=>window.fullscreenToggles)).toBe(0);
    await page.locator('.video-cell').dblclick();
    expect(await page.evaluate(()=>window.fullscreenToggles)).toBe(0);
  } finally {await browser.close();}
});

it('restores focus to the tile after resuming initial privacy mode',async()=>{
  const {browser,page}=await openPlayer({privacyMode:true});
  try {
    await page.locator('button[aria-label="live.resumeStream"]').click();
    await page.locator('button[aria-label="live.resumeStream"]').waitFor({state:'detached'});
    await page.waitForFunction(()=>document.activeElement?.classList.contains('video-cell'));
    expect(await page.evaluate(()=>document.activeElement?.classList.contains('video-cell'))).toBe(true);
    expect(await page.evaluate(()=>window.errors)).toEqual([]);
  } finally {await browser.close();}
});

it('does not steal focus from outside the tile when privacy changes',async()=>{
  const {browser,page}=await openPlayer({});
  try {
    await page.evaluate(()=>{
      const outside=document.createElement('button');
      outside.id='outside-focus';
      document.body.appendChild(outside);
      outside.focus();
      window.config.privacyMode=true;
      window.mount();
    });
    await page.locator('button[aria-label="live.resumeStream"]').waitFor({state:'visible'});
    expect(await page.evaluate(()=>document.activeElement?.id)).toBe('outside-focus');
    await page.evaluate(()=>{window.config.privacyMode=false;window.mount();});
    await page.locator('button[aria-label="live.resumeStream"]').waitFor({state:'detached'});
    expect(await page.evaluate(()=>document.activeElement?.id)).toBe('outside-focus');
  } finally {await browser.close();}
});

it('ignores an old privacy response after the tile changes streams',async()=>{
  const {browser,page}=await openPlayer({privacyMode:true,deferPrivacy:true});
  try {
    await page.locator('button[aria-label="live.resumeStream"]').click();
    await page.waitForFunction(()=>!!window.resolvePrivacyRequest);
    await page.evaluate(()=>{
      window.config.streamId='second';
      window.config.streamName='second camera';
      window.mount();
    });
    await page.waitForFunction(()=>
      document.querySelector('.video-cell')?.dataset.streamName==='second camera');
    await page.evaluate(()=>window.resolvePrivacyRequest());
    await page.waitForFunction(()=>
      window.privacyResolved===true &&
      document.querySelector('.video-cell')?.dataset.streamName==='second camera' &&
      !!document.querySelector('button[aria-label="live.resumeStream"]') &&
      !document.querySelector('button[aria-label="live.resumeStream"]')?.disabled);
    await page.evaluate(()=>new Promise(resolve=>requestAnimationFrame(()=>requestAnimationFrame(resolve))));
    expect(await page.locator('button[aria-label="live.resumeStream"]').isVisible()).toBe(true);
    expect(await page.locator('button[aria-label="live.resumeStream"]').isEnabled()).toBe(true);
    expect(await page.evaluate(()=>window.errors)).toEqual([]);
  } finally {await browser.close();}
});

it.each([true,false])('resets local privacy when stream ID changes: %s',async changeId=>{
  const {browser,page}=await openPlayer({enabled:true,showControls:true});
  try {
    await page.locator('button[aria-label="live.pauseForPrivacy"]').click();
    await page.locator('.btn-primary').click();
    await page.locator('button[aria-label="live.resumeStream"]').waitFor({state:'visible'});
    await page.evaluate(shouldChangeId=>{
      if(shouldChangeId) {
        window.config.streamId='second';
      } else {
        window.config.streamName='second camera';
      }
      window.mount();
    },changeId);
    await page.locator('button[aria-label="live.resumeStream"]').waitFor({state:'detached'});
    expect(await page.evaluate(()=>document.querySelector('.video-cell')?.dataset.streamName)).toBe(changeId?'fixture camera':'second camera');
    expect(await page.evaluate(()=>window.errors)).toEqual([]);
  } finally {await browser.close();}
});

it.each([true,false])('resets fallback transport when stream ID changes: %s',async changeId=>{
  const {browser,page}=await openPlayer({enabled:false});
  try {
    await page.evaluate(()=>window.emitFatal('other'));
    await page.waitForFunction(()=>window.requests.some(r=>r.url==='/hls/fixture%20camera/index.m3u8'));
    const playerCount=await page.evaluate(()=>window.players.length);
    await page.evaluate(shouldChangeId=>{
      if(shouldChangeId) {
        window.config.streamId='second';
      } else {
        window.config.streamName='second camera';
      }
      window.mount();
    },changeId);
    await page.waitForFunction(({previousCount,shouldChangeId})=>window.players.length>previousCount&&
      window.players.slice(previousCount).some(p=>!p.destroyed&&p.sources.some(s=>
        String(s?.src||s).includes(shouldChangeId?'src=fixture%20camera':'src=second%20camera'))),
    {previousCount:playerCount,shouldChangeId:changeId});
    expect(await page.evaluate(()=>window.errors)).toEqual([]);
  } finally {await browser.close();}
});

it.each(['exhausted','networkExhausted','unrecoverable','native'])(
  '%s recovery falls back when go2rtc was only auto-detected',async branch=>{
    const {browser,page}=await openPlayer({enabled:false,nativePlayer:branch==='native'});
    try {
      if(branch==='native') await page.locator('video').dispatchEvent('error');
      else await page.evaluate(value=>window.emitFatal(
        value==='exhausted'?'media':value==='networkExhausted'?'network':'other',
        value==='exhausted'||value==='networkExhausted'?4:1,
      ),branch);
      expect(await page.evaluate(()=>window.errors)).toEqual([]);
      await page.waitForFunction(currentBranch=>{
        const fallbackUrl='/hls/fixture%20camera/index.m3u8';
        const requested=window.requests.some(r=>r.url===fallbackUrl);
        if(!requested) return false;
        if(currentBranch==='native') {
          return (window.nativeSources||[]).some(s=>String(s).includes(fallbackUrl));
        }
        return (window.players||[]).some(p=>
          Array.isArray(p?.sources) && p.sources.some(s=>String(s?.src||s).includes(fallbackUrl)),
        );
      },branch);
      expect(await page.evaluate(()=>window.failures)).toEqual([]);
      if(branch!=='native') expect(await page.evaluate(()=>window.players[0].destroyed)).toBe(1);
    } finally {await browser.close();}
  },
);

it.each(['exhausted','networkExhausted','unrecoverable','native'])(
  '%s recovery reports failure without fallback when go2rtc is enabled',async branch=>{
    const {browser,page}=await openPlayer({enabled:true,nativePlayer:branch==='native'});
    try {
      if(branch==='native') await page.locator('video').dispatchEvent('error');
      else await page.evaluate(value=>window.emitFatal(
        value==='exhausted'?'media':value==='networkExhausted'?'network':'other',
        value==='exhausted'||value==='networkExhausted'?4:1,
      ),branch);
      expect(await page.evaluate(()=>window.errors)).toEqual([]);
      await page.waitForFunction(()=>window.failures.length>0);
      // The error overlay may fetch the latest recording for its last-frame
      // image; only manifest requests indicate an HLS transport fallback.
      expect(await page.evaluate(()=>window.requests.map(r=>r.url).filter(url=>url.includes('.m3u8'))))
        .toEqual(['/go2rtc/api/stream.m3u8?src=fixture%20camera&mp4=flac']);
      expect(await page.evaluate(()=>window.failures[0])).toBe(
        branch==='unrecoverable'?'fixture fatal':'live.go2rtcHlsFailedRetry',
      );
    } finally {await browser.close();}
  },
);

it('retries the go2rtc master manifest and prevents pending recovery after unmount',async()=>{
  const {browser,page}=await openPlayer({enabled:true});
  try {
    await page.evaluate(()=>{window.emitFatal('network');window.recoveries.shift()();});
    expect(await page.evaluate(()=>({starts:window.players[0].starts,sources:window.players[0].sources})))
      .toEqual({starts:1,sources:Array(2).fill('/go2rtc/api/stream.m3u8?src=fixture%20camera&mp4=flac')});
    await page.evaluate(()=>{window.emitFatal('media');window.recoveries.shift()();});
    expect(await page.evaluate(()=>({media:window.players[0].mediaRecoveries,sources:window.players[0].sources,
      failures:window.failures})))
      .toEqual({media:1,sources:Array(2).fill('/go2rtc/api/stream.m3u8?src=fixture%20camera&mp4=flac'),failures:[]});
    await page.evaluate(()=>{window.emitFatal('network');window.emitFatal('media');window.unmount();});
    // Preact schedules passive effect cleanup after paint; observe its completion.
    await page.waitForFunction(()=>window.cleanups>0);
    await page.evaluate(()=>{window.recoveries.splice(0).forEach(fn=>fn());
      window.players[0].emit('error',{fatal:true,type:'other'});});
    expect(await page.evaluate(()=>({starts:window.players[0].starts,media:window.players[0].mediaRecoveries,
      destroyed:window.players[0].destroyed,errors:window.errors})))
      .toEqual({starts:1,media:1,destroyed:1,errors:[]});
  } finally {await browser.close();}
});

it.each(['manifestLoadError','manifestLoadTimeOut'])(
  'reloads the native manifest after %s',async detail=>{
    const {browser,page}=await openPlayer({forceNative:true});
    try {
      await page.waitForFunction(()=>window.players.some(p=>!p.destroyed&&
        p.sources.includes('/hls/fixture%20camera/index.m3u8')));
      await page.evaluate(value=>{
        window.captureRecovery=true;
        window.players.find(p=>!p.destroyed).emit('error',{fatal:true,type:'network',details:value});
        window.captureRecovery=false;
        window.recoveries.shift()();
      },detail);
      expect(await page.evaluate(()=>({sources:window.players.find(p=>!p.destroyed).sources,
        starts:window.players.find(p=>!p.destroyed).starts,errors:window.errors})))
        .toEqual({sources:Array(2).fill('/hls/fixture%20camera/index.m3u8'),starts:1,errors:[]});
    } finally {await browser.close();}
  },
);

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
  const {browser,page}=await openPlayer({enabled:true,forceNative:true,nativePlayer:true});
  try {
    const nativeManifest='/hls/fixture%20camera/index.m3u8';
    await page.waitForFunction(url=>window.requests.some(r=>r.url===url)&&
      window.nativeSources.includes(url),nativeManifest);
    expect(await page.evaluate(()=>window.requests.length>0&&
      window.requests.every(r=>r.url==='/hls/fixture%20camera/index.m3u8'))).toBe(true);
    expect(await page.evaluate(()=>window.players.length)).toBe(0);
    expect(await page.evaluate(()=>window.settingsReads.includes('enabled'))).toBe(false);
    await page.locator('video').dispatchEvent('error');
    await page.waitForFunction(()=>window.failures.includes('live.hlsStreamFailedToLoad'));
    expect(await page.evaluate(()=>window.errors)).toEqual([]);
  } finally {await browser.close();}
});

it('force-native selects the lightNVR manifest when hls.js is available',async()=>{
  const {browser,page}=await openPlayer({enabled:true,forceNative:true,nativePlayer:false});
  try {
    const nativeManifest='/hls/fixture%20camera/index.m3u8';
    await page.waitForFunction(url=>window.requests.some(r=>r.url===url)&&
      window.players.some(p=>!p.destroyed&&p.sources.includes(url)),nativeManifest);
    expect(await page.evaluate(()=>window.requests.length>0&&
      window.requests.every(r=>r.url==='/hls/fixture%20camera/index.m3u8'))).toBe(true);
    expect(await page.evaluate(()=>window.nativeSources.length)).toBe(0);
    expect(await page.evaluate(()=>window.settingsReads.includes('enabled'))).toBe(false);
  } finally {await browser.close();}
});
