import { getWebRTCAudioDirection } from '../js/components/preact/liveStreamPolicy.ts';

const direction = (audioDisabled, backchannelEnabled, listeningRequested) =>
  getWebRTCAudioDirection({ audioDisabled, backchannelEnabled, listeningRequested });

describe('getWebRTCAudioDirection', () => {
  test('a muted tile requests no audio', () => {
    expect(direction(false, false, false)).toBe('none');
  });

  test('a tile requests audio once the viewer asks to listen', () => {
    expect(direction(false, false, true)).toBe('recvonly');
  });

  test('two-way talk keeps its sender from the start', () => {
    expect(direction(false, true, false)).toBe('sendrecv');
  });

  test('the instance audio policy overrides every other setting', () => {
    expect(direction(true, false, true)).toBe('none');
    expect(direction(true, true, true)).toBe('none');
  });
});
