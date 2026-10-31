import { test } from 'node:test';
import assert from 'node:assert/strict';
import fs from 'node:fs';
import YAML from 'yaml';

const additions = ['test_mp4_writer_watchdog', 'test_mp4_segment_timestamps', 'test_mp4_faststart', 'test_stream_state_disposal', 'test_delete_stream_launch_failure', 'test_put_stream_launch_failure', 'test_storage_recording_faults', 'test_storage_writer_backpressure', 'test_storage_deletion_ledger', 'test_mp4_path_collision', 'test_incomplete_media_validation'];
const job = YAML.parse(fs.readFileSync('.github/workflows/integration-test.yml', 'utf8')).jobs['unity-tests'];
const build = job.steps.find(step => step.name === 'Build Unity tests').run;
const run = job.steps.find(step => step.name === 'Run Unity tests').run;
const selection = new RegExp(run.match(/-R "([^"]+)"/)[1]);
for (const target of additions) {
  test('Unity CI builds and executes verified ' + target, () => {
    assert.match(build, new RegExp('\\b' + target + '\\b'));
    assert.ok(selection.test(target), 'missing CTest selection for ' + target);
  });
}
test('Unity CI rejects empty discovery and does not mask execution failure', () => {
  assert.match(run, /--no-tests=error/);
  assert.doesNotMatch(run, /\|\|\s*(true|:)|continue-on-error/);
});
