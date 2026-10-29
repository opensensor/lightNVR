/** Classify server failures without exposing RTSP credentials, URLs or commands.
 * Only transient signaling failures should consume the HTTP retry budget. */
export function classifyLiveVideoError(value) {
  const message = String(value || '');
  if (/dial tcp|no route to host|connection refused|host is unreachable|network is unreachable|i\/o timeout/i.test(message)) {
    return { key: 'live.cannotConnectToSource', retryable: false };
  }
  if (/ffmpeg guard:|cannot allocate memory|out of memory|signal: killed/i.test(message)) {
    return { key: 'live.videoConversionResources', retryable: false };
  }
  if (/PPS changed between slices|could not find ref with POC|error constructing the frame RPS|error (?:while )?decoding|invalid data found when processing input/i.test(message)) {
    return { key: 'live.cameraVideoDecodeFailed', retryable: false };
  }
  if (/codecs? (?:not matched|not supported)|unsupported codec|no supported codecs/i.test(message)) {
    return { key: 'live.videoCodecUnavailable', retryable: false };
  }
  if (/exec\/pipe|ffmpeg|libx264|failed to (?:open|initiali[sz]e).*encoder|no (?:va|cuda).*device|hardware.*(?:failed|error)/i.test(message)) {
    return { key: 'live.videoConversionFailed', retryable: false };
  }
  return { key: 'live.streamError', retryable: true };
}

/** Metadata and audio playback alone are not a successfully decoded video. */
export function hasDecodedVideo(video) {
  return !!video && video.videoWidth > 0 && video.readyState >= 2 && !video.paused;
}

export function h264ReceiverCodecs(capabilities) {
  return (capabilities?.codecs || []).filter(codec => codec.mimeType?.toLowerCase() === 'video/h264');
}

export function shouldRetryWithH264({ alreadyRetried, codec, useSubStream = false }) {
  // A codec hint describes the main stream only. Unknown/substream codecs can
  // also negotiate HEVC, so allow one compatibility attempt for those cases.
  return !alreadyRetried && (useSubStream || String(codec || '').toLowerCase() !== 'h264');
}
