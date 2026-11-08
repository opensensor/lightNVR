import test from 'node:test';
import assert from 'node:assert/strict';
import {readFile, writeFile, mkdtemp, rm} from 'node:fs/promises';
import path from 'node:path';
import os from 'node:os';
import {pathToFileURL} from 'node:url';
import Ajv2020 from 'ajv/dist/2020.js';
import addFormats from 'ajv-formats';
import {root, schemaDocument, generateArtifacts} from '../../../scripts/contracts/generate.mjs';
import {configureContractCodegen, normalizeRepeatedCounterChecks, normalizeKnownObjectChecks} from '../../../scripts/contracts/validator-codegen.mjs';

// The oracle is unmodified Ajv. Exercise every public guard against independent
// valid seeds and property/type mutations, comparing diagnostics and no-effects.
function seeds(schema, defs, depth=0) {
  if(depth>8)return [null];
  if(schema===false)return [null];
  if(schema===true)return [null,{},'data'];
  if(schema.$ref)return seeds(defs[schema.$ref.split('/').at(-1)],defs,depth+1);
  if('const' in schema)return [schema.const];
  if(schema.enum)return schema.enum;
  const union=schema.anyOf||schema.oneOf;
  if(union)return union.flatMap(s=>seeds(s,defs,depth+1));
  if(schema.allOf)return [Object.assign({},...schema.allOf.map(s=>seeds(s,defs,depth+1)[0]))];
  const type=Array.isArray(schema.type)?schema.type.find(t=>t!=='null'):schema.type;
  if(type==='object'||schema.properties) {
    const properties=schema.properties||{}, required=new Set(schema.required||[]);
    const all=Object.fromEntries(Object.entries(properties).map(([k,s])=>[k,seeds(s,defs,depth+1)[0]]));
    return [Object.fromEntries(Object.entries(all).filter(([k])=>required.has(k))),all];
  }
  if(type==='array') {
    if(schema.prefixItems)return [schema.prefixItems.map(s=>seeds(s,defs,depth+1)[0])];
    const item=seeds(schema.items||true,defs,depth+1)[0];
    return [Array.from({length:schema.minItems||0},()=>structuredClone(item)),[item]];
  }
  if(type==='integer'||type==='number')return [Math.max(schema.minimum??0,(schema.exclusiveMinimum??-1)+1),schema.maximum??100];
  if(type==='boolean')return [true,false];
  if(type==='null')return [null];
  const formats={uuid:'00000000-0000-4000-8000-000000000001','date-time':'2026-11-06T12:00:00Z',date:'2026-11-06',time:'12:00:00Z',ipv4:'127.0.0.1',ipv6:'::1',uri:'https://example.invalid/path',email:'fixture@example.invalid'};
  return [formats[schema.format]||'a'.repeat(Math.max(1,schema.minLength||0)),''];
}
function mutations(value) {
  const wrong=[null,false,0,'wrong',[],{}];
  const result=[value,...wrong,NaN,Infinity];
  if(value && typeof value==='object' && !Array.isArray(value)) {
    result.push({...value,unexpected:'fixture'});
    for(const key of Object.keys(value)) {
      const missing={...value};delete missing[key];result.push(missing);
      for(const substitute of wrong)result.push({...value,[key]:substitute});
      if(Array.isArray(value[key]))for(const item of wrong)result.push({...value,[key]:[item]});
    }
  } else if(Array.isArray(value))result.push([...value,null],[...value,{}]);
  return result;
}

test('optimized canonical validators agree with unmodified Ajv across all exports',async(t)=>{
  const source=path.join(root,'docs/api/openapi.json');
  const doc=JSON.parse(await readFile(source,'utf8'));
  const baseline=new Ajv2020({strict:true,allErrors:false,allowUnionTypes:true,validateFormats:true,coerceTypes:false,useDefaults:false,removeAdditional:false});
  addFormats(baseline);baseline.addSchema(schemaDocument(doc));
  const dir=await mkdtemp(path.join(os.tmpdir(),'lightnvr-validator-equivalence-'));
  try {
    const artifacts=await generateArtifacts(source,'web');
    const file=path.join(dir,'validators.mjs');await writeFile(file,artifacts.get('web/generated/api/validators.js'));
    const generated=await import(pathToFileURL(file));
    assert.doesNotMatch(artifacts.get('web/generated/api/validators.js'), /errors\s*===?\s*errors/);
    let cases=0,valid=0,invalid=0;
    for(const name of doc['x-lightnvr-validators']) {
      const oracle=baseline.getSchema(`urn:lightnvr:contracts#/$defs/${name}`),candidate=generated[`validate${name}`];
      const samples=seeds(doc.components.schemas[name],doc.components.schemas);
      if(name==='ObservationEvidence') {
        const base=Object.fromEntries(doc.components.schemas[name].required.map(key=>[key,null]));
        base.asset_id='00000000-0000-4000-8000-000000000001';
        for(const state of doc.components.schemas.ObservationEvidenceState.enum)
          for(const reason of [null,'disabled','invalid'])
            for(const snapshot of [{},{snapshot_url:`/api/observations/${base.asset_id}/snapshot`},{snapshot_url:null}])
              samples.push({...base,state,reason,...snapshot});
      }
      for(const seed of samples) {
        for(const value of mutations(seed)) {
          const original=structuredClone(value),expected=oracle(value);
          const errors=expected?null:oracle.errors.map(({keyword,schemaPath})=>({keyword,schemaPath}));
          assert.equal(candidate(value),expected,`${name}: validity`);
          assert.deepEqual(candidate.errors,errors,`${name}: diagnostic identity`);
          assert.deepEqual(value,original,`${name}: input mutation`);
          cases++;if(expected)valid++;else invalid++;
        }
      }
    }
    t.diagnostic(JSON.stringify({schemas:doc['x-lightnvr-validators'].length,cases,valid,invalid}));
    assert.ok(cases>1000,`only ${cases} comparisons`);
    assert.ok(valid>50,`only ${valid} valid cases`);assert.ok(invalid>500);
  } finally {await rm(dir,{recursive:true,force:true});}
});

test('custom emitters fail closed for unsupported mutating or exhaustive profiles',()=>{
  for(const options of [{allErrors:true},{$data:true},{useDefaults:true},{removeAdditional:true}])
    assert.throws(()=>configureContractCodegen(new Ajv2020(options)),/reviewed non-mutating/);
});

test('union emitters preserve overlapping branches and evaluated property/item boundaries',()=>{
  const branches=[
    {properties:{a:{type:'integer'}},required:['a']},
    {properties:{b:{type:'string'}},required:['b']},
  ];
  const schemas=[
    ...[true,false,{type:'number'}].map(additionalProperties=>({
      type:'object',anyOf:branches,properties:{a:true,b:true},additionalProperties,
    })),
    {type:'object',anyOf:branches,unevaluatedProperties:false},
    {type:'object',oneOf:branches,unevaluatedProperties:false},
    {type:'object',oneOf:[true,...branches],unevaluatedProperties:false},
    {type:'object',properties:{rows:{type:'array',items:{type:'object',oneOf:branches,unevaluatedProperties:false}}}},
    {type:'array',anyOf:[{prefixItems:[{type:'integer'}]},{prefixItems:[{type:'string'},{type:'boolean'}]}],unevaluatedItems:false},
  ];
  const values=[null,{},[],[1],['x',true],['x',true,2],{a:1},{b:'x'},
    {a:1,b:'x'},{a:'wrong'},{b:2},{a:1,extra:2},{a:1,extra:'wrong'},
    {rows:[{a:1},{b:'x'}]},{rows:[{a:1,b:'x'}]},{rows:[{a:'wrong'}]}];
  for(const schema of schemas){
    const oracle=new Ajv2020({strict:false}).compile(schema);
    const ajv=new Ajv2020({strict:false});configureContractCodegen(ajv);
    const candidate=ajv.compile(schema);
    for(const value of values){
      const original=structuredClone(value);
      assert.equal(candidate(value),oracle(value),JSON.stringify({schema,value}));
      assert.deepEqual(candidate.errors,oracle.errors);
      assert.deepEqual(value,original);
    }
  }
});


test('repeated counter checks collapse without folding property reads or calls',()=>{
  const result=normalizeRepeatedCounterChecks('if (0 === _errs5 && 0 === _errs5) check(); if (a.x === 0 && a.x === 0) check(); if (f() === 0 && f() === 0) check();');
  assert.match(result,/if \(0 === _errs5\)/);
  assert.match(result,/a\.x === 0 && a\.x === 0/);
  assert.match(result,/f\(\) === 0 && f\(\) === 0/);
  const loose=normalizeRepeatedCounterChecks('if (errors == errors && errors == errors) check(); if (errors != errors) check(); if (value == value) check(); if (errors == "0" && errors == "0") check();');
  assert.match(loose,/if \(true\)/);
  assert.match(loose,/if \(false\)/);
  assert.match(loose,/value == value/);
  assert.match(loose,/errors == "0" && errors == "0"/);
});

test('conditional emitters preserve clause diagnostics and evaluated properties/items',()=>{
  const condition={properties:{a:{type:'integer'}},required:['a']};
  const then={properties:{b:{type:'string'}},required:['b']};
  const otherwise={properties:{c:{type:'boolean'}},required:['c']};
  const clauses=[{if:condition,then,else:otherwise},{if:condition,then},{if:condition,else:otherwise}];
  const schemas=clauses.flatMap(clause=>[
    {type:'object',...clause,unevaluatedProperties:false},
    {type:'object',allOf:[clause],unevaluatedProperties:false},
    {type:'object',allOf:[{...clause,unevaluatedProperties:false}],additionalProperties:true},
    {type:'object',allOf:[{properties:{child:{type:'object',...clause,unevaluatedProperties:false}}}],additionalProperties:true},
    ...[true,false,{type:'number'}].map(additionalProperties=>({type:'object',allOf:[clause],properties:{a:true,b:true,c:true},additionalProperties})),
    {type:'object',anyOf:[clause,{required:['d'],properties:{d:{type:'number'}}}],unevaluatedProperties:false},
  ]);
  schemas.push(...[
    {then:{prefixItems:[true,{type:'string'}]}},
    {else:{prefixItems:[true,{type:'boolean'}]}},
    {then:{prefixItems:[true,{type:'string'}]},else:{prefixItems:[true,{type:'boolean'}]}},
  ].map(clause=>({type:'array',if:{prefixItems:[{type:'integer'}]},...clause,unevaluatedItems:false})));
  const values=[null,{},[],[1],[1,'x'],['x',true],['x','wrong'],[1,'x',2],{a:1},{b:'x'},
    {a:1,b:'x'},{a:1,b:2},{a:'wrong',c:true},{c:false},{c:'wrong'},
    {a:1,b:'x',extra:2},{a:1,b:'x',extra:'wrong'},{a:1,b:'x',c:true},{d:2},
    {child:{a:1,b:'x'}},{child:{a:1,b:'x',extra:2}},{child:{c:true}}];
  for(const schema of schemas){
    const oracle=new Ajv2020({strict:false}).compile(schema);
    const ajv=new Ajv2020({strict:false});configureContractCodegen(ajv);
    const candidate=ajv.compile(schema);
    for(const value of values){
      const original=structuredClone(value);
      assert.equal(candidate(value),oracle(value),JSON.stringify({schema,value}));
      assert.deepEqual(candidate.errors,oracle.errors,JSON.stringify({schema,value}));
      assert.deepEqual(value,original);
    }
  }
});

test('object guard normalization requires a dominating return and only counter snapshots',()=>{
  const guard='if (!data || "object" != typeof data || Array.isArray(data)) return false;';
  const repeated='if (errors === _errs1) if (data && "object" == typeof data && !Array.isArray(data)) { accept(); } else { reject(); }';
  const normalized=normalizeKnownObjectChecks(`function check(data) { ${guard} let _errs1 = errors; ${repeated} }`);
  assert.doesNotMatch(normalized,/reject\(\)/);
  assert.match(normalized,/accept\(\)/);
  for(const between of ['data = null;','change();','let _errs1 = change();']){
    const kept=normalizeKnownObjectChecks(`function check(data) { ${guard} ${between} ${repeated} }`);
    assert.match(kept,/reject\(\)/);
  }
  assert.match(normalizeKnownObjectChecks(`function check(data) { ${repeated} }`),/reject\(\)/);
  assert.match(normalizeKnownObjectChecks(`function check(data) { ${guard.replace('return false','report()')} ${repeated} }`),/reject\(\)/);
});
