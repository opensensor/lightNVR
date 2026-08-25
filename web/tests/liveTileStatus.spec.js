import { healthStateForStream, recordingModesForStream } from '../js/components/preact/liveTileStatus.js';

describe('live tile status helpers', () => {
  test('preserves every recording mode in display order', () => {
    expect(recordingModesForStream({
      record: true,
      detection_based_recording: true,
      record_on_schedule: true,
    })).toEqual(['constant', 'detection', 'schedule']);
  });

  test('maps playback and stream state to the three health tones', () => {
    expect(healthStateForStream({ status: 'Running' }, { isPlaying: true })).toBe('healthy');
    expect(healthStateForStream({ status: 'Reconnecting' }, { isLoading: true })).toBe('degraded');
    expect(healthStateForStream({ status: 'Running' }, { error: 'no frames' })).toBe('offline');
  });
});
