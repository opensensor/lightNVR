/**
 * BatchDownloadModal – lets the user choose sequential or ZIP batch download.
 */
import { useState, useEffect, useCallback } from 'preact/hooks';
import { showStatusMessage } from './ToastContainer.jsx';

/**
 * Sequential download: trigger one download per recording with a small delay.
 * @param {Array} recordings  Full recording objects (must have .id, .stream, .start_time)
 */
async function downloadSequential(recordings) {
  showStatusMessage(`Starting ${recordings.length} download${recordings.length !== 1 ? 's' : ''}…`);
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
    if (!resp.ok) throw new Error(`Server error ${resp.status}`);
    const { token, total } = await resp.json();

    // Poll status
    let done = false;
    while (!done) {
      await new Promise(r => setTimeout(r, 800));
      const st = await fetch(`/api/recordings/batch-download/status/${token}`);
      if (!st.ok) throw new Error('Status poll failed');
      const { status, current } = await st.json();
      onProgress(current, total);
      if (status === 'complete') { done = true; break; }
      if (status === 'error')   throw new Error('ZIP creation failed on server');
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
    onError(err.message || 'Unknown error');
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

  const selected = recordings.filter(r => !!selectedIds[r.id]);

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
        () => { setBusy(false); onClose(); showStatusMessage('ZIP download started'); },
        (msg) => { setBusy(false); setError(msg); }
      );
    }
  }, [mode, zipName, selected, onClose]);

  if (!isOpen) return null;

  const pct = progress.total > 0 ? Math.round((progress.current / progress.total) * 100) : 0;

  return (
    <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50">
      <div class="bg-card text-card-foreground rounded-lg shadow-xl w-full max-w-md p-6">
        <h2 class="text-lg font-semibold mb-4">Download Selected Recordings</h2>
        <p class="text-sm text-muted-foreground mb-4">
          {selected.length} recording{selected.length !== 1 ? 's' : ''} selected
        </p>

        {/* Mode selector */}
        <div class="flex flex-col gap-3 mb-5">
          <label class="flex items-start gap-3 cursor-pointer">
            <input type="radio" name="dl-mode" value="zip" checked={mode === 'zip'}
              onChange={() => setMode('zip')} class="mt-0.5" />
            <div>
              <div class="font-medium text-sm">ZIP archive (recommended)</div>
              <div class="text-xs text-muted-foreground">All files packaged into a single .zip</div>
            </div>
          </label>
          <label class="flex items-start gap-3 cursor-pointer">
            <input type="radio" name="dl-mode" value="sequential" checked={mode === 'sequential'}
              onChange={() => setMode('sequential')} class="mt-0.5" />
            <div>
              <div class="font-medium text-sm">Sequential downloads</div>
              <div class="text-xs text-muted-foreground">Each file downloaded separately (browser may block multiple tabs)</div>
            </div>
          </label>
        </div>

        {/* ZIP filename input */}
        {mode === 'zip' && (
          <div class="mb-5">
            <label class="block text-sm font-medium mb-1">ZIP filename</label>
            <input
              type="text"
              value={zipName}
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
              <span>{mode === 'zip' ? 'Building ZIP…' : 'Downloading…'}</span>
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
            Cancel
          </button>
          <button
            class="btn-primary px-4 py-2 text-sm disabled:opacity-50 disabled:cursor-not-allowed"
            onClick={handleDownload}
            disabled={busy || selected.length === 0}
          >
            {busy ? 'Working…' : 'Download'}
          </button>
        </div>
      </div>
    </div>
  );
}

