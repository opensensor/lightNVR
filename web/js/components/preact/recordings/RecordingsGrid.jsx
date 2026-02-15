/**
 * RecordingsGrid component for RecordingsView
 * Displays recordings as thumbnail cards in a responsive grid layout.
 * On hover, cycles through 3 thumbnail frames (start, middle, end).
 */

import { h } from 'preact';
import { useState, useEffect, useRef } from 'preact/hooks';
import { formatUtils } from './formatUtils.js';

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
  canDelete
}) {
  const [currentFrame, setCurrentFrame] = useState(1); // Start with middle frame
  const [isHovering, setIsHovering] = useState(false);
  const [imageError, setImageError] = useState(false);
  const intervalRef = useRef(null);

  // Preload all 3 frames on mount
  useEffect(() => {
    for (let i = 0; i < 3; i++) {
      const img = new Image();
      img.src = `/api/recordings/thumbnail/${recording.id}/${i}`;
    }
  }, [recording.id]);

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
        {!imageError ? (
          <img
            src={thumbnailUrl}
            alt={`${recording.stream} recording`}
            class="w-full h-full object-cover transition-opacity duration-300"
            onError={() => setImageError(true)}
            loading="lazy"
          />
        ) : (
          <div class="w-full h-full flex items-center justify-center text-muted-foreground">
            <svg class="w-12 h-12 opacity-30" fill="currentColor" viewBox="0 0 20 20">
              <path fillRule="evenodd" d="M4 3a2 2 0 00-2 2v10a2 2 0 002 2h12a2 2 0 002-2V5a2 2 0 00-2-2H4zm12 12H4l4-8 3 6 2-4 3 6z" clipRule="evenodd" />
            </svg>
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
        <div class="absolute bottom-2 right-2 px-1.5 py-0.5 bg-black/70 text-white text-xs rounded">
          {formatUtils.formatDuration(recording.duration)}
        </div>

        {/* Selection checkbox */}
        {canDelete && (
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
        {recording.protected && (
          <div class="absolute top-2 right-2 px-1.5 py-0.5 bg-yellow-500/90 text-white text-xs rounded flex items-center gap-0.5">
            <svg class="w-3 h-3" fill="currentColor" viewBox="0 0 20 20">
              <path fillRule="evenodd" d="M5 9V7a5 5 0 0110 0v2a2 2 0 012 2v5a2 2 0 01-2 2H5a2 2 0 01-2-2v-5a2 2 0 012-2zm8-2v2H7V7a3 3 0 016 0z" clipRule="evenodd" />
            </svg>
          </div>
        )}
      </div>

      {/* Card info */}
      <div class="p-3">
        <div class="flex items-center justify-between mb-1">
          <span class="font-medium text-sm truncate" title={recording.stream}>
            {recording.stream || 'Unknown'}
          </span>
          <span class="text-xs text-muted-foreground ml-2 whitespace-nowrap">
            {recording.size || ''}
          </span>
        </div>
        <div class="text-xs text-muted-foreground mb-2">
          {formatUtils.formatDateTime(recording.start_time)}
        </div>

        {/* Detection badges */}
        {recording.detection_labels && recording.detection_labels.length > 0 && (
          <div class="flex flex-wrap gap-1 mb-2">
            {recording.detection_labels.map((det, idx) => (
              <span key={idx} class="badge-success text-xs" title={`${det.count} detection${det.count !== 1 ? 's' : ''}`}>
                {det.label}
                {det.count > 1 && <span class="ml-0.5 opacity-75">({det.count})</span>}
              </span>
            ))}
          </div>
        )}

        {/* Action buttons */}
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
            title="Download"
          >
            <svg class="w-4 h-4" fill="currentColor" viewBox="0 0 20 20">
              <path fillRule="evenodd" d="M3 17a1 1 0 011-1h12a1 1 0 110 2H4a1 1 0 01-1-1zm3.293-7.707a1 1 0 011.414 0L9 10.586V3a1 1 0 112 0v7.586l1.293-1.293a1 1 0 111.414 1.414l-3 3a1 1 0 01-1.414 0l-3-3a1 1 0 010-1.414z" clipRule="evenodd" />
            </svg>
          </button>
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
      </div>
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
  canDelete = true
}) {
  return (
    <div class="recordings-container bg-card text-card-foreground rounded-lg shadow overflow-hidden w-full">
      {/* Batch actions - only show if user can delete */}
      {canDelete && (
        <div class="batch-actions p-3 border-b border-border flex flex-wrap gap-2 items-center">
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
            class="btn-danger disabled:opacity-50 disabled:cursor-not-allowed"
            disabled={getSelectedCount() === 0}
            onClick={() => openDeleteModal('selected')}
          >
            Delete Selected
          </button>
          <button class="btn-danger" onClick={() => openDeleteModal('all')}>
            Delete All Filtered
          </button>
        </div>
      )}

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
            />
          ))
        )}
      </div>
    </div>
  );
}

