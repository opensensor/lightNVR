import {validateRecordingTimeline, validateRecordingTimelineByIds, validateRecordingExportRequest} from '../generated/api/validators.js';
import {timelineSegmentIdentity, reconcileTimelineSegments} from '../js/components/preact/timeline/timelineUtils.ts';

const segment = id => ({
  id, stream: 'camera', camera_uuid: '10000000-0000-4000-8000-000000000001',
  start_time: '2026-08-28 00:00:00', end_time: '2026-08-28 00:00:05',
  start_timestamp: 1787875200, end_timestamp: 1787875205,
  local_start_timestamp: 1787875200, local_end_timestamp: 1787875205,
  in_progress: false, duration: 5, size: '1 KB', capture_method: 'scheduled',
  schedule_restricted: null, has_detection: false,
});
const response = id => ({segments: [segment(id)], stream: '(selected)', start_time: '', end_time: '', segment_count: 1, multi_stream: true});

test.each([42, Number.MAX_SAFE_INTEGER, '9007199254740992', '9007199254740993', '9223372036854775807'])(
  'timeline contracts and ZIP requests accept exact identity %s', id => {
    const wire = JSON.parse(JSON.stringify(response(id)));
    expect(validateRecordingTimeline(wire)).toBe(true);
    expect(validateRecordingTimelineByIds(wire)).toBe(true);
    expect(wire.segments[0].id).toBe(id);
    expect(validateRecordingExportRequest({ids: [id]})).toBe(true);
  }
);

test.each([Number.MAX_SAFE_INTEGER + 1, 0, -1, 1.5, '0', '-1', '1e16', '1junk', '123456789012345678901'])(
  'contracts reject an unsafe or malformed recording identity %s', id => {
    expect(validateRecordingTimeline(response(id))).toBe(false);
    expect(validateRecordingTimelineByIds(response(id))).toBe(false);
    expect(validateRecordingExportRequest({ids: [id]})).toBe(false);
  }
);

test('adjacent large IDs remain distinct while reconciling polled timeline segments', () => {
  const ids = ['9007199254740992', '9007199254740993'];
  const existing = ids.map(segment);
  const incoming = JSON.parse(JSON.stringify(existing));
  expect(existing.map(timelineSegmentIdentity)).toEqual(ids.map(id => `id:${id}`));
  const reconciled = reconcileTimelineSegments(existing, incoming);
  expect(reconciled.authoritative.map(row => row.id)).toEqual(ids);
  expect(reconciled.added).toEqual([]);
  expect(reconciled.removed).toEqual([]);
  expect(reconciled.updated).toEqual([]);
  const removed = reconcileTimelineSegments(existing, [incoming[1]]);
  expect(removed.removed.map(row => row.id)).toEqual([ids[0]]);
});
