export function recordingModesForStream(stream = {}) {
  const modes = [];
  if (stream.record) modes.push('constant');
  if (stream.detection_based_recording) modes.push('detection');
  if (stream.record_on_schedule) modes.push('schedule');
  return modes;
}

export function shouldShowLiveTileStatus(showLabels = true) {
  return showLabels !== false;
}

export function healthStateForStream(stream = {}, playback = {}) {
  const status = String(stream.status || stream.state || '').toLowerCase();
  if (playback.error || ['error', 'failed', 'stopped', 'offline'].includes(status)) {
    return 'offline';
  }
  if (playback.isPlaying && ['running', 'playing', 'connected', 'ready', ''].includes(status)) {
    return 'healthy';
  }
  if (playback.isLoading || ['starting', 'reconnecting', 'recovering', 'stopping', 'buffering'].includes(status)) {
    return 'degraded';
  }
  return playback.isPlaying ? 'healthy' : 'degraded';
}
