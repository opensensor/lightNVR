import {readFile, writeFile, mkdir, readdir, mkdtemp, rm} from 'node:fs/promises';
import {execFileSync} from 'node:child_process';
import {fileURLToPath} from 'node:url';
import path from 'node:path';
import os from 'node:os';
import openapiTS, {astToString} from 'openapi-typescript';
import Ajv2020 from 'ajv/dist/2020.js';
import addFormats from 'ajv-formats';
import standaloneCode from 'ajv/dist/standalone/index.js';
import {buildSync} from 'esbuild';

export const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../..');
export const nativePath = 'include/web/openapi_document_generated.h';
export const webPaths = ['schema.d.ts', 'validators.js', 'validators.d.ts'].map(n => `web/generated/api/${n}`);
const quote = JSON.stringify;

export function validateSource(source, projected = false) {
  execFileSync('python3', [path.join(root, 'scripts/contracts/generate_native.py'), '--source', source, '--validate-only', ...(projected ? ['--projected'] : [])], {stdio: 'pipe'});
}

function schemaDocument(doc) {
  const schemaMaps = new Set(['properties', 'patternProperties', '$defs', 'definitions', 'dependentSchemas']);
  const schemaArrays = new Set(['allOf', 'anyOf', 'oneOf', 'prefixItems']);
  const schemaValues = new Set(['additionalProperties', 'unevaluatedProperties', 'items', 'additionalItems', 'contains',
    'propertyNames', 'not', 'if', 'then', 'else', 'unevaluatedItems', 'contentSchema']);
  const convertMap = map => {
    if (!map || typeof map !== 'object' || Array.isArray(map)) throw new Error('Expected a schema map object');
    return Object.fromEntries(Object.entries(map).map(([name, schema]) => [name, convert(schema)]));
  };
  const convert = schema => {
    if (!schema || typeof schema !== 'object' || Array.isArray(schema)) return schema;
    return Object.fromEntries(Object.entries(schema).flatMap(([key, child]) => {
      if (key.startsWith('x-') || ['example', 'discriminator', 'xml', 'externalDocs'].includes(key)) return [];
      if (key === '$ref') {
        if (typeof child !== 'string' || !child.startsWith('#/components/schemas/')) throw new Error('Validator schemas may only reference component schemas');
        return [[key, child.replace('#/components/schemas/', '#/$defs/')]];
      }
      if (schemaMaps.has(key)) return [[key, convertMap(child)]];
      if (schemaArrays.has(key)) return [[key, child.map(convert)]];
      if (schemaValues.has(key)) return [[key, convert(child)]];
      // enum/default/examples/const contain application data, not schemas.
      // Unknown keywords remain intact so strict Ajv compilation rejects them.
      return [[key, child]];
    }));
  };
  return {$schema: 'https://json-schema.org/draft/2020-12/schema', $id: 'urn:lightnvr:contracts', $defs: convertMap(doc.components?.schemas || {})};
}

export async function generateArtifacts(source, target = 'all') {
  if (!['all', 'native', 'web'].includes(target)) throw new Error('Target must be native, web or all');
  validateSource(source);
  const files = new Map();
  if (target !== 'web') {
    const temp = await mkdtemp(path.join(os.tmpdir(), 'lightnvr-contract-native-'));
    try {
      const output = path.join(temp, 'document.h');
      execFileSync('python3', [path.join(root, 'scripts/contracts/generate_native.py'), '--source', source, '--output', output], {stdio: 'pipe'});
      files.set(nativePath, await readFile(output, 'utf8'));
    } finally { await rm(temp, {recursive: true, force: true}); }
  }
  if (target !== 'native') {
    const doc = JSON.parse(await readFile(source, 'utf8'));
    const names = [...(doc['x-lightnvr-validators'] || Object.keys(doc.components?.schemas || {}))].sort();
    files.set(webPaths[0], astToString(await openapiTS(doc, {alphabetize: true, defaultNonNullable: false})));
    // Runtime consumers need validity, not an exhaustive payload error report.
    // Fail fast to bound invalid-response work and avoid redundant accumulator guards.
    const ajv = new Ajv2020({strict: true, allErrors: false, allowUnionTypes: true, validateFormats: true,
      coerceTypes: false, useDefaults: false, removeAdditional: false, code: {source: true, esm: true}});
    addFormats(ajv);
    ajv.addSchema(schemaDocument(doc));
    const schemaRef = name => `urn:lightnvr:contracts#/$defs/${name.replaceAll('~', '~0').replaceAll('/', '~1')}`;
    // Validate every component, even if no runtime export was selected for it.
    for (const name of Object.keys(doc.components?.schemas || {})) ajv.getSchema(schemaRef(name));
    const exports = Object.fromEntries(names.map(name => [`__contract_${name}`, schemaRef(name)]));
    let raw = standaloneCode(ajv, exports).replace(/export const (__contract_[A-Za-z0-9_]+) = /g, 'const $1 = ');
    raw += '\n' + names.map(name => `export function validate${name}(value) {\n  const valid = __contract_${name}(value);\n  validate${name}.errors = valid ? null : (__contract_${name}.errors || []).map(({keyword, schemaPath}) => ({keyword, schemaPath}));\n  return valid;\n}\nvalidate${name}.errors = null;`).join('\n');
    const bundle = buildSync({metafile: true, stdin: {contents: raw, resolveDir: root, sourcefile: 'standalone-validators.js'}, bundle: true, write: false,
      format: 'esm', platform: 'browser', target: 'es2020', legalComments: 'none', logLevel: 'silent'});
    if (Object.values(bundle.metafile.outputs).some(output => output.imports.length)) throw new Error('Generated validators contain unresolved imports');
    for (const input of Object.keys(bundle.metafile.inputs)) {
      if (input !== 'standalone-validators.js' && !/^node_modules\/(ajv\/dist\/runtime|ajv-formats\/dist)\//.test(input)) throw new Error(`Unexpected validator runtime input: ${input}`);
    }
    const bundled = bundle.outputFiles[0].text;
    files.set(webPaths[1], '// Generated by scripts/contracts/generate.mjs; do not edit.\n' + bundled);
    files.set(webPaths[2], '// Generated by scripts/contracts/generate.mjs; do not edit.\nimport type { components } from "./schema";\n' +
      'export interface ContractValidationError { readonly keyword: string; readonly schemaPath: string }\n' +
      'export interface ContractValidator<T> { (value: unknown): value is T; errors: readonly ContractValidationError[] | null }\n' +
      names.map(name => `export declare const validate${name}: ContractValidator<components["schemas"][${quote(name)}]>;`).join('\n') + '\n');
  }
  return files;
}

export async function writeArtifacts(files, outputRoot) {
  if (files.has(webPaths[0])) {
    const dir = path.join(outputRoot, 'web/generated/api');
    await mkdir(dir, {recursive: true});
    for (const entry of await readdir(dir)) {
      if (!webPaths.some(file => path.basename(file) === entry)) await rm(path.join(dir, entry), {recursive: true, force: true});
    }
  }
  for (const [file, content] of files) {
    const output = path.join(outputRoot, file);
    await mkdir(path.dirname(output), {recursive: true});
    await writeFile(output, content);
  }
}

export async function checkArtifacts(files, outputRoot) {
  for (const [file, content] of files) {
    if (await readFile(path.join(outputRoot, file), 'utf8').catch(() => null) !== content) throw new Error(`Generated drift: ${file}`);
  }
  if (files.has(webPaths[0])) {
    const extras = (await readdir(path.join(outputRoot, 'web/generated/api'))).filter(name => !webPaths.some(file => path.basename(file) === name));
    if (extras.length) throw new Error(`Unexpected generated files: ${extras.join(', ')}`);
  }
}

async function main() {
  const args = process.argv.slice(2);
  let source = path.join(root, 'docs/api/openapi.json'), outputRoot = root, target = 'all', check = false;
  for (let i = 0; i < args.length; i++) {
    if (args[i] === '--check') check = true;
    else if (args[i].startsWith('--target=')) target = args[i].slice(9);
    else if (args[i] === '--source') source = path.resolve(args[++i]);
    else if (args[i] === '--output-root') outputRoot = path.resolve(args[++i]);
    else throw new Error(`Unknown argument: ${args[i]}`);
  }
  const files = await generateArtifacts(source, target);
  if (check) {
    const temp = await mkdtemp(path.join(os.tmpdir(), 'lightnvr-contract-check-'));
    try { await writeArtifacts(files, temp); await checkArtifacts(files, outputRoot); }
    finally { await rm(temp, {recursive: true, force: true}); }
  } else await writeArtifacts(files, outputRoot);
  console.log(`${check ? 'Checked' : 'Generated'} ${files.size} contract artifacts (${target})`);
}
if (process.argv[1] && path.resolve(process.argv[1]) === fileURLToPath(import.meta.url)) {
  main().catch(error => { console.error(error.message); if (error.stderr) console.error(String(error.stderr)); process.exitCode = 1; });
}
