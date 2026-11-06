import test from 'node:test';
import assert from 'node:assert/strict';
import {readFile, writeFile, mkdtemp, rm} from 'node:fs/promises';
import path from 'node:path';
import os from 'node:os';
import {pathToFileURL} from 'node:url';
import Ajv2020 from 'ajv/dist/2020.js';
import addFormats from 'ajv-formats';
import {root, schemaDocument, generateArtifacts} from '../../../scripts/contracts/generate.mjs';
import {configureContractCodegen} from '../../../scripts/contracts/validator-codegen.mjs';

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
    let cases=0,valid=0,invalid=0;
    for(const name of doc['x-lightnvr-validators']) {
      const oracle=baseline.getSchema(`urn:lightnvr:contracts#/$defs/${name}`),candidate=generated[`validate${name}`];
      for(const seed of seeds(doc.components.schemas[name],doc.components.schemas)) {
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
