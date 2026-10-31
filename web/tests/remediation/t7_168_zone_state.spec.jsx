/** First activation must preserve the native checkbox value while selecting its zone. */
const path = require('node:path');
const fixtureZones = [{id:'one',name:'Entry',enabled:true,color:'#3b82f6',polygon:[{x:.1,y:.1},{x:.4,y:.1},{x:.2,y:.4}]}];
const t = key => 'translated:' + key;
const browserTest = process.env.T7_168_ZONE_BROWSER === '1' ? it : it.skip;
async function openZoneBrowser() {
 const {chromium} = require('../../../node_modules/playwright');
 const {build} = require('esbuild');
 const webRoot = path.resolve(__dirname, '../..');
 const output = await build({stdin:{resolveDir:webRoot,loader:'jsx',contents:`
  import {h,render} from 'preact';
  import {ZoneEditor} from './js/components/preact/ZoneEditor.jsx';
  const zones=${JSON.stringify(fixtureZones)};
  window.calls={saved:[],closed:0,events:[],requests:[]};
  window.fetch=async(url,options={})=>{if(options.method==='POST'){window.calls.requests.push({url,method:options.method,body:JSON.parse(options.body)});return{ok:true,json:async()=>({})};}return{ok:true,json:async()=>{window.zonesLoaded=true;return{zones};}};};
  window.mount=()=>render(h(ZoneEditor,{streamName:'Entry',streamWidth:640,streamHeight:360,zones,onClose:()=>window.calls.closed++,onZonesChange:value=>window.calls.saved.push(value)}),document.getElementById('root'));
  for(const capture of[true,false])for(const type of['click','input','change'])document.addEventListener(type,event=>{if(event.target.type==='checkbox')window.calls.events.push({type,capture,checked:event.target.checked,selected:document.querySelector('[aria-pressed]')?.getAttribute('aria-pressed')});},capture);
 `},bundle:true,write:false,format:'iife',jsx:'automatic',jsxImportSource:'preact',plugins:[{name:'fixture',setup(builder){
  builder.onResolve({filter:/(?:i18n|settings-utils)\.js$/},args=>({path:args.path,namespace:'fixture'}));
  builder.onLoad({filter:/.*/,namespace:'fixture'},()=>({contents:"export const t=key=>'translated:'+key;export const useI18n=()=>({t});export const getGo2rtcBaseUrl=async()=>'/go2rtc';",loader:'js'}));
 }}]});
 const browser=await chromium.launch({headless:true});
 const page=await browser.newPage({viewport:{width:1200,height:1000}});page.setDefaultTimeout(5000);
 await page.route('http://fixture.invalid/**',route=>route.fulfill({contentType:route.request().url()==='http://fixture.invalid/'?'text/html':'image/svg+xml',body:route.request().url()==='http://fixture.invalid/'?'<div id="root"></div>':'<svg xmlns="http://www.w3.org/2000/svg" width="640" height="360"><rect width="640" height="360" fill="gray"/></svg>'}));
 await page.goto('http://fixture.invalid/');
 // Standalone fixture layout: no shared build assets, server or operator state.
 await page.addStyleTag({content:'canvas{max-width:640px;max-height:360px}button,input{margin:4px}'});
 await page.addScriptTag({content:output.outputFiles[0].text});await page.evaluate(()=>window.mount());
 try{await page.waitForFunction(()=>window.zonesLoaded&&document.querySelector('input[type=checkbox]'),{},{timeout:5000});}catch(error){await browser.close();throw error;}
 return{browser,page};
}
browserTest.each([['pointer',false],['keyboard',false],['label',false],['pointer',true],['keyboard',true],['label',true]])('first zone checkbox activation preserves value and exact save (method=%s, preselect=%s)',async(method,preselect)=>{
 const{browser,page}=await openZoneBrowser();
 try{
  const checkbox=page.getByRole('checkbox',{name:t('common.enabled'),exact:true});
  const select=page.getByRole('button',{name:t('zoneEditor.selectZone')+' Entry',exact:true});
  expect(await checkbox.isChecked()).toBe(true);expect(await select.getAttribute('aria-pressed')).toBe('false');
  if(preselect)await select.click();
  const activate=async()=>{if(method==='pointer')await checkbox.click();
  else if(method==='label')await checkbox.locator('..').locator('span').click();
  else{
   // Reach the checkbox solely through native Tab navigation; never click a field/card.
   for(let index=0;index<20&&!await checkbox.evaluate(element=>element===document.activeElement);index++)await page.keyboard.press('Tab');
   expect(await checkbox.evaluate(element=>element===document.activeElement)).toBe(true);
   await page.keyboard.press('Space');
  }};
  await activate();const firstChecked=await checkbox.isChecked();
  expect(await checkbox.evaluate(element=>element===document.activeElement)).toBe(true);
  await activate();await activate();
  const checked=await checkbox.isChecked();const selected=await select.getAttribute('aria-pressed');
  await page.getByRole('button',{name:t('zoneEditor.saveZones'),exact:true}).click();
  await page.waitForFunction(()=>window.calls.saved.length===1);
  const calls=await page.evaluate(()=>window.calls);
  if(checked!==false)console.log('ZONE_FIRST_TOGGLE_TRACE',JSON.stringify({method,preselect,checked,selected,calls}));
  expect(calls.events.filter(event=>event.type==='change'&&!event.capture)).toHaveLength(3);
  expect({firstChecked,checked,selected,saved:calls.saved,closed:calls.closed,requests:calls.requests}).toEqual({firstChecked:false,checked:false,selected:'true',saved:[[{...fixtureZones[0],enabled:false}]],closed:1,requests:[{url:'/api/streams/Entry/zones',method:'POST',body:{zones:[{...fixtureZones[0],enabled:false}]}}]});
 }finally{await browser.close();}
},30000);
