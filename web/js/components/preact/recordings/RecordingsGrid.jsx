/**
 * RecordingsGrid component for RecordingsView
 * Displays recordings as thumbnail cards in a responsive grid layout.
 * On hover, cycles through 3 thumbnail frames (start, middle, end).
 */

import { h } from 'preact';
import { useState, useEffect, useRef, useCallback } from 'preact/hooks';
import { formatUtils } from './formatUtils.js';
import { queueThumbnailLoad, Priority } from '../../../request-queue.js';

/** Card elements that can be hidden via the settings cog */
const HIDEABLE_ELEMENTS = [
  { key: 'stream', label: 'Stream Name' },
  { key: 'duration', label: 'Duration' },
  { key: 'detections', label: 'Detections' },
  { key: 'protected', label: 'Protected Badge' },
  { key: 'actions', label: 'Actions' },
];

/**
 * Settings cog dropdown for grid cards – mirrors ColumnConfigDropdown from RecordingsTable
 */
function CardConfigDropdown({ hiddenColumns, toggleColumn }) {
  const [open, setOpen] = useState(false);
  const ref = useRef(null);

  useEffect(() => {
    if (!open) return;
    const handler = (e) => { if (ref.current && !ref.current.contains(e.target)) setOpen(false); };
    document.addEventListener('mousedown', handler);
    return () => document.removeEventListener('mousedown', handler);
  }, [open]);

  return (
    <div ref={ref} class="relative inline-block">
      <button
        type="button"
        class="p-1.5 rounded hover:bg-muted/70 transition-colors"
        onClick={() => setOpen(o => !o)}
        title="Configure visible card elements"
      >
        <svg class="w-4 h-4 text-muted-foreground" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2}
            d="M10.325 4.317c.426-1.756 2.924-1.756 3.35 0a1.724 1.724 0 002.573 1.066c1.543-.94 3.31.826 2.37 2.37a1.724 1.724 0 001.066 2.573c1.756.426 1.756 2.924 0 3.35a1.724 1.724 0 00-1.066 2.573c.94 1.543-.826 3.31-2.37 2.37a1.724 1.724 0 00-2.573 1.066c-.426 1.756-2.924 1.756-3.35 0a1.724 1.724 0 00-2.573-1.066c-1.543.94-3.31-.826-2.37-2.37a1.724 1.724 0 00-1.066-2.573c-1.756-.426-1.756-2.924 0-3.35a1.724 1.724 0 001.066-2.573c-.94-1.543.826-3.31 2.37-2.37.996.608 2.296.07 2.572-1.065z" />
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M15 12a3 3 0 11-6 0 3 3 0 016 0z" />
        </svg>
      </button>
      {open && (
        <div class="absolute right-0 mt-1 w-48 bg-card border border-border rounded-lg shadow-lg z-50 py-1">
          <div class="px-3 py-1.5 text-xs font-semibold text-muted-foreground uppercase tracking-wider border-b border-border">
            Visible Elements
          </div>
          {HIDEABLE_ELEMENTS.map(el => (
            <label key={el.key} class="flex items-center gap-2 px-3 py-1.5 hover:bg-muted/50 cursor-pointer text-sm">
              <input
                type="checkbox"
                checked={!hiddenColumns[el.key]}
                onChange={() => toggleColumn(el.key)}
                class="w-3.5 h-3.5 rounded"
                style={{ accentColor: 'hsl(var(--primary))' }}
              />
              {el.label}
            </label>
          ))}
        </div>
      )}
    </div>
  );
}

/**
 * Single recording card with thumbnail and hover animation
 */
function RecordingCard({
  recording,
  playRecording,
  downloadRecording,
  deleteRecording,
  toggleProtection,
  selectedRecordings,
  toggleRecordingSelection,
  canDelete,
  selectionMode,
  hiddenColumns
}) {
  const [currentFrame, setCurrentFrame] = useState(1); // Start with middle frame
  const [isHovering, setIsHovering] = useState(false);
  const [loadState, setLoadState] = useState('loading'); // 'loading', 'loaded', 'error'
  const intervalRef = useRef(null);
  const preloadedRef = useRef(false);
  const imgErrorCountRef = useRef(0);

  /** Load (or reload) the middle-frame thumbnail. */
  const loadThumbnail = useCallback(() => {
    const url = `/api/recordings/thumbnail/${recording.id}/1`;
    setLoadState('loading');
    imgErrorCountRef.current = 0;
    queueThumbnailLoad(url, Priority.HIGH)
      .then(() => setLoadState('loaded'))
      .catch(() => setLoadState('error'));
  }, [recording.id]);

  // Handle <img> element load errors — if the decoded image was evicted
  // from the browser cache between queueThumbnailLoad success and render,
  // retry once rather than permanently showing "Failed to load".
  const handleImageError = useCallback(() => {
    imgErrorCountRef.current++;
    if (imgErrorCountRef.current <= 1) {
      loadThumbnail(); // retry once
    } else {
      setLoadState('error');
    }
  }, [loadThumbnail]);

  // Preload the middle frame (index 1) on mount with HIGH priority since it's visible
  useEffect(() => {
    loadThumbnail();
  }, [loadThumbnail]);

  // Preload the other two frames when user first hovers (LOW priority background task)
  useEffect(() => {
    if (isHovering && !preloadedRef.current) {
      preloadedRef.current = true;
      for (const i of [0, 2]) {
        const url = `/api/recordings/thumbnail/${recording.id}/${i}`;
        queueThumbnailLoad(url, Priority.LOW).catch(() => {
          // Silently ignore preload failures
        });
      }
    }
  }, [isHovering, recording.id]);

  // Cycle through frames on hover
  useEffect(() => {
    if (isHovering) {
      intervalRef.current = setInterval(() => {
        setCurrentFrame(prev => (prev + 1) % 3);
      }, 800);
    } else {
      if (intervalRef.current) {
        clearInterval(intervalRef.current);
        intervalRef.current = null;
      }
      setCurrentFrame(1); // Reset to middle frame
    }
    return () => {
      if (intervalRef.current) clearInterval(intervalRef.current);
    };
  }, [isHovering]);

  const thumbnailUrl = `/api/recordings/thumbnail/${recording.id}/${currentFrame}`;
  const isSelected = !!selectedRecordings[recording.id];
  const show = (key) => !hiddenColumns[key];

  return (
    <div
      class={`recording-card relative bg-card text-card-foreground rounded-lg shadow overflow-hidden cursor-pointer group transition-all duration-200 hover:shadow-lg ${isSelected ? 'ring-2' : ''}`}
      style={isSelected ? { ringColor: 'hsl(var(--primary))' } : {}}
      onMouseEnter={() => setIsHovering(true)}
      onMouseLeave={() => setIsHovering(false)}
    >
      {/* Thumbnail area */}
      <div
        class="relative aspect-video bg-muted overflow-hidden"
        onClick={() => playRecording(recording)}
      >
        {loadState === 'loaded' ? (
          <img
            src={thumbnailUrl}
            alt={`${recording.stream} recording`}
            class="w-full h-full object-cover transition-opacity duration-300"
            onError={handleImageError}
          />
        ) : loadState === 'loading' ? (
          <div class="w-full h-full flex flex-col items-center justify-center text-muted-foreground">
            <svg class="w-12 h-12 opacity-30 animate-pulse" fill="currentColor" viewBox="0 0 20 20">
              <path fillRule="evenodd" d="M4 3a2 2 0 00-2 2v10a2 2 0 002 2h12a2 2 0 002-2V5a2 2 0 00-2-2H4zm12 12H4l4-8 3 6 2-4 3 6z" clipRule="evenodd" />
            </svg>
            <span class="text-xs mt-2 opacity-50">Loading...</span>
          </div>
        ) : (
          <div
            class="w-full h-full flex flex-col items-center justify-center text-muted-foreground bg-muted/50 cursor-pointer"
            onClick={(e) => { e.stopPropagation(); loadThumbnail(); }}
            title="Click to retry"
          >
            <svg class="w-12 h-12 opacity-40 text-destructive" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M12 9v2m0 4h.01m-6.938 4h13.856c1.54 0 2.502-1.667 1.732-3L13.732 4c-.77-1.333-2.694-1.333-3.464 0L3.34 16c-.77 1.333.192 3 1.732 3z" />
            </svg>
            <span class="text-xs mt-2 opacity-60">Tap to retry</span>
          </div>
        )}

        {/* Play overlay on hover */}
        <div class="absolute inset-0 flex items-center justify-center bg-black/30 opacity-0 group-hover:opacity-100 transition-opacity duration-200">
          <svg class="w-12 h-12 text-white drop-shadow-lg" fill="currentColor" viewBox="0 0 20 20">
            <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clipRule="evenodd" />
          </svg>
        </div>

        {/* Frame indicator dots */}
        {isHovering && (
          <div class="absolute bottom-2 left-1/2 -translate-x-1/2 flex gap-1">
            {[0, 1, 2].map(i => (
              <div
                key={i}
                class={`w-1.5 h-1.5 rounded-full transition-colors ${currentFrame === i ? 'bg-white' : 'bg-white/50'}`}
              />
            ))}
          </div>
        )}

        {/* Duration badge */}
        {show('duration') && (
          <div class="absolute bottom-2 right-2 px-1.5 py-0.5 bg-black/70 text-white text-xs rounded">
            {formatUtils.formatDuration(recording.duration)}
          </div>
        )}

        {/* Selection checkbox — only visible in selection mode */}
        {canDelete && selectionMode && (
          <div class="absolute top-2 left-2" onClick={(e) => e.stopPropagation()}>
            <input
              type="checkbox"
              checked={isSelected}
              onChange={() => toggleRecordingSelection(recording.id)}
              class="w-4 h-4 rounded focus:ring-2 cursor-pointer"
              style={{ accentColor: 'hsl(var(--primary))' }}
            />
          </div>
        )}

        {/* Protected badge */}
        {show('protected') && recording.protected && (
          <div class="absolute top-2 right-2 px-1.5 py-0.5 bg-yellow-500/90 text-white text-xs rounded flex items-center gap-0.5">
            <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
              <path fillRule="evenodd" d="M5 9V7a5 5 0 0110 0v2a2 2 0 012 2v5a2 2 0 01-2 2H5a2 2 0 01-2-2v-5a2 2 0 012-2zm8-2v2H7V7a3 3 0 016 0z" clipRule="evenodd" />
            </svg>
          </div>
        )}
      </div>

      {/* Card info — only render if at least one info element is visible */}
      {(show('stream') || show('detections') || show('actions')) && (
        <div class="p-3">
          {show('stream') && (
            <div class="flex items-center justify-between mb-1">
              <span class="font-medium text-sm truncate" title={recording.stream}>
                {recording.stream || 'Unknown'}
              </span>
              <span class="text-xs text-muted-foreground ml-2 whitespace-nowrap">
                {formatUtils.formatDateTime(recording.start_time)}
              </span>
            </div>
          )}

          {/* Detection badges */}
          {show('detections') && (
            <div class={`flex flex-wrap gap-1 min-h-[22px] ${show('actions') ? 'mb-2' : ''}`}>
              {recording.detection_labels && recording.detection_labels.length > 0 ? (
                recording.detection_labels.map((det, idx) => (
                  <span key={idx} class="badge-success text-xs" title={`${det.count} detection${det.count !== 1 ? 's' : ''}`}>
                    {det.label}
                    {det.count > 1 && <span class="ml-0.5 opacity-75">({det.count})</span>}
                  </span>
                ))
              ) : (
                <span class="text-xs text-muted-foreground/40 select-none">&nbsp;</span>
              )}
            </div>
          )}

          {/* Action buttons */}
          {show('actions') && (
            <div class="flex items-center gap-1 pt-1 border-t border-border">
              <button
                class="p-1 rounded-full focus:outline-none"
                style={{ color: 'hsl(var(--primary))' }}
                onClick={() => playRecording(recording)}
                title="Play"
              >
                <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                  <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zM9.555 7.168A1 1 0 008 8v4a1 1 0 001.555.832l3-2a1 1 0 000-1.664l-3-2z" clipRule="evenodd" />
                </svg>
              </button>
              <button
                class="p-1 rounded-full focus:outline-none"
                style={{ color: 'hsl(var(--success))' }}
                onClick={() => downloadRecording(recording)}
                title={`Download${recording.size ? ' (' + recording.size + ')' : ''}`}
              >
                <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                  <path fillRule="evenodd" d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm3.293-7.707a1 1 0 011.414 0L9 10.586V3a1 1 0 112 0v7.586l1.293-1.293a1 1 0 111.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z" clipRule="evenodd" />
                </svg>
              </button>
              <a
                class="p-1 rounded-full focus:outline-none inline-flex"
                style={{ color: 'hsl(var(--info))' }}
                href={`timeline.html?stream=${encodeURIComponent(recording.stream)}&date=${recording.start_time ? recording.start_time.slice(0, 10) : ''}&time=${recording.start_time ? recording.start_time.slice(11, 19) : ''}`}
                title="View in Timeline"
              >
                <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                  <path fillRule="evenodd" d="M10 18a8 8 0 100-16 8 8 0 000 16zm1-12a1 1 0 10-2 0v4a1 1 0 00.293.707l2.828 2.829a1 1 0 101.415-1.415L11 9.586V6z" clipRule="evenodd" />
                </svg>
              </a>
              <button
                class="p-1 rounded-full focus:outline-none"
                style={{ color: recording.protected ? 'hsl(var(--warning))' : 'hsl(var(--muted-foreground))' }}
                onClick={() => toggleProtection && toggleProtection(recording)}
                title={recording.protected ? 'Unprotect' : 'Protect'}
              >
                <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                  {recording.protected ? (
                    <path fillRule="evenodd" d="M5 9V7a5 5 0 0110 0v2a2 2 0 012 2v5a2 2 0 01-2 2H5a2 2 0 01-2-2v-5a2 2 0 012-2zm8-2v2H7V7a3 3 0 016 0z" clipRule="evenodd" />
                  ) : (
                    <path d="M10 2a5 5 0 00-5 5v2a2 2 0 00-2 2v5a2 2 0 002 2h10a2 2 0 002-2v-5a2 2 0 00-2-2H7V7a3 3 0 015.905-.75 1 1 0 001.937-.5A5.002 5.002 0 0010 2z" />
                  )}
                </svg>
              </button>
              {canDelete && (
                <button
                  class="p-1 rounded-full focus:outline-none ml-auto"
                  style={{ color: 'hsl(var(--danger))' }}
                  onClick={() => deleteRecording(recording)}
                  title="Delete"
                >
                  <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
                    <path fillRule="evenodd" d="M9 2a1 1 0 00-.894.553L7.382 4H4a1 1 0 000 2v10a2 2 0 002 2h8a2 2 0 002-2V6a1 1 0 100-2h-3.382l-.724-1.447A1 1 0 0011 2H9zM7 8a1 1 0 012 0v6a1 1 0 11-2 0V8zm5-1a1 1 0 00-1 1v6a1 1 0 102 0V8a1 1 0 00-1-1z" clipRule="evenodd" />
                  </svg>
                </button>
              )}
            </div>
          )}
        </div>
      )}
    </div>
  );
}

/**
 * RecordingsGrid component
 * @param {Object} props Component props
 * @returns {JSX.Element} RecordingsGrid component
 */
export function RecordingsGrid({
  recordings,
  selectedRecordings,
  toggleRecordingSelection,
  selectAll,
  toggleSelectAll,
  getSelectedCount,
  openDeleteModal,
  playRecording,
  downloadRecording,
  deleteRecording,
  toggleProtection,
  pagination,
  canDelete = true,
  clearSelections,
  hiddenColumns = {},
  toggleColumn = () => {}
}) {
  const [selectionMode, setSelectionMode] = useState(false);

  /** Exit selection mode and clear all selections */
  const exitSelectionMode = useCallback(() => {
    setSelectionMode(false);
    if (clearSelections) clearSelections();
  }, [clearSelections]);

  return (
    <div class="recordings-container bg-card text-card-foreground rounded-lg shadow overflow-hidden w-full">
      {/* Toolbar */}
      <div class="batch-actions px-3 py-2.5 border-b border-border flex flex-wrap gap-2 items-center">
        {canDelete && (
          <>
            {selectionMode ? (
              <>
                <div class="flex items-center gap-2 mr-2">
                  <input
                    type="checkbox"
                    checked={selectAll}
                    onChange={toggleSelectAll}
                    class="w-4 h-4 rounded focus:ring-2"
                    style={{ accentColor: 'hsl(var(--primary))' }}
                  />
                  <span class="text-sm text-muted-foreground">
                    {getSelectedCount() > 0
                      ? `${getSelectedCount()} recording${getSelectedCount() !== 1 ? 's' : ''} selected`
                      : 'Select all'}
                  </span>
                </div>
                <button
                  class="btn-danger text-xs px-2 py-1 disabled:opacity-50 disabled:cursor-not-allowed"
                  disabled={getSelectedCount() === 0}
                  onClick={() => openDeleteModal('selected')}
                >
                  Delete Selected
                </button>
                <button class="btn-danger text-xs px-2 py-1" onClick={() => openDeleteModal('all')}>
                  Delete All Filtered
                </button>
                <button
                  class="text-sm px-2 py-1 rounded hover:bg-muted/70 transition-colors text-muted-foreground"
                  onClick={exitSelectionMode}
                  title="Exit selection mode"
                >
                  Cancel
                </button>
              </>
            ) : (
              <button
                class="flex items-center gap-1.5 text-sm px-2 py-1 rounded hover:bg-muted/70 transition-colors text-muted-foreground"
                onClick={() => setSelectionMode(true)}
                title="Enter selection mode"
              >
                <svg class="w-4 h-4" fill="none" stroke="currentColor" viewBox="0 0 24 24">
                  <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2}
                    d="M9 5H7a2 2 0 00-2 2v12a2 2 0 002 2h10a2 2 0 002-2V7a2 2 0 00-2-2h-2M9 5a2 2 0 002 2h2a2 2 0 002-2M9 5a2 2 0 012-2h2a2 2 0 012 2m-6 9l2 2 4-4" />
                </svg>
                Select
              </button>
            )}
          </>
        )}
        <div class="ml-auto">
          <CardConfigDropdown hiddenColumns={hiddenColumns} toggleColumn={toggleColumn} />
        </div>
      </div>

      {/* Grid of cards */}
      <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4 p-4">
        {recordings.length === 0 ? (
          <div class="col-span-full text-center text-muted-foreground py-8">
            {pagination.totalItems === 0 ? 'No recordings found' : 'Loading recordings...'}
          </div>
        ) : (
          recordings.map(recording => (
            <RecordingCard
              key={recording.id}
              recording={recording}
              playRecording={playRecording}
              downloadRecording={downloadRecording}
              deleteRecording={deleteRecording}
              toggleProtection={toggleProtection}
              selectedRecordings={selectedRecordings}
              toggleRecordingSelection={toggleRecordingSelection}
              canDelete={canDelete}
              selectionMode={selectionMode}
              hiddenColumns={hiddenColumns}
            />
          ))
        )}
      </div>
    </div>
  );
}

