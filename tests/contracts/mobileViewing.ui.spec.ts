import {test, expect, ok, admin} from './permissionCases';
import type {Browser} from '@playwright/test';
import {credentials} from './fixture-server.mjs';
import type {Server} from './permissionCases';
import {seedRecordings} from './fixtures/recordings/seed';

async function phoneContext(server: Server, browser: Browser) {
  const context = await browser.newContext({isMobile:true,hasTouch:true});
  const response = await context.request.post(`${server.baseURL}/api/auth/login`, {data:credentials.admin});
  if (!response.ok()) { await context.close(); throw new Error(`Mobile login failed: ${response.status()}`); }
  return context;
}

for (const width of [360, 390, 768]) {
  test(`mobile live ${width}px: camera-first controls, navigation and stable players`, async ({server,browser}, info) => {
    for (const name of ['A','B']) await ok(server, `/api/streams/${name}`, admin, 'PUT', {enabled:true,streaming_enabled:true,record:false});
    const context = await phoneContext(server,browser), page = await context.newPage();
    try {
      page.setDefaultTimeout(10000);
      await page.setViewportSize({width,height:844});
      await page.addInitScript(() => {
        localStorage.setItem('lightnvr-live-availability','all');
        localStorage.setItem('lightnvr-live-navigator-collapsed','false');
        localStorage.setItem('lightnvr-theme','dark');
      });
      await page.goto(`${server.baseURL}/index.html`);
      const grid=page.locator('#video-grid'), options=page.getByRole('button',{name:'View options'});
      await expect(grid.locator('.video-cell').first()).toBeVisible();
      await expect(options).toHaveAttribute('aria-expanded','false');
      await expect(page.locator('#live-transport-options')).toBeHidden();
      const initial=await grid.boundingBox();
      expect(initial!.y).toBeLessThan(260);
      expect(initial!.width).toBeGreaterThan(width-32);
      const video=await grid.locator('video').first().elementHandle();
      await page.screenshot({path:info.outputPath(`live-${width}.png`)});
      await options.tap();
      await expect(page.locator('#live-transport-options')).toBeVisible();
      const names=page.getByRole('button',{name:'Camera names',exact:true});
      await names.tap();
      await expect.poll(()=>page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
      await page.screenshot({path:info.outputPath(`options-${width}.png`)});
      await expect(names).toHaveAttribute('aria-pressed','false');
      await page.keyboard.press('Escape');
      await expect(options).toBeFocused();
      await expect(options).toHaveAttribute('aria-expanded','false');
      expect(await video!.evaluate(node=>node===document.querySelector('#video-grid video'))).toBe(true);
      await expect.poll(()=>page.evaluate(()=>document.documentElement.scrollWidth<=innerWidth)).toBe(true);
      await page.getByRole('combobox',{name:'Cameras per page'}).selectOption('4');
      const menu=page.getByRole('button',{name:'Toggle menu'});
      await menu.click();
      await expect(menu).toHaveAttribute('aria-expanded','true');
      const after=await grid.boundingBox();expect(after!.y).toBe(initial!.y);
      await expect(page.locator('#mobile-navigation').getByRole('link',{name:'Recordings',exact:true})).toBeVisible();
      await page.keyboard.press('Escape');
      await expect(menu).toBeFocused();
      await expect(page.locator('#mobile-navigation')).toHaveCount(0);
      await menu.tap();
      await expect(page.locator('#mobile-navigation')).toBeVisible();
      const menuBox = await page.locator('#mobile-navigation').boundingBox();
      await page.touchscreen.tap(width / 2, menuBox!.y + menuBox!.height + 12);
      await expect(page.locator('#mobile-navigation')).toHaveCount(0);
      expect(await page.evaluate(()=>localStorage.getItem('lightnvr-live-navigator-collapsed'))).toBe('false');
    } finally {await context.close();}
  });
}

test('desktop live retains expanded toolbar and selected layout',async({server,browser})=>{
 const context=await server.browserContext(browser),page=await context.newPage();
 try {
  page.setDefaultTimeout(10000);
  await page.setViewportSize({width:1440,height:900});
  await page.goto(`${server.baseURL}/index.html?cols=2&rows=2`);
  await expect(page.locator('#live-page')).toBeVisible();
  await expect(page.locator('.live-mobile-options')).toBeHidden();
  await expect(page.locator('#live-transport-options')).toBeVisible();
  await expect(page.getByRole('button',{name:'Camera names',exact:true})).toBeVisible();
  await expect(page.locator('#video-grid')).toHaveCSS('grid-template-columns',/^\d+(\.\d+)?px \d+(\.\d+)?px$/);
 }finally{await context.close();}
});

test('phone recordings: discover filters, play actual media inline, rotate and return',async({server,browser},info)=>{
 seedRecordings(server);
 const settings=await ok(server,'/api/settings');
 const context=await phoneContext(server,browser),page=await context.newPage();
 try {
  page.setDefaultTimeout(10000);
  await page.setViewportSize({width:390,height:844});
  await page.addInitScript(()=>{localStorage.setItem('recordings_view_mode','table');localStorage.setItem('recordings_filters_collapsed','false');});
  await page.route('**/api/settings',route=>route.fulfill({json:{...settings,generate_thumbnails:true}}));
  await page.goto(`${server.baseURL}/recordings.html`);
  await expect(page.getByRole('tab',{name:'Grid',exact:true})).toHaveAttribute('aria-selected','true');
  await expect(page.locator('#filters-sidebar')).toHaveCount(0);
  await expect(page.locator('.recording-card').first()).toBeVisible();
  const filters=page.locator('.recordings-mobile-filters');await filters.click();
  await expect(page.locator('#filters-sidebar')).toBeVisible();
  await page.locator('#apply-filters-btn').click();
  await expect(filters).toBeFocused();await expect(page.locator('#filters-sidebar')).toHaveCount(0);
  await page.screenshot({path:info.outputPath('recordings-phone.png')});
  const play=page.locator('.recording-card').getByRole('button',{name:'Play',exact:true}).filter({visible:true}).first();
  await play.click();
  const modal=page.locator('#video-preview-modal'),video=modal.locator('video');
  await expect(modal.getByRole('dialog')).toBeVisible();
  await expect(video).toHaveAttribute('playsinline','');
  const close=modal.getByRole('button',{name:'Close',exact:true});
  await expect(close).toBeFocused();await page.keyboard.press('Shift+Tab');
  expect(await modal.evaluate(node=>node.contains(document.activeElement))).toBe(true);
  await expect.poll(()=>video.evaluate(node=>(node as HTMLVideoElement).readyState)).toBeGreaterThanOrEqual(2);
  await video.evaluate(async node=>{const player=node as HTMLVideoElement;player.currentTime=0;await player.play();});
  await expect.poll(()=>video.evaluate(node=>(node as HTMLVideoElement).currentTime)).toBeGreaterThan(0.1);
  await modal.locator('[data-speed="2"]').tap();
  await expect(modal.locator('[data-speed="2"]')).toHaveAttribute('aria-pressed','true');
  expect(await video.evaluate(node=>(node as HTMLVideoElement).playbackRate)).toBe(2);
  await page.screenshot({path:info.outputPath('playback-phone.png')});
  await page.setViewportSize({width:844,height:390});
  const box=await video.boundingBox();expect(box!.height).toBeGreaterThan(150);expect(box!.y+box!.height).toBeLessThanOrEqual(390);
  await page.screenshot({path:info.outputPath('playback-landscape.png')});
  await modal.getByRole('button',{name:'Close',exact:true}).click();
  await expect(modal).toHaveCount(0);await expect(play).toBeFocused();
  expect(await page.evaluate(()=>localStorage.getItem('recordings_view_mode'))).toBe('table');
  expect(await page.evaluate(()=>localStorage.getItem('recordings_filters_collapsed'))).toBe('false');
 }finally{await context.close();}
});
