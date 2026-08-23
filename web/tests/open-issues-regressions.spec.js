import { shouldFallbackFullscreenToSubStream } from '../js/components/preact/liveStreamPolicy.js';
import { formatUtils } from '../js/components/preact/recordings/formatUtils.js';
import { formatUptime } from '../js/components/preact/system/SystemUtils.js';
import { shouldSeekPlaybackPosition } from '../js/components/preact/timeline/timelineUtils.js';

describe('open issue regressions', () => {
  test('never formats uptime as a negative duration (#475)', () => {
    expect(formatUptime(-34)).toBe('0s');
    expect(formatUptime('not-a-number')).toBe('0s');
    expect(formatUptime(90061)).toBe('1d 1h 1m 1s');
  });

  test('distinguishes always-on capture from scheduled capture (#472)', () => {
    expect(formatUtils.formatCaptureMethod('continuous')).toBe('Continuous');
    expect(formatUtils.formatCaptureMethod('scheduled')).toBe('Scheduled');
  });

  test('falls back only for a failing fullscreen main-stream upgrade (#468)', () => {
    expect(shouldFallbackFullscreenToSubStream({
      fullscreenUpgraded: true,
      effectiveUseSubStream: false,
      noVideoCheckCount: 1,
    })).toBe(true);
    expect(shouldFallbackFullscreenToSubStream({
      fullscreenUpgraded: true,
      effectiveUseSubStream: false,
      connectionQuality: 'poor',
    })).toBe(true);
    expect(shouldFallbackFullscreenToSubStream({
      fullscreenUpgraded: false,
      effectiveUseSubStream: false,
      connectionQuality: 'bad',
    })).toBe(false);
    expect(shouldFallbackFullscreenToSubStream({
      fullscreenUpgraded: true,
      effectiveUseSubStream: true,
      noVideoCheckCount: 1,
    })).toBe(false);
  });

  test('does not turn 4x playback time updates into repeated video seeks (#495)', () => {
    const segmentStart = 100;

    // A normal 4x timeupdate may advance the media clock by more than the old
    // one-second seek threshold, but the video is already at that position.
    expect(shouldSeekPlaybackPosition(105.2, 104, segmentStart, 5.2)).toBe(false);

    // A timeline scrub that moves away from the media clock still needs a seek.
    expect(shouldSeekPlaybackPosition(120, 105.2, segmentStart, 5.2)).toBe(true);
  });
});
