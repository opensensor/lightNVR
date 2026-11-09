import test, {before, after} from 'node:test';
import assert from 'node:assert/strict';
import {readFile, writeFile, mkdir, mkdtemp, rm} from 'node:fs/promises';
import {execFileSync, spawnSync} from 'node:child_process';
import {pathToFileURL} from 'node:url';
import path from 'node:path';
import os from 'node:os';
import {root, generateArtifacts, writeArtifacts, checkArtifacts, nativePath, webPaths} from '../../../scripts/contracts/generate.mjs';
const source = path.join(root,'tests/contracts/tooling/openapi.json');
let directory, artifacts, validators;
before(async () => {
  directory = await mkdtemp(path.join(os.tmpdir(),'lightnvr-contract-tooling-'));
  await writeFile(path.join(directory,'package.json'),'{"type":"module"}');
  artifacts = await generateArtifacts(source);
  await writeArtifacts(artifacts,directory);
  validators = await import(pathToFileURL(path.join(directory,webPaths[1])));
});
after(async () => { if(directory) await rm(directory,{recursive:true,force:true}); });
const payload = () => ({identity:{id:'00000000-0000-4000-8000-000000000001'},count:0,variant:'ready',extra:{kept:true}});

test('2020-12 standalone validators preserve data, validate formats/unions/null/absence and sanitize diagnostics', () => {
  const value = payload(), before = structuredClone(value);
  assert.equal(validators.validateFixture(value),true);
  assert.deepEqual(value,before); assert.equal(Object.hasOwn(value,'label'),false);
  for(const optional of [null,'', '🐱'.repeat(20)]) assert.equal(validators.validateFixture({...value,optional}),true);
  assert.equal(validators.validateFixture({...value,variant:2,created:'2026-11-05T12:30:00Z'}),true);
  for(const invalid of [{...value,count:'0'},{...value,identity:{}},{...value,identity:{id:'not-a-uuid'}},{...value,created:'not-a-date'},{...value,optional:'🐱'.repeat(21)},{...value,variant:false},null,[]]) {
    const saved=structuredClone(invalid);
    assert.equal(validators.validateFixture(invalid),false);
    assert.deepEqual(invalid,saved);
    assert.ok(validators.validateFixture.errors.length);
    for(const error of validators.validateFixture.errors) assert.deepEqual(Object.keys(error).sort(),['keyword','schemaPath']);
  }
  const multipleInvalid = {...value, count: '0', created: 'not-a-date'};
  const savedInvalid = structuredClone(multipleInvalid);
  assert.equal(validators.validateFixture(multipleInvalid), false);
  assert.equal(validators.validateFixture.errors.length, 1);
  assert.deepEqual(multipleInvalid, savedInvalid);
  assert.equal(validators.validateTuple(['first',2]),true);
  assert.equal(validators.validateTuple(['first','2']),false);
  assert.equal(validators.validateTuple(['first',2,3]),false);
  assert.equal(validators.validateFixture(value),true); assert.equal(validators.validateFixture.errors,null);
  const literalNames = ['example','discriminator','xml','externalDocs','x-foo','$ref'];
  const literal = Object.fromEntries(literalNames.map(name => [name, 1]));
  assert.equal(validators.validateLiteralNames(literal),true);
  for (const name of literalNames) {
    assert.equal(validators.validateLiteralNames({...literal,[name]:'wrong'}),false, name);
    const missing={...literal};delete missing[name];
    assert.equal(validators.validateLiteralNames(missing),false, name);
  }
});

test('native Python and web generation are deterministic; drift check fails without rewriting artifacts', async () => {
  const again = await generateArtifacts(source);
  assert.deepEqual([...again], [...artifacts]);
  execFileSync('python3',[path.join(root,'scripts/contracts/generate_native.py'),'--source',source,'--output',path.join(directory,'direct.h')]);
  assert.equal(await readFile(path.join(directory,'direct.h'),'utf8'),artifacts.get(nativePath));
  await checkArtifacts(again,directory);
  const corrupt = path.join(directory,webPaths[0]);
  await writeFile(corrupt,'deliberate drift\n');
  const result=spawnSync(process.execPath,[path.join(root,'scripts/contracts/generate.mjs'),'--check','--source',source,'--output-root',directory],{encoding:'utf8'});
  assert.equal(result.status,1); assert.match(result.stderr,/Generated drift/);
  assert.equal(await readFile(corrupt,'utf8'),'deliberate drift\n');
  await writeFile(corrupt,artifacts.get(webPaths[0]));
  await checkArtifacts(artifacts,directory);
  assert.deepEqual([...await generateArtifacts(source,'native')].map(([p])=>p),[nativePath]);
  assert.deepEqual([...await generateArtifacts(source,'web')].map(([p])=>p),webPaths);
});

test('generation rejects unsupported policy metadata, remote/dynamic references and schema keywords', async () => {
  const original=JSON.parse(await readFile(source,'utf8'));
  const mutations=[
    doc=>{doc.paths['/api/fixture'].get['x-lightnvr-authorization'].kind='unknown';},
    doc=>{doc.paths['/api/fixture'].get['x-lightnvr-authorization'].typo=true;},
    doc=>{doc.paths['/api/fixture'].get['x-lightnvr-authorization']={version:1,kind:'non_scoped_identity',requirements:[]};},
    doc=>{doc.paths['/api/fixture'].get['x-lightnvr-authorization']={version:1,kind:'non_scoped_identity',extra:true};},
    doc=>{doc.paths['/api/fixture'].get['x-lightnvr-authorization']={version:1,kind:'identity_management',requirements:[]};},
    doc=>{doc.paths['/api/fixture'].get['x-lightnvr-authorization']={version:1,kind:'camera',requirements:[{action:'super.admin',resource:'raw'}]};},
    doc=>{delete doc.paths['/api/fixture'].get['x-lightnvr-authorization'];},
    doc=>{doc.components.schemas.Identity.$ref='https://example.invalid/remote.json';},
    doc=>{doc.components.schemas.Identity.$dynamicRef='#elsewhere';},
    doc=>{doc.components.schemas.Fixture.misspelledSchemaKeyword=true;},
    doc=>{doc.components.schemas.Identity.misspelledUnselectedKeyword=true;},
  ];
  for(let i=0;i<mutations.length;i++) {
    const doc=structuredClone(original);mutations[i](doc);
    const invalid=path.join(directory,`invalid-${i}.json`);await writeFile(invalid,JSON.stringify(doc));
    await assert.rejects(generateArtifacts(invalid));
  }
});

test('generated declarations accept unknown and narrow correctly under strict TS 5 and application TS 6', async () => {
  const positive=`import {validateFixture} from './web/generated/api/validators';\nimport type {components} from './web/generated/api/schema';\nconst wire: unknown = JSON.parse('{}');\nif (validateFixture(wire)) { const n: number = wire.count; const id: string = wire.identity.id; void [n,id]; }\nconst good: components['schemas']['Fixture'] = {identity:{id:'synthetic'},count:0,variant:1}; void good;\n`;
  await writeFile(path.join(directory,'consumer.ts'),positive);
  const config={compilerOptions:{strict:true,noEmit:true,skipLibCheck:false,target:'ES2022',module:'ESNext',moduleResolution:'Bundler',types:[],rootDir:'.'},files:['consumer.ts']};
  await writeFile(path.join(directory,'tsconfig.json'),JSON.stringify(config));
  for(const compiler of ['node_modules/typescript/bin/tsc','web/node_modules/typescript/bin/tsc']) {
    const run=()=>spawnSync(process.execPath,[path.join(root,compiler),'-p',path.join(directory,'tsconfig.json')],{encoding:'utf8'});
    const good=run();assert.equal(good.status,0,good.stdout);
    await writeFile(path.join(directory,'consumer.ts'),positive+"const wrong: components['schemas']['Fixture'] = {identity:{id:'synthetic'},count:'zero',variant:1}; void wrong;\n");
    const bad=run();assert.notEqual(bad.status,0);assert.match(bad.stdout,/not assignable to type 'number'/);
    await writeFile(path.join(directory,'consumer.ts'),positive);
  }
});

test('independent OpenAPI lint checks canonical and actual projection files, rejecting empty input sets', async () => {
  execFileSync(process.execPath,[path.join(root,'scripts/contracts/lint.mjs'),'--source',source],{stdio:'pipe'});
  const projections=path.join(directory,'projections');await mkdir(projections);
  const empty=spawnSync(process.execPath,[path.join(root,'scripts/contracts/lint.mjs'),'--projections',projections],{encoding:'utf8'});
  assert.notEqual(empty.status,0);assert.match(empty.stderr,/No projection JSON/);
  const doc=JSON.parse(await readFile(source,'utf8')); delete doc.paths['/api/fixture'].get['x-lightnvr-authorization'];
  await writeFile(path.join(projections,'projected.json'),JSON.stringify(doc));
  execFileSync(process.execPath,[path.join(root,'scripts/contracts/lint.mjs'),'--projections',projections],{stdio:'pipe'});
  doc.paths={}; doc.components={}; delete doc['x-lightnvr-validators'];
  await writeFile(path.join(projections,'empty.json'),JSON.stringify(doc));
  // Empty personalized specs remain valid; do not retain unresolved root security.
  delete doc.security; await writeFile(path.join(projections,'empty.json'),JSON.stringify(doc));
  execFileSync(process.execPath,[path.join(root,'scripts/contracts/lint.mjs'),'--projections',projections],{stdio:'pipe'});
});

for (const kind of ['non_scoped_identity','identity_management']) test(`${kind} metadata accepts only the reviewed version-1 simple shape`, async () => {
  const doc=JSON.parse(await readFile(source,'utf8'));
  doc.paths['/api/fixture'].get['x-lightnvr-authorization']={version:1,kind};
  const candidate=path.join(directory,'non-scoped.json');
  await writeFile(candidate,JSON.stringify(doc));
  assert.equal((await generateArtifacts(candidate)).size,4);
});

for (const kind of ['non_scoped_global','non_scoped_camera','global_or_camera']) test(`${kind} validates action requirements and rejects unknown metadata`, async () => {
  const doc=JSON.parse(await readFile(source,'utf8'));
  const predicate={version:1,kind,requirements:[{action:kind==='non_scoped_global'?'users.manage':'live.view',resource:'raw'}]};
  doc.paths['/api/fixture'].get['x-lightnvr-authorization']=predicate;
  const candidate=path.join(directory,`${kind}.json`);await writeFile(candidate,JSON.stringify(doc));
  assert.equal((await generateArtifacts(candidate)).size,4);
  for(const invalid of [{...predicate,requirements:[]},{...predicate,requirements:[{action:'unknown.action',resource:'raw'}]},{...predicate,extra:true},...(['non_scoped_global','global_or_camera'].includes(kind)?[{...predicate,requirements:[{action:'users.manage',resource:'live'}]}]:[])]){
    doc.paths['/api/fixture'].get['x-lightnvr-authorization']=invalid;await writeFile(candidate,JSON.stringify(doc));
    await assert.rejects(generateArtifacts(candidate));
  }
});

test('native generator rejects malformed validator names with validation errors', async () => {
  const original = JSON.parse(await readFile(source, 'utf8'));
  for (const selected of [{}, [null], [1], [[]], [{}], ['Fixture', 'Fixture'], ['missing'], ['bad-name']]) {
    const candidate = path.join(directory, 'invalid-native-validators.json');
    await writeFile(candidate, JSON.stringify({...original, 'x-lightnvr-validators': selected}));
    const result = spawnSync('python3', [path.join(root, 'scripts/contracts/generate_native.py'),
      '--source', candidate, '--validate-only'], {encoding: 'utf8'});
    assert.notEqual(result.status, 0);
    assert.match(result.stderr, /ValueError: (Validator names|x-lightnvr-validators)/);
    assert.doesNotMatch(result.stderr, /TypeError/);
  }
});

test('reserved-word schema names retain their prefixed public validator names', async () => {
  const doc=JSON.parse(await readFile(source,'utf8'));
  const names=['class','default','await','yield','Class','with_underscore'];
  for(const name of names)doc.components.schemas[name]={type:'string'};
  doc['x-lightnvr-validators']=names;
  const candidate=path.join(directory,'reserved-validator-names.json');
  await writeFile(candidate,JSON.stringify(doc));
  const generated=await generateArtifacts(candidate,'web');
  const modulePath=path.join(directory,'reserved-validators.mjs');
  await writeFile(modulePath,generated.get(webPaths[1]));
  const module=await import(pathToFileURL(modulePath));
  assert.deepEqual(Object.keys(module).sort(),names.map(name=>`validate${name}`).sort());
  for(const name of names){
    const validate=module[`validate${name}`];
    assert.equal(validate('valid'),true);
    assert.equal(validate(null),false);
    assert.match(generated.get(webPaths[2]),new RegExp(`export declare const validate${name}:`));
    assert.ok(generated.get(webPaths[2]).includes(`components["schemas"]["${name}"]`));
  }
});

test('web generation rejects unsafe selected schema names before interpolation', async () => {
  const original=JSON.parse(await readFile(source,'utf8'));
  for(const name of ['with-dash','with/slash','with~tilde','1Thing','_Thing','$Thing',"Thing);throw new Error('injected');//"]){
    for(const explicit of [true,false]){
      const doc=structuredClone(original);
      doc.components.schemas[name]={type:'string'};
      if(explicit)doc['x-lightnvr-validators']=[name];
      else delete doc['x-lightnvr-validators'];
      const candidate=path.join(directory,'unsafe-validator-name.json');
      await writeFile(candidate,JSON.stringify(doc));
      await assert.rejects(generateArtifacts(candidate,'web'),error=>{
        assert.match(String(error.stderr),/Validator names must identify named component schemas/);
        return true;
      });
    }
  }
});

test('native generator reads UTF-8 schemas even under an ASCII locale', async () => {
  const doc = JSON.parse(await readFile(source, 'utf8'));
  doc.info.description = 'Camera café 🐱';
  const candidate = path.join(directory, 'unicode-native.json');
  await writeFile(candidate, JSON.stringify(doc), 'utf8');
  const result = spawnSync('python3', ['-X', 'utf8=0', path.join(root, 'scripts/contracts/generate_native.py'),
    '--source', candidate, '--validate-only'], {encoding: 'utf8',
    env: {...process.env, LC_ALL: 'C', PYTHONCOERCECLOCALE: '0'}});
  assert.equal(result.status, 0, result.stderr);
});

test('native authorization requirements reject non-string actions and resources', async () => {
  const original = JSON.parse(await readFile(source, 'utf8'));
  for (const field of ['action', 'resource']) {
    for (const value of [null, true, 42, [], {}, 'unknown']) {
      const doc = structuredClone(original);
      doc.paths['/api/fixture'].get['x-lightnvr-authorization'] = {
        version: 1, kind: 'camera', requirements: [{action: 'live.view', resource: 'raw', [field]: value}],
      };
      const candidate = path.join(directory, 'invalid-native-requirement.json');
      await writeFile(candidate, JSON.stringify(doc));
      const result = spawnSync('python3', [path.join(root, 'scripts/contracts/generate_native.py'),
        '--source', candidate, '--validate-only'], {encoding: 'utf8'});
      assert.notEqual(result.status, 0);
      assert.match(result.stderr, /ValueError: Unsupported authorization action or resource/);
      assert.doesNotMatch(result.stderr, /TypeError/);
    }
  }
});

test('native discriminator mappings accept local references and reject malformed values', async () => {
  const original = JSON.parse(await readFile(source, 'utf8'));
  const cases = [
    [{identity: 'Identity'}, true], [{identity: '#/components/schemas/Identity'}, true],
    ...[null, true, 42, [], {}, 'Missing', '#/components/schemas/Missing', 'https://example.invalid/schema']
      .map(value => [{identity: value}, false]),
    ...[null, [], 'Identity', 42].map(value => [value, false]),
  ];
  for (const [mapping, valid] of cases) {
    const doc = structuredClone(original);
    doc.components.schemas.Fixture.discriminator = {propertyName: 'variant', mapping};
    const candidate = path.join(directory, 'native-discriminator.json');
    await writeFile(candidate, JSON.stringify(doc));
    const result = spawnSync('python3', [path.join(root, 'scripts/contracts/generate_native.py'),
      '--source', candidate, '--validate-only'], {encoding: 'utf8'});
    if (valid) assert.equal(result.status, 0, result.stderr);
    else {
      assert.notEqual(result.status, 0);
      assert.match(result.stderr, /ValueError:/);
      assert.doesNotMatch(result.stderr, /TypeError|AttributeError/);
    }
  }
});

test('native generation rejects path-item refs consistently with runtime projection', async () => {
  const original = JSON.parse(await readFile(source, 'utf8'));
  const referenced = structuredClone(original.paths['/api/fixture']);
  for (const projected of [false, true]) {
    for (const variant of ['valid', 'missing-authorization', 'missing-operation-id', 'sibling', 'cycle', 'non-object']) {
      const doc = structuredClone(original);
      doc.components.pathItems = {Fixture: structuredClone(referenced)};
      if (variant === 'missing-authorization') delete doc.components.pathItems.Fixture.get['x-lightnvr-authorization'];
      if (variant === 'missing-operation-id') delete doc.components.pathItems.Fixture.get.operationId;
      if (variant === 'cycle') doc.components.pathItems.Fixture = {$ref: '#/components/pathItems/Fixture'};
      if (variant === 'non-object') doc.components.pathItems.Fixture = [];
      doc.paths['/api/fixture'] = {$ref: '#/components/pathItems/Fixture'};
      if (variant === 'sibling') doc.paths['/api/fixture'].post = {operationId: 'unvalidatedSibling'};
      const candidate = path.join(directory, 'native-path-reference.json');
      await writeFile(candidate, JSON.stringify(doc));
      const result = spawnSync('python3', [path.join(root, 'scripts/contracts/generate_native.py'),
        '--source', candidate, '--validate-only', ...(projected ? ['--projected'] : [])], {encoding: 'utf8'});
      assert.notEqual(result.status, 0, `${variant}, projected=${projected}`);
      assert.match(result.stderr, /ValueError: Path item references are not supported/);
    }
  }
  assert.equal((await generateArtifacts(source)).size, 4);
});

test('native validation checks component containers before walking discriminator references', async () => {
  const original = JSON.parse(await readFile(source, 'utf8'));
  for (const field of ['components', 'schemas']) {
    for (const value of [null, [], 'invalid', 42, true]) {
      const doc = structuredClone(original);
      doc.paths['/api/fixture'].get.responses['200'].content['application/json'].schema = {
        discriminator: {propertyName: 'kind', mapping: {fixture: 'Fixture'}},
      };
      if (field === 'components') doc.components = value;
      else doc.components.schemas = value;
      const candidate = path.join(directory, 'native-component-container.json');
      await writeFile(candidate, JSON.stringify(doc));
      const result = spawnSync('python3', [path.join(root, 'scripts/contracts/generate_native.py'),
        '--source', candidate, '--validate-only'], {encoding: 'utf8'});
      assert.notEqual(result.status, 0);
      assert.match(result.stderr, field === 'components'
        ? /ValueError: Contract components must be an object/
        : /ValueError: Contract components.schemas must be an object/);
      assert.doesNotMatch(result.stderr, /TypeError|AttributeError/);
    }
  }
  for (const extra of [{}, {components: {}}, {components: {schemas: {}}}]) {
    const candidate = path.join(directory, 'native-optional-components.json');
    await writeFile(candidate, JSON.stringify({openapi: '3.1.1', paths: {}, ...extra}));
    const result = spawnSync('python3', [path.join(root, 'scripts/contracts/generate_native.py'),
      '--source', candidate, '--validate-only'], {encoding: 'utf8'});
    assert.equal(result.status, 0, result.stderr);
  }
});
