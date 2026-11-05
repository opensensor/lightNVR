import { createPushToTalk } from '../js/utils/push-to-talk.ts';

function deferred() {
  let resolve;
  let reject;
  const promise = new Promise((yes, no) => { resolve = yes; reject = no; });
  return { promise, resolve, reject };
}
function microphone() {
  const track = { stop: jest.fn() };
  return { track, getTracks: () => [track], getAudioTracks: () => [track] };
}
function setup() {
  const callbacks = { onStarted: jest.fn(), onStopped: jest.fn(), onDetachError: jest.fn() };
  const sender = { replaceTrack: jest.fn().mockResolvedValue(undefined) };
  return { ...callbacks, sender, session: createPushToTalk(callbacks) };
}
const flush = async () => { for (let i = 0; i < 8; i++) await Promise.resolve(); };

test('release while permission is pending stops the late microphone without sending', async () => {
  const { session, sender, onStarted } = setup();
  const permission = deferred();
  const stream = microphone();
  const start = session.start(sender, () => permission.promise);
  session.stop();
  permission.resolve(stream);
  await expect(start).resolves.toBe(false);
  expect(stream.track.stop).toHaveBeenCalledTimes(1);
  expect(sender.replaceTrack).not.toHaveBeenCalled();
  expect(onStarted).not.toHaveBeenCalled();
});

test('release during replaceTrack detaches after attachment finishes', async () => {
  const { session, sender, onStarted } = setup();
  const attached = deferred();
  sender.replaceTrack.mockImplementationOnce(() => attached.promise);
  const stream = microphone();
  const start = session.start(sender, async () => stream);
  await flush();
  session.stop();
  expect(stream.track.stop).toHaveBeenCalledTimes(1);
  attached.resolve();
  await expect(start).resolves.toBe(false);
  await flush();
  expect(sender.replaceTrack.mock.calls).toEqual([[stream.track], [null]]);
  expect(onStarted).not.toHaveBeenCalled();
});

test('a new press waits for old attachment and detachment without being cleared by them', async () => {
  const { session, sender, onStarted } = setup();
  const attached = deferred();
  sender.replaceTrack.mockImplementationOnce(() => attached.promise);
  const oldStream = microphone();
  const newStream = microphone();
  const first = session.start(sender, async () => oldStream);
  await flush();
  session.stop();
  const second = session.start(sender, async () => newStream);
  await flush();
  attached.resolve();
  await expect(first).resolves.toBe(false);
  await expect(second).resolves.toBe(true);
  expect(sender.replaceTrack.mock.calls).toEqual([[oldStream.track], [null], [newStream.track]]);
  expect(newStream.track.stop).not.toHaveBeenCalled();
  expect(onStarted).toHaveBeenCalledTimes(1);
  expect(onStarted).toHaveBeenCalledWith(newStream);
});

test('a superseded permission request cannot stop the current stream', async () => {
  const { session, sender, onStarted } = setup();
  const permission = deferred();
  const oldStream = microphone();
  const newStream = microphone();
  const first = session.start(sender, () => permission.promise);
  await session.start(sender, async () => newStream);
  permission.resolve(oldStream);
  await expect(first).resolves.toBe(false);
  expect(oldStream.track.stop).toHaveBeenCalledTimes(1);
  expect(newStream.track.stop).not.toHaveBeenCalled();
  expect(sender.replaceTrack.mock.calls).toEqual([[newStream.track]]);
  expect(onStarted).toHaveBeenCalledTimes(1);
});

test('stale permission errors do not affect the current request', async () => {
  const { session, sender } = setup();
  const permission = deferred();
  const first = session.start(sender, () => permission.promise);
  const stream = microphone();
  await session.start(sender, async () => stream);
  permission.reject(new Error('denied'));
  await expect(first).resolves.toBe(false);
  expect(stream.track.stop).not.toHaveBeenCalled();
});

test('attachment failure releases the microphone and reports the current error', async () => {
  const { session, sender, onStarted } = setup();
  sender.replaceTrack.mockRejectedValueOnce(new Error('closed sender'));
  const stream = microphone();
  await expect(session.start(sender, async () => stream)).rejects.toThrow('closed sender');
  expect(stream.track.stop).toHaveBeenCalledTimes(1);
  expect(onStarted).not.toHaveBeenCalled();
  await flush();
  expect(sender.replaceTrack).toHaveBeenLastCalledWith(null);
});

test('cleanup stops an active microphone and its monitoring', async () => {
  const { session, sender, onStopped } = setup();
  const stream = microphone();
  await session.start(sender, async () => stream);
  onStopped.mockClear();
  session.stop();
  expect(stream.track.stop).toHaveBeenCalledTimes(1);
  expect(onStopped).toHaveBeenCalledTimes(1);
  await flush();
  expect(sender.replaceTrack).toHaveBeenLastCalledWith(null);
});

test('a reconnect keeps cleanup on the old sender', async () => {
  const { session, sender } = setup();
  const oldAttachment = deferred();
  sender.replaceTrack.mockImplementationOnce(() => oldAttachment.promise);
  const first = session.start(sender, async () => microphone());
  await flush();
  session.stop();
  const newSender = { replaceTrack: jest.fn().mockResolvedValue(undefined) };
  const stream = microphone();
  await session.start(newSender, async () => stream);
  oldAttachment.resolve();
  await first;
  await flush();
  expect(sender.replaceTrack).toHaveBeenLastCalledWith(null);
  expect(newSender.replaceTrack.mock.calls).toEqual([[stream.track]]);
});

test('a stream without an audio track is released', async () => {
  const { session, sender } = setup();
  const stream = microphone();
  stream.getAudioTracks = () => [];
  await expect(session.start(sender, async () => stream)).rejects.toThrow('no audio track');
  expect(stream.track.stop).toHaveBeenCalledTimes(1);
});
