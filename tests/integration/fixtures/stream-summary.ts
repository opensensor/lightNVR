import { expect } from '@playwright/test';
import type { components } from '../../../web/generated/api/schema';
import { validateStreamSummaryPage } from '../../../web/generated/api/validators.js';

type Summary = components['schemas']['StreamSummaryPrivileged'];

/** Complete wire fixtures, checked against the same contract as the consumer. */
export function streamSummary(name: string, overrides: Partial<Summary> = {}): Summary {
  return {
    name, camera_uuid: name, location_uuid: '', location_name: '', location_path: '',
    enabled: true, record: false, record_on_schedule: false, detection_based_recording: false,
    isOnvif: false, recording_active: false, codec: 'h264', tags: '',
    width: 640, height: 480, fps: 15, protocol: 0, status: 'Running', availability: 'live',
    first_video_at: 0, last_frame_ts: 0, last_recording_at: 0, current_fps: 15,
    can_configure: true, ...overrides,
  };
}

export function streamSummaryPage(streams: Summary[]): components['schemas']['StreamSummaryPage'] {
  const result = { streams, total: streams.length, page: 1, page_size: 50,
    total_pages: streams.length ? 1 : 0 };
  expect(validateStreamSummaryPage(result), 'Mock stream summary must satisfy the wire contract').toBe(true);
  return result;
}
