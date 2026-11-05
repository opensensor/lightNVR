/** Exercise the actual list/grid controls for damaged and unchecked recordings. */
const translations = require('../../public/locales/en.json');
const t = key => translations[key] || key;

async function openBrowser(view, issue) {
  const { chromium } = require('../../../node_modules/playwright');
  const { build } = require('esbuild');
  const path = require('node:path');
  const compiled = await build({
    stdin: {
      resolveDir: path.resolve(__dirname, '../..'), loader: 'jsx',
      contents: `
        import {h,render} from 'preact';
        import {RecordingsTable} from './js/components/preact/recordings/RecordingsTable.tsx';
        import {RecordingsGrid} from './js/components/preact/recordings/RecordingsGrid.tsx';
        window.translations=${JSON.stringify(translations)};
        window.calls={play:0,download:0,protect:0,delete:0,thumbnail:0};
        const recording={id:1,stream:'Entry',file_path:'/recordings/entry.mp4',start_time:'2026-11-01T10:00:00Z',
          end_time:'2026-11-01T10:01:00Z',duration:60,size_bytes:1024,in_progress:false,protected:false,
          playback_issue:${JSON.stringify(issue)},tags:[],detections:[]};
        window.mount=(protectedValue=false,hideActions=false)=>render(h(${view === 'table' ? 'RecordingsTable' : 'RecordingsGrid'}, {
          recordings:[{...recording,protected:protectedValue}],collapsed:false,toggleCollapsed:()=>{},
          selectedRecordings:{},toggleRecordingSelection:()=>{},selectAll:false,toggleSelectAll:()=>{},
          getSelectedCount:()=>0,openDeleteModal:()=>{},openDownloadModal:()=>{},
          playRecording:()=>window.calls.play++,downloadRecording:()=>window.calls.download++,
          deleteRecording:()=>window.calls.delete++,toggleProtection:()=>window.calls.protect++,
          pagination:{totalItems:1},canDelete:true,clearSelections:()=>{},
          hiddenColumns:{tags:true,detections:true,stream:true,duration:true,actions:hideActions},
          sortField:'start_time',sortDirection:'desc',sortBy:()=>{},recordingsTableBodyRef:{current:null}
        }),document.getElementById('root'));window.mount();`
    },
    bundle:true,write:false,format:'iife',jsx:'automatic',jsxImportSource:'preact',
    plugins:[{name:'fixture',setup(build){
      build.onResolve({filter:/(?:i18n\.ts|formatUtils\.ts|TagsOverlay\.tsx|request-queue\.ts)$/},args=>({path:args.path,namespace:'fixture'}));
      build.onLoad({filter:/.*/,namespace:'fixture'},args=>({loader:'js',contents:
        args.path.endsWith('i18n.ts') ? 'export const useI18n=()=>({t:key=>window.translations[key]||key});' :
        args.path.endsWith('formatUtils.ts') ? `export const formatUtils={formatDateTime:()=> 'Nov 1, 10:00',formatDuration:()=> '1:00',formatCaptureMethod:()=> 'Continuous',getTimelineUrl:()=> '#timeline',liveRecordingDuration:()=>60};` :
        args.path.endsWith('TagsOverlay.tsx') ? 'export const TagIcon=()=>null;export const TagsOverlay=()=>null;export const BulkTagsOverlay=()=>null;' :
        'export const Priority={NORMAL:1,HIGH:2,LOW:0};export const invalidateThumbnailLoad=()=>{};export const queueThumbnailLoad=()=>{window.calls.thumbnail++;return Promise.resolve("data:image/gif;base64,R0lGODlhAQABAIAAAAAAAP///yH5BAEAAAAALAAAAAABAAEAAAIBRAA7");};'
      }));
    }}]
  });
  const browser=await chromium.launch({headless:true});
  const page=await browser.newPage();
  page.setDefaultTimeout(5000);
  await page.setContent('<style>svg{width:20px;height:20px}button{margin:4px}.sr-only{display:none}</style><div id="root"></div>');
  await page.addScriptTag({content:compiled.outputFiles[0].text});
  await page.getByRole('note').waitFor();
  return {browser,page};
}

it.each(['table','grid'])('%s explains damaged recordings while retaining recovery and management actions',async view=>{
  const {browser,page}=await openBrowser(view,'invalid_media');
  try {
    const note=page.getByRole('note');
    expect(await note.innerText()).toContain(t('recordings.playbackUnavailable'));
    expect(await note.innerText()).toContain(t('recordings.playbackIssue.invalid_media'));
    expect(await note.innerText()).toContain(t('recordings.playbackRecovery'));
    expect(await note.innerText()).toContain(t('recordings.playbackRetention'));
    expect(await page.getByRole('button',{name:t('recordings.play'),exact:true}).isDisabled()).toBe(true);
    await page.getByRole('button',{name:t('recordings.downloadOriginal'),exact:true}).click();
    await page.getByTitle(view === 'table' ? t('recordings.protect') : 'Protect',{exact:true}).click();
    await page.getByRole('button',{name:t('common.delete'),exact:true}).click();
    expect(await page.evaluate(()=>window.calls)).toEqual({play:0,download:1,protect:1,delete:1,thumbnail:0});
    await page.evaluate(()=>window.mount(true,true));
    await page.waitForFunction(text=>document.querySelector('[role=note]').textContent.includes(text),t('recordings.playbackProtected'));
  } finally {await browser.close();}
});

it.each(['table','grid'])('%s leaves playback available when the check is inconclusive',async view=>{
  const {browser,page}=await openBrowser(view,'probe_limit');
  try {
    expect(await page.getByRole('note').innerText()).toContain(t('recordings.playbackUnverified'));
    expect(await page.getByRole('note').innerText()).toContain(t('recordings.playbackIssue.probe_limit'));
    const play=page.getByRole('button',{name:t('recordings.play'),exact:true});
    expect(await play.isDisabled()).toBe(false);
    await play.click();
    expect(await page.evaluate(()=>window.calls.play)).toBe(1);
  } finally {await browser.close();}
});
