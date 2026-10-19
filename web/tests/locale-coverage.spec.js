import fs from 'node:fs';
import path from 'node:path';
import { parse } from '@babel/parser';

const localeDir = path.resolve(__dirname, '../public/locales');
const readCatalog = (code) => JSON.parse(fs.readFileSync(path.join(localeDir, `${code}.json`), 'utf8'));
const manifest = JSON.parse(fs.readFileSync(path.join(localeDir, 'manifest.json'), 'utf8'));
const english = readCatalog('en');
const englishKeys = Object.keys(english);
const englishKeySet = new Set(englishKeys);
const placeholders = (value) => [...value.matchAll(/\{[^{}]+\}/g)].map(([token]) => token).sort();
const markup = (value) => [...value.matchAll(/<[^>]+>/g)].map(([token]) => token).sort();
const exampleUrls = (value) => [...value.matchAll(/(?:https?|rtsps?|rtmps?|s3):\/\/[A-Za-z0-9_@.:/…-]+/g)].map(([url]) => url.replace(/\.$/, ''));
const protocolNames = ['LightNVR', 'lightNVR', 'go2rtc', 'ONVIF', 'RTSP', 'WebRTC', 'HLS', 'MSE', 'MQTT', 'S3', 'PTZ', 'ePTZ', 'TOTP'];
const shortcutNames = ['Alt', 'Ctrl/Cmd', 'Shift', 'Enter', 'Escape'];
const shortcutKeys = new Set([
  'live.plan.sketch.hint.select',
  'live.plan.sketch.hint.wall',
  'live.plan.sketch.hint.area',
  'timeline.help.zoomControls',
  'timeline.help.panControls',
  'timeline.inlineHint',
]);

test('static translation keys used by the web UI exist in English', () => {
  const sourceRoot = path.resolve(__dirname, '../js');
  const sourceFiles = (directory) => fs.readdirSync(directory, { withFileTypes: true }).flatMap((entry) => {
    const file = path.join(directory, entry.name);
    return entry.isDirectory() ? sourceFiles(file) : /\.jsx?$/.test(entry.name) ? [file] : [];
  });
  const missing = [];

  for (const file of sourceFiles(sourceRoot)) {
    const source = fs.readFileSync(file, 'utf8');
    const ast = parse(source, { sourceType: 'module', plugins: ['jsx'] });
    const visit = (node) => {
      if (!node || typeof node !== 'object') return;
      if (Array.isArray(node)) return node.forEach(visit);
      if (node.type === 'CallExpression' && ['t', 'translate'].includes(node.callee?.name)) {
        const key = node.arguments?.[0];
        if (key?.type === 'StringLiteral' && !englishKeySet.has(key.value)) {
          missing.push(`${path.relative(sourceRoot, file)}: ${key.value}`);
        }
      }
      for (const [property, child] of Object.entries(node)) {
        if (!['loc', 'start', 'end'].includes(property)) visit(child);
      }
    };
    visit(ast);
  }

  expect(missing).toEqual([]);
});

describe('locale catalog coverage', () => {
  for (const { code } of manifest.locales) {
    if (code === 'en') continue;

    test(`${code} has valid keys and preserved literals`, () => {
      const catalog = readCatalog(code);
      const missing = englishKeys.filter((key) => !(key in catalog));
      const orphans = Object.keys(catalog).filter((key) => !englishKeySet.has(key));
      const invalid = Object.entries(catalog).flatMap(([key, value]) => {
        if (typeof value !== 'string' || !value.trim()) return [key];
        if (/__\s*P\d+\s*__/i.test(value)) return [key];
        if (!englishKeySet.has(key)) return [];
        if (JSON.stringify(placeholders(value)) !== JSON.stringify(placeholders(english[key]))) return [key];
        if (JSON.stringify(markup(value)) !== JSON.stringify(markup(english[key]))) return [key];
        if (exampleUrls(english[key]).some((url) => !value.includes(url))) return [key];
        if (shortcutKeys.has(key) && shortcutNames.some((name) => english[key].includes(name) && !value.includes(name))) return [key];
        return protocolNames.some((name) => english[key].includes(name)
          && !value.includes(name)
          && !(name === 'lightNVR' && value.includes('LightNVR'))) ? [key] : [];
      });

      const coverage = ((englishKeys.length - missing.length) / englishKeys.length * 100).toFixed(1);
      console.info(`${code}: ${coverage}% coverage (${missing.length} missing keys)`);
      if (missing.length) console.warn(`${code} missing ${missing.length} English keys`);
      expect(orphans).toEqual([]);
      expect(invalid).toEqual([]);
    });
  }
});
