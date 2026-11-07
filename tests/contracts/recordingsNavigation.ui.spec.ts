import {test,expect,ok} from './permissionCases';
import {seedRecordings} from './fixtures/recordings/seed';

const views=['Table','Grid','Timeline','Investigation','Plates'];
for (const width of [360,390,768,1440]) {
 test(`footage navigation and thumbnail-free grid at ${width}px`,async({server,browser},info)=>{
  seedRecordings(server);
  const config=await ok(server,'/api/client-config');
  const context=await server.browserContext(browser),page=await context.newPage();
  try {
   await page.setViewportSize({width,height:900});
   await page.addInitScript(()=>{localStorage.setItem('recordings_view_mode','table');localStorage.setItem('recordings_mobile_view_mode','grid');});
   await page.route('**/api/client-config',route=>route.fulfill({json:{...config,generate_thumbnails:false}}));
   const thumbnails:string[]=[];page.on('request',request=>{if(request.url().includes('/api/recordings/thumbnail/'))thumbnails.push(request.url());});
   await page.goto(`${server.baseURL}/recordings.html?view=grid&stream=A&sort=start_time&order=desc`);
   const nav=page.locator('.recordings-navigation');
   await expect(nav.getByRole('link')).toHaveText(views);
   await expect(nav.getByRole('link',{name:'Grid',exact:true})).toHaveAttribute('aria-current','page');
   await expect(page.locator('.recording-card').first()).toBeVisible();
   await expect(page.locator('.recording-card img')).toHaveCount(0);
   await expect.poll(()=>page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
   expect(thumbnails).toEqual([]);
   await page.getByRole('combobox',{name:'Sort by',exact:true}).selectOption('duration');
   await expect(page).toHaveURL(/sort=duration/);
   await page.getByRole('button',{name:'Reverse sort order'}).click();
   await expect(page).toHaveURL(/order=desc/);
   await page.screenshot({path:info.outputPath(`grid-no-thumbnails-${width}.png`)});
   const play=page.locator('.recording-card').getByRole('button',{name:'Play',exact:true}).filter({visible:true}).first();
   await play.click();
   const video=page.locator('#video-preview-modal video');
   await expect.poll(()=>video.evaluate(node=>(node as HTMLVideoElement).readyState)).toBeGreaterThanOrEqual(2);
   await video.evaluate(async node=>{const v=node as HTMLVideoElement;await v.play();});
   await expect.poll(()=>video.evaluate(node=>(node as HTMLVideoElement).currentTime)).toBeGreaterThan(0);
   await page.locator('#video-preview-modal').getByRole('button',{name:'Close',exact:true}).click();
   await expect(play).toBeFocused();
   await nav.getByRole('link',{name:'Table',exact:true}).click();
   await expect(nav.getByRole('link',{name:'Table',exact:true})).toHaveAttribute('aria-current','page');
   await expect(page).toHaveURL(/view=table/);
   await nav.getByRole('link',{name:'Grid',exact:true}).click();
   for (const name of ['Timeline','Investigation','Plates']) {
    await page.locator('.recordings-navigation').getByRole('link',{name,exact:true}).click();
    await expect(page.locator('.recordings-navigation').getByRole('link')).toHaveText(views);
    await expect(page.locator('.recordings-navigation').getByRole('link',{name,exact:true})).toHaveAttribute('aria-current','page');
    // Wait for camera summaries/initial selection before following the next view.
    const next=name==='Timeline'?'Investigation':name==='Investigation'?'Plates':'Timeline';
    await expect(page.locator('.recordings-navigation').getByRole('link',{name:next,exact:true})).toHaveAttribute('href',new RegExp(server.cameras.A.camera_uuid));
    if(name==='Plates')await expect(page.getByLabel('Camera',{exact:true})).toHaveValue(server.cameras.A.camera_uuid);
    await expect.poll(()=>page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
    await page.screenshot({path:info.outputPath(`${name.toLowerCase()}-${width}.png`)});
   }
   await page.locator('.recordings-navigation').getByRole('link',{name:'Grid',exact:true}).click();
   await expect(page).toHaveURL(/stream=A/);await expect(page).toHaveURL(/sort=duration/);
   await expect(page.locator('.recording-card').first()).toBeVisible();
   if(width<=768)expect(await page.evaluate(()=>localStorage.getItem('recordings_view_mode'))).toBe('table');
  }finally{await context.close();}
 });
}

test('recordings failure keeps filters and retry reachable, then recovers',async({server,browser})=>{
 seedRecordings(server);
 const context=await server.browserContext(browser),page=await context.newPage();
 try {
  await page.setViewportSize({width:390,height:844});
  let fail=true;
  await page.route('**/api/recordings?*',route=>fail?route.fulfill({status:503,json:{error:'Fixture unavailable'}}):route.continue());
  await page.goto(`${server.baseURL}/recordings.html?view=grid`);
  await expect(page.getByRole('alert')).toBeVisible();
  await expect(page.locator('.recordings-mobile-filters')).toBeVisible();
  fail=false;await page.getByRole('alert').getByRole('button',{name:'Retry',exact:true}).click();
  await expect(page.locator('.recording-card').first()).toBeVisible();
  await expect(page.getByRole('alert')).toHaveCount(0);
 }finally{await context.close();}
});

for (const action of ['Delete Selected', 'Delete All Filtered']) {
for (const dismissal of ['button', 'escape']) {
test(`thumbnail-free selection survives cancelling ${action} via ${dismissal}`,async({server,browser})=>{
 seedRecordings(server);
 const config=await ok(server,'/api/client-config');
 const context=await server.browserContext(browser),page=await context.newPage();
 try{
  await page.setViewportSize({width:390,height:844});
  await page.route('**/api/client-config',route=>route.fulfill({json:{...config,generate_thumbnails:false}}));
  await page.goto(`${server.baseURL}/recordings.html?view=grid&stream=A`);
  await page.getByRole('button',{name:'Select',exact:true}).click();
  await page.locator('.recording-card').first().getByRole('checkbox').check();
  await expect(page.getByText('1 recordings selected',{exact:true})).toBeVisible();
  const deleteRequests:string[]=[];
  page.on('request',request=>{if(request.method()==='POST'&&request.url().includes('/api/recordings/batch-delete'))deleteRequests.push(request.url());});
  const trigger=page.getByRole('button',{name:action,exact:true});
  await trigger.click();
  await expect(page.getByRole('dialog')).toBeVisible();
  await expect(page.getByRole('dialog').getByRole('button',{name:'Cancel',exact:true})).toBeFocused();
  if(dismissal==='escape')await page.keyboard.press('Escape');
  else await page.getByRole('dialog').getByRole('button',{name:'Cancel',exact:true}).click();
  await expect(page.getByRole('dialog')).toHaveCount(0);
  await expect(page.getByText('1 recordings selected',{exact:true})).toBeVisible();
  await expect(page.locator('.recording-card').first().getByRole('checkbox')).toBeChecked();
  await expect(trigger).toBeFocused();
  await expect(page.getByRole('button',{name:'Delete Selected',exact:true})).toBeEnabled();
  await page.getByRole('button',{name:'Cancel',exact:true}).click();
  await expect(page.locator('.recording-card input[type="checkbox"]')).toHaveCount(0);
  await expect(page.getByRole('button',{name:'Select',exact:true})).toBeVisible();
  expect((await ok(server,'/api/recordings?stream=A')).recordings.length).toBeGreaterThan(0);
  expect(deleteRequests).toEqual([]);
 }finally{await context.close();}
});
}
}

test('scoped phone viewer only sees allowed cameras and cannot delete footage',async({server,browser})=>{
 seedRecordings(server);
 const config=await ok(server,'/api/client-config');
 const context=await server.browserContext(browser,'viewer'),page=await context.newPage();
 try {
  await page.setViewportSize({width:390,height:844});
  await page.route('**/api/client-config',route=>route.fulfill({json:{...config,generate_thumbnails:false}}));
  await page.goto(`${server.baseURL}/recordings.html?view=grid`);
  await expect(page.locator('.recording-card')).toHaveCount(4);
  await expect(page.locator('.recording-card-heading [title="B"]')).toHaveCount(0);
  await expect(page.getByRole('button',{name:'Delete',exact:true})).toHaveCount(0);
  await expect(page.getByRole('button',{name:'Select',exact:true})).toHaveCount(0);
  await expect.poll(()=>page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
 }finally{await context.close();}
});

for (const destination of ['Timeline','Investigation']) {
 test(`selected footage returns from ${destination} with the original list scope`,async({server,browser})=>{
  seedRecordings(server);
  const context=await server.browserContext(browser),page=await context.newPage();
  try {
   await page.setViewportSize({width:390,height:844});
   await page.goto(`${server.baseURL}/recordings.html?view=grid&stream=A&tag=visible-tag&sort=duration&order=asc`);
   await expect(page.locator('.recording-card')).toHaveCount(1);
   await page.getByRole('button',{name:'Select',exact:true}).click();
   await page.locator('.recording-card').getByRole('checkbox').check();
   const original=new URL(page.url()).search;
   await page.getByRole('button',{name:new RegExp(`^.*${destination} \\(1\\)$`)}).click();
   await expect(page).toHaveURL(new RegExp(`/${destination.toLowerCase()}\\.html\\?`));
   if(destination==='Timeline') {
    expect(await page.evaluate(()=>sessionStorage.getItem('lightnvr_recordings_return_url'))).toBe(`recordings.html${original}`);
   } else {
    expect(new URL(page.url()).searchParams.get('return_to')).toBe(`recordings.html${original}`);
   }
   const back=page.locator('.recordings-navigation').getByRole('link',{name:'Grid',exact:true});
   await expect(back).toHaveAttribute('href',`recordings.html${original}`);
   await back.click();
   await expect(page.locator('.recording-card')).toHaveCount(1);
   expect(new URL(page.url()).search).toBe(original);
  } finally {await context.close();}
 });
}
