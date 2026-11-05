#!/usr/bin/env node
/** Reproducible installed bytes and per-entry local asset closure (includes lazy imports). */
import {readdir, readFile, writeFile} from 'node:fs/promises';
import {execFileSync} from 'node:child_process';
import {createHash} from 'node:crypto';
import {gzipSync} from 'node:zlib';
import path from 'node:path';
import os from 'node:os';
const args = process.argv.slice(2);
const get = key => { const i = args.indexOf(key); if (i < 0 || !args[i+1] || args[i+1].startsWith('--')) throw new Error(`Required: ${key}`); return args[i+1]; };
const dist = path.resolve(get('--dist')), profile = get('--profile'), out = get('--out');
if (!['default','modern'].includes(profile)) throw new Error('Profile must be default or modern');
const files = new Map();
async function walk(dir) {
  for (const entry of (await readdir(dir,{withFileTypes:true})).sort((a,b) => a.name.localeCompare(b.name))) {
    const absolute = path.join(dir,entry.name);
    if (entry.isDirectory()) await walk(absolute);
    else if (entry.isFile()) {
      const bytes = await readFile(absolute);
      files.set(path.relative(dist,absolute).split(path.sep).join('/'), {bytes, size: bytes.length, sha256: createHash('sha256').update(bytes).digest('hex')});
    } else throw new Error(`Unexpected non-file asset: ${absolute}`);
  }
}
await walk(dist);
if (!files.has('streams.html')) throw new Error('Expected built streams.html');
function references(name) {
  if (!/\.(html|js|css)$/.test(name)) return [];
  const source = files.get(name).bytes.toString('utf8'), refs = new Set();
  // Vite's HTML src/href/data-src, ESM imports, dynamic imports, System.register
  // dependencies and CSS URLs are local quoted strings. Include dynamic assets.
  for (const match of source.matchAll(/["'`]([^"'`\s<>]+)["'`]|url\(([^)\s]+)\)/g)) {
    const value = (match[1] || match[2]).split(/[?#]/)[0];
    if (/^(?:[a-z]+:|\/\/|#)/i.test(value)) continue;
    const candidates = [value.replace(/^\//,''), path.posix.normalize(path.posix.join(path.posix.dirname(name),value))];
    for (const candidate of candidates) if (files.has(candidate) && !/\.(?:gz|html)$/.test(candidate)) refs.add(candidate);
  }
  return [...refs].sort();
}
const entries = {};
for (const name of [...files.keys()].filter(n => n.endsWith('.html'))) {
  const seen = new Set();
  const visit = n => { if (seen.has(n)) return; seen.add(n); for (const r of references(n)) visit(r); };
  visit(name);
  const assets = [...seen].sort();
  entries[name] = {bytes: assets.reduce((n,f) => n+files.get(f).size,0), gzipBytes: assets.reduce((n,f) => n+gzipSync(files.get(f).bytes,{level:9}).length,0), files: assets};
}
const git = (...args) => execFileSync('git',args,{encoding:'utf8'}).trim();
async function sourceDigest(paths) {
  const names = [...new Set(git('ls-files','--cached','--others','--exclude-standard','--',...paths).split('\n').filter(Boolean))].sort();
  const hash = createHash('sha256');
  for (const name of names) { hash.update(name); hash.update('\0'); hash.update(await readFile(name)); hash.update('\0'); }
  return {sha256: hash.digest('hex'), files: names};
}
const report = {
  formatVersion: 1, profile, sourceRevision: git('rev-parse','HEAD'),
  relevantSource: await sourceDigest(['src','include','CMakeLists.txt','web','package.json','package-lock.json','scripts/measure-contract-assets.mjs','tests/contracts']),
  trackedDiffSha256: createHash('sha256').update(git('diff','HEAD','--','src','include','CMakeLists.txt','web','package.json','package-lock.json')).digest('hex'),
  environment: {node:process.version, platform:process.platform, arch:process.arch, release:os.release()},
  methodology: 'Installed bytes include every dist file, including precompressed .gz copies. Per-entry bytes are a unique local asset closure, including discoverable lazy imports and both legacy/modern scripts, not initial network transfer. Navigation to other HTML entries, runtime API media and computed asset URLs are excluded. gzipBytes sums level-9 gzip of each closure member.',
  installedBytes: [...files.values()].reduce((n,f) => n+f.size,0), fileCount: files.size,
  entries,
  files: Object.fromEntries([...files].map(([name,{size,sha256}]) => [name,{bytes:size,sha256}]))
};
await writeFile(out, `${JSON.stringify(report,null,2)}\n`);
console.log(`${profile}: ${report.installedBytes} installed bytes; streams closure ${entries['streams.html'].bytes} bytes (${entries['streams.html'].gzipBytes} gzip); ${out}`);
