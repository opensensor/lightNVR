/** Exercise optimistic single-recording tag edits with real Preact and delayed writes. */
const path = require('node:path');
const {build} = require('esbuild');
const {chromium} = require('../../../node_modules/playwright');

async function openOverlay() {
  const compiled = await build({
    stdin: {
      resolveDir: path.resolve(__dirname, '../..'), loader: 'jsx',
      contents: `
        import {h, render} from 'preact';
        import {TagsOverlay} from './js/components/preact/recordings/TagsOverlay.tsx';
        window.calls = {requests:[], changes:[], messages:[]};
        window.api = {
          getAllRecordingTags:()=>Promise.resolve(['alpha','beta','suggested']),
          setRecordingTags:(...args)=>{
            window.calls.requests.push(args);
            return new Promise((resolve,reject)=>{
              window.succeed=resolve; window.fail=()=>reject(new Error('fixture failure'));
            });
          }
        };
        render(h(TagsOverlay, {
          recording:{id:'9007199254740993',tags:['alpha','beta']}, onClose:()=>{},
          onTagsChanged:(...args)=>window.calls.changes.push(args)
        }), document.getElementById('root'));
      `
    },
    bundle:true, write:false, format:'iife', jsx:'automatic', jsxImportSource:'preact',
    plugins:[{name:'fixture',setup(build){
      build.onResolve({filter:/(?:i18n\.ts|recordingsAPI\.tsx|ToastContainer\.tsx)$/},args=>({path:args.path,namespace:'fixture'}));
      build.onLoad({filter:/.*/,namespace:'fixture'},args=>({loader:'js',contents:
        args.path.endsWith('i18n.ts') ? 'export const useI18n=()=>({t:key=>key});' :
        args.path.endsWith('recordingsAPI.tsx') ? 'export const recordingsAPI={getAllRecordingTags:()=>window.api.getAllRecordingTags(),setRecordingTags:(...args)=>window.api.setRecordingTags(...args)};' :
        'export const showStatusMessage=(...args)=>window.calls.messages.push(args);'
      }));
    }}]
  });
  const browser=await chromium.launch({headless:true});
  try {
    const page=await browser.newPage();
    await page.setContent('<div id="root"></div>');
    await page.addScriptTag({content:compiled.outputFiles[0].text});
    await page.getByTitle('Remove tag "alpha"',{exact:true}).waitFor();
    await settle(page);
    return {browser,page};
  } catch(error) { await browser.close();throw error; }
}
async function settle(page) {
  await page.evaluate(()=>new Promise(resolve=>requestAnimationFrame(()=>requestAnimationFrame(()=>resolve()))));
}
async function complete(page, success) {
  await page.evaluate(success=>success?window.succeed():window.fail(),success);
  await settle(page);
}
const remove=(page,tag)=>page.getByTitle(`Remove tag "${tag}"`,{exact:true});
const input=page=>page.getByPlaceholder('recordings.addTagPlaceholder',{exact:true});

it('failed removal restores tags and permits a successful retry without overlapping writes',async()=>{
  const {browser,page}=await openOverlay();
  try {
    // Both events run before Preact can disable the second button.
    await page.evaluate(()=>{
      document.querySelector('[title="Remove tag \\"alpha\\""]').click();
      document.querySelector('[title="Remove tag \\"beta\\""]').click();
    });
    await settle(page);
    expect(await page.evaluate(()=>window.calls.requests)).toEqual([['9007199254740993',['beta']]]);
    expect(await remove(page,'alpha').count()).toBe(0);
    expect(await remove(page,'beta').isDisabled()).toBe(true);
    expect(await input(page).isDisabled()).toBe(true);
    await complete(page,false);
    expect(await remove(page,'alpha').isEnabled()).toBe(true);
    expect(await remove(page,'beta').isEnabled()).toBe(true);
    expect(await page.evaluate(()=>window.calls.changes)).toEqual([]);
    expect(await page.evaluate(()=>window.calls.messages)).toEqual([]);
    await remove(page,'alpha').click();await complete(page,true);
    expect(await remove(page,'alpha').count()).toBe(0);
    expect(await page.evaluate(()=>window.calls.changes)).toEqual([['9007199254740993',['beta']]]);
    expect(await page.evaluate(()=>window.calls.messages.length)).toBe(1);
  } finally {await browser.close();}
},30000);

it('failed addition restores tags and input; success is kept before another failed edit',async()=>{
  const {browser,page}=await openOverlay();
  try {
    await input(page).fill(' new ');await input(page).press('Enter');await settle(page);
    expect(await remove(page,'new').isDisabled()).toBe(true);
    await complete(page,false);
    expect(await remove(page,'new').count()).toBe(0);
    expect(await input(page).inputValue()).toBe('new');
    expect(await page.evaluate(()=>window.calls.changes)).toEqual([]);
    await input(page).press('Enter');await complete(page,true);
    expect(await remove(page,'new').isEnabled()).toBe(true);
    await remove(page,'beta').click();await complete(page,false);
    for(const tag of ['alpha','beta','new'])expect(await remove(page,tag).isEnabled()).toBe(true);
    expect(await page.evaluate(()=>window.calls.changes)).toEqual([['9007199254740993',['alpha','beta','new']]]);
    expect(await page.evaluate(()=>window.calls.messages.length)).toBe(1);
  } finally {await browser.close();}
},30000);
