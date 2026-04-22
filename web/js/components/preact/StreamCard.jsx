/**
 * LightNVR Web Interface StreamCard Component
 *
 * Per-stream card used by the responsive grid layout on the Streams page
 * (PRD UXD_01 §5.5, task T5, driving issue #399). Extracted from
 * StreamsView.jsx so the card renders as an isolated unit and the parent
 * stays focused on data flow.
 *
 * Layout (top → bottom):
 *   1. Snapshot thumbnail (go2rtc `frame.jpeg` proxy, neutral placeholder
 *      on error). 16:9 aspect ratio.
 *   2. Title row: stream name + overall status badge.
 *   3. Mode badges: recording-mode badge (Constant + Detection +
 *      Schedule joined by " + ") and a transport badge (TCP/UDP).
 *   4. Details disclosure — binding (admin URL) + source URL. Collapsed
 *      behind a `<details>` on < 640 px viewports, visible-always above
 *      that.
 *   5. Action row: edit, clone, snapshot link-out, enable/disable toggle,
 *      delete. Network-bound actions use T1's `<AsyncButton>` so the
 *      double-tap / rapid-resubmit bug from #399 can't fire twice.
 *
 * The responsive grid container lives in StreamsView (`grid
 * grid-cols-[repeat(auto-fill,minmax(360px,1fr))] gap-4`); this
 * component only styles a single card.
 */

import { useState } from 'preact/hooks';
import { AsyncButton } from './AsyncButton.jsx';
import { obfuscateUrlCredentials, urlHasCredentials } from '../../utils/url-utils.js';

/**
 * Collect the active recording modes for a stream in display order.
 *
 * Fixes the #399 "Enabled (Detection)" bug where constant recording was
 * hidden whenever detection was also on. The list is always returned in
 * the order [Constant, Detection, Schedule] — any subset may be active.
 *
 * @param {Object} stream
 * @returns {string[]} e.g. ['Constant', 'Detection'] or ['Schedule'] or []
 */
export function collectRecordingModes(stream, t) {
  const modes = [];
  // `record` is the canonical constant-recording flag. When
  // `record_on_schedule` is true the scheduled window governs when
  // recording runs — surface that distinctly from a pure "always on"
  // constant recording.
  const isConstant = !!stream.record && !stream.record_on_schedule;
  const isScheduled = !!stream.record && !!stream.record_on_schedule;
  // detection_based_recording is the DB field name the backend returns
  // (see SteamsView's existing access at line 1559 of the pre-T5 file).
  const isDetection = !!stream.detection_based_recording;

  if (isConstant) modes.push(t ? t('streams.continuous') : 'Constant');
  if (isDetection) modes.push(t ? t('streams.detection') : 'Detection');
  if (isScheduled) modes.push(t ? t('streams.scheduled') : 'Schedule');
  return modes;
}

/**
 * Map an RTSP-transport numeric protocol field to a label.
 *   0 → TCP, 1 → UDP, everything else → AUTO
 *
 * The Streams list exposes this per-stream transport preference; it's
 * the closest thing to a "transport" surface at the list level (live
 * playback transports like HLS/MSE/WebRTC are chosen per-viewer, not
 * per-stream, so they're not available here).
 */
function protocolLabel(stream) {
  const p = stream.protocol;
  if (p === 0 || p === '0') return 'TCP';
  if (p === 1 || p === '1') return 'UDP';
  return 'AUTO';
}

/**
 * Overall status → badge colour/label lookup. Mirrors the existing
 * table's `statusColor` / `statusLabel` switch (StreamsView pre-T5).
 */
function statusBadge(stream, t) {
  const s = stream.status;
  let color = 'hsl(var(--muted-foreground))';
  let label = s || (t ? t('common.unknown') : 'Unknown');
  switch (s) {
    case 'Running':
      color = 'hsl(var(--success))';
      label = t ? t('streams.running') : 'Running';
      break;
    case 'Starting':
    case 'Reconnecting':
    case 'Stopping':
      color = 'hsl(var(--warning, 45 93% 47%))';
      label = t
        ? (s === 'Starting' ? t('streams.starting')
          : s === 'Reconnecting' ? t('streams.reconnecting')
          : t('streams.stopping'))
        : s;
      break;
    case 'Error':
      color = 'hsl(var(--danger))';
      label = t ? t('streams.error') : 'Error';
      break;
    case 'Stopped':
      color = 'hsl(var(--danger))';
      label = t ? t('streams.stopped') : 'Stopped';
      break;
    default:
      break;
  }
  return { color, label };
}

/**
 * Build the snapshot URL (go2rtc frame proxy). Returns null if we can't
 * form a URL (no window in SSR/test environments).
 */
function buildSnapshotUrl(streamName) {
  if (typeof window === 'undefined' || !streamName) return null;
  // Cache-bust so cards refresh the frame whenever the page mounts.
  const ts = Date.now();
  return `${window.location.origin}/go2rtc/api/frame.jpeg?src=${encodeURIComponent(streamName)}&t=${ts}`;
}

/**
 * Neutral snapshot placeholder — used when go2rtc has no frame yet or
 * the <img> errors out.
 */
function SnapshotPlaceholder({ label }) {
  return (
    <div class="w-full aspect-video bg-muted flex items-center justify-center text-muted-foreground text-xs">
      <div class="flex flex-col items-center gap-1">
        <svg class="w-8 h-8 opacity-50" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
            d="M3 9a2 2 0 012-2h.93a2 2 0 001.664-.89l.812-1.22A2 2 0 0110.07 4h3.86a2 2 0 011.664.89l.812 1.22A2 2 0 0018.07 7H19a2 2 0 012 2v9a2 2 0 01-2 2H5a2 2 0 01-2-2V9z" />
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
            d="M15 13a3 3 0 11-6 0 3 3 0 016 0z" />
        </svg>
        <span>{label}</span>
      </div>
    </div>
  );
}

/**
 * StreamCard — one card per stream.
 *
 * @param {Object} props
 * @param {Object} props.stream                 Stream record from /api/streams
 * @param {boolean} props.canModifyStreams      Whether user may edit/delete
 * @param {boolean} props.shouldHideCredentials Whether to mask URL creds
 * @param {boolean} props.selectionMode         Whether bulk-select is active
 * @param {boolean} props.isSelected            Whether THIS card is selected
 * @param {Function} props.onToggleSelect       (evt, name) => void
 * @param {Function} props.onEdit               (name) => void
 * @param {Function} props.onClone              (name) => void
 * @param {Function} props.onOpenDelete         (stream) => void
 * @param {Function} props.onEnable             (name) => Promise
 * @param {Function} props.onDisable            (name) => Promise
 * @param {(k: string, params?: Object) => string} props.t i18n translator
 */
export function StreamCard({
  stream,
  canModifyStreams,
  shouldHideCredentials,
  selectionMode,
  isSelected,
  onToggleSelect,
  onEdit,
  onClone,
  onOpenDelete,
  onEnable,
  onDisable,
  t
}) {
  const [revealed, setRevealed] = useState(false);
  const [snapshotBroken, setSnapshotBroken] = useState(false);

  const snapshotSrc = !snapshotBroken ? buildSnapshotUrl(stream.name) : null;
  const { color: statusColor, label: statusLabel } = statusBadge(stream, t);
  const modes = collectRecordingModes(stream, t);
  const modeText = modes.length > 0 ? modes.join(' + ') : t('streams.notRecording');
  // Spell the full set out for screen readers so the badge's
  // joined-with-plus visual doesn't obscure the meaning.
  const modeAriaLabel = modes.length > 0
    ? t('streams.recordingModesList', { modes: modes.join(', ') })
    : t('streams.notRecording');
  const transportLabel = protocolLabel(stream);
  const hasAdminLauncher = !shouldHideCredentials && /^https?:\/\//i.test(stream.admin_url || '');

  // AsyncButton handlers — return the parent promise so pending/error
  // state surfaces on the card.
  const handleEnable = () => Promise.resolve(onEnable(stream.name));
  const handleDisable = () => Promise.resolve(onDisable(stream.name));

  return (
    <div
      class={`stream-card bg-card text-card-foreground rounded-lg shadow overflow-hidden flex flex-col border border-border ${
        isSelected ? 'ring-2 ring-primary' : ''
      }`}
      data-stream={stream.name}
    >
      {/* Snapshot — clickable to toggle selection when in selection mode */}
      <div
        class={`relative w-full aspect-video bg-muted flex-shrink-0 ${
          selectionMode ? 'cursor-pointer' : ''
        }`}
        onClick={selectionMode ? (e) => onToggleSelect(e, stream.name) : undefined}
      >
        {snapshotSrc ? (
          <img
            src={snapshotSrc}
            alt={t('streams.snapshotFor', { name: stream.name })}
            class="w-full h-full object-cover"
            onError={() => setSnapshotBroken(true)}
            loading="lazy"
          />
        ) : (
          <SnapshotPlaceholder label={t('streams.snapshotUnavailable')} />
        )}

        {/* Enabled/disabled dot overlay, top-left */}
        <span
          class="absolute top-2 left-2 w-3 h-3 rounded-full border border-white/70 shadow"
          style={{ backgroundColor: stream.enabled ? 'hsl(var(--success))' : 'hsl(var(--danger))' }}
          title={stream.enabled ? t('common.enabled') : t('common.disabled')}
        ></span>

        {/* Bulk-select checkbox overlay */}
        {canModifyStreams && selectionMode && (
          <div class="absolute top-2 right-2">
            <input
              type="checkbox"
              class="w-5 h-5 rounded cursor-pointer accent-primary"
              checked={!!isSelected}
              onChange={(e) => onToggleSelect(e, stream.name)}
              onClick={(e) => e.stopPropagation()}
            />
          </div>
        )}
      </div>

      {/* Body */}
      <div class="p-3 flex flex-col gap-2 flex-1">
        {/* Title + status */}
        <div class="flex items-start justify-between gap-2">
          <h3 class="font-bold text-sm truncate" title={stream.name}>{stream.name}</h3>
          <span
            class="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium whitespace-nowrap flex-shrink-0"
            style={{
              backgroundColor: statusColor,
              color: 'white',
              opacity: stream.enabled ? 1 : 0.6
            }}
          >
            {statusLabel}
          </span>
        </div>

        {/* Resolution + FPS (compact secondary line) */}
        <div class="flex items-center gap-2 text-xs text-muted-foreground">
          <span>{stream.width || '?'}×{stream.height || '?'}</span>
          <span>•</span>
          <span>{stream.fps || '?'} {t('streams.fps')}</span>
          {stream.codec && <>
            <span>•</span>
            <span class="uppercase">{stream.codec}</span>
          </>}
        </div>

        {/* Mode badges row */}
        <div class="flex flex-wrap gap-1.5">
          {/* Recording-mode badge — #399 fix: show all active modes */}
          <span
            class="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium"
            style={
              modes.length > 0
                ? { backgroundColor: 'hsl(var(--success) / 0.15)', color: 'hsl(var(--success))', border: '1px solid hsl(var(--success) / 0.4)' }
                : { backgroundColor: 'hsl(var(--muted))', color: 'hsl(var(--muted-foreground))', border: '1px solid hsl(var(--border))' }
            }
            aria-label={modeAriaLabel}
            title={modeAriaLabel}
          >
            {modeText}
          </span>

          {/* Transport badge */}
          <span
            class="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium"
            style={{
              backgroundColor: 'hsl(var(--primary) / 0.12)',
              color: 'hsl(var(--primary))',
              border: '1px solid hsl(var(--primary) / 0.35)'
            }}
            title={t('streams.transportLabel', { transport: transportLabel })}
          >
            {transportLabel}
          </span>

          {/* ONVIF badge when applicable */}
          {stream.isOnvif && (
            <span
              class="inline-flex items-center px-2 py-0.5 rounded-full text-xs font-medium bg-muted text-muted-foreground border border-border"
              title="ONVIF"
            >
              ONVIF
            </span>
          )}
        </div>

        {/* Details disclosure — collapsed on <640px, always-open above.
             Two parallel DOM branches so there is no flash on resize. */}
        <details class="sm:hidden text-xs mt-1" data-stream-details="mobile">
          <summary class="cursor-pointer text-muted-foreground select-none">
            {t('streams.details')}
          </summary>
          <div class="mt-2">
            <DetailsBlock
              stream={stream}
              revealed={revealed}
              setRevealed={setRevealed}
              shouldHideCredentials={shouldHideCredentials}
              hasAdminLauncher={hasAdminLauncher}
              t={t}
            />
          </div>
        </details>
        <div class="hidden sm:block text-xs" data-stream-details="desktop">
          <DetailsBlock
            stream={stream}
            revealed={revealed}
            setRevealed={setRevealed}
            shouldHideCredentials={shouldHideCredentials}
            hasAdminLauncher={hasAdminLauncher}
            t={t}
          />
        </div>

        {/* Action row */}
        <div class="flex items-center gap-1 pt-1 mt-auto border-t border-border -mx-3 px-3 -mb-1">
          {hasAdminLauncher && (
            <a
              class="p-1.5 rounded-md min-h-11 min-w-11 inline-flex items-center justify-center focus:outline-none focus:ring-2 focus:ring-primary"
              style={{ color: 'hsl(var(--primary))' }}
              onMouseOver={(e) => { e.currentTarget.style.backgroundColor = 'hsl(var(--primary) / 0.1)'; }}
              onMouseOut={(e) => { e.currentTarget.style.backgroundColor = 'transparent'; }}
              href={stream.admin_url}
              target="_blank"
              rel="noopener noreferrer"
              title={t('streams.openCameraAdminPage')}
              aria-label={t('streams.openAdminPageFor', { name: stream.name })}
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M14 3h7m0 0v7m0-7L10 14" />
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M5 5h5M5 5v14h14v-5" />
              </svg>
            </a>
          )}

          {canModifyStreams && (
            <button
              type="button"
              class="p-1.5 rounded-md min-h-11 min-w-11 inline-flex items-center justify-center focus:outline-none focus:ring-2 focus:ring-primary"
              style={{ color: 'hsl(var(--primary))' }}
              onMouseOver={(e) => { e.currentTarget.style.backgroundColor = 'hsl(var(--primary) / 0.1)'; }}
              onMouseOut={(e) => { e.currentTarget.style.backgroundColor = 'transparent'; }}
              onClick={() => onEdit(stream.name)}
              title={t('common.edit')}
              aria-label={t('common.edit')}
            >
              <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                <path d="M13.586 3.586a2 2 0 112.828 2.828l-.793.793-2.828-2.828.793-.793zM11.379 5.793L3 14.172V17h2.828l8.38-8.379-2.83-2.828z" />
              </svg>
            </button>
          )}

          {canModifyStreams && (
            <button
              type="button"
              class="p-1.5 rounded-md min-h-11 min-w-11 inline-flex items-center justify-center focus:outline-none focus:ring-2 focus:ring-primary"
              style={{ color: 'hsl(var(--success))' }}
              onMouseOver={(e) => { e.currentTarget.style.backgroundColor = 'hsl(var(--success) / 0.1)'; }}
              onMouseOut={(e) => { e.currentTarget.style.backgroundColor = 'transparent'; }}
              onClick={() => onClone(stream.name)}
              title={t('streams.cloneStream')}
              aria-label={t('streams.cloneStream')}
            >
              <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M8 16H6a2 2 0 01-2-2V6a2 2 0 012-2h8a2 2 0 012 2v2m-6 12h8a2 2 0 002-2v-8a2 2 0 00-2-2h-8a2 2 0 00-2 2v8a2 2 0 002 2z" />
              </svg>
            </button>
          )}

          {/* Enable/Disable toggle — network-bound, so AsyncButton
              so the #399 rapid-tap double-submit can't fire twice. */}
          {canModifyStreams && (
            stream.enabled ? (
              <AsyncButton
                type="button"
                class="p-1.5 rounded-md min-h-11 min-w-11 inline-flex items-center justify-center focus:outline-none focus:ring-2 focus:ring-primary"
                style={{ color: 'hsl(var(--success))' }}
                onClick={handleDisable}
                confirmText={t('streams.disableStreamConfirm')}
                title={t('streams.toggleDisable')}
                aria-label={t('streams.toggleDisable')}
                idleLabel={(
                  <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                      d="M13 10V3L4 14h7v7l9-11h-7z" />
                  </svg>
                )}
              />
            ) : (
              <AsyncButton
                type="button"
                class="p-1.5 rounded-md min-h-11 min-w-11 inline-flex items-center justify-center focus:outline-none focus:ring-2 focus:ring-primary"
                style={{ color: 'hsl(var(--muted-foreground))' }}
                onClick={handleEnable}
                title={t('streams.toggleEnable')}
                aria-label={t('streams.toggleEnable')}
                idleLabel={(
                  <svg class="w-5 h-5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                    <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                      d="M13 10V3L4 14h7v7l9-11h-7z" />
                  </svg>
                )}
              />
            )
          )}

          {/* Delete opens the StreamDeleteModal (which already uses
              AsyncButton internally for the network calls). The card-
              level launcher is not itself network-bound, so a plain
              button is fine here. */}
          {canModifyStreams && (
            <button
              type="button"
              class="p-1.5 rounded-md min-h-11 min-w-11 inline-flex items-center justify-center focus:outline-none focus:ring-2 focus:ring-primary"
              style={{ color: 'hsl(var(--danger))' }}
              onMouseOver={(e) => { e.currentTarget.style.backgroundColor = 'hsl(var(--danger) / 0.1)'; }}
              onMouseOut={(e) => { e.currentTarget.style.backgroundColor = 'transparent'; }}
              onClick={() => onOpenDelete(stream)}
              title={t('common.delete')}
              aria-label={t('common.delete')}
            >
              <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                <path fill-rule="evenodd"
                  d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z"
                  clip-rule="evenodd" />
              </svg>
            </button>
          )}

          {!canModifyStreams && !hasAdminLauncher && (
            <span class="text-muted-foreground text-xs">—</span>
          )}
        </div>
      </div>
    </div>
  );
}

/**
 * DetailsBlock — binding (admin_url) + source URL with credential mask
 * toggle. Shared by the mobile `<details>` disclosure and the
 * always-visible desktop block.
 */
function DetailsBlock({ stream, revealed, setRevealed, shouldHideCredentials, hasAdminLauncher, t }) {
  const showRevealToggle = urlHasCredentials(stream.url) && !shouldHideCredentials;
  const sourceUrlDisplay = revealed && !shouldHideCredentials
    ? stream.url
    : obfuscateUrlCredentials(stream.url);
  return (
    <div class="space-y-1.5">
      {/* Binding / admin URL */}
      <div class="flex items-start gap-1.5">
        <span class="font-medium text-muted-foreground flex-shrink-0">
          {t('streams.binding')}:
        </span>
        {hasAdminLauncher ? (
          <a
            href={stream.admin_url}
            target="_blank"
            rel="noopener noreferrer"
            class="font-mono text-xs truncate text-ellipsis overflow-hidden min-w-0 flex-1 text-primary hover:underline"
            title={stream.admin_url}
          >
            {stream.admin_url}
          </a>
        ) : (
          <span
            class="font-mono text-xs truncate text-ellipsis overflow-hidden min-w-0 flex-1"
            title={stream.admin_url || t('streams.notAvailable')}
          >
            {stream.admin_url || t('streams.notAvailable')}
          </span>
        )}
      </div>

      {/* Source URL */}
      <div class="flex items-start gap-1.5">
        <span class="font-medium text-muted-foreground flex-shrink-0">
          {t('common.url')}:
        </span>
        <span
          class="font-mono text-xs truncate text-ellipsis overflow-hidden min-w-0 flex-1"
          title={sourceUrlDisplay}
        >
          {sourceUrlDisplay}
        </span>
        {showRevealToggle && (
          <button
            type="button"
            class="flex-shrink-0 p-0.5 rounded text-muted-foreground hover:text-foreground transition-colors focus:outline-none"
            onClick={(e) => { e.stopPropagation(); setRevealed(v => !v); }}
            title={revealed ? t('streams.hideCredentials') : t('streams.showCredentials')}
            aria-label={revealed ? t('streams.hideCredentials') : t('streams.showCredentials')}
          >
            {revealed ? (
              <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M13.875 18.825A10.05 10.05 0 0112 19c-4.478 0-8.268-2.943-9.543-7a9.97 9.97 0 011.563-3.029m5.858.908a3 3 0 114.243 4.243M9.878 9.878l4.242 4.242M9.88 9.88l-3.29-3.29m7.532 7.532l3.29 3.29M3 3l3.59 3.59m0 0A9.953 9.953 0 0112 5c4.478 0 8.268 2.943 9.543 7a10.025 10.025 0 01-4.132 4.411m0 0L21 21" />
              </svg>
            ) : (
              <svg class="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
                <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2"
                  d="M2.458 12C3.732 7.943 7.523 5 12 5c4.478 0 8.268 2.943 9.542 7-1.274 4.057-5.064 7-9.542 7-4.477 0-8.268-2.943-9.542-7z" />
              </svg>
            )}
          </button>
        )}
      </div>

      {/* Sub-stream URL (if present) */}
      {stream.sub_stream_url && (
        <div class="flex items-start gap-1.5">
          <span class="font-medium text-muted-foreground flex-shrink-0">
            {t('streams.subStreamUrl') || 'Sub-stream'}:
          </span>
          <span
            class="font-mono text-xs truncate text-ellipsis overflow-hidden min-w-0 flex-1"
            title={obfuscateUrlCredentials(stream.sub_stream_url)}
          >
            {obfuscateUrlCredentials(stream.sub_stream_url)}
          </span>
        </div>
      )}
    </div>
  );
}

export default StreamCard;
