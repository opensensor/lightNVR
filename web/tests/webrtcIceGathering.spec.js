import {
  DEFAULT_ICE_GATHERING_TIMEOUT_MS,
  ICE_GATHER_OUTCOME,
  waitForIceGathering,
} from '../js/utils/webrtc-ice-gathering.js';

function fakePeerConnection(initialState = 'gathering') {
  const listeners = {};
  return {
    iceGatheringState: initialState,
    listenerCount(type) {
      return (listeners[type] || []).length;
    },
    addEventListener(type, fn) {
      listeners[type] = listeners[type] || [];
      listeners[type].push(fn);
    },
    removeEventListener(type, fn) {
      if (!listeners[type]) return;
      listeners[type] = listeners[type].filter((l) => l !== fn);
    },
    setGatheringState(state) {
      this.iceGatheringState = state;
      (listeners.icegatheringstatechange || []).slice().forEach((fn) => fn());
    },
  };
}

describe('waitForIceGathering', () => {
  test('exposes a 1s default bound', () => {
    expect(DEFAULT_ICE_GATHERING_TIMEOUT_MS).toBe(1000);
  });

  test('resolves immediately when gathering is already complete', async () => {
    const pc = fakePeerConnection('complete');
    await expect(waitForIceGathering(pc, 50)).resolves.toBe(ICE_GATHER_OUTCOME.COMPLETE);
    expect(pc.listenerCount('icegatheringstatechange')).toBe(0);
  });

  test('resolves immediately for a missing peer connection', async () => {
    await expect(waitForIceGathering(null, 50)).resolves.toBe(ICE_GATHER_OUTCOME.COMPLETE);
  });

  test('resolves complete when gathering finishes before the timeout', async () => {
    const pc = fakePeerConnection('gathering');
    const promise = waitForIceGathering(pc, 1000);
    expect(pc.listenerCount('icegatheringstatechange')).toBe(1);
    pc.setGatheringState('complete');
    await expect(promise).resolves.toBe(ICE_GATHER_OUTCOME.COMPLETE);
    expect(pc.listenerCount('icegatheringstatechange')).toBe(0);
  });

  test('resolves timeout when gathering stalls past the bound', async () => {
    const pc = fakePeerConnection('gathering');
    await expect(waitForIceGathering(pc, 25)).resolves.toBe(ICE_GATHER_OUTCOME.TIMEOUT);
    expect(pc.listenerCount('icegatheringstatechange')).toBe(0);
  });

  test('resolves timeout immediately for a non-positive bound', async () => {
    const pc = fakePeerConnection('gathering');
    await expect(waitForIceGathering(pc, 0)).resolves.toBe(ICE_GATHER_OUTCOME.TIMEOUT);
    expect(pc.listenerCount('icegatheringstatechange')).toBe(0);
  });

  test('ignores intermediate states that are not complete', async () => {
    const pc = fakePeerConnection('new');
    const promise = waitForIceGathering(pc, 250);
    let settled = false;
    promise.then(() => {
      settled = true;
    });
    pc.setGatheringState('gathering');
    await new Promise((r) => setTimeout(r, 25));
    expect(settled).toBe(false);
    pc.setGatheringState('complete');
    await expect(promise).resolves.toBe(ICE_GATHER_OUTCOME.COMPLETE);
  });

  test('resolves aborted when the abort signal fires', async () => {
    const pc = fakePeerConnection('gathering');
    const controller = new AbortController();
    const promise = waitForIceGathering(pc, 1000, controller.signal);
    controller.abort();
    await expect(promise).resolves.toBe(ICE_GATHER_OUTCOME.ABORTED);
    expect(pc.listenerCount('icegatheringstatechange')).toBe(0);
  });

  test('resolves aborted immediately when the signal is already aborted', async () => {
    const pc = fakePeerConnection('gathering');
    const controller = new AbortController();
    controller.abort();
    await expect(waitForIceGathering(pc, 1000, controller.signal))
      .resolves.toBe(ICE_GATHER_OUTCOME.ABORTED);
    expect(pc.listenerCount('icegatheringstatechange')).toBe(0);
  });

  test('resolves only once when events race the timeout', async () => {
    const pc = fakePeerConnection('gathering');
    const outcomes = [];
    const promise = waitForIceGathering(pc, 25).then((outcome) => {
      outcomes.push(outcome);
      return outcome;
    });
    pc.setGatheringState('complete');
    await expect(promise).resolves.toBe(ICE_GATHER_OUTCOME.COMPLETE);
    // Fire more transitions after settling; must not throw or re-resolve.
    pc.setGatheringState('complete');
    await new Promise((r) => setTimeout(r, 50));
    expect(outcomes).toEqual([ICE_GATHER_OUTCOME.COMPLETE]);
  });
});
