/*
 * Браузерная проверка мобильного Live View с моками API.
 * Запуск: node tests/e2e/mobile-favorites.browser.cjs
 * Нужен Playwright в node_modules или переменная PLAYWRIGHT_MODULE.
 */
const assert = require('node:assert/strict');
const fs = require('node:fs');
const path = require('node:path');
const os = require('node:os');
const { chromium } = require(process.env.PLAYWRIGHT_MODULE || 'playwright');

function luminance(color) {
  const channels = color.match(/\d+(?:\.\d+)?/g).slice(0, 3).map((value) => Number(value) / 255);
  const linear = channels.map((channel) => channel <= 0.04045 ? channel / 12.92 : ((channel + 0.055) / 1.055) ** 2.4);
  return 0.2126 * linear[0] + 0.7152 * linear[1] + 0.0722 * linear[2];
}

function contrastRatio(foreground, background) {
  const a = luminance(foreground);
  const b = luminance(background);
  return (Math.max(a, b) + 0.05) / (Math.min(a, b) + 0.05);
}

const baseUrl = process.env.MOBILE_BROWSER_BASE_URL || 'http://127.0.0.1:4173/';
const artifactDir = process.env.MOBILE_BROWSER_ARTIFACTS || path.join(os.tmpdir(), 'nvr-mobile-browser');
const streams = Array.from({ length: 80 }, (_, index) => ({
  camera_uuid: `00000000-0000-4000-8000-${String(index + 1).padStart(12, '0')}`,
  name: `Камера ${String(index + 1).padStart(2, '0')}`,
  enabled: true,
  streaming_enabled: true,
  status: 'playing',
  connection_status: 'connected',
  playback_transport: 'webrtc',
  availability: 'live',
}));

async function main() {
  fs.mkdirSync(artifactDir, { recursive: true });
  const browser = await chromium.launch({ headless: true });
  try {
    const context = await browser.newContext({ viewport: { width: 390, height: 844 }, isMobile: true, hasTouch: true });
    const page = await context.newPage();
    const inaccessibleUuid = '00000000-0000-4000-8000-999999999999';
    let releasePreferences;
    const preferencesGate = new Promise((resolve) => { releasePreferences = resolve; });
    let releaseIce;
    const iceGate = new Promise((resolve) => { releaseIce = resolve; });
    const state = { mode: 'auto', favorites: [{ camera_uuid: inaccessibleUuid, created_at: 1790899200 }], failNextFavorite: false, delayPreferences: true, delayIce: false };
    const requests = [];
    const transportRequests = [];
    const iceRequests = [];
    await page.addInitScript(() => {
      const NativePeerConnection = window.RTCPeerConnection;
      const active = new Set();
      const closed = new WeakSet();
      window.__rtcMountCounts = { active: 0, maximum: 0, created: 0, closed: 0, closeCalls: 0 };
      window.RTCPeerConnection = class CountedPeerConnection extends NativePeerConnection {
        constructor(...args) {
          super(...args);
          active.add(this);
          window.__rtcMountCounts.created += 1;
          window.__rtcMountCounts.active = active.size;
          window.__rtcMountCounts.maximum = Math.max(window.__rtcMountCounts.maximum, active.size);
          const close = this.close.bind(this);
          this.close = (...closeArgs) => {
            window.__rtcMountCounts.closeCalls += 1;
            if (!closed.has(this)) {
              closed.add(this);
              window.__rtcMountCounts.closed += 1;
              active.delete(this);
              window.__rtcMountCounts.active = active.size;
            }
            return close(...closeArgs);
          };
        }
      };
    });
    await page.route('**/*', async (route) => {
      const request = route.request();
      const url = new URL(request.url());
      if (url.pathname === '/go2rtc/api/webrtc') {
        transportRequests.push({ method: request.method(), path: url.pathname });
        const fakeAnswer = (request.postData() || '')
          .replace(/a=setup:actpass/g, 'a=setup:active')
          .replace(/a=sendrecv/g, 'a=recvonly');
        return route.fulfill({ status: 200, contentType: 'application/sdp', body: fakeAnswer });
      }
      if (!url.pathname.startsWith('/api/')) return route.continue();
      requests.push({ method: request.method(), path: url.pathname });
      if (url.pathname === '/api/ice-servers') {
        iceRequests.push(Date.now());
        if (state.delayIce) await iceGate;
        return route.fulfill({ status: 200, contentType: 'application/json', body: '[]' });
      }
      let payload;
      let status = 200;
      if (url.pathname === '/api/auth/verify') {
        payload = { authenticated: true, auth_enabled: true, demo_mode: false, id: 17, username: 'browser-test', role: 'admin' };
      } else if (url.pathname === '/api/client-config') {
        payload = { audioDisabled: true, autoDisabled: false, webrtcDisabled: false, hlsDisabled: true, mseDisabled: true, go2rtc_api_port: 4173, go2rtc_enabled: true };
      } else if (url.pathname === '/api/setup/status') {
        payload = { complete: true };
      } else if (url.pathname === '/api/ui/preferences') {
        if (request.method() === 'GET' && state.delayPreferences) await preferencesGate;
        if (request.method() === 'PUT') state.mode = JSON.parse(request.postData() || '{}').ui_mode;
        payload = { ui_mode: state.mode, configurable: true };
      } else if (url.pathname === '/api/ui/favorites' && request.method() === 'GET') {
        payload = { favorites: state.favorites, configurable: true };
      } else if (url.pathname === '/api/ui/favorites' && request.method() === 'POST') {
        if (state.failNextFavorite) {
          state.failNextFavorite = false;
          status = 500;
          payload = { error: 'test failure' };
        } else {
          const { camera_uuid: cameraUuid } = JSON.parse(request.postData() || '{}');
          const existing = state.favorites.find((item) => item.camera_uuid === cameraUuid);
          payload = existing || { camera_uuid: cameraUuid, created_at: Math.floor(Date.now() / 1000) };
          if (!existing) state.favorites.push(payload);
        }
      } else if (url.pathname.startsWith('/api/ui/favorites/') && request.method() === 'DELETE') {
        const cameraUuid = decodeURIComponent(url.pathname.split('/').pop());
        state.favorites = state.favorites.filter((item) => item.camera_uuid !== cameraUuid);
        payload = { success: true };
      } else if (url.pathname === '/api/streams') {
        payload = { streams, total_pages: 1, page: 1, page_size: 100 };
      } else {
        payload = {};
      }
      return route.fulfill({ status, contentType: 'application/json', body: JSON.stringify(payload) });
    });

    await page.goto(baseUrl);
    await page.waitForTimeout(300);
    assert.equal(await page.locator('.mobile-camera-card .video-cell video').count(), 0, 'плееры не должны монтироваться до ответа user preferences');
    state.delayPreferences = false;
    releasePreferences();
    await page.getByRole('tab', { name: 'Все камеры' }).waitFor({ state: 'visible' });
    await page.locator('.mobile-camera-card').first().waitFor({ state: 'visible' });
    assert.equal(await page.locator('.mobile-camera-card').count(), 4, 'на странице должны быть максимум четыре карточки');
    await page.waitForTimeout(500);
    const initialMounts = await page.locator('.mobile-camera-card .video-cell video').count();
    const initialTransportRequests = transportRequests.length;
    const rtcCounts = await page.evaluate(() => window.__rtcMountCounts);
    assert.ok(initialMounts > 0, 'видимая камера должна монтировать настоящий transport component с video element');
    assert.ok(initialMounts <= 4, `в DOM монтируется больше четырёх video elements: ${initialMounts}`);
    assert.ok(initialTransportRequests <= 4, `создано больше четырёх transport-запросов до смены страницы: ${initialTransportRequests}`);
    assert.ok(rtcCounts.maximum <= 4, `больше четырёх RTCPeerConnection были активны одновременно: ${JSON.stringify(rtcCounts)}`);
    const modeSelect = page.locator('#ui-mode-select');
    assert.equal(await modeSelect.inputValue(), 'auto');
    const lightTab = await page.locator('.mobile-live-tabs button[aria-selected="true"]').evaluate((element) => {
      const style = getComputedStyle(element);
      return { color: style.color, background: style.backgroundColor };
    });
    const lightTabContrast = contrastRatio(lightTab.color, lightTab.background);
    assert.ok(lightTabContrast >= 4.5, `контраст выбранной вкладки в светлой теме ${lightTabContrast.toFixed(2)} ниже 4.5:1`);
    await page.screenshot({ path: path.join(artifactDir, '01-phone-portrait.png'), fullPage: true });

    const star = page.getByRole('button', { name: 'Добавить Камера 01 в избранное' });
    await star.click();
    await page.getByRole('button', { name: 'Убрать Камера 01 из избранного' }).waitFor({ state: 'visible' });
    await page.reload();
    await page.getByRole('button', { name: 'Убрать Камера 01 из избранного' }).waitFor({ state: 'visible' });
    await page.getByRole('button', { name: 'Убрать Камера 01 из избранного' }).click();
    await page.getByRole('button', { name: 'Добавить Камера 01 в избранное' }).waitFor({ state: 'visible' });
    await page.getByRole('button', { name: 'Добавить Камера 01 в избранное' }).click();
    await page.getByRole('button', { name: 'Убрать Камера 01 из избранного' }).waitFor({ state: 'visible' });
    state.favorites = streams.slice(0, 5).map((stream, index) => ({ camera_uuid: stream.camera_uuid, created_at: 1790899200 + (4 - index) }));
    await page.reload();
    await page.getByRole('tab', { name: 'Избранное' }).click();
    await page.waitForFunction(() => JSON.stringify([...document.querySelectorAll('.mobile-camera-heading h2')].map((heading) => heading.textContent)) === JSON.stringify(['Камера 05', 'Камера 04', 'Камера 03', 'Камера 02']));
    assert.deepEqual(
      await page.locator('.mobile-camera-heading h2').allTextContents(),
      ['Камера 05', 'Камера 04', 'Камера 03', 'Камера 02'],
      'числовой created_at должен сортировать камеры по времени добавления, а не UUID',
    );
    await page.getByRole('button', { name: 'Дальше' }).click();
    await page.getByRole('button', { name: 'Убрать Камера 01 из избранного' }).click();
    await page.locator('.mobile-pagination').waitFor({ state: 'detached' });
    assert.deepEqual(await page.locator('.mobile-camera-heading h2').allTextContents(), ['Камера 05', 'Камера 04', 'Камера 03', 'Камера 02']);
    state.favorites = [{ camera_uuid: inaccessibleUuid, created_at: 1790899200 }];
    await page.reload();
    await page.getByRole('tab', { name: 'Избранное' }).click();
    await page.getByRole('heading', { name: 'В избранном пока пусто' }).waitFor({ state: 'visible' });
    assert.equal(await page.locator('.mobile-camera-card').count(), 0, 'недоступная избранная камера не должна отображаться');
    await page.getByRole('tab', { name: 'Все камеры' }).click();
    state.failNextFavorite = true;
    await page.getByRole('button', { name: 'Добавить Камера 02 в избранное' }).click();
    await page.getByRole('alert').filter({ hasText: 'Не удалось обновить избранное' }).waitFor({ state: 'visible' });

    await page.locator('.mobile-camera-card').first().waitFor({ state: 'visible' });
    await page.waitForFunction(() => document.querySelector('.mobile-pagination button:last-child') && !document.querySelector('.mobile-pagination button:last-child').disabled);
    await page.getByRole('button', { name: 'Дальше' }).click();
    assert.deepEqual(await page.locator('.mobile-camera-heading h2').allTextContents(), ['Камера 05', 'Камера 06', 'Камера 07', 'Камера 08']);
    await page.waitForTimeout(300);
    const afterPageChange300ms = await page.evaluate(() => window.__rtcMountCounts);
    await page.waitForTimeout(1700);
    const afterPageChange2s = await page.evaluate(() => window.__rtcMountCounts);
    const nextPageMounts = await page.locator('.mobile-camera-card .video-cell video').count();
    const afterPageChangeRtcCounts = await page.evaluate(() => window.__rtcMountCounts);
    assert.ok(nextPageMounts > 0 && nextPageMounts <= 4 && afterPageChange2s.active <= 4,
      `после перехода страницы активны лишние плееры: mounts=${nextPageMounts}, rtc=${JSON.stringify(afterPageChangeRtcCounts)}`);
    await page.getByRole('tab', { name: 'Все камеры' }).click();
    const previousPageButton = page.getByRole('button', { name: 'Назад' });
    while (await previousPageButton.isEnabled()) {
      await previousPageButton.click();
      await page.waitForTimeout(100);
    }
    for (let turn = 0; turn < 19; turn += 1) {
      await page.getByRole('button', { name: 'Дальше' }).click();
      await page.waitForTimeout(150);
    }
    for (let turn = 0; turn < 19; turn += 1) {
      await page.getByRole('button', { name: 'Назад' }).click();
      await page.waitForTimeout(100);
    }
    await page.waitForTimeout(1850);
    const after20PageTransitions = await page.evaluate(() => window.__rtcMountCounts);
    assert.ok(after20PageTransitions.active <= 4, `после 20 переходов накопились активные плееры: ${JSON.stringify(after20PageTransitions)}`);
    state.delayIce = true;
    const iceRequestBaseline = iceRequests.length;
    await page.getByRole('button', { name: 'Дальше' }).click();
    for (let poll = 0; poll < 25 && iceRequests.length <= iceRequestBaseline; poll += 1) await page.waitForTimeout(20);
    assert.ok(iceRequests.length > iceRequestBaseline, 'ожидался запрос ICE во время смены страницы');
    await page.getByRole('button', { name: 'Назад' }).click();
    await page.waitForTimeout(100);
    releaseIce();
    state.delayIce = false;
    await page.waitForTimeout(2000);
    const afterLateIceResolution = await page.evaluate(() => window.__rtcMountCounts);
    const lateIceVideoMounts = await page.locator('.mobile-camera-card .video-cell video').count();
    assert.ok(afterLateIceResolution.active <= 4, `late ICE response оставил лишний RTCPeerConnection: ${JSON.stringify(afterLateIceResolution)}`);
    assert.equal(afterLateIceResolution.created - afterLateIceResolution.closed, afterLateIceResolution.active,
      `каждый созданный RTCPeerConnection после ICE unmount должен быть либо активен, либо закрыт ровно один раз: ${JSON.stringify(afterLateIceResolution)}`);
    assert.equal(afterLateIceResolution.active, lateIceVideoMounts,
      `после ICE unmount число активных соединений должно совпасть с текущими video elements: ${JSON.stringify({ afterLateIceResolution, lateIceVideoMounts })}`);
    await page.locator('.mobile-camera-heading h2').filter({ hasText: 'Камера 01' }).waitFor({ state: 'visible' });

    await modeSelect.selectOption('mobile');
    await page.waitForFunction(() => document.querySelector('#ui-mode-select')?.value === 'mobile');
    await page.setViewportSize({ width: 1440, height: 900 });
    await page.waitForTimeout(150);
    assert.equal(await page.locator('.mobile-camera-card').count(), 4, 'явный мобильный режим сохраняется на широком экране');
    const explicitMobileMetrics = await page.locator('.mobile-camera-card').first().evaluate((card) => {
      const player = card.querySelector('.mobile-camera-player');
      const video = player?.querySelector('.video-cell');
      const media = video?.querySelector('video');
      return { cardHeight: card.getBoundingClientRect().height, playerHeight: player?.getBoundingClientRect().height, videoHeight: video?.getBoundingClientRect().height, mediaHeight: media?.getBoundingClientRect().height, width: card.getBoundingClientRect().width };
    });
    assert.ok(explicitMobileMetrics.videoHeight >= 180, `explicit-mobile transport схлопнулся на desktop viewport: ${JSON.stringify(explicitMobileMetrics)}`);
    await page.screenshot({ path: path.join(artifactDir, '02-explicit-mobile-desktop-viewport.png'), fullPage: true });
    const fullscreenButton = page.locator('.mobile-camera-card .fullscreen-btn').first();
    await fullscreenButton.waitFor({ state: 'visible' });
    await fullscreenButton.click({ force: true });
    await page.waitForFunction(() => Boolean(document.fullscreenElement || document.body.classList.contains('pseudo-native-fullscreen-active')), null, { timeout: 5000 }).catch(async () => {
      await fullscreenButton.evaluate((button) => button.click());
      await page.waitForFunction(() => Boolean(document.fullscreenElement || document.body.classList.contains('pseudo-native-fullscreen-active')), null, { timeout: 5000 }).catch(async () => {
        const state = await page.evaluate(() => ({ fullscreen: Boolean(document.fullscreenElement), pseudo: document.body.classList.contains('pseudo-native-fullscreen-active'), activeElement: document.activeElement?.outerHTML?.slice(0, 300) }));
        throw new Error(`Полноэкранный режим не включился: ${JSON.stringify(state)}`);
      });
    });
    await page.locator('#mobile-camera-search').evaluate((element) => {
      element.value = 'no matching camera';
      element.dispatchEvent(new Event('input', { bubbles: true }));
    });
    await page.waitForFunction(() => document.querySelectorAll('.mobile-camera-card').length === 0);
    await page.waitForFunction(() => !document.fullscreenElement && !document.body.classList.contains('pseudo-native-fullscreen-active'));
    await modeSelect.selectOption('auto');
    await page.waitForFunction(() => !document.querySelector('.mobile-camera-card'));
    await page.locator('#live-page').waitFor({ state: 'visible' });
    await page.locator('.video-cell').first().waitFor({ state: 'visible', timeout: 10000 });
    const desktopMetrics = await page.evaluate(() => {
      const grid = document.querySelector('.video-container');
      const toolbar = document.querySelector('.ui-mode-control');
      const footer = document.querySelector('footer');
      const box = (element) => { const rect = element?.getBoundingClientRect(); return rect && { top: rect.top, bottom: rect.bottom, height: rect.height, width: rect.width }; };
      return { viewport: { width: innerWidth, height: innerHeight, scrollHeight: document.documentElement.scrollHeight }, toolbar: box(toolbar), grid: box(grid), footer: box(footer), cells: document.querySelectorAll('.video-cell').length };
    });
    await page.screenshot({ path: path.join(artifactDir, '03-desktop-auto-1440x900.png'), fullPage: true });
    await page.setViewportSize({ width: 1440, height: 1080 });
    const desktop1080Metrics = await page.evaluate(() => ({ viewport: { width: innerWidth, height: innerHeight, scrollHeight: document.documentElement.scrollHeight }, page: document.querySelector('#live-page')?.getBoundingClientRect().toJSON(), footer: document.querySelector('footer')?.getBoundingClientRect().toJSON(), cells: document.querySelectorAll('#live-page .video-cell').length }));
    await page.screenshot({ path: path.join(artifactDir, '04-desktop-auto-1440x1080.png'), fullPage: true });

    await page.setViewportSize({ width: 932, height: 430 });
    const touchEmulation = await context.newCDPSession(page);
    await touchEmulation.send('Emulation.setTouchEmulationEnabled', { enabled: true, maxTouchPoints: 1 });
    const coarsePointerAt932 = await page.evaluate(() => window.matchMedia('(pointer: coarse)').matches);
    await modeSelect.selectOption('desktop');
    await modeSelect.selectOption('auto');
    await page.waitForFunction(() => document.querySelector('#ui-mode-select')?.value === 'auto');
    await page.locator('.mobile-camera-card').first().waitFor({ state: 'visible', timeout: 10000 }).catch(async () => {
      throw new Error(`Авто mobile не включился при 932×430: ${JSON.stringify(await page.evaluate(() => ({ value: document.querySelector('#ui-mode-select')?.value, coarse: matchMedia('(pointer: coarse)').matches, touchPoints: navigator.maxTouchPoints, cards: document.querySelectorAll('.mobile-camera-card').length, width: innerWidth })))}`);
    });
    const automaticLandscapeCards = await page.locator('.mobile-camera-card').count();
    assert.ok(coarsePointerAt932, 'эмуляция браузера должна сообщать coarse pointer в landscape phone');
    assert.ok(automaticLandscapeCards > 0, 'auto mode должен включить mobile Live View на landscape phone 932×430');
    await page.setViewportSize({ width: 844, height: 390 });
    const coarsePointerAt844 = await page.evaluate(() => window.matchMedia('(pointer: coarse)').matches);
    await modeSelect.selectOption('mobile');
    await page.locator('.mobile-camera-card').first().waitFor({ state: 'visible' });
    const overflow = await page.evaluate(() => ({ viewport: document.documentElement.clientWidth, content: document.documentElement.scrollWidth }));
    assert.ok(overflow.content <= overflow.viewport, `горизонтальная прокрутка: ${JSON.stringify(overflow)}`);
    await page.screenshot({ path: path.join(artifactDir, '05-phone-landscape.png'), fullPage: true });

    await page.evaluate(() => document.documentElement.classList.add('dark'));
    const colors = await page.locator('.mobile-camera-card').first().evaluate((card) => {
      const tab = document.querySelector('.mobile-live-tabs button[aria-selected="true"]');
      const input = document.querySelector('#mobile-camera-search');
      const control = document.querySelector('#ui-mode-select');
      const style = (element) => {
        const computed = getComputedStyle(element);
        return { color: computed.color, background: computed.backgroundColor, border: computed.borderColor };
      };
      return { card: style(card), text: style(card.querySelector('h2')), input: style(input), control: style(control), tab: style(tab) };
    });
    assert.ok(colors.card.background && colors.card.background !== 'rgba(0, 0, 0, 0)', `не разрешён фон карточки: ${JSON.stringify(colors)}`);
    assert.ok(colors.text.color && colors.input.background && colors.control.background && colors.tab.color);
    const darkTabContrast = contrastRatio(colors.tab.color, colors.tab.background);
    assert.ok(darkTabContrast >= 4.5, `контраст тёмной выбранной вкладки ${darkTabContrast.toFixed(2)} ниже 4.5:1`);
    await page.screenshot({ path: path.join(artifactDir, '06-theme-dark.png'), fullPage: true });

    const result = { result: 'passed', artifactDir, initialMounts, initialTransportRequests, rtcCounts, afterPageChange300ms, afterPageChange2s, afterPageChangeRtcCounts, after20PageTransitions, afterLateIceResolution, lightTabContrast, explicitMobileMetrics, desktopMetrics, desktop1080Metrics, coarsePointerAt932, coarsePointerAt844, automaticLandscapeCards, overflow, darkTabContrast, darkThemeColors: colors, apiCallCount: requests.length, transportRequests: transportRequests.length, iceRequests: iceRequests.length };
    fs.writeFileSync(path.join(artifactDir, 'results.json'), `${JSON.stringify(result, null, 2)}\n`, 'utf8');
    console.log(JSON.stringify(result, null, 2));
    await context.close();
  } finally {
    await browser.close();
  }
}

main().catch((error) => { console.error(error); process.exitCode = 1; });
