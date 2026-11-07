/** Real query observers and affected controls; responses pause at the fetch boundary. */
async function openBrowser(view) {
  const {chromium}=require('../../../node_modules/playwright');
  const {build}=require('esbuild');
  const path=require('node:path');
  const baseline=require('../../../tests/contracts/fixtures/baseline-payloads.json').cases;
  const compiled=await build({
    stdin:{resolveDir:path.resolve(__dirname,'../..'),loader:'jsx',contents:`
      import {h,render} from 'preact';
      import {QueryClient,QueryClientProvider} from './js/query-client.ts';
      import {StreamsView} from './js/components/preact/StreamsView.tsx';
      import {UsersView} from './js/components/preact/UsersView.tsx';
      import {WorkspacesTab} from './js/components/preact/settings/WorkspacesTab.tsx';
      const summary=${JSON.stringify(baseline['admin-summary'].body)};summary.streams[0]={...summary.streams[0],name:'Camera',enabled:true};summary.streams=[summary.streams[0]];summary.total=1;summary.total_pages=1;
      const users={users:[{id:2,username:'Fixture user',email:'',role:2,role_name:'viewer',created_at:0,updated_at:0,last_login:0,is_active:true,password_change_locked:false,must_change_password:false,totp_enabled:false,authorization_mode:'legacy',allowed_login_cidrs:null}]};
      const workspace=visible=>({workspaces:[{key:'live.navigator',label:'Navigator',description:'Fixture',visible,configurable:true}]});
      window.writes=[];window.pending=[];window.messages=[];
      const json=(value,status=200)=>new Response(JSON.stringify(value),{status,headers:{'Content-Type':'application/json'}});
      window.fetch=async(input,options={})=>{
        const url=String(input),method=options.method||'GET';
        if(method!=='GET'){
          const body=options.body?JSON.parse(options.body):null;
          window.writes.push({url,method,body});
          return new Promise(resolve=>window.pending.push(ok=>resolve(ok?json(url==='/api/ui/workspaces'?workspace(body.workspaces['live.navigator']):url.endsWith('/login-lockout/clear')?{success:true,cleared:true,message:'Login lockout cleared'}:{}):json({error:'Fixture rejection'},400))));
        }
        if(url.startsWith('/api/streams?'))return json(summary);
        if(url==='/api/auth/users')return json(users);
        if(url==='/api/ui/workspaces')return json(workspace(true));
        if(url==='/api/detection/models')return json({models:[]});
        return json({});
      };
      const client=new QueryClient({defaultOptions:{queries:{retry:false},mutations:{retry:false}}});
      render(h(QueryClientProvider,{client},h(${view==='streams'?'StreamsView':view==='users'?'UsersView':'WorkspacesTab'},{t:key=>key})),document.getElementById('root'));
    `},bundle:true,write:false,format:'iife',jsx:'automatic',jsxImportSource:'preact',alias:{react:'preact/compat','react-dom':'preact/compat'},
    plugins:[{name:'fixture',setup(builder){
      builder.onResolve({filter:/(?:i18n\.ts|auth-utils\.ts|ToastContainer\.tsx|StreamCard\.tsx|HealthView\.tsx|FleetView\.tsx|EventRoutingView\.tsx|UsersTable\.tsx|AddUserModal\.tsx|DeleteUserModal\.tsx|ApiKeyModal\.tsx|TotpSetupModal\.tsx|AccessPolicyModal\.tsx|RoleManagerModal\.tsx|AuditHistoryModal\.tsx)$/},args=>({path:args.path,namespace:'fixture'}));
      builder.onLoad({filter:/.*/,namespace:'fixture'},args=>({loader:'jsx',resolveDir:path.resolve(__dirname,'../..'),contents:
        args.path.endsWith('i18n.ts')?'export const useI18n=()=>({t:key=>key});export const t=key=>key;':
        args.path.endsWith('auth-utils.ts')?'export const validateSession=async()=>({valid:true,role:"admin"});':
        args.path.endsWith('ToastContainer.tsx')?'export const showStatusMessage=(...args)=>window.messages.push(args);':
        args.path.endsWith('StreamCard.tsx')?'export const StreamCard=({stream,onOpenDelete})=><button onClick={()=>onOpenDelete(stream)}>Camera actions</button>;':
        args.path.endsWith('UsersTable.tsx')?'export const UsersTable=({users,onEdit})=><button onClick={()=>onEdit(users[0])}>Edit fixture user</button>;':
        `export const ${path.basename(args.path,'.tsx')}=()=>null;`
      }));
    }}]
  });
  const browser=await chromium.launch({headless:true});const page=await browser.newPage();page.setDefaultTimeout(4000);
  const errors=[];page.on('pageerror',e=>errors.push(e.message));
  await page.route('http://fixture.invalid/**',route=>route.fulfill({contentType:'text/html',body:'<style>svg{width:16px;height:16px}button{margin:3px}</style><div id="root"></div>'}));
  await page.goto('http://fixture.invalid/');await page.addScriptTag({content:compiled.outputFiles[0].text});
  return {browser,page,errors};
}
async function finish(page,ok){await page.evaluate(ok=>window.pending.shift()(ok),ok);}
async function pending(page){await page.waitForFunction(()=>window.pending.length===1);}
async function count(page){return page.evaluate(()=>window.writes.length);}

it.each([true,false])('workspace save disables immediately, rejects duplicate changes and recovers (success=%s)',async ok=>{
  const {browser,page,errors}=await openBrowser('workspaces');try{
    const box=page.getByRole('checkbox');await box.waitFor();
    await box.evaluate(el=>{el.click();el.click();});
    await page.waitForFunction(()=>window.writes.length>0);
    expect(await count(page)).toBe(1);expect(await box.isDisabled()).toBe(true);
    await box.evaluate(el=>el.click());expect(await count(page)).toBe(1);
    await finish(page,ok);await page.waitForFunction(()=>!document.querySelector('input[type=checkbox]').disabled);
    await box.click();await pending(page);expect(await count(page)).toBe(2);
    await finish(page,true);expect(errors).toEqual([]);
  }finally{await browser.close();}
});

it.each([true,false])('clear-login-lockout disables immediately and permits retry after settlement (success=%s)',async ok=>{
  const {browser,page,errors}=await openBrowser('users');try{
    await page.getByRole('button',{name:'Edit fixture user',exact:true}).click();
    const clear=page.getByRole('button',{name:'users.clearLoginLockout',exact:true});await clear.waitFor();
    await clear.evaluate(el=>{el.click();el.click();});await page.waitForFunction(()=>window.writes.length>0);
    expect(await count(page)).toBe(1);
    const busy=page.getByRole('button',{name:'users.clearingLoginLockout',exact:true});expect(await busy.isDisabled()).toBe(true);
    await busy.evaluate(el=>el.click());expect(await count(page)).toBe(1);
    await finish(page,ok);await clear.waitFor();expect(await clear.isDisabled()).toBe(false);
    await clear.click();await pending(page);expect(await count(page)).toBe(2);await finish(page,true);expect(errors).toEqual([]);
  }finally{await browser.close();}
});

it.each([['disable',true],['disable',false],['delete',true],['delete',false]])('stream %s shows pending, blocks dismissal and duplicate commit, then settles (success=%s)',async(action,ok)=>{
  const {browser,page,errors}=await openBrowser('streams');try{
    await page.getByRole('button',{name:'Camera actions',exact:true}).click();
    let commit;
    if(action==='disable'){
      await page.getByRole('button',{name:'common.disable',exact:true}).click();
      commit=page.getByRole('button',{name:'streams.disableStreamConfirm',exact:true});
    }else{
      await page.getByRole('button',{name:'common.delete',exact:true}).click();
      commit=page.getByRole('button',{name:'streams.yesDeletePermanently',exact:true});
    }
    await commit.evaluate(el=>{el.click();el.click();});await pending(page);expect(await count(page)).toBe(1);
    const heading=page.getByRole('heading',{name:action==='disable'?'streams.disablingStream':'streams.deletingStream',exact:true});await heading.waitFor();
    await page.keyboard.press('Escape');expect(await heading.count()).toBe(1);
    expect(await page.getByRole('button',{name:'common.delete',exact:true}).count()).toBe(0);
    await finish(page,ok);await heading.waitFor({state:'detached'});
    await page.getByRole('button',{name:'Camera actions',exact:true}).click();
    await page.getByRole('heading',{name:'streams.streamActions',exact:true}).waitFor();
    expect(await count(page)).toBe(1);expect(errors).toEqual([]);
  }finally{await browser.close();}
});
