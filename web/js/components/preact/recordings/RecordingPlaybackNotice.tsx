import { useI18n } from '../../../i18n.ts';
import type { Recording } from '../../../types/core.ts';
import { recordingPlaybackReasonKey, recordingPlaybackUnavailable } from '../../../utils/recording-media-status.ts';

export function RecordingPlaybackNotice({ recording }: { recording: Pick<Recording, 'playback_issue' | 'protected'> }) {
  const { t } = useI18n();
  if (!recording.playback_issue) return null;
  return <div role="note" className="rounded border border-[hsl(var(--warning))] bg-[hsl(var(--warning-muted))] p-2 text-xs whitespace-normal max-w-sm">
    <strong>{t(recordingPlaybackUnavailable(recording) ? 'recordings.playbackUnavailable' : 'recordings.playbackUnverified')}</strong>
    <p className="mt-1">{t(recordingPlaybackReasonKey(recording))}</p>
    <p className="mt-1">{t('recordings.playbackRecovery')}</p>
    <p className="mt-1">{t(recording.protected ? 'recordings.playbackProtected' : 'recordings.playbackRetention')}</p>
  </div>;
}
