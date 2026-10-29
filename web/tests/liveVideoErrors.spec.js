import fs from 'node:fs';
import path from 'node:path';
import { classifyLiveVideoError, hasDecodedVideo, h264ReceiverCodecs, shouldRetryWithH264 } from '../js/utils/live-video-errors.js';

const english = JSON.parse(fs.readFileSync(path.resolve(__dirname, '../public/locales/en.json'), 'utf8'));

test.each([
  ['streams: dial tcp 192.0.2.1:554: connect: no route to host', 'live.cannotConnectToSource'],
  ['ffmpeg guard: pid 123 RSS 1600 MiB exceeds 1536 MiB; killing transcoder', 'live.videoConversionResources'],
  ['exec/pipe: signal: killed', 'live.videoConversionResources'],
  ['PPS changed between slices', 'live.cameraVideoDecodeFailed'],
  ['Could not find ref with POC 12', 'live.cameraVideoDecodeFailed'],
  ['Error constructing the frame RPS', 'live.cameraVideoDecodeFailed'],
  ['streams: codecs not matched: H265 => H264', 'live.videoCodecUnavailable'],
  ['streams: exec/pipe: EOF', 'live.videoConversionFailed'],
])('classifies %s without repeatedly retrying incompatible media', (body, key) => {
  expect(classifyLiveVideoError(body)).toEqual({ key, retryable: false });
  expect(english[key]).toBeTruthy();
});

test('keeps transient registration and signaling errors retryable', () => {
  expect(classifyLiveVideoError('streams: stream not found')).toEqual({
    key: 'live.streamError', retryable: true,
  });
  expect(classifyLiveVideoError(null).retryable).toBe(true);
});

test('does not expose credentials, query tokens or commands in displayed errors', () => {
  const failure = classifyLiveVideoError('exec/pipe: ffmpeg -i rtsp://admin:secret@camera/main?token=private failed');
  expect(JSON.stringify(failure) + english[failure.key]).not.toMatch(/secret|private|admin|rtsp:/);
});

test('metadata and audio-only playback cannot satisfy the video startup deadline', () => {
  expect(hasDecodedVideo(null)).toBe(false);
  expect(hasDecodedVideo({ videoWidth: 1920, readyState: 1, paused: false })).toBe(false);
  expect(hasDecodedVideo({ videoWidth: 0, readyState: 4, paused: false })).toBe(false);
  expect(hasDecodedVideo({ videoWidth: 1920, readyState: 4, paused: true })).toBe(false);
  expect(hasDecodedVideo({ videoWidth: 1920, readyState: 2, paused: false })).toBe(true);
});

test('compatibility retry advertises H.264 only while preserving its codec parameters', () => {
  const h264 = { mimeType: 'video/H264', clockRate: 90000, sdpFmtpLine: 'profile-level-id=42e01f;packetization-mode=1' };
  expect(h264ReceiverCodecs({ codecs: [
    { mimeType: 'video/H265' }, h264, { mimeType: 'video/VP8' }, { mimeType: 'video/rtx' },
  ] })).toEqual([h264]);
  expect(h264ReceiverCodecs(undefined)).toEqual([]);
});

test('tries compatibility once for HEVC/unknown, respecting independent substream codecs', () => {
  expect(shouldRetryWithH264({ codec: 'hevc', alreadyRetried: false })).toBe(true);
  expect(shouldRetryWithH264({ codec: '', alreadyRetried: false })).toBe(true);
  expect(shouldRetryWithH264({ codec: 'hevc', alreadyRetried: true })).toBe(false);
  expect(shouldRetryWithH264({ codec: 'H264', alreadyRetried: false })).toBe(false);
  expect(shouldRetryWithH264({ codec: 'h264', useSubStream: true, alreadyRetried: false })).toBe(true);
});
