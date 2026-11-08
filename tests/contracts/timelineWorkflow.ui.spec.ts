import {test,expect} from './permissionCases';
import {seedBusyDay} from './fixtures/recordings/busy-day';

for(const width of [390,1440]) {
 test(`busy-day timeline scrubbing preserves paused and playing intent at ${width}px`,async({server,browser},info)=>{
  const fixture=seedBusyDay(server),context=await server.browserContext(browser),page=await context.newPage();
  try {
   await page.setViewportSize({width,height:900});
   const cdp=await context.newCDPSession(page);await cdp.send('Emulation.setTimezoneOverride',{timezoneId:'UTC'});
   if(width===390)await cdp.send('Emulation.setTouchEmulationEnabled',{enabled:true});
   const response=page.waitForResponse(r=>r.url().includes('/api/timeline/segments?'));
   await page.goto(`${server.baseURL}/timeline.html?stream=A&date=${fixture.date}&time=08:00:05`);
   expect((await (await response).json()).segment_count).toBe(fixture.count);
   const video=page.locator('#video-player video'),cursor=page.getByTestId('timeline-cursor-hit-area');
   await expect.poll(()=>video.evaluate(v=>(v as HTMLVideoElement).readyState)).toBeGreaterThanOrEqual(2);
   await expect.poll(()=>video.evaluate(v=>(v as HTMLVideoElement).currentTime)).toBeCloseTo(5,0);
   await expect.poll(()=>page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
   if(width===390){
    const player=(await page.locator('#video-player').boundingBox())!;
    expect(player.y).toBeLessThan(230);expect(player.width).toBeGreaterThan(width*.94);
    expect((await page.locator('.timeline-primary-controls').boundingBox())!.height).toBeLessThan(60);
    await expect(page.locator('#timeline-player-options')).toBeHidden();
    await video.evaluate(v=>v.setAttribute('data-mounted-media','original'));
    await page.locator('#timeline-context-toggle').click();
    await expect(page.locator('#stream-selector')).toBeVisible();
    await page.locator('#stream-selector').press('Escape');
    await expect(page.locator('#timeline-context-toggle')).toBeFocused();
    await page.locator('#timeline-options-toggle').click();
    await expect(page.locator('#timeline-player-options')).toBeVisible();
    await expect(video).toHaveAttribute('data-mounted-media','original');
   }
   for(let i=0;i<12;i++)if(await page.locator('#zoom-in-button').isEnabled())await page.locator('#zoom-in-button').click();
   if(width===390)await page.locator('#timeline-options-toggle').click();
   const bar=page.locator('.timeline-segment').filter({hasNot:page.locator('.in-progress')}).first();
   const assertStablePreview=async()=>{
    const labels=await page.locator('#time-display').evaluate(async el=>{
     const samples=[];
     for(let i=0;i<6;i++){samples.push(el.textContent);await new Promise(resolve=>setTimeout(resolve,100));}
     return samples;
    });
    expect(new Set(labels).size).toBe(1);
   };
   const seek=async(seconds:number,hold=false)=>{
    await cursor.scrollIntoViewIfNeeded();
    const hit=(await cursor.boundingBox())!,line=(await page.getByTestId('timeline-cursor-line').boundingBox())!,segment=(await bar.boundingBox())!;
    const x=hit.x+hit.width/2,y=hit.y+hit.height/2,target=segment.x+seconds*segment.width/30+(x-line.x-line.width/2);
    if(width===390) {
     await cdp.send('Input.dispatchTouchEvent',{type:'touchStart',touchPoints:[{x,y,id:1}]});
     await cdp.send('Input.dispatchTouchEvent',{type:'touchMove',touchPoints:[{x:target,y,id:1}]});
     if(hold)await assertStablePreview();
     await cdp.send('Input.dispatchTouchEvent',{type:'touchEnd',touchPoints:[]});
    } else {
     await page.mouse.move(x,y);await page.mouse.down();await page.mouse.move(target,y,{steps:5});if(hold)await assertStablePreview();await page.mouse.up();
    }
   };
   await seek(15);
   await expect.poll(()=>video.evaluate(v=>(v as HTMLVideoElement).currentTime)).toBeCloseTo(15,0);
   expect(await video.evaluate(v=>(v as HTMLVideoElement).paused)).toBe(true);
   await page.locator('#play-button').click();
   await expect.poll(()=>video.evaluate(v=>(v as HTMLVideoElement).paused)).toBe(false);
   await seek(8,true);
   await expect.poll(()=>video.evaluate(v=>(v as HTMLVideoElement).currentTime)).toBeLessThan(12);
   await expect.poll(()=>video.evaluate(v=>(v as HTMLVideoElement).paused)).toBe(false);
   await page.locator('#play-button').click();
   // A gap resolves to actual footage and explains the move without starting playback.
   if(width===390)await page.locator('#timeline-options-toggle').click();
   await page.locator('#zoom-out-button').click();await page.locator('#zoom-out-button').click();
   if(width===390)await page.locator('#timeline-options-toggle').click();
   await seek(60);
   await expect(page.getByText('No recording at that time. Moved to the nearest available footage.',{exact:true})).toBeVisible();
   await expect.poll(()=>video.evaluate(v=>(v as HTMLVideoElement).currentTime)).toBeCloseTo(30,0);
   expect(await video.evaluate(v=>(v as HTMLVideoElement).paused)).toBe(true);
   expect(await video.evaluate(v=>(v as HTMLVideoElement).currentSrc)).toContain('/1000?');
   // A secondary mouse button must not seek the recording track.
   const segment=(await bar.boundingBox())!;
   await page.mouse.click(segment.x+segment.width/3,segment.y+segment.height/2,{button:'right'});
   expect(await video.evaluate(v=>(v as HTMLVideoElement).currentTime)).toBeCloseTo(30,0);
   // Continuing from the end frame advances naturally when the user presses Play.
   await page.locator('#play-button').click();
   await expect.poll(()=>video.evaluate(v=>(v as HTMLVideoElement).currentSrc)).toContain('/1001?');
   await expect.poll(()=>video.evaluate(v=>(v as HTMLVideoElement).paused)).toBe(false);
   // paused flips as soon as play() is requested; the player seeks to the cursor once the
   // new segment's metadata loads, so only jump near its end after playback is under way.
   await expect.poll(()=>video.evaluate(v=>{const e=v as HTMLVideoElement;return e.readyState>=2&&e.currentTime>0;})).toBe(true);
   await video.evaluate(v=>{(v as HTMLVideoElement).currentTime=29.8;});
   await expect.poll(()=>video.evaluate(v=>(v as HTMLVideoElement).currentSrc)).toContain('/1002?');
   await expect.poll(()=>video.evaluate(v=>(v as HTMLVideoElement).paused)).toBe(false);
   await page.locator('#play-button').click();
   await expect(page.getByText('No recording at that time. Moved to the nearest available footage.',{exact:true})).toBeHidden();
   await page.evaluate(()=>window.scrollTo(0,0));
   await page.screenshot({path:info.outputPath(`timeline-${width}.png`)});
   if(width===390){
    const box=await cursor.boundingBox();expect(box!.width).toBeGreaterThanOrEqual(44);expect(box!.height).toBeGreaterThanOrEqual(44);
    await page.emulateMedia({reducedMotion:'reduce'});
    await page.locator('#timeline-container').scrollIntoViewIfNeeded();
    const ruler=page.locator('.timeline-ruler'),track=(await page.locator('.timeline-segments').boundingBox())!;
    const before=Number(await ruler.getAttribute('data-timeline-start-hour')),mediaTime=await video.evaluate(v=>(v as HTMLVideoElement).currentTime);
    const y=track.y+track.height/2;
    await cdp.send('Input.dispatchTouchEvent',{type:'touchStart',touchPoints:[{x:track.x+track.width*.75,y,id:1}]});
    await cdp.send('Input.dispatchTouchEvent',{type:'touchMove',touchPoints:[{x:track.x+track.width*.25,y,id:1}]});
    await cdp.send('Input.dispatchTouchEvent',{type:'touchEnd',touchPoints:[]});
    await expect.poll(async()=>Number(await ruler.getAttribute('data-timeline-start-hour'))).toBeGreaterThan(before);
    const range=async()=>Number(await ruler.getAttribute('data-timeline-end-hour'))-Number(await ruler.getAttribute('data-timeline-start-hour'));
    const beforePinch=await range();
    await cdp.send('Input.dispatchTouchEvent',{type:'touchStart',touchPoints:[{x:track.x+track.width*.25,y,id:1},{x:track.x+track.width*.75,y,id:2}]});
    await cdp.send('Input.dispatchTouchEvent',{type:'touchMove',touchPoints:[{x:track.x+track.width*.4,y,id:1},{x:track.x+track.width*.6,y,id:2}]});
    await cdp.send('Input.dispatchTouchEvent',{type:'touchEnd',touchPoints:[]});
    await expect.poll(range).toBeGreaterThan(beforePinch);
    expect(await video.evaluate(v=>(v as HTMLVideoElement).currentTime)).toBeCloseTo(mediaTime,1);
    expect(await video.evaluate(v=>(v as HTMLVideoElement).paused)).toBe(true);
    for(const viewportWidth of [360,768]) {
     await page.setViewportSize({width:viewportWidth,height:900});
     expect((await page.locator('#video-player').boundingBox())!.width).toBeGreaterThan(viewportWidth*.94);
     expect((await page.locator('.timeline-primary-controls').boundingBox())!.height).toBeLessThan(60);
     await expect.poll(()=>page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
    }
    await page.setViewportSize({width:390,height:900});
    await page.getByRole('button',{name:'Enter Fullscreen',exact:true}).click();
    await expect.poll(()=>page.evaluate(()=>document.fullscreenElement?.getAttribute('data-testid'))).toBe('timeline-video-container');
    await page.getByRole('button',{name:'Exit Fullscreen',exact:true}).click();
    await expect.poll(()=>page.evaluate(()=>document.fullscreenElement===null)).toBe(true);
    await expect(video).toHaveAttribute('data-mounted-media','original');
    await page.evaluate(()=>{document.documentElement.dir='rtl';document.documentElement.style.fontSize='125%';});
    await expect.poll(()=>page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
    await page.evaluate(()=>window.scrollTo(0,0));
    await page.screenshot({path:info.outputPath('timeline-rtl-large-text.png')});
    await page.evaluate(()=>{document.documentElement.dir='ltr';document.documentElement.style.fontSize='';});
    await page.setViewportSize({width:844,height:390});
    await page.locator('.timeline-workspace').evaluate(el=>window.scrollTo({top:window.scrollY+el.getBoundingClientRect().top-8,behavior:'instant'}));
    const landscape=(await page.locator('#video-player').boundingBox())!;
    expect(landscape.y).toBeGreaterThanOrEqual(0);expect(landscape.y+landscape.height).toBeLessThanOrEqual(390);
    expect(landscape.width).toBeGreaterThan(844*.65);expect(landscape.height).toBeGreaterThan(390*.65);
    await expect.poll(()=>page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
    await expect(video).toHaveAttribute('data-mounted-media','original');
    expect(await video.evaluate(v=>(v as HTMLVideoElement).paused)).toBe(true);
    await page.screenshot({path:info.outputPath('timeline-landscape.png')});
   }
  }finally{await context.close();}
 });
}

test('scoped busy-day investigation and interrupted footage remain usable after rotation',async({server,browser},info)=>{
 const fixture=seedBusyDay(server),context=await server.browserContext(browser,'viewer'),page=await context.newPage();
 try {
  await page.setViewportSize({width:390,height:844});
  const cdp=await context.newCDPSession(page);await cdp.send('Emulation.setTimezoneOverride',{timezoneId:'UTC'});
  await cdp.send('Emulation.setTouchEmulationEnabled',{enabled:true});
  await page.goto(`${server.baseURL}/timeline.html?stream=A&date=${fixture.date}&time=08:00:05`);
  await expect(page.locator('#stream-selector')).toHaveValue('A');
  await expect(page.locator('#stream-selector option[value="B"]')).toHaveCount(0);
  await page.locator('.recordings-navigation').getByRole('link',{name:'Investigation',exact:true}).click();
  await expect(page.locator('.investigation-player video')).toHaveCount(1);
  const video=page.locator('.investigation-player video');
  await expect.poll(()=>video.evaluate(v=>(v as HTMLVideoElement).readyState)).toBeGreaterThanOrEqual(2);
  await expect(page.getByRole('group',{name:'Camera selection',exact:true}).getByRole('checkbox',{name:'A',exact:true})).toBeChecked();
  await expect(page.getByRole('checkbox',{name:'B',exact:true})).toHaveCount(0);
  await page.locator('.investigation-play-button').click();
  await expect.poll(()=>video.evaluate(v=>(v as HTMLVideoElement).paused)).toBe(false);
  const shared=page.locator('.investigation-scrubber');
  await shared.focus();await shared.press('ArrowRight');
  await expect.poll(()=>video.evaluate(v=>(v as HTMLVideoElement).paused)).toBe(false);
  const slider=page.locator('.investigation-player-controls input[type="range"]');
  await slider.scrollIntoViewIfNeeded();const rect=(await slider.boundingBox())!,y=rect.y+rect.height/2;
  await cdp.send('Input.dispatchTouchEvent',{type:'touchStart',touchPoints:[{x:rect.x+rect.width/3,y,id:1}]});
  await cdp.send('Input.dispatchTouchEvent',{type:'touchMove',touchPoints:[{x:rect.x+rect.width/2,y,id:1}]});
  await cdp.send('Input.dispatchTouchEvent',{type:'touchEnd',touchPoints:[]});
  await expect.poll(()=>video.evaluate(v=>(v as HTMLVideoElement).paused)).toBe(false);
  await expect.poll(()=>video.evaluate(v=>(v as HTMLVideoElement).currentTime)).toBeGreaterThan(10);
  await page.locator('.investigation-play-button').click();
  await page.setViewportSize({width:844,height:390});
  await expect.poll(()=>page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
  await page.screenshot({path:info.outputPath('investigation-landscape.png')});
  await page.goto(`${server.baseURL}/timeline.html?stream=A&date=${fixture.date}&time=18:00:05`);
  await expect(page.locator('#video-player [role="status"]')).toContainText('Playback is unavailable for this interrupted recording');
  await page.locator('#timeline-options-toggle').click();
  await expect(page.getByRole('button',{name:'Previous recording',exact:true})).toBeEnabled();
  await page.getByRole('button',{name:'Previous recording',exact:true}).click();
  await expect.poll(()=>page.locator('#video-player video').evaluate(v=>(v as HTMLVideoElement).readyState)).toBeGreaterThanOrEqual(2);
  await expect(page.locator('#video-player [role="status"]')).toHaveCount(0);
 }finally{await context.close();}
});
