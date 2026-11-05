import { createPushToTalkKeyboard } from '../js/utils/push-to-talk-keyboard.ts';
import { createPushToTalk } from '../js/utils/push-to-talk.ts';

function key(key, repeat = false) { return { key, repeat, preventDefault: jest.fn() }; }
function setup() {
  const start = jest.fn();
  const stop = jest.fn();
  return { start, stop, keyboard: createPushToTalkKeyboard(start, stop) };
}

test.each([' ', 'Enter'])('%p starts once while held and stops on release', (value) => {
  const { start, stop, keyboard } = setup();
  const down = key(value);
  keyboard.keyDown(down);
  keyboard.keyDown(key(value, true));
  keyboard.keyDown(key(value));
  expect(down.preventDefault).toHaveBeenCalled();
  expect(start).toHaveBeenCalledTimes(1);
  const up = key(value);
  keyboard.keyUp(up);
  expect(up.preventDefault).toHaveBeenCalled();
  expect(stop).toHaveBeenCalledTimes(1);
  keyboard.keyDown(key(value));
  expect(start).toHaveBeenCalledTimes(2);
});

test('ignores unrelated keys and release of a different activation key', () => {
  const { start, stop, keyboard } = setup();
  const unrelated = key('a');
  keyboard.keyDown(unrelated);
  keyboard.keyUp(unrelated);
  expect(unrelated.preventDefault).not.toHaveBeenCalled();
  expect(start).not.toHaveBeenCalled();
  keyboard.keyDown(key(' '));
  keyboard.keyDown(key('Enter'));
  keyboard.keyUp(key('Enter'));
  expect(start).toHaveBeenCalledTimes(1);
  expect(stop).not.toHaveBeenCalled();
  keyboard.keyUp(key(' '));
  expect(stop).toHaveBeenCalledTimes(1);
});

test('focus loss cancels and requires a fresh press, not an autorepeat', () => {
  const { start, stop, keyboard } = setup();
  keyboard.keyDown(key('Enter'));
  keyboard.cancel();
  expect(stop).toHaveBeenCalledTimes(1);
  keyboard.keyDown(key('Enter', true));
  expect(start).toHaveBeenCalledTimes(1);
  keyboard.keyUp(key('Enter'));
  expect(stop).toHaveBeenCalledTimes(1);
  keyboard.keyDown(key('Enter'));
  expect(start).toHaveBeenCalledTimes(2);
});

test.each(['keyup', 'focus loss'])('%s cancels a pending microphone permission request', async (reason) => {
  let resolve;
  const pending = new Promise((done) => { resolve = done; });
  const track = { stop: jest.fn() };
  const stream = { getTracks: () => [track], getAudioTracks: () => [track] };
  const sender = { replaceTrack: jest.fn().mockResolvedValue(undefined) };
  const session = createPushToTalk({ onStarted: jest.fn(), onStopped: jest.fn(), onDetachError: jest.fn() });
  let started;
  const acquire = jest.fn(() => pending);
  const keyboard = createPushToTalkKeyboard(() => { started = session.start(sender, acquire); }, session.stop);
  keyboard.keyDown(key(' '));
  keyboard.keyDown(key(' ', true));
  expect(acquire).toHaveBeenCalledTimes(1);
  if (reason === 'keyup') keyboard.keyUp(key(' '));
  else keyboard.cancel();
  resolve(stream);
  await expect(started).resolves.toBe(false);
  expect(track.stop).toHaveBeenCalledTimes(1);
  expect(sender.replaceTrack).not.toHaveBeenCalled();
});
