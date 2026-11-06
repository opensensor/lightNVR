/** Independent wire and malformed-fixture checks for the author-maintained pilot. */
import test, {before, after} from 'node:test';
import assert from 'node:assert/strict';
import {readFile, writeFile, mkdtemp, rm} from 'node:fs/promises';
import {tmpdir} from 'node:os';
import path from 'node:path';
import {pathToFileURL, fileURLToPath} from 'node:url';
import {generateArtifacts, writeArtifacts} from '../../../../scripts/contracts/generate.mjs';
const root = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '../../../..');
let dir, validators, baseline;
before(async () => {
  dir = await mkdtemp(path.join(tmpdir(), 'lightnvr-T4-schema-'));
  await writeFile(path.join(dir, 'package.json'), '{"type":"module"}\n');
  const files = await generateArtifacts(path.join(root, 'docs/api/openapi.json'), 'web');
  await writeArtifacts(files, dir);
  validators = await import(pathToFileURL(path.join(dir, 'web/generated/api/validators.js')));
  baseline = JSON.parse(await readFile(path.join(root, 'tests/contracts/fixtures/baseline-payloads.json'), 'utf8')).cases;
});
after(async () => {if (dir) await rm(dir, {recursive:true, force:true});});
const valid = (name, value) => {
  const validate = validators[`validate${name}`];
  assert.equal(validate(value), true, `${name}: ${JSON.stringify(validate.errors)}`);
};
const invalid = (name, value) => assert.equal(validators[`validate${name}`](value), false, `${name} accepted malformed fixture`);

test('all twelve independently captured real-server payloads satisfy specific response schemas', () => {
  assert.equal(Object.keys(baseline).length, 12);
  for (const [name, fixture] of Object.entries(baseline)) {
    const schema = fixture.status >= 400 ? 'ApiError'
      : name.includes('summary') ? 'StreamSummaryPage'
      : name.includes('full') ? 'StreamFullResponse'
      : name === 'admin-update' ? 'StreamPrivacyUpdated'
      : 'StreamCompatibilityList';
    const original = structuredClone(fixture.body);
    valid(schema, fixture.body);
    assert.deepEqual(fixture.body, original, `${name} was mutated`);
  }
});

test('surface-specific summaries preserve empty/zero/additive fields and reject malformed administrative rows', () => {
  const empty = {streams:[],total:0,page:1,page_size:50,total_pages:0};
  for (const name of ['StreamSummaryPage','LiveStreamSummaryPage','LprStreamSummaryPage']) valid(name, empty);
  valid('StreamCompatibilityList', []);
  const admin = structuredClone(baseline['admin-summary'].body);
  admin.streams[0].future_additive_field = {kept:true};
  valid('StreamSummaryPage', admin);
  assert.deepEqual(admin.streams[0].future_additive_field, {kept:true});
  // These live/LPR variants are source-traced synthetic fixtures, not claimed
  // to have been captured in T2; T16 proves them through the actual server.
  const live = structuredClone(baseline['viewer-summary'].body);
  for (const row of live.streams) {
    delete row.admin_url;
    Object.assign(row, {streaming_enabled:false,playback_transport:'auto',eptz_config:'',detection_model:'',privacy_mode:false,can_control_privacy:false,has_sub_stream:false,ptz_enabled:false,backchannel_enabled:false,go2rtc_hls_available:false});
  }
  valid('LiveStreamSummaryPage', live);
  const lpr = {...empty,streams:[{camera_uuid:'synthetic',name:'IdentityOnly'}],total:1,total_pages:1};
  valid('LprStreamSummaryPage', lpr);
  invalid('StreamSummaryPage', lpr);
  invalid('LiveStreamSummaryPage', lpr);
  delete admin.streams[0].enabled;
  invalid('StreamSummaryPage', admin);
  const wrong = structuredClone(baseline['viewer-summary'].body);
  wrong.streams[0].admin_url = 'must-not-be-exposed';
  invalid('StreamSummaryPage', wrong);
});

test('full wrappers require fields, preserve zero, and enforce redacted versus configurable values', () => {
  const full = structuredClone(baseline['admin-full'].body);
  Object.assign(full.stream, {retention_days:0,detection_retention_days:0,detection_threshold:0,tier_critical_multiplier:0});
  valid('StreamFullResponse', full);
  valid('StreamFullResponse', baseline['viewer-full'].body);
  const leak = structuredClone(baseline['viewer-full'].body);
  leak.stream.onvif_password = 'synthetic-forbidden';
  invalid('StreamFullResponse', leak);
  const bad = structuredClone(full); delete bad.stream.name;
  invalid('StreamFullResponse', bad);
  invalid('StreamFullResponse', {stream:{name:'partial'}});
  invalid('StreamFullResponse', {});
  bad.stream.name = 'A'; bad.stream.enabled = 'false';
  invalid('StreamFullResponse', bad);
  const schedule = structuredClone(full); schedule.stream.recording_schedule = [true];
  invalid('StreamFullResponse', schedule);
});

test('update DTO separates privacy and ordinary values, nullable clear operations and accepted versus applied responses', () => {
  for (const payload of [{},{name:'A',retention_days:0,detection_threshold:0},
    {tags:null,admin_url:null,motion_trigger_source:null,go2rtc_source_override:null,sub_stream_url:null,detection_url:null,publish_url:null},
    {isOnvif:null,record_audio:null,protocol:null},
    {set_privacy_mode:true},{name:'A',set_privacy_mode:false},
    {enable_disabled:true,enabled:true},{future_additive_field:42},
    {retention_days:365.5},{retention_days:-1.5}]) valid('StreamUpdateRequest',payload);
  for (const payload of [{set_privacy_mode:true,enabled:false},{set_privacy_mode:false,future_additive_field:1},
    {enable_disabled:true,enabled:false},{enabled:'false'},{retention_days:366},
    {retention_days:-2},{playback_transport:'invalid'},{eptz_config:{}},null,[]]) invalid('StreamUpdateRequest',payload);
  valid('StreamUpdateAccepted',{success:true,message:'Accepted'});
  valid('StreamUpdateAccepted',{success:true,message:'Accepted',onvif_status:'error',onvif_message:'Synthetic probe failed'});
  invalid('StreamUpdateAccepted',{success:true});
  valid('StreamPrivacyUpdated',{success:true,privacy_mode:true,stream_registered:false});
  invalid('StreamPrivacyUpdated',{success:true,privacy_mode:true});
});

test('error variants and discovery envelope remain honest and non-mutating', async () => {
  valid('ApiError',{error:'Denied'});
  valid('ApiError',{valid:false,error:{message:'Synthetic YAML error'}});
  invalid('ApiError',{error:4});
  const canonical = JSON.parse(await readFile(path.join(root,'docs/api/openapi.json'),'utf8'));
  valid('OpenApiDocument',canonical);
  const empty = {...canonical,paths:{},components:{}};
  delete empty['x-lightnvr-validators'];
  valid('OpenApiDocument',empty);
  delete empty['x-lightnvr-coverage'];
  invalid('OpenApiDocument',empty);
});

test('recording action requests preserve IDs precedence and positive fractional numeric input', () => {
  invalid('RecordingDeletionRequest', {ids:[],filter:{stream:'A'}});
  valid('RecordingDeletionRequest', {ids:'ignored',filter:{stream:'A'}});
  valid('RecordingDeletionRequest', {filter:{stream:'A'}});
  valid('RecordingDeletionRequest', {ids:[101]});
  for (const name of ['RecordingBatchTagsRequest','RecordingExportRequest']) {
    invalid(name, {ids:[0]});
    invalid(name, {ids:[-1]});
    valid(name, {ids:[101.9]});
  }
});
