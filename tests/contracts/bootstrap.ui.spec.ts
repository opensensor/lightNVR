import {test,expect,ok} from './permissionCases';

for(const failure of ['unavailable','malformed'] as const){
 test(`live bootstrap preserves disabled transports and audio when setup status is ${failure}`,async({server,browser})=>{
  const config=await ok(server,'/api/client-config');
  const context=await server.browserContext(browser),page=await context.newPage();
  try{
   await page.route('**/api/client-config',route=>route.fulfill({json:{...config,go2rtc_enabled:true,audio_disabled:true,auto_disabled:true,webrtc_disabled:true,mse_disabled:true,hls_disabled:false}}));
   await page.route('**/api/setup/status',route=>failure==='unavailable'
    ?route.fulfill({status:503,json:{error:'Setup status temporarily unavailable'}})
    :route.fulfill({json:{complete:'do-not-use-malformed-status'}}));
   await page.goto(`${server.baseURL}/index.html`);
   await expect(page.locator('#live-page')).toBeVisible();
   const header=page.locator('#live-page .page-header');
   await expect(header.getByRole('link',{name:'WebRTC',exact:true})).toHaveCount(0);
   await expect(header.getByRole('link',{name:'MSE',exact:true})).toHaveCount(0);
   await expect(header.getByText('Auto',{exact:true})).toHaveCount(0);
   await expect(header.getByText('HLS',{exact:true})).toBeVisible();
   await expect(page.getByTestId('hls-view')).toBeVisible();
   await expect(page.getByRole('button',{name:'Unmute camera audio',exact:true})).toHaveCount(0);
  }finally{await context.close();}
 });
}
