#!/usr/bin/env node
// SPDX-License-Identifier: GPL-3.0-or-later
// Copyright (C) 2026 OpenSensor Engineering and LightNVR contributors
//
// T4: web asset reporter / budget checker.  Node built-ins only.
//
//   node scripts/quality/assets.mjs --capture --profile <default|modern> [--force]
//       Build web/ with the profile's flags (sourcemap pass for dependency
//       composition, then the production pass), analyse web/dist and archive
//       it with config, sha256 hashes and report JSON under
//       build/eng-baseline/1b8d4332/web-<profile>/.  Never edits budgets.
//   node scripts/quality/assets.mjs --check --profile <p> [--dist D] [--web-root W] [--budgets F]
//       Compare an existing dist against quality/web-budgets.json.  Exits 1 on
//       missing/stale/mismatched artifacts or any budget overrun.  Read-only.
//   node scripts/quality/assets.mjs --report [--dist D]
//       Print the analysis (report + flat metrics) as JSON.
//   node scripts/quality/assets.mjs --propose --profile <p> [--from report.json]
//       Print budgets derived from a report with the documented headroom
//       policy.  Output goes to stdout only; a human copies it into the file.
//
// Byte vocabulary (never mixed):
//   installed  - bytes on disk after `cp -r web/dist/*` (originals + .gz copies)
//   raw        - uncompressed size of an original file
//   gzip       - zlib gzip level 9 of the original (computed, comparable)
//   brotli     - brotli quality 11 of the original (computed, informational:
//                the LightNVR server does not serve brotli)
//   served     - what the LightNVR static handler sends a gzip-capable
//                browser: the precompressed `<file>.gz` sibling if present,
//                otherwise the raw file (src/web/libuv_connection.c)
// Aggregates are computed over *sets* of files, so a chunk shared by several
// entries is counted once in every total.

import fs from 'node:fs';
import path from 'node:path';
import zlib from 'node:zlib';
import crypto from 'node:crypto';
import os from 'node:os';
import { spawnSync } from 'node:child_process';
import { fileURLToPath, pathToFileURL } from 'node:url';

const SCRIPT_DIR = path.dirname(fileURLToPath(import.meta.url));
export const REPO_ROOT = path.resolve(SCRIPT_DIR, '..', '..');
const DEFAULT_WEB_ROOT = path.join(REPO_ROOT, 'web');
const DEFAULT_BUDGETS = path.join(REPO_ROOT, 'quality', 'web-budgets.json');
export const BASELINE_REVISION = '1b8d4332';
const ARCHIVE_ROOT = path.join(REPO_ROOT, 'build', 'eng-baseline', BASELINE_REVISION);

const PRECOMPRESSED_EXTS = ['.gz', '.br'];
const IMAGE_EXTS = new Set(['.png', '.jpg', '.jpeg', '.gif', '.svg', '.ico', '.webp', '.avif']);
const FONT_EXTS = new Set(['.woff', '.woff2', '.ttf', '.otf', '.eot']);
const SIZE_KINDS = ['raw', 'gzip', 'brotli', 'served'];

// Inputs of the Vite build, relative to web/.  Newer than the build => stale.
const SOURCE_DIRS = ['js', 'css', 'img', 'fonts', 'public'];
const SOURCE_FILES = [
  'vite.config.js',
  'vite-plugin-theme-inject.js',
  'postcss.config.js',
  'tailwind.config.js',
  'package.json',
  'package-lock.json',
  'node_modules/.package-lock.json',
];

// Headroom policy used by --propose (documented in quality/web-budgets.json).
// Builds are byte-for-byte reproducible (verified: two default builds of the
// same tree had identical sha256 for all 221 files), so headroom only absorbs
// intentional small growth; anything larger needs a reviewed budget change.
export const BUDGET_POLICY = {
  totalsPct: 3,
  totalsMinSlackBytes: 4096,
  entryPct: 5,
  entryMinSlackBytes: 2048,
};

// Env vars that alter the Vite output; scrubbed before every capture build.
const BUILD_ENV_SCRUB = /^(LIGHTNVR_WEB_.*|SENTRY_.*|VITE_SENTRY_.*|BUILD_SOURCEMAPS)$/;

// ---------------------------------------------------------------------------
// Filesystem helpers

function toPosix(p) {
  return p.split(path.sep).join('/');
}

function walkFiles(dir, { skip = () => false } = {}) {
  const out = [];
  const visit = (d) => {
    let entries;
    try {
      entries = fs.readdirSync(d, { withFileTypes: true });
    } catch (err) {
      if (err.code === 'ENOENT') return;
      throw err;
    }
    for (const e of entries) {
      const p = path.join(d, e.name);
      if (skip(p, e)) continue;
      if (e.isDirectory()) visit(p);
      else if (e.isFile()) out.push(p);
    }
  };
  visit(dir);
  return out.sort();
}

function sha256(buf) {
  return crypto.createHash('sha256').update(buf).digest('hex');
}

function gzipSize(buf) {
  return zlib.gzipSync(buf, { level: 9 }).length;
}

function brotliSize(buf) {
  return zlib.brotliCompressSync(buf, {
    params: {
      [zlib.constants.BROTLI_PARAM_QUALITY]: 11,
      [zlib.constants.BROTLI_PARAM_SIZE_HINT]: buf.length,
    },
  }).length;
}

function zeroSizes() {
  return { raw: 0, gzip: 0, brotli: 0, served: 0, files: 0 };
}

// ---------------------------------------------------------------------------
// Classification

function precompressedBase(rel, fileSet) {
  for (const ext of PRECOMPRESSED_EXTS) {
    if (rel.endsWith(ext) && fileSet.has(rel.slice(0, -ext.length))) {
      return rel.slice(0, -ext.length);
    }
  }
  return null;
}

function isLegacyName(rel) {
  return /-legacy[-.]/.test(path.posix.basename(rel));
}

function classify(rel) {
  const ext = path.posix.extname(rel).toLowerCase();
  if (rel.startsWith('locales/')) return 'locale';
  if (ext === '.map') return 'sourcemap';
  if (ext === '.js' || ext === '.mjs') return isLegacyName(rel) ? 'legacyJs' : 'modernJs';
  if (ext === '.css') return 'css';
  if (ext === '.html') return 'html';
  if (IMAGE_EXTS.has(ext)) return 'image';
  if (FONT_EXTS.has(ext)) return 'font';
  return 'other';
}

// ---------------------------------------------------------------------------
// Reference parsing (HTML / JS / CSS)

function isLocalRef(ref) {
  if (!ref) return false;
  const r = ref.trim();
  if (r === '' || r === '.' || r === './' || r.startsWith('#')) return false;
  if (/^(?:[a-z][a-z0-9+.-]*:|\/\/)/i.test(r)) return false; // data:, http:, //cdn
  return true;
}

function parseAttrs(tag) {
  const attrs = {};
  const re = /([^\s=/>]+)(?:\s*=\s*(?:"([^"]*)"|'([^']*)'|([^\s>]+)))?/g;
  const body = tag.replace(/^<\w+/, '').replace(/\/?>$/, '');
  let m;
  while ((m = re.exec(body))) {
    attrs[m[1].toLowerCase()] = m[2] ?? m[3] ?? m[4] ?? '';
  }
  return attrs;
}

export function parseHtml(html) {
  const refs = { modern: [], legacy: [], preload: [], css: [], other: [] };
  for (const [tag] of html.matchAll(/<script\b[^>]*>/gi)) {
    const a = parseAttrs(tag);
    const nomodule = 'nomodule' in a;
    if (nomodule) {
      if (isLocalRef(a.src)) refs.legacy.push(a.src);
      if (isLocalRef(a['data-src'])) refs.legacy.push(a['data-src']);
    } else if (isLocalRef(a.src)) {
      (a.type === 'module' ? refs.modern : refs.other).push(a.src);
    }
  }
  for (const [tag] of html.matchAll(/<link\b[^>]*>/gi)) {
    const a = parseAttrs(tag);
    if (!isLocalRef(a.href)) continue;
    const rel = (a.rel || '').toLowerCase().split(/\s+/);
    if (rel.includes('modulepreload')) refs.preload.push(a.href);
    else if (rel.includes('stylesheet')) refs.css.push(a.href);
    else refs.other.push(a.href);
  }
  return refs;
}

export function parseJs(code) {
  const statics = [];
  const dynamics = [];
  // import"./x.js" | import{a as b}from"./x.js" | import*as n from"./x.js"
  const staticRe = /(?<![.\w$])import\s*(?!\()(?:[\w$*{}\s,]*?\bfrom\s*)?["']([^"'\n]+)["']/g;
  const exportRe = /(?<![.\w$])export\s*(?:\*(?:\s*as\s+[\w$]+)?|\{[^}]*\})\s*from\s*["']([^"'\n]+)["']/g;
  // import("./x.js") and SystemJS's t.import("./x.js")
  const dynamicRe = /\bimport\(\s*["'`]([^"'`\n]+)["'`]\s*\)/g;
  const systemRe = /System\.register\(\s*\[([^\]]*)\]/g;
  const mapDepsRe = /__vite__mapDeps\s*=\s*\([^]*?m\.f\s*=\s*\[([^\]]*)\]/g;
  for (const m of code.matchAll(staticRe)) statics.push(m[1]);
  for (const m of code.matchAll(exportRe)) statics.push(m[1]);
  for (const m of code.matchAll(dynamicRe)) dynamics.push(m[1]);
  for (const m of code.matchAll(systemRe)) {
    for (const s of m[1].matchAll(/["']([^"']+)["']/g)) statics.push(s[1]);
  }
  // Vite preload dependency lists: fetched together with a dynamic import.
  for (const m of code.matchAll(mapDepsRe)) {
    for (const s of m[1].matchAll(/["']([^"']+)["']/g)) dynamics.push(s[1]);
  }
  const rel = (r) => /^\.{1,2}\//.test(r);
  const ok = (r) => rel(r) && !r.includes("${");
  return { statics: statics.filter(ok), dynamics: dynamics.filter(ok) };
}

export function parseCss(code) {
  const refs = [];
  const text = code
    .replace(/\/\*[^]*?\*\//g, '')
    // data: URIs may contain nested url(#id) references; drop them first.
    .replace(/(["'])data:[^]*?\1/g, '""')
    .replace(/url\(\s*data:[^)]*\)/g, '');
  for (const m of text.matchAll(/@import\s+(?:url\(\s*)?["']?([^"')\s;]+)["']?/g)) {
    // Bare specifiers (e.g. `@import "tailwindcss"` in uncompiled copies) are
    // package names for a CSS toolchain, not files a browser can fetch.
    if (/^(?:\.{1,2}\/|\/)|\.css$/i.test(m[1])) refs.push(m[1]);
  }
  for (const m of text.matchAll(/url\(\s*(["']?)([^"')]+)\1\s*\)/g)) refs.push(m[2]);
  return [...new Set(refs)].filter(isLocalRef);
}

function resolveRef(fromRel, ref, fileSet) {
  const clean = ref.split(/[?#]/)[0];
  const candidates = [
    path.posix.normalize(path.posix.join(path.posix.dirname(fromRel), clean)),
    path.posix.normalize(clean.replace(/^\.\//, '')),
  ];
  for (const c of candidates) {
    if (!c.startsWith('..') && fileSet.has(c)) return { rel: c, found: true };
  }
  return { rel: candidates[0], found: false };
}

// ---------------------------------------------------------------------------
// Analysis

/**
 * Analyse a Vite dist directory.
 * @param {string} distDir
 * @returns {object} report (see flattenMetrics for the budgetable numbers)
 */
export function analyzeDist(distDir) {
  if (!fs.existsSync(distDir) || !fs.statSync(distDir).isDirectory()) {
    const err = new Error(`missing dist directory: ${distDir}`);
    err.code = 'T4_MISSING_DIST';
    throw err;
  }
  const absFiles = walkFiles(distDir);
  const allRels = absFiles.map((f) => toPosix(path.relative(distDir, f)));
  const fileSet = new Set(allRels);

  const files = new Map();
  for (const rel of allRels) {
    const buf = fs.readFileSync(path.join(distDir, rel));
    const base = precompressedBase(rel, fileSet);
    files.set(rel, {
      path: rel,
      bytes: buf.length,
      sha256: sha256(buf),
      category: base ? 'precompressed' : classify(rel),
      precompressedOf: base,
      buf,
    });
  }
  const originals = [...files.values()].filter((f) => !f.precompressedOf);
  const sizeCache = new Map();
  const sizesOf = (rel) => {
    if (sizeCache.has(rel)) return sizeCache.get(rel);
    const f = files.get(rel);
    const gz = files.get(`${rel}.gz`);
    const s = {
      raw: f.bytes,
      gzip: gzipSize(f.buf),
      brotli: brotliSize(f.buf),
      served: gz ? gz.bytes : f.bytes,
    };
    sizeCache.set(rel, s);
    return s;
  };
  const sumSet = (rels) => {
    const total = zeroSizes();
    for (const rel of new Set(rels)) {
      if (!files.has(rel)) continue;
      const s = sizesOf(rel);
      for (const k of SIZE_KINDS) total[k] += s[k];
      total.files += 1;
    }
    return total;
  };

  // --- reference graph
  const missing = [];
  const edges = new Map(); // rel -> { statics:Set, dynamics:Set }
  const addEdge = (from, ref, kind) => {
    const r = resolveRef(from, ref, fileSet);
    if (!r.found) {
      missing.push({ ref, resolved: r.rel, referrer: from });
      return null;
    }
    if (!edges.has(from)) edges.set(from, { statics: new Set(), dynamics: new Set() });
    edges.get(from)[kind].add(r.rel);
    return r.rel;
  };
  for (const f of originals) {
    if (f.category === 'modernJs' || f.category === 'legacyJs') {
      const { statics, dynamics } = parseJs(f.buf.toString('utf8'));
      statics.forEach((r) => addEdge(f.path, r, 'statics'));
      dynamics.forEach((r) => addEdge(f.path, r, 'dynamics'));
    } else if (f.category === 'css') {
      parseCss(f.buf.toString('utf8')).forEach((r) => addEdge(f.path, r, 'statics'));
    }
  }
  const closure = (starts, kinds) => {
    const seen = new Set();
    const stack = [...starts];
    while (stack.length) {
      const cur = stack.pop();
      if (seen.has(cur)) continue;
      seen.add(cur);
      const e = edges.get(cur);
      if (!e) continue;
      for (const k of kinds) for (const n of e[k]) if (!seen.has(n)) stack.push(n);
    }
    return seen;
  };

  // --- entries (top-level HTML files)
  const entries = {};
  const reachable = new Set();
  const htmlFiles = originals.filter((f) => f.category === 'html' && !f.path.includes('/'));
  for (const h of htmlFiles) {
    const name = h.path.replace(/\.html$/, '');
    const refs = parseHtml(h.buf.toString('utf8'));
    const resolveAll = (list) => list.map((r) => addEdge(h.path, r, 'statics')).filter(Boolean);
    const modernStart = resolveAll([...refs.modern, ...refs.preload]);
    const legacyStart = resolveAll(refs.legacy);
    const cssStart = resolveAll(refs.css);
    const otherStart = resolveAll(refs.other);

    const variant = (starts) => {
      const initial = closure([...starts, ...cssStart], ['statics']);
      const all = closure([...initial], ['statics', 'dynamics']);
      const lazy = [...all].filter((r) => !initial.has(r));
      all.forEach((r) => reachable.add(r));
      const pick = (set, cats) => [...set].filter((r) => cats.includes(files.get(r)?.category)).sort();
      return {
        initialJs: pick(initial, ['modernJs', 'legacyJs']),
        initialCss: pick(initial, ['css']),
        initialOther: pick(initial, ['image', 'font', 'other']),
        lazyJs: pick(lazy, ['modernJs', 'legacyJs']),
        lazyCss: pick(lazy, ['css']),
      };
    };
    const modern = variant(modernStart);
    const legacy = legacyStart.length ? variant(legacyStart) : null;
    otherStart.forEach((r) => reachable.add(r));
    reachable.add(h.path);

    const summarize = (v) =>
      v && {
        initialJs: { ...sumSet(v.initialJs), list: v.initialJs },
        initialCss: { ...sumSet(v.initialCss), list: v.initialCss },
        lazyJs: { ...sumSet(v.lazyJs), list: v.lazyJs },
        lazyCss: { ...sumSet(v.lazyCss), list: v.lazyCss },
        firstLoad: sumSet([h.path, ...v.initialJs, ...v.initialCss, ...otherStart]),
      };
    entries[name] = {
      html: sumSet([h.path]),
      icons: { ...sumSet(otherStart), list: otherStart },
      modern: summarize(modern),
      legacy: summarize(legacy),
    };
  }

  // --- shared / lazy chunk attribution (each chunk listed once)
  const chunkUse = new Map();
  for (const [name, e] of Object.entries(entries)) {
    for (const v of ['modern', 'legacy']) {
      if (!e[v]) continue;
      for (const rel of e[v].initialJs.list) {
        const u = chunkUse.get(rel) || { initial: new Set(), lazy: new Set() };
        u.initial.add(name);
        chunkUse.set(rel, u);
      }
      for (const rel of e[v].lazyJs.list) {
        const u = chunkUse.get(rel) || { initial: new Set(), lazy: new Set() };
        u.lazy.add(name);
        chunkUse.set(rel, u);
      }
    }
  }
  const chunkRow = (rel, names) => ({
    file: rel,
    variant: isLegacyName(rel) ? 'legacy' : 'modern',
    entries: [...names].sort(),
    ...sizesOf(rel),
  });
  const shared = [];
  const lazy = [];
  for (const [rel, u] of [...chunkUse.entries()].sort()) {
    if (u.initial.size > 1) shared.push(chunkRow(rel, u.initial));
    if (u.initial.size === 0 && u.lazy.size > 0) lazy.push(chunkRow(rel, u.lazy));
  }

  // --- installed bytes
  const installed = {
    totalBytes: 0,
    fileCount: files.size,
    originalBytes: 0,
    precompressedCopyBytes: 0,
    byCategory: {},
    locales: { totalBytes: 0, perLocale: {} },
    unreferenced: { bytes: 0, list: [] },
  };
  for (const f of files.values()) {
    installed.totalBytes += f.bytes;
    if (f.precompressedOf) installed.precompressedCopyBytes += f.bytes;
    else installed.originalBytes += f.bytes;
    const cat = f.precompressedOf ? classify(f.precompressedOf) : f.category;
    const c = (installed.byCategory[cat] ||= { originalBytes: 0, precompressedBytes: 0, files: 0 });
    if (f.precompressedOf) c.precompressedBytes += f.bytes;
    else {
      c.originalBytes += f.bytes;
      c.files += 1;
    }
    if (f.path.startsWith('locales/')) {
      installed.locales.totalBytes += f.bytes;
      const code = path.posix.basename(f.path).replace(/\.(json|png)(\.gz|\.br)?$/, '');
      if (code !== 'manifest' && !code.startsWith('.')) {
        installed.locales.perLocale[code] = (installed.locales.perLocale[code] || 0) + f.bytes;
      }
    }
  }
  for (const f of originals) {
    const cat = f.category;
    if (['modernJs', 'legacyJs', 'css'].includes(cat) && !reachable.has(f.path)) {
      const gz = files.get(`${f.path}.gz`);
      installed.unreferenced.bytes += f.bytes + (gz ? gz.bytes : 0);
      installed.unreferenced.list.push(f.path);
    }
  }

  // --- unique transfer totals (every original once)
  const byCat = (cat) => originals.filter((f) => f.category === cat).map((f) => f.path);
  const transfer = {
    unique: {
      modernJs: sumSet(byCat('modernJs')),
      legacyJs: sumSet(byCat('legacyJs')),
      css: sumSet(byCat('css')),
      html: sumSet(byCat('html')),
      localeJson: sumSet(byCat('locale').filter((r) => r.endsWith('.json'))),
    },
  };

  // --- locale manifest
  let localeCodes = null;
  const manifestRel = 'locales/manifest.json';
  if (files.has(manifestRel)) {
    try {
      const mf = JSON.parse(files.get(manifestRel).buf.toString('utf8'));
      localeCodes = (mf.locales || []).map((l) => l.code).sort();
    } catch {
      localeCodes = null;
    }
  }

  const legacyChunks = originals.filter((f) => f.category === 'legacyJs').length;
  const report = {
    distDir,
    build: {
      legacyChunks,
      legacyEntries: Object.values(entries).filter((e) => e.legacy).length,
      localeManifest: files.has(manifestRel),
      locales: localeCodes,
      viteManifest: fileSet.has('.vite/manifest.json'),
      newestMtimeMs: Math.max(0, ...absFiles.map((f) => fs.statSync(f).mtimeMs)),
      buildTimeMs: htmlFiles.length
        ? Math.min(...htmlFiles.map((h) => fs.statSync(path.join(distDir, h.path)).mtimeMs))
        : null,
    },
    installed,
    transfer,
    entries,
    chunks: { shared, lazy },
    missing,
    files: [...files.values()].map(({ buf, ...rest }) => rest),
  };
  return report;
}

/** Flatten a report into dotted numeric metrics that budgets can reference. */
export function flattenMetrics(report) {
  const m = {};
  const put = (prefix, sizes) => {
    for (const k of SIZE_KINDS) m[`${prefix}.${k}`] = sizes[k];
  };
  const i = report.installed;
  m['installed.totalBytes'] = i.totalBytes;
  m['installed.fileCount'] = i.fileCount;
  m['installed.originalBytes'] = i.originalBytes;
  m['installed.precompressedCopyBytes'] = i.precompressedCopyBytes;
  m['installed.unreferencedBytes'] = i.unreferenced.bytes;
  m['installed.locales.totalBytes'] = i.locales.totalBytes;
  for (const [cat, c] of Object.entries(i.byCategory)) {
    m[`installed.byCategory.${cat}.bytes`] = c.originalBytes + c.precompressedBytes;
  }
  for (const [code, bytes] of Object.entries(i.locales.perLocale)) {
    m[`installed.locales.perLocale.${code}.bytes`] = bytes;
  }
  for (const [k, s] of Object.entries(report.transfer.unique)) put(`transfer.unique.${k}`, s);
  for (const [name, e] of Object.entries(report.entries)) {
    put(`entries.${name}.html`, e.html);
    for (const v of ['modern', 'legacy']) {
      if (!e[v]) continue;
      for (const part of ['initialJs', 'initialCss', 'lazyJs', 'lazyCss', 'firstLoad']) {
        put(`entries.${name}.${v}.${part}`, e[v][part]);
      }
    }
  }
  for (const v of ['modern', 'legacy']) {
    const rows = report.chunks.shared.filter((c) => c.variant === v);
    m[`chunks.shared.${v}.count`] = rows.length;
    m[`chunks.shared.${v}.served`] = rows.reduce((a, c) => a + c.served, 0);
    const lz = report.chunks.lazy.filter((c) => c.variant === v);
    m[`chunks.lazy.${v}.count`] = lz.length;
    m[`chunks.lazy.${v}.served`] = lz.reduce((a, c) => a + c.served, 0);
  }
  m['build.legacyChunks'] = report.build.legacyChunks;
  m['build.localeCount'] = report.build.locales ? report.build.locales.length : 0;
  return m;
}

// ---------------------------------------------------------------------------
// Staleness / expectations / budgets

export function newestSource(webRoot) {
  let newest = { file: null, mtimeMs: 0 };
  const consider = (abs) => {
    const st = fs.statSync(abs);
    if (st.mtimeMs > newest.mtimeMs) newest = { file: toPosix(path.relative(webRoot, abs)), mtimeMs: st.mtimeMs };
  };
  for (const d of SOURCE_DIRS) walkFiles(path.join(webRoot, d)).forEach(consider);
  for (const f of SOURCE_FILES) {
    const abs = path.join(webRoot, f);
    if (fs.existsSync(abs)) consider(abs);
  }
  for (const e of fs.existsSync(webRoot) ? fs.readdirSync(webRoot) : []) {
    if (e.endsWith('.html')) consider(path.join(webRoot, e));
  }
  return newest;
}

function sourceLocales(webRoot) {
  const mf = path.join(webRoot, 'public', 'locales', 'manifest.json');
  if (!fs.existsSync(mf)) return null;
  return JSON.parse(fs.readFileSync(mf, 'utf8')).locales.map((l) => l.code).sort();
}

function pct(value, budget) {
  return `${(((value - budget) / budget) * 100).toFixed(1)}%`;
}

/**
 * Validate a dist against a profile definition ({expect, entries, budgets}).
 * Pure with respect to the budgets: it only reads.
 */
export function checkProfile({ distDir, webRoot, profileName, profile }) {
  const failures = [];
  const rows = [];
  if (!profile || typeof profile !== 'object') {
    return { ok: false, failures: [`unknown profile "${profileName}"`], rows };
  }
  if (!fs.existsSync(path.join(distDir, 'index.html'))) {
    return { ok: false, failures: [`missing build output: ${path.join(distDir, 'index.html')} (run the profile build first)`], rows };
  }
  const report = analyzeDist(distDir);
  const metrics = flattenMetrics(report);

  for (const name of profile.entries || []) {
    if (!report.entries[name]) failures.push(`missing entry page: ${name}.html`);
  }
  for (const miss of report.missing) {
    failures.push(`missing artifact: ${miss.resolved} (referenced as "${miss.ref}" by ${miss.referrer})`);
  }
  if (!report.build.localeManifest) failures.push('missing artifact: locales/manifest.json');

  // Freshness: every build input must be older than the oldest emitted page.
  const src = newestSource(webRoot);
  if (report.build.buildTimeMs === null) {
    failures.push('missing build output: no HTML entry pages in dist');
  } else if (src.file && src.mtimeMs > report.build.buildTimeMs) {
    failures.push(
      `stale build: web/${src.file} (${new Date(src.mtimeMs).toISOString()}) is newer than dist ` +
        `(${new Date(report.build.buildTimeMs).toISOString()}); rebuild before checking`,
    );
  }

  // Profile flags actually reflected in the output.
  const expect = profile.expect || {};
  if (expect.legacy === true && (report.build.legacyChunks === 0 || report.build.legacyEntries === 0)) {
    failures.push(`profile mismatch: "${profileName}" expects legacy bundles but dist has none`);
  }
  if (expect.legacy === false && (report.build.legacyChunks > 0 || report.build.legacyEntries > 0)) {
    failures.push(
      `profile mismatch: "${profileName}" is modern-only but dist has ${report.build.legacyChunks} legacy chunks`,
    );
  }
  if (expect.locales !== undefined && report.build.locales) {
    const want = expect.locales === 'all' ? sourceLocales(webRoot) : [...expect.locales].sort();
    if (want && want.join(',') !== report.build.locales.join(',')) {
      failures.push(
        `profile mismatch: locale set [${report.build.locales.join(',')}] != expected [${want.join(',')}]`,
      );
    }
  }

  const budgets = profile.budgets || {};
  if (Object.keys(budgets).length === 0) {
    failures.push(`profile "${profileName}" defines no budgets`);
  }
  for (const [key, limit] of Object.entries(budgets)) {
    if (!Number.isInteger(limit) || limit <= 0) {
      failures.push(`invalid budget ${key}: ${JSON.stringify(limit)} (must be a positive integer)`);
      continue;
    }
    if (!(key in metrics)) {
      failures.push(`unknown budget metric ${key} (not produced by this dist)`);
      continue;
    }
    const value = metrics[key];
    const baseline = profile.baseline?.[key];
    rows.push({ key, value, budget: limit, baseline, used: value / limit });
    if (value > limit) {
      failures.push(`over budget: ${key} = ${value} > ${limit} (+${pct(value, limit)})`);
    }
  }
  return { ok: failures.length === 0, failures, rows, metrics, report };
}

/** Budgets = baseline + headroom (see BUDGET_POLICY). */
export function proposeBudget(key, baseline, policy = BUDGET_POLICY) {
  const entry = key.startsWith('entries.');
  const p = entry ? policy.entryPct : policy.totalsPct;
  const slack = entry ? policy.entryMinSlackBytes : policy.totalsMinSlackBytes;
  return Math.ceil(Math.max(baseline * (1 + p / 100), baseline + slack));
}

export function budgetKeysFor(metrics) {
  const keys = [
    'installed.totalBytes',
    'installed.originalBytes',
    'installed.precompressedCopyBytes',
    'installed.locales.totalBytes',
    'transfer.unique.modernJs.served',
    'transfer.unique.modernJs.gzip',
    'transfer.unique.modernJs.raw',
    'transfer.unique.legacyJs.served',
    'transfer.unique.css.served',
    'transfer.unique.html.served',
    'chunks.shared.modern.served',
  ];
  for (const k of Object.keys(metrics)) {
    if (/^entries\.[^.]+\.(modern|legacy)\.(firstLoad|initialJs|lazyJs)\.served$/.test(k)) keys.push(k);
    if (/^entries\.[^.]+\.modern\.initialCss\.served$/.test(k)) keys.push(k);
  }
  return keys.filter((k) => k in metrics && metrics[k] > 0).sort();
}

// ---------------------------------------------------------------------------
// Dependency composition from source maps (capture only)

const B64 = Object.fromEntries([...'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/'].map((c, i) => [c, i]));

function decodeVlq(str) {
  const out = [];
  let value = 0;
  let shift = 0;
  for (const ch of str) {
    const digit = B64[ch];
    value += (digit & 31) << shift;
    if (digit & 32) shift += 5;
    else {
      out.push(value & 1 ? -(value >>> 1) : value >>> 1);
      value = 0;
      shift = 0;
    }
  }
  return out;
}

export function packageOf(source) {
  const s = source.replace(/\\/g, '/').replace(/^[a-z]+:\/\//i, '');
  const idx = s.lastIndexOf('node_modules/');
  if (idx >= 0) {
    const parts = s.slice(idx + 'node_modules/'.length).split('/');
    return parts[0].startsWith('@') ? `${parts[0]}/${parts[1]}` : parts[0];
  }
  if (s.startsWith('\0') || /^(?:vite|rolldown|\\0)/.test(s) || s.includes('vite/')) return '(bundler-runtime)';
  const web = s.replace(/^(?:\.\.\/)+/, '');
  const m = web.match(/^(?:web\/)?(js|css)\/(?:([^/]+)\/)?/);
  if (m) return `(first-party) ${m[1]}/${m[2] && !m[2].includes('.') ? m[2] : ''}`.replace(/\/$/, '');
  return `(other) ${web.split('/')[0]}`;
}

/** Attribute generated (minified) bytes of each chunk to source packages. */
export function composition(distDir) {
  const result = { modern: { byPackage: {}, total: 0 }, legacy: { byPackage: {}, total: 0 }, chunks: {} };
  for (const mapAbs of walkFiles(distDir).filter((f) => f.endsWith('.js.map'))) {
    const jsAbs = mapAbs.slice(0, -4);
    if (!fs.existsSync(jsAbs)) continue;
    const rel = toPosix(path.relative(distDir, jsAbs));
    const variant = isLegacyName(rel) ? 'legacy' : 'modern';
    const map = JSON.parse(fs.readFileSync(mapAbs, 'utf8'));
    const lines = fs.readFileSync(jsAbs, 'utf8').split('\n');
    const pkgs = (map.sources || []).map((s) => packageOf(`${map.sourceRoot || ''}${s}`));
    const per = {};
    const add = (pkg, n) => {
      if (n <= 0) return;
      per[pkg] = (per[pkg] || 0) + n;
    };
    let srcIdx = 0;
    const mapLines = (map.mappings || '').split(';');
    for (let li = 0; li < lines.length; li++) {
      const line = lines[li];
      const segs = li < mapLines.length && mapLines[li] ? mapLines[li].split(',') : [];
      let col = 0;
      const decoded = [];
      for (const seg of segs) {
        const f = decodeVlq(seg);
        col += f[0];
        let pkg = '(unmapped)';
        if (f.length >= 4) {
          srcIdx += f[1];
          pkg = pkgs[srcIdx] ?? '(unmapped)';
        }
        decoded.push([col, pkg]);
      }
      let covered = 0;
      for (let si = 0; si < decoded.length; si++) {
        const [start, pkg] = decoded[si];
        const end = si + 1 < decoded.length ? decoded[si + 1][0] : line.length;
        if (si === 0 && start > 0) add('(unmapped)', Buffer.byteLength(line.slice(0, start)));
        add(pkg, Buffer.byteLength(line.slice(start, end)));
        covered = end;
      }
      if (decoded.length === 0) add('(unmapped)', Buffer.byteLength(line));
      else if (covered < line.length) add('(unmapped)', Buffer.byteLength(line.slice(covered)));
      if (li < lines.length - 1) add('(newline)', 1);
    }
    result.chunks[rel] = per;
    for (const [pkg, n] of Object.entries(per)) {
      result[variant].byPackage[pkg] = (result[variant].byPackage[pkg] || 0) + n;
      result[variant].total += n;
    }
  }
  for (const v of ['modern', 'legacy']) {
    result[v].byPackage = Object.fromEntries(Object.entries(result[v].byPackage).sort((a, b) => b[1] - a[1]));
  }
  return result;
}

// ---------------------------------------------------------------------------
// Capture

function sh(cmd, args, opts = {}) {
  const r = spawnSync(cmd, args, { encoding: 'utf8', maxBuffer: 64 * 1024 * 1024, ...opts });
  return { status: r.status, stdout: r.stdout || '', stderr: r.stderr || '', error: r.error };
}

function git(args) {
  const r = sh('git', args, { cwd: REPO_ROOT });
  if (r.status !== 0) throw new Error(`git ${args.join(' ')} failed: ${r.stderr}`);
  return r.stdout.trim();
}

function buildEnv(profileEnv, extra = {}) {
  const env = {};
  for (const [k, v] of Object.entries(process.env)) if (!BUILD_ENV_SCRUB.test(k)) env[k] = v;
  return { ...env, ...profileEnv, ...extra };
}

function runBuild(webRoot, build, extraEnv, logFile) {
  const env = buildEnv(build.env || {}, extraEnv);
  const [cmd, ...args] = build.command;
  const started = Date.now();
  const r = sh(cmd, args, { cwd: webRoot, env });
  fs.writeFileSync(logFile, `$ ${Object.entries({ ...(build.env || {}), ...extraEnv }).map(([k, v]) => `${k}=${v} `).join('')}${build.command.join(' ')}\n${r.stdout}\n${r.stderr}`);
  if (r.status !== 0) throw new Error(`build failed (exit ${r.status}); see ${logFile}`);
  return Date.now() - started;
}

function capture({ profileName, budgetsFile, webRoot, force, allowWebDrift }) {
  const doc = readBudgets(budgetsFile);
  const profile = doc.profiles[profileName];
  if (!profile?.build?.command) throw new Error(`profile "${profileName}" has no build definition in ${budgetsFile}`);
  const distDir = path.join(webRoot, 'dist');
  const archive = path.join(ARCHIVE_ROOT, `web-${profileName}`);
  if (fs.existsSync(archive) && !force) {
    throw new Error(`archive already exists: ${archive} (baselines are immutable; pass --force to replace)`);
  }

  const head = git(['rev-parse', 'HEAD']);
  const drift = git(['diff', '--stat', BASELINE_REVISION, 'HEAD', '--', 'web/']);
  if (drift && !allowWebDrift) {
    throw new Error(`web/ differs between ${BASELINE_REVISION} and HEAD; refusing to label it a ${BASELINE_REVISION} baseline:\n${drift}`);
  }
  const dirty = git(['status', '--porcelain', '--untracked-files=all', '--', 'web/']);
  if (dirty) throw new Error(`web/ has uncommitted changes; refusing to capture:\n${dirty}`);

  const tmp = fs.mkdtempSync(path.join(os.tmpdir(), `t4-capture-${profileName}-`));
  console.error(`[T4] ${profileName}: sourcemap pass (dependency composition)`);
  const mapsMs = runBuild(webRoot, profile.build, { BUILD_SOURCEMAPS: 'true' }, path.join(tmp, 'build-sourcemaps.log'));
  const comp = composition(distDir);
  const mapsChunks = new Set(Object.keys(comp.chunks));

  console.error(`[T4] ${profileName}: production pass`);
  const prodMs = runBuild(webRoot, profile.build, {}, path.join(tmp, 'build.log'));
  const report = analyzeDist(distDir);
  const metrics = flattenMetrics(report);
  const prodJs = report.files.filter((f) => /\.js$/.test(f.path)).map((f) => f.path);
  comp.chunkNameMatch = {
    productionJs: prodJs.length,
    matchedInSourcemapPass: prodJs.filter((p) => mapsChunks.has(p)).length,
  };

  const verdict = checkProfile({
    distDir,
    webRoot,
    profileName,
    profile: { ...profile, budgets: { 'installed.totalBytes': Number.MAX_SAFE_INTEGER } },
  });
  if (!verdict.ok) throw new Error(`captured dist does not satisfy the profile:\n  ${verdict.failures.join('\n  ')}`);

  fs.rmSync(archive, { recursive: true, force: true });
  fs.mkdirSync(archive, { recursive: true });
  fs.cpSync(distDir, path.join(archive, 'dist'), { recursive: true, preserveTimestamps: true });
  for (const f of ['build.log', 'build-sourcemaps.log']) fs.copyFileSync(path.join(tmp, f), path.join(archive, f));
  fs.rmSync(tmp, { recursive: true, force: true });

  const hashOf = (rel) => {
    const abs = path.join(webRoot, rel);
    return fs.existsSync(abs) ? sha256(fs.readFileSync(abs)) : null;
  };
  const viteVersion = (() => {
    try {
      return JSON.parse(fs.readFileSync(path.join(webRoot, 'node_modules', 'vite', 'package.json'), 'utf8')).version;
    } catch {
      return null;
    }
  })();
  const config = {
    profile: profileName,
    plannedBaselineRevision: BASELINE_REVISION,
    builtRevision: head,
    webTreeDiffFromBaseline: drift || '(none)',
    capturedAt: new Date().toISOString(),
    cwd: 'web',
    command: profile.build.command.join(' '),
    env: profile.build.env || {},
    scrubbedEnvPattern: String(BUILD_ENV_SCRUB),
    sourcemapPass: `BUILD_SOURCEMAPS=true ${profile.build.command.join(' ')} (composition only; output discarded)`,
    buildDurationsMs: { sourcemaps: mapsMs, production: prodMs },
    toolchain: {
      node: process.version,
      npm: sh('npm', ['--version']).stdout.trim(),
      vite: viteVersion,
      platform: `${os.platform()}-${os.arch()}`,
    },
    inputHashes: Object.fromEntries(
      ['package.json', 'package-lock.json', 'vite.config.js', 'vite-plugin-theme-inject.js', 'postcss.config.js', 'js/version.js'].map(
        (f) => [`web/${f}`, hashOf(f)],
      ),
    ),
    versionJs: fs.existsSync(path.join(webRoot, 'js', 'version.js'))
      ? fs.readFileSync(path.join(webRoot, 'js', 'version.js'), 'utf8')
      : null,
  };
  fs.writeFileSync(path.join(archive, 'config.json'), `${JSON.stringify(config, null, 2)}\n`);
  fs.writeFileSync(
    path.join(archive, 'sha256sums.txt'),
    report.files.map((f) => `${f.sha256}  dist/${f.path}`).join('\n') + '\n',
  );
  const { files, ...reportNoFiles } = report;
  fs.writeFileSync(
    path.join(archive, 'report.json'),
    `${JSON.stringify({ ...reportNoFiles, distDir: 'web/dist', files, metrics, composition: comp }, null, 2)}\n`,
  );
  const proposed = Object.fromEntries(budgetKeysFor(metrics).map((k) => [k, proposeBudget(k, metrics[k])]));
  return { archive, metrics, proposed, config };
}

// ---------------------------------------------------------------------------
// CLI

function readBudgets(file) {
  if (!fs.existsSync(file)) throw new Error(`budgets file not found: ${file}`);
  const doc = JSON.parse(fs.readFileSync(file, 'utf8'));
  if (!doc || typeof doc.profiles !== 'object') throw new Error(`budgets file has no "profiles": ${file}`);
  return doc;
}

function parseArgs(argv) {
  const opts = { flags: new Set() };
  for (let i = 0; i < argv.length; i++) {
    const a = argv[i];
    if (['--profile', '--dist', '--web-root', '--budgets', '--from'].includes(a)) {
      if (i + 1 >= argv.length) throw new Error(`${a} needs a value`);
      opts[a.slice(2)] = argv[++i];
    } else if (a.startsWith('--')) opts.flags.add(a.slice(2));
    else throw new Error(`unexpected argument: ${a}`);
  }
  return opts;
}

function fmt(n) {
  return n === undefined ? '-' : n.toLocaleString('en-US');
}

export function main(argv) {
  let opts;
  try {
    opts = parseArgs(argv);
  } catch (err) {
    console.error(`usage error: ${err.message}`);
    return 2;
  }
  const webRoot = path.resolve(opts['web-root'] || DEFAULT_WEB_ROOT);
  const distDir = path.resolve(opts.dist || path.join(webRoot, 'dist'));
  const budgetsFile = path.resolve(opts.budgets || DEFAULT_BUDGETS);
  const mode = ['capture', 'check', 'report', 'propose'].filter((m) => opts.flags.has(m));
  if (mode.length !== 1) {
    console.error('usage: assets.mjs (--capture|--check|--propose) --profile <name> | --report [--dist DIR]');
    return 2;
  }
  try {
    if (mode[0] === 'report') {
      const report = analyzeDist(distDir);
      process.stdout.write(`${JSON.stringify({ ...report, metrics: flattenMetrics(report) }, null, 2)}\n`);
      return 0;
    }
    if (!opts.profile) {
      console.error('usage error: --profile is required');
      return 2;
    }
    if (mode[0] === 'propose') {
      const metrics = opts.from
        ? JSON.parse(fs.readFileSync(opts.from, 'utf8')).metrics
        : flattenMetrics(analyzeDist(distDir));
      const out = Object.fromEntries(budgetKeysFor(metrics).map((k) => [k, proposeBudget(k, metrics[k])]));
      const baseline = Object.fromEntries(Object.keys(out).map((k) => [k, metrics[k]]));
      process.stdout.write(`${JSON.stringify({ baseline, budgets: out }, null, 2)}\n`);
      return 0;
    }
    if (mode[0] === 'capture') {
      const r = capture({
        profileName: opts.profile,
        budgetsFile,
        webRoot,
        force: opts.flags.has('force'),
        allowWebDrift: opts.flags.has('allow-web-drift'),
      });
      console.log(`[T4] archived ${opts.profile} baseline -> ${path.relative(REPO_ROOT, r.archive)}`);
      console.log(`[T4] built revision ${r.config.builtRevision}; installed ${fmt(r.metrics['installed.totalBytes'])} bytes`);
      return 0;
    }
    // check
    const doc = readBudgets(budgetsFile);
    const profile = doc.profiles[opts.profile];
    const result = checkProfile({ distDir, webRoot, profileName: opts.profile, profile });
    if (result.rows.length) {
      const w = Math.max(...result.rows.map((r) => r.key.length));
      console.log(`${'metric'.padEnd(w)}  ${'baseline'.padStart(11)}  ${'current'.padStart(11)}  ${'budget'.padStart(11)}  used`);
      for (const r of result.rows) {
        console.log(
          `${r.key.padEnd(w)}  ${fmt(r.baseline).padStart(11)}  ${fmt(r.value).padStart(11)}  ${fmt(r.budget).padStart(11)}  ${(r.used * 100).toFixed(1)}%`,
        );
      }
    }
    if (result.ok) {
      console.log(`[T4] ${opts.profile}: OK (${result.rows.length} budgets)`);
      return 0;
    }
    for (const f of result.failures) console.error(`FAIL: ${f}`);
    console.error(`[T4] ${opts.profile}: ${result.failures.length} failure(s)`);
    return 1;
  } catch (err) {
    console.error(`FAIL: ${err.message}`);
    return 1;
  }
}

if (process.argv[1] && import.meta.url === pathToFileURL(path.resolve(process.argv[1])).href) {
  process.exitCode = main(process.argv.slice(2));
}
