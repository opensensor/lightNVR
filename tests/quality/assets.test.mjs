// SPDX-License-Identifier: GPL-3.0-or-later
// Tests for scripts/quality/assets.mjs (T4: web asset budgets).
//
// Every test builds a small synthetic `web/` tree with a `dist/` output in a
// temporary directory, so the tests never touch the real web/dist and need no
// Vite build. Run with: node --test tests/quality/assets.test.mjs

import { test, after } from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import zlib from 'node:zlib';
import crypto from 'node:crypto';
import { spawnSync } from 'node:child_process';
import { fileURLToPath } from 'node:url';

const here = path.dirname(fileURLToPath(import.meta.url));
const repoRoot = path.resolve(here, '..', '..');
const script = path.join(repoRoot, 'scripts', 'quality', 'assets.mjs');

const assets = await import(script);
const { analyzeDist, flattenMetrics, checkProfile, parseCss, parseJs } = assets;

const tempRoots = [];
after(() => {
  for (const r of tempRoots) fs.rmSync(r, { recursive: true, force: true });
});

// Deterministic pseudo-random text: compresses to a predictable, non-trivial size.
function noise(seed, length) {
  let out = '';
  let x = seed;
  while (out.length < length) {
    x = (x * 1103515245 + 12345) & 0x7fffffff;
    out += (x % 36).toString(36);
  }
  return out;
}

function write(file, content, { gz = true } = {}) {
  fs.mkdirSync(path.dirname(file), { recursive: true });
  fs.writeFileSync(file, content);
  // Mirror vite-plugin-compression: only files >= 1 KiB get a .gz sibling.
  if (gz && Buffer.byteLength(content) >= 1024) {
    fs.writeFileSync(`${file}.gz`, zlib.gzipSync(content, { level: 9 }));
  }
}

/**
 * Builds <tmp>/web with sources and a dist/ that looks like Vite output:
 *  - index.html  -> entry-a.js (static: shared.js, dynamic: lazy.js) + a.css
 *  - other.html  -> entry-b.js (static: shared.js)
 *  - optional legacy (nomodule/System.register) twins
 *  - locales/{manifest.json,en.json,de.json}
 */
function makeFixture({ legacy = true, locales = ['en', 'de'] } = {}) {
  const root = fs.mkdtempSync(path.join(os.tmpdir(), 't4-assets-'));
  tempRoots.push(root);
  const web = path.join(root, 'web');
  const dist = path.join(web, 'dist');

  // Sources (only their mtimes matter for staleness).
  write(path.join(web, 'js', 'app.js'), 'export const x = 1;\n', { gz: false });
  write(path.join(web, 'index.html'), '<html></html>\n', { gz: false });
  write(path.join(web, 'vite.config.js'), 'export default {};\n', { gz: false });

  const shared = `export const s=${JSON.stringify(noise(1, 6000))};\n`;
  const lazy = `export const l=${JSON.stringify(noise(2, 3000))};\n`;
  const entryA = `import{s}from"./shared-AAAA.js";const t=${JSON.stringify(noise(3, 2000))};import("./lazy-BBBB.js").then(m=>m.l);\n`;
  const entryB = `import"./shared-AAAA.js";const u=${JSON.stringify(noise(4, 1500))};\n`;
  const css = `.a{color:red}/*${noise(5, 1200)}*/\n`;
  write(path.join(dist, 'assets', 'shared-AAAA.js'), shared);
  write(path.join(dist, 'assets', 'lazy-BBBB.js'), lazy);
  write(path.join(dist, 'assets', 'entry-a-CCCC.js'), entryA);
  write(path.join(dist, 'assets', 'entry-b-DDDD.js'), entryB);
  write(path.join(dist, 'css', 'a-EEEE.css'), css);

  let legacyA = '';
  let legacyB = '';
  if (legacy) {
    write(path.join(dist, 'assets', 'polyfills-legacy-PPPP.js'), `/*${noise(6, 4000)}*/\n`);
    write(path.join(dist, 'assets', 'shared-legacy-QQQQ.js'),
      `System.register([],function(){return{execute:function(){var s=${JSON.stringify(noise(7, 6500))}}}});\n`);
    write(path.join(dist, 'assets', 'lazy-legacy-RRRR.js'),
      `System.register([],function(){var l=${JSON.stringify(noise(8, 3200))}});\n`);
    write(path.join(dist, 'assets', 'entry-a-legacy-SSSS.js'),
      `System.register(["./shared-legacy-QQQQ.js"],function(e,t){t.import("./lazy-legacy-RRRR.js");var z=${JSON.stringify(noise(9, 2100))}});\n`);
    write(path.join(dist, 'assets', 'entry-b-legacy-TTTT.js'),
      `System.register(["./shared-legacy-QQQQ.js"],function(){var y=${JSON.stringify(noise(10, 1600))}});\n`);
    legacyA = `<script nomodule crossorigin id="vite-legacy-polyfill" src="./assets/polyfills-legacy-PPPP.js"></script>
<script nomodule crossorigin id="vite-legacy-entry" data-src="./assets/entry-a-legacy-SSSS.js">System.import(1)</script>`;
    legacyB = `<script nomodule crossorigin id="vite-legacy-polyfill" src="./assets/polyfills-legacy-PPPP.js"></script>
<script nomodule crossorigin id="vite-legacy-entry" data-src="./assets/entry-b-legacy-TTTT.js">System.import(1)</script>`;
  }

  write(path.join(dist, 'index.html'), `<!DOCTYPE html><html><head>
<script type="module" crossorigin src="./assets/entry-a-CCCC.js"></script>
<link rel="modulepreload" crossorigin href="./assets/shared-AAAA.js">
<link rel="stylesheet" crossorigin href="./css/a-EEEE.css">
</head><body>${noise(11, 900)}<script nomodule>x</script><script src="."></script>${legacyA}</body></html>\n`);
  write(path.join(dist, 'other.html'), `<!DOCTYPE html><html><head>
<script type="module" crossorigin src="./assets/entry-b-DDDD.js"></script>
<link rel="modulepreload" crossorigin href="./assets/shared-AAAA.js">
</head><body>${legacyB}</body></html>\n`);

  const manifest = { locales: locales.map((code) => ({ code, name: code })) };
  write(path.join(dist, 'locales', 'manifest.json'), JSON.stringify(manifest), { gz: false });
  for (const code of locales) {
    write(path.join(dist, 'locales', `${code}.json`), JSON.stringify({ k: noise(code.length * 13, 5000) }));
    write(path.join(dist, 'locales', `${code}.png`), Buffer.alloc(300, 7), { gz: false });
  }
  // Source locale manifest: lists every available locale.
  write(path.join(web, 'public', 'locales', 'manifest.json'),
    JSON.stringify({ locales: [{ code: 'en' }, { code: 'de' }] }), { gz: false });

  // Make sources clearly older than the build output.
  const past = new Date(Date.now() - 3600_000);
  for (const f of ['js/app.js', 'index.html', 'vite.config.js', 'public/locales/manifest.json']) {
    fs.utimesSync(path.join(web, f), past, past);
  }
  return { root, web, dist, sizes: { shared, lazy, entryA, entryB, css } };
}

function bytes(s) {
  return Buffer.byteLength(s);
}
function gz9(s) {
  return zlib.gzipSync(s, { level: 9 }).length;
}
function br11(s) {
  return zlib.brotliCompressSync(s, {
    params: { [zlib.constants.BROTLI_PARAM_QUALITY]: 11 },
  }).length;
}
function fileSize(p) {
  return fs.statSync(p).size;
}

function profileDef(overrides = {}) {
  return {
    expect: { legacy: true, locales: 'all' },
    entries: ['index', 'other'],
    budgets: {},
    ...overrides,
  };
}

function writeBudgets(dir, profiles) {
  const file = path.join(dir, 'web-budgets.json');
  fs.writeFileSync(file, `${JSON.stringify({ schemaVersion: 1, profiles }, null, 2)}\n`);
  return file;
}

function runCli(args) {
  return spawnSync(process.execPath, [script, ...args], { encoding: 'utf8' });
}

test('accounts installed vs transfer bytes and never double-counts shared chunks', () => {
  const fx = makeFixture();
  const report = analyzeDist(fx.dist);
  const m = flattenMetrics(report);

  // Installed bytes: every file on disk exactly once, including .gz copies.
  let total = 0;
  let gzTotal = 0;
  const walk = (d) => {
    for (const e of fs.readdirSync(d, { withFileTypes: true })) {
      const p = path.join(d, e.name);
      if (e.isDirectory()) walk(p);
      else {
        total += fileSize(p);
        if (e.name.endsWith('.gz')) gzTotal += fileSize(p);
      }
    }
  };
  walk(fx.dist);
  assert.equal(m['installed.totalBytes'], total);
  assert.equal(m['installed.precompressedCopyBytes'], gzTotal);
  assert.equal(m['installed.originalBytes'], total - gzTotal);

  // Unique modern JS: each chunk once, although shared-AAAA.js is used by two entries.
  const modernJs = [fx.sizes.shared, fx.sizes.lazy, fx.sizes.entryA, fx.sizes.entryB];
  assert.equal(m['transfer.unique.modernJs.raw'], modernJs.reduce((a, s) => a + bytes(s), 0));
  assert.equal(m['transfer.unique.modernJs.gzip'], modernJs.reduce((a, s) => a + gz9(s), 0));
  assert.equal(m['transfer.unique.modernJs.brotli'], modernJs.reduce((a, s) => a + br11(s), 0));
  // "served" is what the LightNVR server sends: the precompressed .gz sibling.
  const servedShared = fileSize(path.join(fx.dist, 'assets', 'shared-AAAA.js.gz'));
  assert.ok(servedShared < bytes(fx.sizes.shared));

  // Per-entry initial JS = entry + its static closure; lazy chunks are separate.
  assert.equal(m['entries.index.modern.initialJs.raw'], bytes(fx.sizes.entryA) + bytes(fx.sizes.shared));
  assert.equal(m['entries.index.modern.initialJs.gzip'], gz9(fx.sizes.entryA) + gz9(fx.sizes.shared));
  assert.equal(m['entries.index.modern.lazyJs.raw'], bytes(fx.sizes.lazy));
  assert.equal(m['entries.other.modern.initialJs.raw'], bytes(fx.sizes.entryB) + bytes(fx.sizes.shared));
  assert.equal(m['entries.other.modern.lazyJs.raw'], 0);
  assert.equal(m['entries.index.modern.initialCss.raw'], bytes(fx.sizes.css));
  assert.equal(
    m['entries.index.modern.initialJs.served'],
    fileSize(path.join(fx.dist, 'assets', 'entry-a-CCCC.js.gz')) + servedShared,
  );

  // The shared chunk is reported once, attributed to both entries.
  const shared = report.chunks.shared.find((c) => c.file === 'assets/shared-AAAA.js');
  assert.ok(shared, 'shared chunk listed');
  assert.deepEqual([...shared.entries].sort(), ['index', 'other']);
  assert.ok(report.chunks.lazy.some((c) => c.file === 'assets/lazy-BBBB.js'));

  // Legacy graph resolved through System.register and t.import().
  assert.ok(m['entries.index.legacy.initialJs.raw'] > 0);
  assert.equal(
    m['entries.index.legacy.lazyJs.raw'],
    fileSize(path.join(fx.dist, 'assets', 'lazy-legacy-RRRR.js')),
  );
  assert.equal(m['build.legacyChunks'] > 0, true);

  // Locales are counted as installed bytes, including their .gz copies.
  const locBytes = ['en', 'de'].reduce((a, c) => {
    const base = path.join(fx.dist, 'locales', `${c}.json`);
    return a + fileSize(base) + fileSize(`${base}.gz`) + fileSize(path.join(fx.dist, 'locales', `${c}.png`));
  }, fileSize(path.join(fx.dist, 'locales', 'manifest.json')));
  assert.equal(m['installed.locales.totalBytes'], locBytes);
  assert.equal(m['build.localeCount'], 2);
});

test('check passes for a fresh, in-budget fixture', () => {
  const fx = makeFixture();
  const result = checkProfile({
    distDir: fx.dist,
    webRoot: fx.web,
    profileName: 'default',
    profile: profileDef({ budgets: { 'installed.totalBytes': 10_000_000 } }),
  });
  assert.deepEqual(result.failures, []);
  assert.equal(result.ok, true);
});

test('check rejects a missing dist and a missing referenced artifact', () => {
  const fx = makeFixture();
  const profile = profileDef({ budgets: { 'installed.totalBytes': 10_000_000 } });

  fs.rmSync(path.join(fx.dist, 'assets', 'shared-AAAA.js'));
  let result = checkProfile({ distDir: fx.dist, webRoot: fx.web, profileName: 'default', profile });
  assert.equal(result.ok, false);
  assert.ok(result.failures.some((f) => /missing/i.test(f)), result.failures.join('\n'));

  fs.rmSync(path.join(fx.dist, 'other.html'));
  result = checkProfile({ distDir: fx.dist, webRoot: fx.web, profileName: 'default', profile });
  assert.ok(result.failures.some((f) => /missing entry.*other/i.test(f)), result.failures.join('\n'));

  fs.rmSync(fx.dist, { recursive: true });
  result = checkProfile({ distDir: fx.dist, webRoot: fx.web, profileName: 'default', profile });
  assert.equal(result.ok, false);
  assert.ok(result.failures.some((f) => /missing/i.test(f)));
});

test('check rejects a stale dist (sources newer than the build)', () => {
  const fx = makeFixture();
  const future = new Date(Date.now() + 60_000);
  fs.utimesSync(path.join(fx.web, 'js', 'app.js'), future, future);
  const result = checkProfile({
    distDir: fx.dist,
    webRoot: fx.web,
    profileName: 'default',
    profile: profileDef({ budgets: { 'installed.totalBytes': 10_000_000 } }),
  });
  assert.equal(result.ok, false);
  assert.ok(result.failures.some((f) => /stale/i.test(f) && /js\/app\.js/.test(f)), result.failures.join('\n'));
});

test('check rejects a dist built for a different profile', () => {
  const fx = makeFixture({ legacy: true, locales: ['en', 'de'] });
  const result = checkProfile({
    distDir: fx.dist,
    webRoot: fx.web,
    profileName: 'modern',
    profile: profileDef({ expect: { legacy: false, locales: ['en'] }, budgets: { 'installed.totalBytes': 10_000_000 } }),
  });
  assert.equal(result.ok, false);
  assert.ok(result.failures.some((f) => /legacy/i.test(f)), result.failures.join('\n'));
  assert.ok(result.failures.some((f) => /locale/i.test(f)), result.failures.join('\n'));
});

test('check rejects a deliberately oversized artifact and unknown budget metrics', () => {
  const fx = makeFixture();
  const baseline = flattenMetrics(analyzeDist(fx.dist));
  const profile = profileDef({
    budgets: {
      'installed.totalBytes': baseline['installed.totalBytes'] + 100,
      'entries.index.modern.initialJs.served': baseline['entries.index.modern.initialJs.served'] + 100,
    },
  });
  assert.equal(checkProfile({ distDir: fx.dist, webRoot: fx.web, profileName: 'default', profile }).ok, true);

  // Grow the shared chunk by ~200 KiB of incompressible data.
  const sharedPath = path.join(fx.dist, 'assets', 'shared-AAAA.js');
  const big = `${fs.readFileSync(sharedPath, 'utf8')}const pad=${JSON.stringify(crypto.randomBytes(150_000).toString('base64'))};\n`;
  write(sharedPath, big);
  const result = checkProfile({ distDir: fx.dist, webRoot: fx.web, profileName: 'default', profile });
  assert.equal(result.ok, false);
  assert.ok(result.failures.some((f) => /installed\.totalBytes/.test(f) && /over budget/i.test(f)), result.failures.join('\n'));
  assert.ok(result.failures.some((f) => /entries\.index\.modern\.initialJs\.served/.test(f)), result.failures.join('\n'));

  const unknown = checkProfile({
    distDir: fx.dist,
    webRoot: fx.web,
    profileName: 'default',
    profile: profileDef({ budgets: { 'no.such.metric': 5 } }),
  });
  assert.equal(unknown.ok, false);
  assert.ok(unknown.failures.some((f) => /no\.such\.metric/.test(f)));

  const empty = checkProfile({ distDir: fx.dist, webRoot: fx.web, profileName: 'default', profile: profileDef() });
  assert.equal(empty.ok, false, 'a profile without budgets must not pass');
});

test('CLI --check exits nonzero on overrun and never rewrites the budgets file', () => {
  const fx = makeFixture();
  const budgetsFile = writeBudgets(fx.root, {
    default: profileDef({ budgets: { 'installed.totalBytes': 1000 } }),
  });
  const before = fs.readFileSync(budgetsFile);
  const beforeStat = fs.statSync(budgetsFile);

  const res = runCli(['--check', '--profile', 'default', '--dist', fx.dist, '--web-root', fx.web, '--budgets', budgetsFile]);
  assert.notEqual(res.status, 0, res.stdout + res.stderr);
  assert.match(res.stdout + res.stderr, /over budget/i);

  const after = fs.readFileSync(budgetsFile);
  assert.ok(before.equals(after), 'budgets file content changed');
  assert.equal(fs.statSync(budgetsFile).mtimeMs, beforeStat.mtimeMs, 'budgets file was rewritten');
  assert.deepEqual(fs.readdirSync(fx.root).sort(), ['web', 'web-budgets.json'], 'no sidecar files written');

  // Unknown profile and missing budgets file are hard failures too.
  assert.notEqual(runCli(['--check', '--profile', 'nope', '--dist', fx.dist, '--web-root', fx.web, '--budgets', budgetsFile]).status, 0);
  assert.notEqual(runCli(['--check', '--profile', 'default', '--dist', fx.dist, '--web-root', fx.web, '--budgets', path.join(fx.root, 'absent.json')]).status, 0);
});

test('CLI --check succeeds for an in-budget fixture and --report prints JSON', () => {
  const fx = makeFixture();
  const budgetsFile = writeBudgets(fx.root, {
    default: profileDef({ budgets: { 'installed.totalBytes': 10_000_000 } }),
  });
  const ok = runCli(['--check', '--profile', 'default', '--dist', fx.dist, '--web-root', fx.web, '--budgets', budgetsFile]);
  assert.equal(ok.status, 0, ok.stdout + ok.stderr);

  const rep = runCli(['--report', '--dist', fx.dist]);
  assert.equal(rep.status, 0, rep.stderr);
  const parsed = JSON.parse(rep.stdout);
  assert.equal(parsed.metrics['installed.totalBytes'] > 0, true);
});

test('reference parsers ignore data URIs, comments, bare CSS packages and template imports', () => {
  const css = `/* see the @import line */@import "tailwindcss";@import "./theme/x.css";
.a{background:url("data:image/svg+xml,%3Csvg filter='url(%23a)'%3E")}.b{background:url(../img/b.png)}`;
  assert.deepEqual(parseCss(css).sort(), ['../img/b.png', './theme/x.css']);

  const js = 'import{a as b}from"./a-1.js";import"./b-2.js";export*from"./c-3.js";' +
    'x.import("./d-4.js");import(`./e-${n}.js`);import"data:text/javascript,1";' +
    'const __vite__mapDeps=(i,m=__vite__mapDeps,d=(m.f||(m.f=["./f-5.js","../css/g.css"])))=>i.map(i=>d[i]);';
  const r = parseJs(js);
  assert.deepEqual(r.statics.sort(), ['./a-1.js', './b-2.js', './c-3.js']);
  assert.deepEqual(r.dynamics.sort(), ['../css/g.css', './d-4.js', './f-5.js']);
});
