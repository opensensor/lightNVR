#!/usr/bin/env node
/** Production-entry request capture against an owned empty real server. */
import assert from 'node:assert/strict';
import {readFile, writeFile} from 'node:fs/promises';
import {createHash} from 'node:crypto';
import path from 'node:path';
import {chromium} from 'playwright';
import {startFixture} from './fixture-server.mjs';

const args=process.argv.slice(2);
function option(name) {
  const index=args.indexOf(name);
  assert(index>=0 && args[index+1] && !args[index+1].startsWith('--'), `Required: ${name}`);
  return args[index+1];
}
const binary=path.resolve(option('--binary'));
const webRoot=path.resolve(option('--web-root'));
const profile=option('--profile'), output=option('--output');
assert(['default','modern'].includes(profile));
const hash=bytes=>createHash('sha256').update(bytes).digest('hex');
const report={status:'running',profile,binarySha256:hash(await readFile(binary)),
  harnessSha256:hash(await readFile(new URL(import.meta.url))),
  methodology:'Fresh Chromium context per production entry, owned real authenticated server with zero cameras, HTTP cache disabled by routing, one-second settled observation. Only paths/counts/hashes are recorded; this checks asset isolation, not every interaction or authorization.',entries:[]};
let server,browser,failed=true;
try {
  server=await startFixture({binary,webRoot,seed:false});
  await server.json('/api/setup/status',{method:'POST',body:{complete:true}});
  browser=await chromium.launch({headless:true});
  for(const entry of ['index.html','streams.html','recordings.html','settings.html','api-docs.html']) {
    const context=await server.browserContext(browser);
    await context.route('**/*',route=>route.continue());
    const page=await context.newPage(), requests=new Set(), errors=[];
    let externalRequests=0;
    page.on('request',request=>{
      const url=new URL(request.url());
      if(url.origin!==server.baseURL) externalRequests++;
      else requests.add(url.pathname);
    });
    page.on('pageerror',()=>errors.push('pageerror'));
    try {
      const response=await page.goto(`${server.baseURL}/${entry}`,{waitUntil:'networkidle',timeout:30000});
      assert.equal(response.status(),200);
      if(entry==='api-docs.html') await page.locator('.opblock').first().waitFor({timeout:15000});
      else await page.locator('body').waitFor();
      await page.waitForTimeout(1000);
      const vendor=[...requests].filter(value=>value.startsWith('/api-docs-vendor/')).sort();
      const row={entry,requests:[...requests].sort(),vendor,externalRequests,pageErrors:errors.length,
        htmlSha256:hash(await readFile(path.join(webRoot,entry)))};
      report.entries.push(row);
      assert.equal(externalRequests,0,`${entry}: unexpected external request`);
      assert.equal(errors.length,0,`${entry}: browser page error`);
      assert(entry==='api-docs.html'?vendor.length>=2:vendor.length===0,`${entry}: explorer vendor isolation failed`);
    } finally {await context.close();}
  }
  failed=false;
} finally {
  try {if(browser)await browser.close();if(server)await server.stop({failed});}
  catch(error){failed=true;throw error;}
  finally {
    report.status=failed?'failed':'passed';
    await writeFile(output,JSON.stringify(report,null,2)+'\n',{flag:'wx',mode:0o600});
  }
}
console.log(`${profile}: production asset isolation passed; ${output}`);
