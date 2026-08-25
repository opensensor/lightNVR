#!/usr/bin/env node

import { mkdir, readFile, readdir, writeFile } from 'node:fs/promises';
import { dirname, extname, join, relative, resolve } from 'node:path';
import { fileURLToPath } from 'node:url';

const MOBILE_WIDTH_PX = 375;
const REQUIRED_SIZE_PX = 44;
const repositoryRoot = resolve(dirname(fileURLToPath(import.meta.url)), '..');
const sourceRoot = join(repositoryRoot, 'web/js/components/preact');
const distRoot = join(repositoryRoot, 'web/dist');
const defaultOutput = join(distRoot, 'touch-target-audit.csv');

function parseArguments(argv) {
  const outputIndex = argv.indexOf('--output');
  return {
    output: outputIndex >= 0 && argv[outputIndex + 1]
      ? resolve(process.cwd(), argv[outputIndex + 1])
      : defaultOutput,
  };
}

async function walkFiles(root, acceptedExtensions) {
  const files = [];
  const entries = await readdir(root, { withFileTypes: true });
  for (const entry of entries) {
    const path = join(root, entry.name);
    if (entry.isDirectory()) {
      files.push(...await walkFiles(path, acceptedExtensions));
    } else if (acceptedExtensions.has(extname(entry.name))) {
      files.push(path);
    }
  }
  return files;
}

function lineNumberAt(source, index) {
  let line = 1;
  for (let cursor = 0; cursor < index; cursor += 1) {
    if (source.charCodeAt(cursor) === 10) line += 1;
  }
  return line;
}

function attributeValue(openingTag, name) {
  const expression = new RegExp(
    '\\b' + name + '\\s*=\\s*(?:"([^"]*)"|\'([^\']*)\'|\\{\\s*"([^"]*)"\\s*\\}|\\{\\s*\'([^\']*)\'\\s*\\}|\\{\\s*`([^`]*)`\\s*\\})',
    's'
  ).exec(openingTag);
  return expression ? expression.slice(1).find((value) => value !== undefined) : '';
}

function classTokens(openingTag) {
  const classValue = attributeValue(openingTag, 'class(?:Name)?');
  return classValue
    .replace(/\$\{[^}]*\}/g, ' ')
    .split(/\s+/)
    .map((token) => token.trim())
    .filter(Boolean);
}

function selectorFor(tagName, openingTag) {
  const id = attributeValue(openingTag, 'id');
  const classes = classTokens(openingTag)
    .filter((token) => /^[A-Za-z0-9_:/.[\]-]+$/.test(token))
    .slice(0, 4);
  return `${tagName}${id ? `#${id}` : ''}${classes.map((name) => `.${name}`).join('')}`;
}

function isInteractive(tagName, openingTag) {
  const lowerName = tagName.toLowerCase();
  if (lowerName === 'input' && attributeValue(openingTag, 'type').toLowerCase() === 'hidden') {
    return false;
  }
  if (['button', 'input', 'select', 'textarea', 'asyncbutton'].includes(lowerName)) {
    return true;
  }
  if (lowerName === 'a' || lowerName === 'link') {
    return /\b(?:href|to)\s*=/.test(openingTag);
  }
  return attributeValue(openingTag, 'role').toLowerCase() === 'button';
}

function isChoiceInput(tagName, openingTag) {
  if (tagName.toLowerCase() !== 'input') return false;
  return ['checkbox', 'radio'].includes(attributeValue(openingTag, 'type').toLowerCase());
}

function hasAssociatedLabel(source, openingTag, index) {
  const id = attributeValue(openingTag, 'id');
  if (id) {
    const escapedId = id.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
    const explicitLabel = new RegExp(
      `<label\\b[^>]*\\b(?:for|htmlFor)\\s*=\\s*(?:"${escapedId}"|'${escapedId}')`,
      'i'
    );
    if (explicitLabel.test(source)) return true;
  }

  const labelStart = source.lastIndexOf('<label', index);
  const labelEnd = source.lastIndexOf('</label>', index);
  if (labelStart <= labelEnd) return false;
  const openingEnd = source.indexOf('>', labelStart);
  if (openingEnd < 0 || openingEnd > index) return false;
  const labelClasses = classTokens(source.slice(labelStart, openingEnd + 1));
  return labelClasses.includes('cursor-pointer') || labelClasses.includes('touch-target');
}

function dimensionFromTokens(tokens, axis) {
  const names = axis === 'width'
    ? ['min-w-', 'w-', 'size-']
    : ['min-h-', 'h-', 'size-'];
  let result = 0;
  for (const token of tokens) {
    const mobileToken = token.replace(/^(?:max-sm:|sm:)/, '');
    for (const prefix of names) {
      if (!mobileToken.startsWith(prefix)) continue;
      const value = mobileToken.slice(prefix.length);
      if (value === 'full' || value === 'screen') return Number.POSITIVE_INFINITY;
      if (/^\[(\d+(?:\.\d+)?)px\]$/.test(value)) {
        result = Math.max(result, Number(value.slice(1, -3)));
      } else if (/^\d+(?:\.\d+)?$/.test(value)) {
        result = Math.max(result, Number(value) * 4);
      }
    }
  }
  return result;
}

function compiledFloors(css) {
  const normalized = css.replace(/\s+/g, '');
  const tokenMatch = /--touch-target-min:([\d.]+)rem/.exec(normalized);
  if (!tokenMatch) {
    throw new Error('Compiled CSS is missing --touch-target-min; run the production build before the audit.');
  }
  const targetSize = Number(tokenMatch[1]) * 16;
  const hasMinimumDeclarations = normalized.includes('min-height:var(--touch-target-min)')
    && normalized.includes('min-width:var(--touch-target-min)');
  const hasMobileQuery = normalized.includes('@media(max-width:639px)');
  const hasInteractiveSelectors = ['button', 'a[href]', '[role=button]', 'input', 'select', 'textarea']
    .every((selector) => normalized.includes(selector));
  const hasFocusFallback = normalized.includes(':focus-visible')
    && normalized.includes('outline:2pxsolidhsl(var(--ring))!important');

  if (!hasMinimumDeclarations || !hasMobileQuery || !hasInteractiveSelectors) {
    throw new Error('Compiled CSS is missing the mobile interactive-element floor.');
  }
  if (!hasFocusFallback) {
    throw new Error('Compiled CSS is missing the focus-visible fallback.');
  }
  return { targetSize, hasFocusFallback };
}

function csvCell(value) {
  const text = String(value);
  return /[",\n]/.test(text) ? `"${text.replaceAll('"', '""')}"` : text;
}

async function main() {
  const { output } = parseArguments(process.argv.slice(2));
  const cssFiles = await walkFiles(distRoot, new Set(['.css']));
  if (cssFiles.length === 0) {
    throw new Error(`No compiled CSS found under ${relative(repositoryRoot, distRoot)}; run npm run build first.`);
  }
  const css = (await Promise.all(cssFiles.map((path) => readFile(path, 'utf8')))).join('\n');
  const floors = compiledFloors(css);
  const sourceFiles = await walkFiles(sourceRoot, new Set(['.jsx', '.js', '.tsx', '.ts']));
  const audited = [];

  for (const path of sourceFiles) {
    const source = await readFile(path, 'utf8');
    const tagPattern = /<([A-Za-z][A-Za-z0-9.]*)\b[\s\S]*?>/g;
    for (const match of source.matchAll(tagPattern)) {
      const [openingTag, tagName] = match;
      if (!isInteractive(tagName, openingTag)) continue;

      const tokens = classTokens(openingTag);
      const declaredWidth = dimensionFromTokens(tokens, 'width');
      const declaredHeight = dimensionFromTokens(tokens, 'height');
      const choiceInput = isChoiceInput(tagName, openingTag);
      const targetFloor = !choiceInput || hasAssociatedLabel(source, openingTag, match.index)
        ? floors.targetSize
        : 0;
      const computedWidth = Math.max(declaredWidth, targetFloor);
      const computedHeight = Math.max(declaredHeight, targetFloor);
      audited.push({
        file: relative(repositoryRoot, path),
        line: lineNumberAt(source, match.index),
        selector: selectorFor(tagName, openingTag),
        width: computedWidth,
        height: computedHeight,
      });
    }
  }

  const findings = audited.filter(({ width, height }) => (
    width < REQUIRED_SIZE_PX || height < REQUIRED_SIZE_PX
  ));
  const header = ['file', 'line', 'selector', 'computed_width_px', 'computed_height_px'];
  const rows = findings.map((finding) => [
    finding.file,
    finding.line,
    finding.selector,
    Number.isFinite(finding.width) ? finding.width : 'viewport',
    Number.isFinite(finding.height) ? finding.height : 'viewport',
  ]);
  const csv = [header, ...rows]
    .map((row) => row.map(csvCell).join(','))
    .join('\n');

  await mkdir(dirname(output), { recursive: true });
  await writeFile(output, `${csv}\n`);
  console.log(`Touch target audit: ${audited.length} interactive JSX elements checked at ${MOBILE_WIDTH_PX}px.`);
  console.log(`Focus-visible fallback: ${floors.hasFocusFallback ? 'present' : 'missing'}.`);
  console.log(`Findings below ${REQUIRED_SIZE_PX}x${REQUIRED_SIZE_PX}px: ${findings.length}.`);
  console.log(`CSV: ${relative(repositoryRoot, output)}`);
  process.exitCode = 0;
}

main().catch((error) => {
  console.error(`Touch target audit failed: ${error.message}`);
  process.exitCode = 1;
});
