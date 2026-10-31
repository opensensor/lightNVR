import assert from 'node:assert/strict';
import { readFileSync } from 'node:fs';
import test from 'node:test';
import { __resetI18nStateForTests, setLocalePreference, t } from '../../web/js/i18n.js';

const english = JSON.parse(readFileSync(new URL('../../web/public/locales/en.json', import.meta.url), 'utf8'));
const examples = [
  ['streamsConfig.go2rtcSourcePlaceholder', {}, 'rtsp://admin:pass@camera/stream#transport=tcp\n\nOr multi-source:\n- rtsp://admin:pass@camera/main#transport=tcp\n- ffmpeg:camera1#video=h264#hardware'],
  ['settings.apiDetectionDoodsPlaceholder', {}, 'http://localhost:8080/detect'],
  ['settings.apiDetectionOnnxPlaceholder', {}, 'http://localhost:8000/api/v1/detect'],
  ['settings.go2rtcOverridePlaceholder', {}, 'ffmpeg:\n  h264_hw: "-codec:v h264_v4l2m2m"\n\nlog:\n  level: trace'],
  ['settings.storageTargets.objectRootPlaceholder', { bucket: 'front-door-archive' }, 's3://front-door-archive/instance'],
  ['settings.storageTargets.localRootPlaceholder', {}, '/mnt/nvr-hot-01'],
  ['settings.storageTargets.bucketPlaceholder', {}, 'bucket'],
  ['live.plan.canvasLabel', { name: 'Floor 2' }, 'Floor 2 camera plan'],
  ['live.plan.sketch.undoShortcut', {}, 'Undo drawing (Ctrl+Z)'],
  ['live.plan.sketch.redoShortcut', {}, 'Redo drawing (Ctrl+Shift+Z)'],
  ['recordings.thumbnailAlt', { stream: 'Front entrance' }, 'Front entrance recording'],
  ['recordings.detectionCountOne', { count: 1 }, '1 detection'],
  ['recordings.detectionCountMany', { count: 4 }, '4 detections'],
  ['recordings.removeTagTitle', { tag: 'front "entrance"' }, 'Remove tag "front "entrance""'],
];

async function load(context, locale) {
  __resetI18nStateForTests();
  context.mock.method(globalThis, 'fetch', async (url) => ({
    ok: true,
    async json() {
      if (url.endsWith('/manifest.json')) return { locales: [{ code: 'en' }, { code: 'fr' }] };
      if (url.endsWith('/en.json')) return english;
      if (url.endsWith('/fr.json')) return { 'settings.tabsLabel': 'Sections des paramètres' };
      throw new Error(`Unexpected locale URL ${url}`);
    },
  }));
  await setLocalePreference(locale);
  context.after(() => __resetI18nStateForTests());
}

test('attribute keys preserve interpolation, plural labels and technical examples', async (context) => {
  await load(context, 'en');
  for (const [key, params, expected] of examples) assert.equal(t(key, params), expected, key);
  assert.equal(t('recordings.download'), 'Download');
  assert.equal(t('recordings.protect'), 'Protect recording');
  assert.equal(t('recordings.unprotect'), 'Unprotect recording');
});

test('partial locale uses English attribute fallback without losing localized existing keys', async (context) => {
  await load(context, 'fr');
  for (const [key, params, expected] of examples) assert.equal(t(key, params), expected, key);
  assert.equal(t('settings.tabsLabel'), 'Sections des paramètres');
});
