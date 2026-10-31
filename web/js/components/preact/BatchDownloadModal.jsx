/**
 * BatchDownloadModal – lets the user choose sequential or ZIP batch download.
 */
import { useState, useEffect, useCallback } from 'preact/hooks';
import { showStatusMessage } from './ToastContainer.jsx';
import { t as translate, useI18n } from '../../i18n.js';

/**
 * Sequential download: trigger one download per recording with a small delay.
 * @param {Array} recordings  Full recording objects (must have .id, .stream, .start_time)
 */
async function downloadSequential(recordings) {
  showStatusMessage(translate(recordings.length === 1 ? 'recordings.startingDownloadOne' : 'recordings.startingDownloadMany', { count: recordings.length }));
  for (let i = 0; i < recordings.length; i++) {
    const r = recordings[i];
    const url = `/api/recordings/download/${r.id}`;
    const a = document.createElement('a');
    a.href = url;
    a.download = `${r.stream || 'recording'}_${r.id}.mp4`;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    if (i < recordings.length - 1) {
      await new Promise(res => setTimeout(res, 600));
    }
  }
}

/**
 * ZIP download: POST to server, poll for completion, then redirect to result URL.
 * @param {number[]} ids
 * @param {string}  filename
 * @param {Function} onProgress  (current, total) => void
 * @param {Function} onComplete  () => void
 * @param {Function} onError    (msg) => void
 */
async function downloadZip(ids, filename, onProgress, onComplete, onError) {
  try {
    const resp = await fetch('/api/recordings/batch-download', {
      method: 'POST',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ ids, filename }),
    });
    if (!resp.ok) throw new Error(translate('recordings.serverError', { status: resp.status }));
    const { token, total } = await resp.json();

    // Poll status
    while (true) {
      await new Promise(r => setTimeout(r, 800));
      const st = await fetch(`/api/recordings/batch-download/status/${token}`);
      if (!st.ok) throw new Error(translate('recordings.statusPollFailed'));
      const { status, current } = await st.json();
      onProgress(current, total);
      if (status === 'complete') break;
      if (status === 'error')   throw new Error(translate('recordings.zipCreationFailed'));
    }

    // Trigger download
    const a = document.createElement('a');
    a.href = `/api/recordings/batch-download/result/${token}`;
    a.download = filename;
    document.body.appendChild(a);
    a.click();
    document.body.removeChild(a);
    onComplete();
  } catch (err) {
    onError(err.message || translate('recordings.unknownError'));
  }
}

/**
 * BatchDownloadModal component.
 *
 * Props:
 *   isOpen       {boolean}
 *   onClose      {() => void}
 *   recordings   {Array}   – all recording objects that are currently selected
 *   selectedIds  {Object}  – map of id → true for selected recordings
 */
export function BatchDownloadModal({ isOpen, onClose, recordings = [], selectedIds = {} }) {
  const { t } = useI18n();
  const [mode, setMode] = useState('zip');          // 'zip' | 'sequential'
  const [zipName, setZipName] = useState('recordings.zip');
  const [busy, setBusy] = useState(false);
  const [progress, setProgress] = useState({ current: 0, total: 0 });
  const [error, setError] = useState('');

  // Reset state when modal opens
  useEffect(() => {
    if (isOpen) {
      setBusy(false);
      setProgress({ current: 0, total: 0 });
      setError('');
    }
  }, [isOpen]);

  const selected = Object.entries(selectedIds)
    .filter(([_, isSelected]) => !!isSelected)
    .map(([id]) => recordings.find(r => String(r.id) === id) || {
      id: parseInt(id, 10),
      stream: 'recording'
    });

  const handleDownload = useCallback(async () => {
    if (selected.length === 0) return;
    setBusy(true);
    setError('');
    setProgress({ current: 0, total: selected.length });

    if (mode === 'sequential') {
      await downloadSequential(selected);
      setBusy(false);
      onClose();
    } else {
      const ids = selected.map(r => r.id);
      const name = zipName.trim() || 'recordings.zip';
      await downloadZip(
        ids,
        name.endsWith('.zip') ? name : name + '.zip',
        (cur, tot) => setProgress({ current: cur, total: tot }),
        () => { setBusy(false); onClose(); showStatusMessage(t('recordings.zipDownloadStarted')); },
        (msg) => { setBusy(false); setError(msg); }
      );
    }
  }, [mode, zipName, selected, onClose]);

  if (!isOpen) return null;

  const pct = progress.total > 0 ? Math.round((progress.current / progress.total) * 100) : 0;

  return (
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <div class="bg-card text-card-foreground rounded-lg shadow-xl w-full max-w-md p-6">
        <h2 class="text-lg font-semibold mb-4">{t('recordings.downloadSelectedRecordings')}</h2>
        <p class="text-sm text-muted-foreground mb-4">
          {t(selected.length === 1 ? 'recordings.recordingSelectedOne' : 'recordings.recordingSelectedMany', { count: selected.length })}
        </p>

        {/* Mode selector */}
        <div class="flex flex-col gap-3 mb-5">
          <label class="flex items-start gap-3 cursor-pointer">
            <input type="radio" name="dl-mode" aria-label={t('recordings.zipArchiveRecommended')} value="zip" checked={mode === 'zip'}
              onChange={() => setMode('zip')} class="mt-0.5" />
            <div>
              <div class="font-medium text-sm">{t('recordings.zipArchiveRecommended')}</div>
              <div class="text-xs text-muted-foreground">{t('recordings.zipArchiveHelp')}</div>
            </div>
          </label>
          <label class="flex items-start gap-3 cursor-pointer">
            <input type="radio" name="dl-mode" aria-label={t('recordings.sequentialDownloads')} value="sequential" checked={mode === 'sequential'}
              onChange={() => setMode('sequential')} class="mt-0.5" />
            <div>
              <div class="font-medium text-sm">{t('recordings.sequentialDownloads')}</div>
              <div class="text-xs text-muted-foreground">{t('recordings.sequentialDownloadsHelp')}</div>
            </div>
          </label>
        </div>

        {/* ZIP filename input */}
        {mode === 'zip' && (
          <div class="mb-5">
            <label class="block text-sm font-medium mb-1">{t('recordings.zipFilename')}</label>
            <input
              type="text"
              aria-label={t('recordings.zipFilename')} value={zipName}
              onInput={e => setZipName(e.target.value)}
              placeholder="recordings.zip"
              class="w-full px-3 py-2 rounded border border-border bg-background text-sm focus:outline-none focus:ring-2"
              style={{ focusRingColor: 'hsl(var(--primary))' }}
              disabled={busy}
            />
          </div>
        )}

        {/* Progress bar */}
        {busy && (
          <div class="mb-4">
            <div class="flex justify-between text-xs text-muted-foreground mb-1">
              <span>{t(mode === 'zip' ? 'recordings.buildingZip' : 'recordings.downloading')}</span>
              <span>{progress.current} / {progress.total}</span>
            </div>
            <div class="h-2 bg-muted rounded-full overflow-hidden">
              <div class="h-2 rounded-full transition-all"
                style={{ width: `${pct}%`, backgroundColor: 'hsl(var(--primary))' }} />
            </div>
          </div>
        )}

        {/* Error */}
        {error && (
          <p class="text-sm text-destructive mb-3">{error}</p>
        )}

        {/* Actions */}
        <div class="flex justify-end gap-3">
          <button
            class="px-4 py-2 rounded text-sm hover:bg-muted/70 transition-colors text-muted-foreground"
            onClick={onClose}
            disabled={busy}
          >
            {t('common.cancel')}
          </button>
          <button
            class="btn-primary px-4 py-2 text-sm disabled:opacity-50 disabled:cursor-not-allowed"
            onClick={handleDownload}
            disabled={busy || selected.length === 0}
          >
            {t(busy ? 'recordings.working' : 'common.download')}
          </button>
        </div>
      </div>
    </div>
  );
}

