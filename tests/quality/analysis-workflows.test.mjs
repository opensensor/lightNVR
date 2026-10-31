// SPDX-License-Identifier: GPL-3.0-or-later
import {test, after} from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import {spawnSync} from 'node:child_process';
import YAML from 'yaml';
const root=process.cwd();
const files=['static-analysis.yml','codeql-analysis.yml','container-scan.yml'];
const workflows=Object.fromEntries(files.map(name=>[name,YAML.parse(fs.readFileSync(path.join(root,'.github/workflows',name),'utf8'))]));
const scratch=fs.mkdtempSync(path.join(os.tmpdir(),'analysis-workflows-'));
after(()=>fs.rmSync(scratch,{recursive:true,force:true}));
const python = run => {
  const match=run.match(/python3 - <<'PY'\n([\s\S]*?)\nPY(?:\n|$)/);
  assert.ok(match,'workflow has an executable Python block');
  return match[1];
};

test('PR workflow invokes reusable analysis and enforces a full aggregate after all lanes',()=>{
  const main=workflows['static-analysis.yml'];
  assert.ok(main.on.pull_request);
  for(const name of ['codeql-analysis.yml','container-scan.yml']){
    assert.ok(Object.hasOwn(workflows[name].on,'workflow_call'));
    assert.ok(Object.values(main.jobs).some(job=>job.uses===`./.github/workflows/${name}`));
  }
  const aggregate=main.jobs['analysis-coverage'];
  assert.equal(aggregate.if,'always()');
  assert.ok(aggregate.needs.includes('codeql') && aggregate.needs.includes('container'));
  assert.ok(aggregate.steps.some(step=>step.run?.includes('findings.py --check') && step.run.includes('--expected-revision')));
});

test('every workflow shell and inline Python program is syntactically valid',()=>{
  let checked=0;
  for(const workflow of Object.values(workflows))for(const job of Object.values(workflow.jobs))for(const step of job.steps??[]){
    if(!step.run)continue;
    const run=step.run.replace(/\$\{\{[\s\S]*?\}\}/g,'fixture');
    const shell=spawnSync('bash',['-n'],{input:run,encoding:'utf8'});
    assert.equal(shell.status,0,`${step.name}: ${shell.stderr}`);
    for(const match of run.matchAll(/python3 - <<'PY'\n([\s\S]*?)\nPY(?:\n|$)/g)){
      const parsed=spawnSync('python3',['-c','import ast,sys; ast.parse(sys.stdin.read())'],{input:match[1],encoding:'utf8'});
      assert.equal(parsed.status,0,`${step.name}: ${parsed.stderr}`);
    }
    checked++;
  }
  assert.ok(checked>10);
});

test('same-run aggregation preserves hashed paths and refuses wrong-revision evidence',()=>{
  const step=workflows['static-analysis.yml'].jobs['analysis-coverage'].steps.find(x=>x.name==='Merge same-run scanner manifests');
  const program=python(step.run);
  const incoming=path.join(scratch,'incoming');
  const artifact=path.join(incoming,'analysis-fixture');fs.mkdirSync(artifact,{recursive:true});
  fs.writeFileSync(path.join(artifact,'report.json'),'[]');
  const rev='a'.repeat(40);
  const manifest={schema_version:1,revision:rev,scanners:{example:{reports:['report.json'],report_sha256:{'report.json':'hash'},scope:{evidence_sha256:{'report.json':'hash'}}}}};
  fs.writeFileSync(path.join(artifact,'run-manifest.json'),JSON.stringify(manifest));
  const env={...process.env,GITHUB_SHA:rev,INCOMING_DIR:incoming,REPORT_DIR:path.join(scratch,'combined')};
  let result=spawnSync('python3',['-c',program],{cwd:scratch,env,encoding:'utf8'});
  assert.equal(result.status,0,result.stderr);
  const combined=JSON.parse(fs.readFileSync(path.join(env.REPORT_DIR,'run-manifest.json')));
  assert.deepEqual(combined.scanners.example.reports,['analysis-fixture/report.json']);
  assert.equal(combined.scanners.example.scope.evidence_sha256['analysis-fixture/report.json'],'hash');
  manifest.revision='b'.repeat(40);fs.writeFileSync(path.join(artifact,'run-manifest.json'),JSON.stringify(manifest));
  env.REPORT_DIR=path.join(scratch,'wrong-revision');
  result=spawnSync('python3',['-c',program],{cwd:scratch,env,encoding:'utf8'});
  assert.notEqual(result.status,0);
});

test('CodeQL specification invokes database analysis and consumes its actual archive',()=>{
  const step=workflows['codeql-analysis.yml'].jobs.analyze.steps.find(x=>x.name==='Generate CodeQL scanner specification');
  const program=python(step.run);
  const db=path.join(scratch,'build/codeql/cpp');fs.mkdirSync(db,{recursive:true});
  const env={...process.env,CODEQL_BIN:'/tool/codeql',CODEQL_DATABASES:JSON.stringify({cpp:db}),SCANNER_ID:'codeql-cpp',QUERY_SUITE:'codeql/cpp-queries:codeql-suites/cpp-security-extended.qls',LANGUAGE:'c-cpp',REPORT_DIR:'build/reports',SPEC_FILE:'build/scan-spec.json'};
  const result=spawnSync('python3',['-c',program],{cwd:scratch,env,encoding:'utf8'});
  assert.equal(result.status,0,result.stderr);
  const spec=JSON.parse(fs.readFileSync(path.join(scratch,env.SPEC_FILE)));
  assert.deepEqual(spec.commands[0].argv.slice(0,3),['/tool/codeql','database','analyze']);
  assert.equal(spec.scope.kind,'codeql-source-archive');
  assert.equal(spec.scope.archive,'build/codeql/cpp/src.zip');
  assert.ok(spec.commands[0].argv.includes('--sarif-add-baseline-file-info'));
});
