import { useEffect, useMemo, useRef, useState } from 'preact/hooks';

import { useQuery } from '../../../query-client.js';
import { useI18n } from '../../../i18n.js';
import { handleAuthenticationFailure } from '../../../fetch-utils.js';
import { fetchAllStreamSummaries } from '../../../utils/stream-summaries.js';
import { ConfirmDialog } from '../common/ModalDialog.jsx';
import { showStatusMessage } from '../ToastContainer.jsx';
import {
  LPR_SEARCH_LIMIT_MAX,
  buildLprSearchBody,
  createLprFilters,
  deleteLprRead,
  denyLprAction,
  exportLprReads,
  formatLprConfidence,
  lprActionsFor,
  lprTimelineHref,
  searchLprReads,
  validateLprFilters,
} from './lprSearch.js';

const INPUT_CLASS = 'w-full rounded-md border border-input bg-background px-3 py-2 text-sm text-foreground '
  + 'focus:outline-none focus:ring-2 focus:ring-[hsl(var(--primary))]';
const LABEL_CLASS = 'mb-1 block text-sm font-medium';
const SUBNAV_LINK_CLASS = 'rounded-t-lg px-4 py-2 text-sm font-medium transition-colors';

function formatObservedAt(milliseconds) {
  if (!Number.isFinite(milliseconds)) return '—';
  return new Intl.DateTimeFormat(undefined, {
    dateStyle: 'medium',
    timeStyle: 'medium',
  }).format(new Date(milliseconds));
}

function handleUnauthorized() {
  if (!window._authDisabled && !window._demoMode) {
    handleAuthenticationFailure('Received 401 Unauthorized response');
  }
}

function outcomeMessage(t, result) {
  switch (result.outcome) {
    case 'invalid': return t('lpr.error.invalid');
    case 'unavailable': return t('lpr.error.unavailable');
    case 'forbidden': return t('lpr.error.searchForbidden');
    default: return t('lpr.error.generic');
  }
}

function FieldErrors({ id, errors, t }) {
  if (!errors.length) return null;
  return (
    <p id={id} className="mt-1 text-xs text-red-600 dark:text-red-400">
      {errors.map((error) => t(error.key, error.params)).join(' ')}
    </p>
  );
}

export function PlatesView() {
  const { t } = useI18n();
  const [filters, setFilters] = useState(() => createLprFilters());
  const [submittedErrors, setSubmittedErrors] = useState([]);
  const [reads, setReads] = useState(null);
  const [searchedCameraUuid, setSearchedCameraUuid] = useState('');
  const [searching, setSearching] = useState(false);
  const [exporting, setExporting] = useState(false);
  const [requestError, setRequestError] = useState('');
  const [deniedActions, setDeniedActions] = useState({});
  const [deleteCandidate, setDeleteCandidate] = useState(null);
  const searchController = useRef(null);

  const { data: streamData, isLoading: streamsLoading, error: streamsError } = useQuery({
    queryKey: ['lpr-streams', 'summary', 'lpr'],
    queryFn: ({ signal }) => fetchAllStreamSummaries({ surface: 'lpr', signal }),
    staleTime: 30000,
  });
  const cameras = useMemo(() => (Array.isArray(streamData) ? streamData : [])
    .filter((stream) => stream?.camera_uuid), [streamData]);

  // Default to the first visible camera once the list arrives.
  useEffect(() => {
    if (!filters.cameraUuid && cameras.length) {
      setFilters((current) => ({ ...current, cameraUuid: cameras[0].camera_uuid }));
    }
  }, [cameras, filters.cameraUuid]);

  // Drop plate text from memory when leaving the page.
  useEffect(() => () => {
    searchController.current?.abort();
    setReads(null);
  }, []);

  const errorsFor = (field) => submittedErrors.filter((error) => error.field === field);
  const actions = lprActionsFor(deniedActions, searchedCameraUuid);
  const cameraName = (uuid) => cameras.find((camera) => camera.camera_uuid === uuid)?.name;

  const updateFilter = (name) => (event) => {
    const { value } = event.currentTarget;
    setFilters((current) => ({ ...current, [name]: value }));
  };

  const validate = () => {
    const errors = validateLprFilters(filters);
    setSubmittedErrors(errors);
    return errors.length === 0;
  };

  const handleSearch = async (event) => {
    event.preventDefault();
    setRequestError('');
    if (!validate()) return;
    searchController.current?.abort();
    const controller = new AbortController();
    searchController.current = controller;
    setSearching(true);
    try {
      const result = await searchLprReads(buildLprSearchBody(filters), { signal: controller.signal });
      if (controller.signal.aborted) return;
      if (result.outcome === 'ok') {
        setReads(result.reads);
        setSearchedCameraUuid(filters.cameraUuid);
        return;
      }
      if (result.outcome === 'unauthorized') {
        handleUnauthorized();
        return;
      }
      if (result.outcome === 'forbidden') {
        setDeniedActions((current) => denyLprAction(current, filters.cameraUuid, 'search'));
      }
      setReads(null);
      setRequestError(outcomeMessage(t, result));
    } catch (error) {
      if (error?.name !== 'AbortError') setRequestError(t('lpr.error.generic'));
    } finally {
      if (searchController.current === controller) {
        searchController.current = null;
        setSearching(false);
      }
    }
  };

  const handleExport = async () => {
    setRequestError('');
    if (!validate()) return;
    const cameraUuid = filters.cameraUuid;
    setExporting(true);
    try {
      const result = await exportLprReads(buildLprSearchBody(filters, { exporting: true }));
      if (result.outcome === 'ok') {
        const url = URL.createObjectURL(result.blob);
        const anchor = document.createElement('a');
        anchor.href = url;
        anchor.download = 'lightnvr-lpr-export.json';
        document.body.appendChild(anchor);
        anchor.click();
        anchor.remove();
        setTimeout(() => URL.revokeObjectURL(url), 0);
        return;
      }
      if (result.outcome === 'unauthorized') {
        handleUnauthorized();
      } else if (result.outcome === 'forbidden') {
        // Not permitted for this camera: hide the action rather than raise an error.
        setDeniedActions((current) => denyLprAction(current, cameraUuid, 'export'));
      } else {
        setRequestError(outcomeMessage(t, result));
      }
    } catch {
      setRequestError(t('lpr.error.generic'));
    } finally {
      setExporting(false);
    }
  };

  const handleDelete = async (read) => {
    try {
      const result = await deleteLprRead(read.id);
      if (result.outcome === 'ok' || result.outcome === 'not_found') {
        setReads((current) => (current || []).filter((item) => item.id !== read.id));
        showStatusMessage(t('lpr.deleted'), 'success');
      } else if (result.outcome === 'unauthorized') {
        handleUnauthorized();
      } else if (result.outcome === 'forbidden') {
        setDeniedActions((current) => denyLprAction(current, read.camera_uuid, 'delete'));
      } else {
        setRequestError(t('lpr.error.deleteFailed'));
      }
    } catch {
      setRequestError(t('lpr.error.deleteFailed'));
    }
  };

  const describedBy = (field) => (errorsFor(field).length ? `lpr-${field}-error` : undefined);
  const exportVisible = Boolean(filters.cameraUuid)
    && lprActionsFor(deniedActions, filters.cameraUuid).canExport;

  return (
    <section className="lpr-page py-4" aria-labelledby="lpr-heading">
      <div className="mb-4 flex flex-wrap items-end justify-between gap-3 border-b border-border">
        <h1 id="lpr-heading" className="pb-2 text-2xl font-bold">{t('lpr.title')}</h1>
        <nav className="flex flex-wrap" aria-label={t('recordings.views')}>
          <a href="recordings.html" className={`${SUBNAV_LINK_CLASS} text-muted-foreground hover:text-foreground`}>
            {t('nav.recordings')}
          </a>
          <a href="timeline.html" className={`${SUBNAV_LINK_CLASS} text-muted-foreground hover:text-foreground`}>
            {t('nav.timeline')}
          </a>
          <a href="investigation.html" className={`${SUBNAV_LINK_CLASS} text-muted-foreground hover:text-foreground`}>
            {t('nav.investigation')}
          </a>
          <a
            href="plates.html"
            aria-current="page"
            className={`${SUBNAV_LINK_CLASS} bg-card text-card-foreground border border-border border-b-0 -mb-px`}
          >
            {t('nav.plates')}
          </a>
        </nav>
      </div>

      <p className="mb-4 text-sm text-muted-foreground">{t('lpr.description')}</p>

      <form
        className="mb-4 grid gap-4 rounded-lg border border-border bg-card p-4 text-card-foreground md:grid-cols-2 lg:grid-cols-3"
        onSubmit={handleSearch}
        noValidate
        aria-label={t('lpr.filters')}
      >
        <div>
          <label htmlFor="lpr-camera" className="block">
            <span id="lpr-camera-label" className={LABEL_CLASS}>{t('lpr.camera')}</span>
            <select
              id="lpr-camera"
              className={INPUT_CLASS}
              value={filters.cameraUuid}
              onChange={updateFilter('cameraUuid')}
              disabled={streamsLoading || !cameras.length}
              aria-labelledby="lpr-camera-label"
              aria-invalid={errorsFor('camera').length > 0}
              aria-describedby={describedBy('camera')}
            >
              {!cameras.length && (
                <option value="">
                  {streamsLoading ? t('lpr.loadingCameras') : t('lpr.noCameras')}
                </option>
              )}
              {cameras.map((camera) => (
                <option key={camera.camera_uuid} value={camera.camera_uuid}>{camera.name}</option>
              ))}
            </select>
          </label>
          {streamsError && (
            <p className="mt-1 text-xs text-red-600 dark:text-red-400">{t('lpr.error.cameras')}</p>
          )}
          <FieldErrors id="lpr-camera-error" errors={errorsFor('camera')} t={t} />
        </div>

        <div>
          <label htmlFor="lpr-start" className="block">
            <span id="lpr-start-label" className={LABEL_CLASS}>{t('lpr.start')}</span>
            <input
              id="lpr-start"
              type="datetime-local"
              className={INPUT_CLASS}
              value={filters.start}
              onInput={updateFilter('start')}
              aria-labelledby="lpr-start-label"
              aria-invalid={errorsFor('start').length > 0}
              aria-describedby={describedBy('start')}
            />
          </label>
          <FieldErrors id="lpr-start-error" errors={errorsFor('start')} t={t} />
        </div>

        <div>
          <label htmlFor="lpr-end" className="block">
            <span id="lpr-end-label" className={LABEL_CLASS}>{t('lpr.end')}</span>
            <input
              id="lpr-end"
              type="datetime-local"
              className={INPUT_CLASS}
              value={filters.end}
              onInput={updateFilter('end')}
              aria-labelledby="lpr-end-label"
              aria-invalid={errorsFor('end').length > 0}
              aria-describedby={describedBy('end')}
            />
          </label>
          <FieldErrors id="lpr-end-error" errors={errorsFor('end')} t={t} />
        </div>

        <div>
          <label htmlFor="lpr-plate" className="block">
            <span id="lpr-plate-label" className={LABEL_CLASS}>{t('lpr.plate')}</span>
            <input
              id="lpr-plate"
              type="text"
              className={`${INPUT_CLASS} font-mono uppercase`}
              value={filters.plate}
              onInput={updateFilter('plate')}
              autoComplete="off"
              spellCheck={false}
              placeholder={t('lpr.platePlaceholder')}
              aria-labelledby="lpr-plate-label"
              aria-invalid={errorsFor('plate').length > 0}
              aria-describedby={describedBy('plate') || 'lpr-plate-hint'}
            />
          </label>
          <p id="lpr-plate-hint" className="mt-1 text-xs text-muted-foreground">{t('lpr.plateHint')}</p>
          <FieldErrors id="lpr-plate-error" errors={errorsFor('plate')} t={t} />
        </div>

        <fieldset>
          <legend className={LABEL_CLASS}>{t('lpr.match')}</legend>
          <div className="flex gap-4 pt-2 text-sm">
            {['partial', 'exact'].map((mode) => (
              <label key={mode} htmlFor={`lpr-match-${mode}`} className="inline-flex items-center gap-2">
                <input
                  id={`lpr-match-${mode}`}
                  type="radio"
                  name="lpr-match"
                  value={mode}
                  checked={filters.match === mode}
                  onChange={updateFilter('match')}
                  aria-labelledby={`lpr-match-${mode}-label`}
                />
                <span id={`lpr-match-${mode}-label`}>
                  {t(mode === 'exact' ? 'lpr.matchExact' : 'lpr.matchPartial')}
                </span>
              </label>
            ))}
          </div>
          <FieldErrors id="lpr-match-error" errors={errorsFor('match')} t={t} />
        </fieldset>

        <div>
          <label htmlFor="lpr-limit" className="block">
            <span id="lpr-limit-label" className={LABEL_CLASS}>{t('lpr.limit')}</span>
            <input
              id="lpr-limit"
              type="number"
              min="1"
              max={LPR_SEARCH_LIMIT_MAX}
              step="1"
              className={INPUT_CLASS}
              value={filters.limit}
              onInput={updateFilter('limit')}
              aria-labelledby="lpr-limit-label"
              aria-invalid={errorsFor('limit').length > 0}
              aria-describedby={describedBy('limit')}
            />
          </label>
          <FieldErrors id="lpr-limit-error" errors={errorsFor('limit')} t={t} />
        </div>

        <div className="flex flex-wrap items-center gap-3 md:col-span-2 lg:col-span-3">
          <button type="submit" className="btn-primary" disabled={searching}>
            {searching ? t('lpr.searching') : t('lpr.search')}
          </button>
          {exportVisible && (
            <button type="button" className="btn-secondary" onClick={handleExport} disabled={exporting}>
              {exporting ? t('lpr.exporting') : t('lpr.export')}
            </button>
          )}
        </div>
      </form>

      <div aria-live="polite">
        {requestError && (
          <p role="alert" className="mb-4 rounded-md border border-red-300 bg-red-50 p-3 text-sm text-red-800 dark:border-red-800 dark:bg-red-900/20 dark:text-red-300">
            {requestError}
          </p>
        )}

        {reads === null && !requestError && (
          <p className="rounded-lg border border-dashed border-border p-6 text-center text-sm text-muted-foreground">
            {t('lpr.prompt')}
          </p>
        )}

        {reads !== null && reads.length === 0 && (
          <div className="rounded-lg border border-dashed border-border p-6 text-center">
            <p className="font-medium">{t('lpr.emptyTitle')}</p>
            <p className="mt-1 text-sm text-muted-foreground">{t('lpr.emptyBody')}</p>
          </div>
        )}

        {reads !== null && reads.length > 0 && (
          <div className="overflow-x-auto rounded-lg border border-border">
            <table className="min-w-full divide-y divide-border text-sm">
              <caption className="sr-only">{t('lpr.resultsCaption', { count: reads.length })}</caption>
              <thead className="bg-muted/50 text-left">
                <tr>
                  <th scope="col" className="px-3 py-2">{t('lpr.column.plate')}</th>
                  <th scope="col" className="px-3 py-2">{t('lpr.column.time')}</th>
                  <th scope="col" className="px-3 py-2">{t('lpr.column.camera')}</th>
                  <th scope="col" className="px-3 py-2">{t('lpr.column.confidence')}</th>
                  <th scope="col" className="px-3 py-2">{t('lpr.column.direction')}</th>
                  <th scope="col" className="px-3 py-2">{t('lpr.column.vehicle')}</th>
                  <th scope="col" className="px-3 py-2">{t('lpr.column.actions')}</th>
                </tr>
              </thead>
              <tbody className="divide-y divide-border">
                {reads.map((read) => {
                  const attributes = read.attributes || {};
                  const vehicle = [attributes.vehicle_color, attributes.vehicle_type].filter(Boolean).join(' ');
                  const timelineHref = lprTimelineHref(read, cameras);
                  return (
                    <tr key={read.id}>
                      <th scope="row" className="px-3 py-2 font-mono font-semibold">{read.plate}</th>
                      <td className="whitespace-nowrap px-3 py-2">{formatObservedAt(read.observed_at)}</td>
                      <td className="px-3 py-2">{cameraName(read.camera_uuid) || read.stream_name}</td>
                      <td className="px-3 py-2">{formatLprConfidence(read.confidence) || '—'}</td>
                      <td className="px-3 py-2">{attributes.direction || '—'}</td>
                      <td className="px-3 py-2">{vehicle || '—'}</td>
                      <td className="whitespace-nowrap px-3 py-2">
                        <div className="flex gap-2">
                          {timelineHref && (
                            <a
                              href={timelineHref}
                              className="btn-secondary px-2 py-1 text-xs"
                              aria-label={t('lpr.viewRecordingFor', { plate: read.plate })}
                            >
                              {t('lpr.viewRecording')}
                            </a>
                          )}
                          {actions.canDelete && (
                            <button
                              type="button"
                              className="btn-danger px-2 py-1 text-xs"
                              onClick={() => setDeleteCandidate(read)}
                              aria-label={t('lpr.deleteFor', { plate: read.plate })}
                            >
                              {t('common.delete')}
                            </button>
                          )}
                        </div>
                      </td>
                    </tr>
                  );
                })}
              </tbody>
            </table>
          </div>
        )}
      </div>

      <ConfirmDialog
        isOpen={Boolean(deleteCandidate)}
        onClose={() => setDeleteCandidate(null)}
        onConfirm={() => deleteCandidate && handleDelete(deleteCandidate)}
        title={t('lpr.deleteTitle')}
        message={t('lpr.deleteMessage', { plate: deleteCandidate?.plate || '' })}
        confirmLabel={t('common.delete')}
        cancelLabel={t('common.cancel')}
        variant="danger"
      />
    </section>
  );
}
