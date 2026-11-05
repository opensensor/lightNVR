import type { Recording } from '../types/core.ts';

type MediaStatus = Pick<Recording, 'playback_issue'>;

export function recordingPlaybackUnavailable(recording: MediaStatus): boolean {
  return ['invalid_media', 'no_video', 'decode_error'].includes(recording.playback_issue || '');
}

export function recordingPlaybackReasonKey(recording: MediaStatus): string {
  const issue = recording.playback_issue;
  return ['invalid_media', 'no_video', 'decode_error', 'probe_limit', 'decoder_unavailable', 'probe_error'].includes(issue || '')
    ? `recordings.playbackIssue.${issue}` : 'recordings.playbackIssue.probe_error';
}
