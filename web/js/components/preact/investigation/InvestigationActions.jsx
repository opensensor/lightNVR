import { useEffect, useMemo, useRef, useState } from 'preact/hooks';

import { fetchJSON } from '../../../query-client.js';
import { formatUtils } from '../recordings/formatUtils.js';
import {
  formatCursorTime,
  formatDateTimeLocal,
  investigationActionSelection,
  parseDateTimeLocal,
  summarizeInvestigationActionPreview,
} from './investigationUtils.js';

const DOWNLOAD_POLL_INTERVAL_MS = 800;
const DOWNLOAD_TIMEOUT_MS = 5 * 60 * 1000;

function defaultArchiveName(startTime) {
  const date = new Date(startTime * 1000);
  const stamp = Number.isFinite(date.getTime())
    ? date.toISOString().replace(/[:]/g, '-').replace(/\.\d{3}Z$/, 'Z')
    : 'selection';
  return `investigation-${stamp}.zip`;
}

function triggerArchiveDownload(token, filename) {
  const link = document.createElement('a');
  link.href = `/api/recordings/batch-download/result/${encodeURIComponent(token)}`;
  link.download = filename;
  document.body.appendChild(link);
  link.click();
  document.body.removeChild(link);
}

export function InvestigationActions({ timeline, selectedResult, t }) {
  const [open, setOpen] = useState(false);
  const [scope, setScope] = useState('investigation');
  const [startInput, setStartInput] = useState('');
  const [endInput, setEndInput] = useState('');
  const [preview, setPreview] = useState(null);
  const [loadingPreview, setLoadingPreview] = useState(false);
  const [busyAction, setBusyAction] = useState('');
  const [error, setError] = useState('');
  const [outcome, setOutcome] = useState('');
  const [progress, setProgress] = useState({ current: 0, total: 0 });
  const previewController = useRef(null);

  useEffect(() => () => previewController.current?.abort(), []);

  const summary = useMemo(
    () => summarizeInvestigationActionPreview(preview),
    [preview],
  );

  const selectionFor = (nextScope) =>
    investigationActionSelection(timeline, selectedResult, nextScope);

  const requestPreview = async (selection, preserveOutcome = false) => {
    if (!selection) return;
    previewController.current?.abort();
    const controller = new AbortController();
    previewController.current = controller;
    setLoadingPreview(true);
    setPreview(null);
    setError('');
    if (!preserveOutcome) setOutcome('');
    try {
      const data = await fetchJSON('/api/investigations/recordings/preview', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          camera_uuids: selection.cameraUuids,
          start_time: selection.startTime,
          end_time: selection.endTime,
        }),
        signal: controller.signal,
        timeout: 30000,
        retries: 0,
      });
      setPreview(data);
    } catch (requestError) {
      if (!controller.signal.aborted) setError(requestError.message);
    } finally {
      if (!controller.signal.aborted) setLoadingPreview(false);
    }
  };

  const applyScope = (nextScope, load = true) => {
    const selection = selectionFor(nextScope);
    if (!selection) return;
    setScope(nextScope);
    setStartInput(formatDateTimeLocal(selection.startTime, true));
    setEndInput(formatDateTimeLocal(selection.endTime, true));
    setProgress({ current: 0, total: 0 });
    if (load) void requestPreview(selection);
  };

  const show = () => {
    setOpen(true);
    applyScope(selectedResult?.camera_uuid ? 'result' : 'investigation');
  };

  const close = () => {
    if (busyAction) return;
    previewController.current?.abort();
    setOpen(false);
    setError('');
  };

  const refreshPreview = (event) => {
    event?.preventDefault();
    const base = selectionFor(scope);
    const startTime = parseDateTimeLocal(startInput);
    const endTime = parseDateTimeLocal(endInput);
    if (!base || !Number.isFinite(startTime) || !Number.isFinite(endTime) ||
        endTime <= startTime || startTime < timeline.start_time ||
        endTime > timeline.end_time) {
      setPreview(null);
      setError(t('investigation.actions.invalidRange'));
      return;
    }
    void requestPreview({ ...base, startTime, endTime });
  };

  const protectRecordings = async () => {
    if (!summary.canProtect || busyAction) return;
    setBusyAction('protect');
    setError('');
    setOutcome('');
    try {
      const result = await fetchJSON('/api/recordings/batch-protect', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          ids: summary.unprotectedIds,
          protected: true,
        }),
        timeout: 30000,
        retries: 0,
      });
      setOutcome(t('investigation.actions.protectResult', {
        success: result.success_count || 0,
        failed: result.fail_count || 0,
      }));
      const base = selectionFor(scope);
      const startTime = parseDateTimeLocal(startInput);
      const endTime = parseDateTimeLocal(endInput);
      if (base && Number.isFinite(startTime) && Number.isFinite(endTime)) {
        await requestPreview({ ...base, startTime, endTime }, true);
      }
    } catch (requestError) {
      setError(requestError.message);
    } finally {
      setBusyAction('');
    }
  };

  const downloadRecordings = async () => {
    if (!summary.canExport || busyAction || !preview) return;
    setBusyAction('download');
    setError('');
    setOutcome('');
    setProgress({ current: 0, total: summary.recordingCount });
    const filename = defaultArchiveName(preview.start_time);
    try {
      const job = await fetchJSON('/api/recordings/batch-download', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          ids: preview.recordings.map((recording) => recording.id),
          filename,
        }),
        timeout: 30000,
        retries: 0,
      });
      const deadline = Date.now() + DOWNLOAD_TIMEOUT_MS;
      while (Date.now() < deadline) {
        await new Promise((resolve) =>
          window.setTimeout(resolve, DOWNLOAD_POLL_INTERVAL_MS));
        const status = await fetchJSON(
          `/api/recordings/batch-download/status/${encodeURIComponent(job.token)}`,
          { timeout: 15000, retries: 0 },
        );
        setProgress({
          current: status.current || 0,
          total: status.total || job.total || summary.recordingCount,
        });
        if (status.status === 'error') {
          throw new Error(status.error || t('investigation.actions.downloadFailed'));
        }
        if (status.status === 'complete') {
          triggerArchiveDownload(job.token, filename);
          setOutcome(t('investigation.actions.downloadStarted'));
          return;
        }
      }
      throw new Error(t('investigation.actions.downloadTimedOut'));
    } catch (requestError) {
      setError(requestError.message);
    } finally {
      setBusyAction('');
    }
  };

  return (
    <div className="investigation-action-trigger">
      <button
        type="button"
        className="btn-primary"
        disabled={!timeline}
        onClick={show}
      >
        {t('investigation.actions.open')}
      </button>

      {open && (
        <div className="investigation-action-backdrop" role="presentation">
          <section
            className="investigation-action-dialog"
            role="dialog"
            aria-modal="true"
            aria-labelledby="investigation-action-dialog-title"
          >
            <header>
              <div>
                <h2 id="investigation-action-dialog-title">
                  {t('investigation.actions.title')}
                </h2>
                <p>{t('investigation.actions.notEvidenceExport')}</p>
              </div>
              <button
                type="button"
                className="btn-secondary"
                aria-label={t('common.close')}
                disabled={Boolean(busyAction)}
                onClick={close}
              >
                ×
              </button>
            </header>

            <form className="investigation-action-scope" onSubmit={refreshPreview}>
              <fieldset>
                <legend>{t('investigation.actions.scope')}</legend>
                <label>
                  <input
                    type="radio"
                    name="investigation-action-scope"
                    value="result"
                    checked={scope === 'result'}
                    disabled={!selectedResult?.camera_uuid || Boolean(busyAction)}
                    onChange={() => applyScope('result')}
                  />
                  <span>{t('investigation.actions.selectedResult')}</span>
                </label>
                <label>
                  <input
                    type="radio"
                    name="investigation-action-scope"
                    value="investigation"
                    checked={scope === 'investigation'}
                    disabled={Boolean(busyAction)}
                    onChange={() => applyScope('investigation')}
                  />
                  <span>{t('investigation.actions.fullWindow')}</span>
                </label>
              </fieldset>
              <div className="investigation-action-time-fields">
                <label>
                  <span>{t('investigation.start')}</span>
                  <input
                    type="datetime-local"
                    step="1"
                    min={formatDateTimeLocal(timeline.start_time, true)}
                    max={formatDateTimeLocal(timeline.end_time, true)}
                    value={startInput}
                    disabled={Boolean(busyAction)}
                    onInput={(event) => {
                      setStartInput(event.target.value);
                      setPreview(null);
                      setOutcome('');
                    }}
                  />
                </label>
                <label>
                  <span>{t('investigation.end')}</span>
                  <input
                    type="datetime-local"
                    step="1"
                    min={formatDateTimeLocal(timeline.start_time, true)}
                    max={formatDateTimeLocal(timeline.end_time, true)}
                    value={endInput}
                    disabled={Boolean(busyAction)}
                    onInput={(event) => {
                      setEndInput(event.target.value);
                      setPreview(null);
                      setOutcome('');
                    }}
                  />
                </label>
                <button
                  type="submit"
                  className="btn-secondary"
                  disabled={loadingPreview || Boolean(busyAction)}
                >
                  {loadingPreview
                    ? t('investigation.actions.previewing')
                    : t('investigation.actions.preview')}
                </button>
              </div>
            </form>

            {error && <div className="investigation-error" role="alert">{error}</div>}
            {outcome && (
              <div className="investigation-action-outcome" role="status">{outcome}</div>
            )}

            {loadingPreview && (
              <p className="investigation-action-empty">
                {t('investigation.actions.previewing')}
              </p>
            )}

            {preview && (
              <>
                <div className="investigation-action-summary">
                  <strong>{t('investigation.actions.recordingCount', {
                    count: preview.recording_count,
                  })}</strong>
                  <span>{formatUtils.formatFileSize(preview.total_bytes)}</span>
                  <span>{t('investigation.actions.protectedCount', {
                    count: preview.protected_count,
                  })}</span>
                </div>

                {summary.protectDeniedCount > 0 && (
                  <p className="investigation-action-warning">
                    {t('investigation.actions.protectDenied', {
                      count: summary.protectDeniedCount,
                    })}
                  </p>
                )}
                {summary.exportDeniedCount > 0 && (
                  <p className="investigation-action-warning">
                    {t('investigation.actions.exportDenied', {
                      count: summary.exportDeniedCount,
                    })}
                  </p>
                )}

                {preview.recordings.length === 0 ? (
                  <p className="investigation-action-empty">
                    {t('investigation.actions.noRecordings')}
                  </p>
                ) : (
                  <div className="investigation-action-recordings">
                    <table>
                      <thead>
                        <tr>
                          <th>{t('investigation.actions.camera')}</th>
                          <th>{t('investigation.actions.recording')}</th>
                          <th>{t('investigation.start')}</th>
                          <th>{t('investigation.end')}</th>
                          <th>{t('investigation.actions.size')}</th>
                          <th>{t('investigation.protection')}</th>
                        </tr>
                      </thead>
                      <tbody>
                        {preview.recordings.map((recording) => (
                          <tr key={recording.id}>
                            <td>{recording.camera_name}</td>
                            <td>#{recording.id}</td>
                            <td><time>{formatCursorTime(recording.start_time)}</time></td>
                            <td><time>{formatCursorTime(recording.end_time)}</time></td>
                            <td>{formatUtils.formatFileSize(recording.size_bytes)}</td>
                            <td>
                              {recording.protected
                                ? t('investigation.protected')
                                : recording.can_protect
                                  ? t('investigation.unprotected')
                                  : t('investigation.actions.notPermitted')}
                            </td>
                          </tr>
                        ))}
                      </tbody>
                    </table>
                  </div>
                )}

                {busyAction === 'download' && (
                  <div className="investigation-action-progress" role="status">
                    <span>{t('investigation.actions.buildingArchive')}</span>
                    <span>{progress.current}/{progress.total}</span>
                    <progress value={progress.current} max={progress.total || 1} />
                  </div>
                )}

                <div className="investigation-action-dialog-actions">
                  <button
                    type="button"
                    className="btn-secondary"
                    disabled={!summary.canProtect || Boolean(busyAction)}
                    title={!summary.canProtect
                      ? t('investigation.actions.protectUnavailable') : undefined}
                    onClick={protectRecordings}
                  >
                    {busyAction === 'protect'
                      ? t('investigation.actions.protecting')
                      : t('investigation.actions.protectButton', {
                        count: summary.unprotectedIds.length,
                      })}
                  </button>
                  <button
                    type="button"
                    className="btn-primary"
                    disabled={!summary.canExport || Boolean(busyAction)}
                    title={!summary.canExport
                      ? t('investigation.actions.downloadUnavailable') : undefined}
                    onClick={downloadRecordings}
                  >
                    {busyAction === 'download'
                      ? t('investigation.actions.buildingArchive')
                      : t('investigation.actions.downloadButton', {
                        count: summary.recordingCount,
                      })}
                  </button>
                </div>
              </>
            )}
          </section>
        </div>
      )}
    </div>
  );
}
