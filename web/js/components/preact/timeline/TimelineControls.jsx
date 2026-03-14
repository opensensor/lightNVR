/**
 * LightNVR Timeline Controls Component
 * Handles play/pause and zoom controls for the timeline
 */

import { useState, useEffect } from 'preact/hooks';
import { timelineState } from './TimelinePage.jsx';
import { showStatusMessage } from '../ToastContainer.jsx';
import { TagIcon, TagsOverlay } from '../recordings/TagsOverlay.jsx';
import {
  findContainingSegmentIndex,
  findNearestSegmentIndex,
  formatPlaybackTimeLabel,
  getTimelineDayLengthHours,
  MIN_TIMELINE_VIEW_HOURS,
  resolveActiveSegmentIndex,
  resolvePlaybackStreamName,
  timestampToTimelineOffset,
  zoomTimelineRange
} from './timelineUtils.js';
import { useI18n } from '../../../i18n.js';

/**
 * TimelineControls component
 * @returns {JSX.Element} TimelineControls component
 */
export function TimelineControls() {
  const { t } = useI18n();
  const [isPlaying, setIsPlaying] = useState(false);
  const [canZoomIn, setCanZoomIn] = useState(true);
  const [canZoomOut, setCanZoomOut] = useState(true);
  const [timeDisplayText, setTimeDisplayText] = useState('00:00:00');
  const [activeSegmentIndex, setActiveSegmentIndex] = useState(-1);
  const [segmentCount, setSegmentCount] = useState(0);
  const [currentRecordingId, setCurrentRecordingId] = useState(null);
  const [isProtected, setIsProtected] = useState(false);
  const [recordingTags, setRecordingTags] = useState([]);
  const [showTagsOverlay, setShowTagsOverlay] = useState(false);

  useEffect(() => {
    const syncControlsState = (state) => {
      setIsPlaying(state.isPlaying);
      setSegmentCount(state.timelineSegments?.length || 0);
      setActiveSegmentIndex(resolveActiveSegmentIndex(
        state.timelineSegments,
        state.currentSegmentIndex,
        state.currentTime
      ));
      setCurrentRecordingId(state.currentRecordingId ?? null);
      setIsProtected(!!state.currentRecordingProtected);
      setRecordingTags(Array.isArray(state.currentRecordingTags) ? state.currentRecordingTags : []);
      const dayLengthHours = getTimelineDayLengthHours(state.selectedDate);
      const range = (state.timelineEndHour ?? dayLengthHours) - (state.timelineStartHour ?? 0);
      setCanZoomIn(range > MIN_TIMELINE_VIEW_HOURS);
      setCanZoomOut(range < dayLengthHours);

      const streamName = resolvePlaybackStreamName(
        state.timelineSegments,
        state.currentSegmentIndex,
        state.currentTime
      );
      const nextTimeDisplayText = formatPlaybackTimeLabel(state.currentTime, streamName);
      setTimeDisplayText(nextTimeDisplayText || '00:00:00');
    };

    syncControlsState(timelineState);

    const unsubscribe = timelineState.subscribe(syncControlsState);
    return () => unsubscribe();
  }, []);

  useEffect(() => {
    setShowTagsOverlay(false);
  }, [currentRecordingId]);

  // Toggle playback (play/pause)
  const togglePlayback = () => {
    if (isPlaying) {
      pausePlayback();
    } else {
      resumePlayback();
    }
  };

  // Pause playback
  const pausePlayback = () => {
    timelineState.setState({ isPlaying: false });
    const videoPlayer = document.querySelector('#video-player video');
    if (videoPlayer) {
      videoPlayer.pause();
    }
  };

  // Resume playback — finds the right segment for the current cursor position and
  // updates state.  TimelinePlayer.handleVideoPlayback detects the change and drives
  // the actual video loading / seeking / play() call, so there is no need to touch
  // the video element here (which previously caused race conditions and a
  // play→pause→play loop via the checkPlayback polling).
  const resumePlayback = () => {
    if (!timelineState.timelineSegments || timelineState.timelineSegments.length === 0) {
      showStatusMessage(t('timeline.noRecordingsToPlay'), 'warning');
      return;
    }

    let segmentIndex = -1;
    let segmentToPlay = null;
    let relativeTime = 0;

    if (timelineState.currentTime !== null) {
      const containingIndex = findContainingSegmentIndex(
        timelineState.timelineSegments,
        timelineState.currentTime
      );
      if (containingIndex !== -1) {
        segmentIndex = containingIndex;
        segmentToPlay = timelineState.timelineSegments[containingIndex];
        relativeTime = timelineState.currentTime - segmentToPlay.start_timestamp;
      } else {
        const closestIndex = findNearestSegmentIndex(
          timelineState.timelineSegments,
          timelineState.currentTime
        );
        segmentIndex = closestIndex;
        segmentToPlay = timelineState.timelineSegments[closestIndex];
        relativeTime = Math.max(
          0,
          Math.min(
            timelineState.currentTime - segmentToPlay.start_timestamp,
            segmentToPlay.end_timestamp - segmentToPlay.start_timestamp
          )
        );
      }
    } else if (
      timelineState.currentSegmentIndex >= 0 &&
      timelineState.currentSegmentIndex < timelineState.timelineSegments.length
    ) {
      segmentIndex = timelineState.currentSegmentIndex;
      segmentToPlay = timelineState.timelineSegments[segmentIndex];
      relativeTime = 0;
    } else {
      segmentIndex = 0;
      segmentToPlay = timelineState.timelineSegments[0];
      relativeTime = 0;
    }

    timelineState.setState({
      isPlaying: true,
      currentSegmentIndex: segmentIndex,
      currentTime: segmentToPlay.start_timestamp + relativeTime,
      prevCurrentTime: timelineState.currentTime,
      forceReload: true,
    });
  };

  // ── Helper: get the center hour for zoom (cursor position or range midpoint) ──
  const getCenter = () => {
    const s = timelineState.timelineStartHour ?? 0;
    const dayLengthHours = getTimelineDayLengthHours(timelineState.selectedDate);
    const e = timelineState.timelineEndHour ?? dayLengthHours;
    if (timelineState.currentTime !== null) {
      const h = timestampToTimelineOffset(timelineState.currentTime, timelineState.selectedDate);
      // Only use cursor if it's inside the current view
      if (h !== null && h >= s && h <= e) return h;
    }
    return (s + e) / 2;
  };

  // Zoom in — halve the visible range, centered on cursor
  const zoomIn = () => {
    const s = timelineState.timelineStartHour ?? 0;
    const dayLengthHours = getTimelineDayLengthHours(timelineState.selectedDate);
    const e = timelineState.timelineEndHour ?? dayLengthHours;
    const range = e - s;
    if (range <= MIN_TIMELINE_VIEW_HOURS) return;
    const center = getCenter();
    const nextRange = zoomTimelineRange(s, e, 0.5, center, dayLengthHours);
    timelineState.setState({
      timelineStartHour: nextRange.startHour,
      timelineEndHour: nextRange.endHour
    });
  };

  // Zoom out — double the visible range, centered on cursor, capped at 0-24
  const zoomOut = () => {
    const s = timelineState.timelineStartHour ?? 0;
    const dayLengthHours = getTimelineDayLengthHours(timelineState.selectedDate);
    const e = timelineState.timelineEndHour ?? dayLengthHours;
    const range = e - s;
    if (range >= dayLengthHours) return;
    const center = getCenter();
    const nextRange = zoomTimelineRange(s, e, 2, center, dayLengthHours);
    timelineState.setState({
      timelineStartHour: nextRange.startHour,
      timelineEndHour: nextRange.endHour
    });
  };

  // Fit — reset to the auto-fit range computed on data load
  const fitToSegments = () => {
    const fs = timelineState.autoFitStartHour ?? 0;
    const fe = timelineState.autoFitEndHour ?? getTimelineDayLengthHours(timelineState.selectedDate);
    timelineState.setState({ timelineStartHour: fs, timelineEndHour: fe });
  };

  const jumpToAdjacentSegment = (direction) => {
    const segments = timelineState.timelineSegments;
    if (!Array.isArray(segments) || segments.length === 0) {
      showStatusMessage(t('timeline.noRecordingsToNavigate'), 'warning');
      return;
    }

    const currentIndex = resolveActiveSegmentIndex(
      timelineState.timelineSegments,
      timelineState.currentSegmentIndex,
      timelineState.currentTime
    );
    if (currentIndex === -1) {
      showStatusMessage(t('timeline.noActiveRecordingSelected'), 'warning');
      return;
    }

    const targetIndex = currentIndex + direction;
    if (targetIndex < 0 || targetIndex >= segments.length) {
      return;
    }

    const targetSegment = segments[targetIndex];
    timelineState.setState({
      currentSegmentIndex: targetIndex,
      currentTime: targetSegment.start_timestamp,
      prevCurrentTime: timelineState.currentTime,
      isPlaying: timelineState.isPlaying,
      forceReload: true
    });
  };

  const canJumpBackward = activeSegmentIndex > 0;
  const canJumpForward = activeSegmentIndex !== -1 && activeSegmentIndex < segmentCount - 1;

  const handleToggleProtection = async () => {
    if (!currentRecordingId) return;

    const newState = !isProtected;
    try {
      const response = await fetch(`/api/recordings/${currentRecordingId}/protect`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ protected: newState }),
      });

      if (!response.ok) {
        throw new Error(`Failed to ${newState ? 'protect' : 'unprotect'} recording`);
      }

      timelineState.setState({ currentRecordingProtected: newState });
      showStatusMessage(
        newState ? t('recordings.recordingProtected') : t('recordings.recordingProtectionRemoved'),
        'success'
      );
    } catch (error) {
      console.error('Error toggling protection:', error);
      showStatusMessage(t('recordings.errorMessage', { message: error.message }), 'error');
    }
  };

  const handleTagsChanged = (_id, newTags) => {
    timelineState.setState({ currentRecordingTags: newTags });
  };

  return (
    <div className="timeline-controls flex flex-wrap justify-between items-center gap-2 mb-1">
      <div className="flex items-center gap-1.5">
        <button
          id="play-button"
          className={`w-7 h-7 rounded-full flex items-center justify-center focus:outline-none transition-colors shadow-sm ${
            isPlaying
              ? 'bg-red-600 hover:bg-red-700'
              : 'bg-green-600 hover:bg-green-700'
          }`}
          onClick={togglePlayback}
          title={isPlaying ? t('timeline.pause') : t('timeline.playFromCurrentPosition')}
        >
          <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            {isPlaying ? (
              <>
                <rect x="6" y="6" width="4" height="12" rx="1" fill="white" />
                <rect x="14" y="6" width="4" height="12" rx="1" fill="white" />
              </>
            ) : (
              <path d="M8 5.14v14l11-7-11-7z" fill="white" />
            )}
          </svg>
        </button>
        <span className="text-[11px] text-muted-foreground">{t('timeline.playFromCursor')}</span>
      </div>

      {/* Current time display */}
      <div className="flex items-center gap-1">
        <button
          type="button"
          className="w-6 h-6 rounded bg-secondary text-secondary-foreground hover:bg-secondary/80 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center focus:outline-none focus:ring-1 focus:ring-primary transition-colors"
          onClick={() => jumpToAdjacentSegment(-1)}
          title={t('timeline.previousRecording')}
          aria-label={t('timeline.previousRecording')}
          disabled={!canJumpBackward}
        >
          <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M15 19l-7-7 7-7" />
          </svg>
        </button>
        <div id="time-display"
          className="timeline-time-display bg-secondary text-foreground px-2 py-0.5 rounded font-mono text-xs tabular-nums border border-border min-w-[140px] text-center">
          {timeDisplayText}
        </div>
        {currentRecordingId && (
          <button
            type="button"
            className={`w-6 h-6 rounded border flex items-center justify-center focus:outline-none focus:ring-1 focus:ring-primary transition-colors ${
              isProtected
                ? 'border-yellow-500 bg-yellow-500 text-white hover:bg-yellow-600'
                : 'border-border bg-secondary text-secondary-foreground hover:bg-secondary/80'
            }`}
            onClick={handleToggleProtection}
            title={isProtected ? t('recordings.unprotect') : t('recordings.protect')}
            aria-label={isProtected ? t('recordings.unprotect') : t('recordings.protect')}
            aria-pressed={isProtected}
          >
            <svg xmlns="http://www.w3.org/2000/svg" className="h-3.5 w-3.5" fill="none" viewBox="0 0 24 24" stroke="currentColor">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M12 3l7 4v5c0 4.3-2.9 8.2-7 9-4.1-.8-7-4.7-7-9V7l7-4z" />
            </svg>
          </button>
        )}
        {currentRecordingId && (
          <div className="relative inline-block">
            <button
              type="button"
              className="w-6 h-6 rounded bg-secondary text-secondary-foreground hover:bg-secondary/80 flex items-center justify-center focus:outline-none focus:ring-1 focus:ring-primary transition-colors"
              onClick={() => setShowTagsOverlay(!showTagsOverlay)}
              title={t('recordings.manageTags')}
              aria-label={recordingTags.length > 0 ? t('timeline.manageRecordingTagsCount', { count: recordingTags.length }) : t('recordings.manageTags')}
            >
              <TagIcon className="w-3.5 h-3.5" />
              {recordingTags.length > 0 && (
                <span className="absolute -top-1 -right-1 min-w-[14px] h-[14px] px-0.5 rounded-full bg-primary text-primary-foreground text-[9px] leading-[14px] text-center">
                  {recordingTags.length}
                </span>
              )}
            </button>
            {showTagsOverlay && (
              <TagsOverlay
                recording={{ id: currentRecordingId, tags: recordingTags }}
                onClose={() => setShowTagsOverlay(false)}
                onTagsChanged={handleTagsChanged}
              />
            )}
          </div>
        )}
        <button
          type="button"
          className="w-6 h-6 rounded bg-secondary text-secondary-foreground hover:bg-secondary/80 disabled:opacity-50 disabled:cursor-not-allowed flex items-center justify-center focus:outline-none focus:ring-1 focus:ring-primary transition-colors"
          onClick={() => jumpToAdjacentSegment(1)}
          title={t('timeline.nextRecording')}
          aria-label={t('timeline.nextRecording')}
          disabled={!canJumpForward}
        >
          <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M9 5l7 7-7 7" />
          </svg>
        </button>
      </div>

      <div className="flex items-center gap-1">
        <button
          className="px-2 h-6 rounded text-xs bg-secondary text-secondary-foreground hover:bg-secondary/80 focus:outline-none focus:ring-1 focus:ring-primary transition-colors"
          onClick={fitToSegments}
          title={t('timeline.fitToRecordings')}
        >
          {t('timeline.fit')}
        </button>
        <button
          id="zoom-out-button"
          className="w-6 h-6 rounded bg-secondary text-secondary-foreground hover:bg-secondary/80 flex items-center justify-center focus:outline-none focus:ring-1 focus:ring-primary transition-colors"
          onClick={zoomOut}
          title={t('timeline.zoomOut')}
          disabled={!canZoomOut}
        >
          <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M15 12H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </button>
        <button
          id="zoom-in-button"
          className="w-6 h-6 rounded bg-secondary text-secondary-foreground hover:bg-secondary/80 flex items-center justify-center focus:outline-none focus:ring-1 focus:ring-primary transition-colors"
          onClick={zoomIn}
          title={t('timeline.zoomIn')}
          disabled={!canZoomIn}
        >
          <svg xmlns="http://www.w3.org/2000/svg" className="h-4 w-4" fill="none" viewBox="0 0 24 24" stroke="currentColor">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M12 9v3m0 0v3m0-3h3m-3 0H9m12 0a9 9 0 11-18 0 9 9 0 0118 0z" />
          </svg>
        </button>
      </div>
    </div>
  );
}
