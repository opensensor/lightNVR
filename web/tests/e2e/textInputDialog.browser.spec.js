/** Promise settlement, dismissal, focus and stale-action ownership in actual Preact. */
const path = require('node:path');
let browser, page, bundle, errors;
beforeAll(async () => {
  const {build} = require('esbuild');
  const output = await build({
    stdin: {resolveDir: path.resolve(__dirname, '../..'), loader: 'jsx', contents: `
      import {h,render,Fragment} from 'preact';
      import {useState} from 'preact/hooks';
      import {TextInputDialog} from './js/components/preact/common/ModalDialog.tsx';
      window.calls=0;window.closes=0;window.pending=[];window.mode='pending';
      function Fixture() {
        const [open,setOpen]=useState(false),[tick,setTick]=useState(0);
        window.setOpen=setOpen;
        const confirm=(value,checked)=>{
          window.submitted={value,checked};
          window.calls++;setTick(value=>value+1);
          if(window.mode==='throw')throw new Error('PRIVATE-BACKEND-ERROR');
          if(window.mode==='sync')return;
          if(window.mode==='false')return false;
          return new Promise((resolve,reject)=>window.pending.push({resolve,reject}));
        };
        return h(Fragment,null,
          h('button',{id:'opener',onClick:()=>setOpen(true)},'Open'),
          h('span',null,tick),
          h(TextInputDialog,{isOpen:open,title:'Confirm action',initialValue:'Original',checkboxLabel:'Include children',confirmLabel:'Commit',
            onSubmit:confirm,onClose:()=>{window.closes++;setOpen(false);}}));
      }
      render(h(Fixture),document.getElementById('root'));
    `},
    bundle:true,write:false,format:'iife',jsx:'automatic',jsxImportSource:'preact',
    plugins:[{name:'translations',setup(build){
      build.onResolve({filter:/\/i18n\.ts$/},args=>({path:args.path,namespace:'translations'}));
      build.onLoad({filter:/.*/,namespace:'translations'},()=>({contents:"export const useI18n=()=>({t:key=>key});",loader:'js'}));
    }}],
  });
  bundle=output.outputFiles[0].text;
  browser=await require('../../../node_modules/playwright').chromium.launch({headless:true});
});
beforeEach(async()=>{
  page=await browser.newPage();page.setDefaultTimeout(5000);errors=[];page.on('pageerror',error=>errors.push(error.message));
  await page.setContent('<div id="root"></div>');await page.addScriptTag({content:bundle});
  await page.locator('#opener').click();await page.waitForFunction(()=>document.activeElement?.tagName==='INPUT');
});
afterEach(async()=>{await page.close();});
afterAll(async()=>{await browser.close();});
const commit=()=>page.getByRole('button',{name:'Commit',exact:true});
const settle=async(ok=true)=>{
  await page.evaluate(ok=>{const p=window.pending.shift();ok?p.resolve():p.reject(new Error('PRIVATE-BACKEND-ERROR'));},ok);
};
const closed=()=>page.getByRole('dialog').waitFor({state:'detached'});

it('deduplicates immediate clicks, blocks dismissal while pending and restores trigger focus',async()=>{
  await page.getByRole('textbox').fill('  New name  ');
  await page.getByRole('checkbox').check();
  await page.locator('form').evaluate(el=>{
    el.dispatchEvent(new Event('submit',{bubbles:true,cancelable:true}));
    el.dispatchEvent(new Event('submit',{bubbles:true,cancelable:true}));
  });
  expect(await page.evaluate(()=>window.submitted)).toEqual({value:'New name',checked:true});
  expect(await page.evaluate(()=>window.calls)).toBe(1);
  await page.waitForFunction(()=>document.querySelector('[aria-busy="true"]'));
  expect(await page.getByRole('button',{name:'Cancel',exact:true}).isDisabled()).toBe(true);
  expect(await page.getByRole('dialog').evaluate(el=>el.contains(document.activeElement))).toBe(true);
  await page.keyboard.press('Escape');
  await page.getByRole('presentation').dispatchEvent('mousedown');
  expect(await page.evaluate(()=>window.closes)).toBe(0);
  await settle();await closed();
  await page.waitForFunction(()=>document.activeElement?.id==='opener');
  expect(errors).toEqual([]);
});
it.each(['reject','throw'])('keeps %s failures open with safe feedback and permits retry',async(mode)=>{
  if(mode==='throw')await page.evaluate(()=>{window.mode='throw';});
  await commit().click();if(mode==='reject')await settle(false);
  await page.getByRole('alert').waitFor();
  expect(await page.getByRole('alert').textContent()).toBe('common.actionFailed');
  expect(await page.getByText('PRIVATE-BACKEND-ERROR').count()).toBe(0);
  expect(await commit().isEnabled()).toBe(true);
  await page.evaluate(()=>{window.mode='sync';});await commit().click();await closed();
  expect(await page.evaluate(()=>window.calls)).toBe(2);expect(errors).toEqual([]);
});
it('respects false without closing and closes for synchronous success',async()=>{
  await page.evaluate(()=>{window.mode='false';});await commit().click();
  await page.waitForFunction(()=>document.querySelector('[aria-busy="false"]'));
  expect(await page.getByRole('dialog').count()).toBe(1);expect(await page.getByRole('alert').count()).toBe(0);
  await page.evaluate(()=>{window.mode='sync';});await commit().click();await closed();expect(errors).toEqual([]);
});
it.each([true,false])('an old settlement cannot close or contaminate a reopened dialog (success=%s)',async(success)=>{
  await commit().click();await page.waitForFunction(()=>window.pending.length===1);
  await page.evaluate(()=>window.setOpen(false));await closed();
  await page.locator('#opener').click();await page.getByRole('dialog').waitFor();
  await commit().click();await page.waitForFunction(()=>window.pending.length===2);
  await settle(success);await page.evaluate(()=>new Promise(resolve=>setTimeout(resolve,30)));
  expect(await page.getByRole('dialog').count()).toBe(1);
  expect(await page.getByRole('alert').count()).toBe(0);expect(await commit().isDisabled()).toBe(true);
  await settle();await closed();expect(errors).toEqual([]);
});

it('rejects whitespace-only submissions and resets values and errors on reopening',async()=>{
  await page.getByRole('textbox').fill('   ');
  await page.locator('form').dispatchEvent('submit');
  expect(await page.evaluate(()=>window.calls)).toBe(0);
  await page.getByRole('textbox').fill('Changed');
  await page.getByRole('checkbox').check();
  await commit().click();await settle(false);await page.getByRole('alert').waitFor();
  expect(await page.getByRole('textbox').inputValue()).toBe('Changed');
  expect(await page.getByRole('checkbox').isChecked()).toBe(true);
  await page.getByRole('button',{name:'Cancel',exact:true}).click();await closed();
  await page.locator('#opener').click();await page.getByRole('dialog').waitFor();
  expect(await page.getByRole('textbox').inputValue()).toBe('Original');
  expect(await page.getByRole('checkbox').isChecked()).toBe(false);
  expect(await page.getByRole('alert').count()).toBe(0);expect(errors).toEqual([]);
});
