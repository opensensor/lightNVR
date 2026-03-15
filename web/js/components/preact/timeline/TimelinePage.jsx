/**
 * LightNVR Timeline Page Component
 * Main component for the timeline view
 */

import { useState, useEffect, useRef, useCallback, useMemo } from 'preact/hooks';
import { TimelineControls } from './TimelineControls.jsx';
import { TimelineRuler } from './TimelineRuler.jsx';
import { TimelineSegments } from './TimelineSegments.jsx';
import { TimelineCursor } from './TimelineCursor.jsx';
import { TimelinePlayer } from './TimelinePlayer.jsx';
import { CalendarPicker } from './CalendarPicker.jsx';
import { BatchDownloadModal } from '../BatchDownloadModal.jsx';
import { showStatusMessage } from '../ToastContainer.jsx';
import { LoadingIndicator } from '../LoadingIndicator.jsx';
import { useQuery } from '../../../query-client.js';
import { useI18n } from '../../../i18n.js';
import {
  currentDateInputValue,
  formatDateForInput,
  formatDisplayDate,
  getLocalDayIsoRange,
  nowMilliseconds
} from '../../../utils/date-utils.js';
import {
  countSegmentsForDate,
  findFirstVisibleSegmentIndex,
  findContainingSegmentIndex,
  formatTimestampAsLocalDate,
  getSteppedVideoTime,
  getAvailableDatesForSegments,
  getClippedSegmentHourRange,
  getLocalDayBounds,
  getTimelineDayLengthHours,
  localClockTimeToTimestamp,
  panTimelineRange,
  resolveActiveSegmentIndex,
  timelineOffsetToTimestamp,
  timestampToTimelineOffset,
  zoomTimelineRange
} from './timelineUtils.js';

const RECORDINGS_RETURN_URL_KEY = 'lightnvr_recordings_return_url';

// Convert an elapsed timeline offset → Unix timestamp (seconds) for the selected local day.
function timelineHourToTimestamp(hour, selectedDate) {
  return timelineOffsetToTimestamp(hour, selectedDate);
}

// Convert a Unix timestamp to an elapsed timeline offset for the selected local day.
function timestampToTimelineHour(timestamp, selectedDate = null) {
  return timestampToTimelineOffset(timestamp, selectedDate);
}

function isEditableKeyboardTarget(target) {
  if (!(target instanceof Element)) {
    return false;
  }

  const tagName = target.tagName;
  return target.isContentEditable ||
    tagName === 'INPUT' ||
    tagName === 'TEXTAREA' ||
    tagName === 'SELECT';
}

// Global timeline state for child components
const timelineState = {
  streams: [],
  timelineSegments: [],
  selectedStream: null,
  selectedDate: null,
  currentRecordingId: null,
  currentRecordingProtected: false,
  currentRecordingTags: [],
  isPlaying: false,
  currentSegmentIndex: -1,
  timelineStartHour: 0,
  timelineEndHour: 24,
  autoFitStartHour: 0,   // auto-fit range computed from segments
  autoFitEndHour: 24,
  currentTime: null,
  prevCurrentTime: null,
  playbackSpeed: 1.0,
  forceReload: false,
  userControllingCursor: false, // New flag to track if user is controlling cursor
  preserveCursorPosition: false, // New flag to explicitly preserve cursor position
  cursorPositionLocked: false, // New flag to lock the cursor position during playback
  // Utility functions for time conversion
  timelineHourToTimestamp,
  timestampToTimelineHour,
  listeners: new Set(),

  // Last time state was updated
  lastUpdateTime: 0,

  // Pending state updates
  pendingUpdates: {},

  // Prevent re-entrant listener notifications from overlapping state mutations.
  isNotifying: false,

  // Update state and notify listeners
  setState(newState) {
    const now = nowMilliseconds();
    const isTimeOnlyUpdate = newState.currentTime !== undefined &&
      newState.currentSegmentIndex === undefined &&
      // Playback updates need to propagate immediately to keep the cursor and
      // currently playing segment synchronized with the video element.
      newState.isPlaying !== true;

    // Batch frequent time-only updates (≤250 ms apart)
    if (isTimeOnlyUpdate && now - this.lastUpdateTime < 250) {
      return;
    }

    if (this.isNotifying) {
      this.pendingUpdates = { ...this.pendingUpdates, ...newState };
      return;
    }

    Object.assign(this, newState);

    if (newState.forceReload) {
      this.forceReload = false;
    }

    this.lastUpdateTime = now;

    this.isNotifying = true;
    try {
      this.notifyListeners();
    } finally {
      this.isNotifying = false;
    }
  },

  // Subscribe to state changes
  subscribe(listener) {
    this.listeners.add(listener);
    return () => this.listeners.delete(listener);
  },

  // Notify all listeners of state change
  notifyListeners() {
    this.listeners.forEach(listener => listener(this));
  },

  // Flush any pending updates
  flushPendingUpdates() {
    if (!this.isNotifying && Object.keys(this.pendingUpdates).length > 0) {
      const pendingUpdates = this.pendingUpdates;
      this.pendingUpdates = {};
      this.setState(pendingUpdates);
    }
  }
};

/**
 * Parse URL parameters
 */
function parseUrlParams() {
  const params = new URLSearchParams(window.location.search);
  return {
    stream: params.get('stream') || '',
    date: params.get('date') || currentDateInputValue(),
    time: params.get('time') || '',  // Optional HH:MM:SS to auto-seek on load
    ids: params.get('ids') || ''     // Comma-separated recording IDs for selected-recordings mode
  };
}

/**
 * Update URL parameters
 */
function updateUrlParams(stream, date) {
  if (!stream) return;
  const url = new URL(window.location.href);
  url.searchParams.set('stream', stream);
  url.searchParams.set('date', date);
  url.searchParams.delete('time');  // Remove one-time seek param after initial load
  window.history.replaceState({}, '', url);
}

/**
 * Collapsible help panel — replaces the old large static instructions block.
 */
function TimelineHelp({ idsMode, t }) {
  const [open, setOpen] = useState(false);
  return (
    <div className="mt-3">
      <button
        className="flex items-center gap-1 text-xs text-muted-foreground hover:text-foreground transition-colors"
        onClick={() => setOpen(!open)}
      >
        <svg className="w-3.5 h-3.5" fill="none" stroke="currentColor" viewBox="0 0 24 24">
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2}
            d="M13 16h-1v-4h-1m1-4h.01M12 2a10 10 0 100 20 10 10 0 000-20z" />
        </svg>
        {open ? t('timeline.hideHelp') : t('timeline.howToUseTimeline')}
      </button>
      {open && (
        <ul className="mt-1.5 ml-5 text-xs text-muted-foreground list-disc space-y-0.5">
          {!idsMode && <li>{t('timeline.help.selectStreamAndDate')}</li>}
          {idsMode && <li>{t('timeline.help.refineSelections')}</li>}
          <li>{t('timeline.help.positionCursor')}</li>
          <li>{t('timeline.help.dragPlayhead')}</li>
          <li>{t('timeline.help.clickSegment')}</li>
          <li>{t('timeline.help.arrowKeys')}</li>
          <li>{t('timeline.help.playFromCursor')}</li>
          <li>{t('timeline.help.zoomControls')}</li>
          <li>{t('timeline.help.panControls')}</li>
          <li>{t('timeline.help.actions')}</li>
        </ul>
      )}
    </div>
  );
}

/**
 * TimelinePage component
 */
export function TimelinePage() {
  const { t } = useI18n();
  // Get initial values from URL parameters
  const urlParams = parseUrlParams();

  // Detect selected-recordings mode
  const idsMode = urlParams.ids.length > 0;
  const recordingIds = idsMode ? urlParams.ids : '';

  // State
  const [streamsList, setStreamsList] = useState([]);
  const [selectedStream, setSelectedStream] = useState(urlParams.stream);
  const [selectedDate, setSelectedDate] = useState(urlParams.date);
  const [segments, setSegments] = useState([]);
  const [idsLoading, setIdsLoading] = useState(idsMode);
  const [idsSegmentInfo, setIdsSegmentInfo] = useState(null);  // metadata from IDs endpoint
  const [idsTimelineSegments, setIdsTimelineSegments] = useState([]);
  const [isDownloadModalOpen, setIsDownloadModalOpen] = useState(false);
  const [keyboardNavigationMode, setKeyboardNavigationMode] = useState('broad');

  // Refs
  const timelineContainerRef = useRef(null);
  const initialLoadRef = useRef(false);
  const flushIntervalRef = useRef(null);
  const initialTimeRef = useRef(urlParams.time);  // Store initial time param for auto-seek
  const processedDataRef = useRef(null);  // Track last processed timeline data
  const selectedDateRef = useRef(urlParams.date);
  const videoElementRef = useRef(null);

  useEffect(() => {
    selectedDateRef.current = selectedDate;
  }, [selectedDate]);

  const jumpToAdjacentSegment = useCallback((direction) => {
    const activeIndex = resolveActiveSegmentIndex(
      timelineState.timelineSegments,
      timelineState.currentSegmentIndex,
      timelineState.currentTime
    );

    if (activeIndex === -1) {
      return false;
    }

    const targetIndex = activeIndex + direction;
    const targetSegment = timelineState.timelineSegments?.[targetIndex];
    if (!targetSegment) {
      return true;
    }

    timelineState.setState({
      currentSegmentIndex: targetIndex,
      currentTime: targetSegment.start_timestamp,
      prevCurrentTime: timelineState.currentTime,
      isPlaying: timelineState.isPlaying,
      forceReload: true
    });
    return true;
  }, []);

  const seekCurrentVideo = useCallback((directionSeconds) => {
    const videoPlayer = videoElementRef.current;
    if (!(videoPlayer instanceof HTMLVideoElement)) {
      return false;
    }

    const activeIndex = resolveActiveSegmentIndex(
      timelineState.timelineSegments,
      timelineState.currentSegmentIndex,
      timelineState.currentTime
    );
    const activeSegment = timelineState.timelineSegments?.[activeIndex];
    if (!activeSegment) {
      return false;
    }

    // Backward auto-advance: if seeking before the start of this segment, jump to the
    // end of the previous segment (mirrors how forward auto-advance works via handleEnded).
    if (directionSeconds < 0 && videoPlayer.currentTime + directionSeconds < 0) {
      const prevIndex = activeIndex - 1;
      const prevSegment = timelineState.timelineSegments?.[prevIndex];
      if (prevSegment) {
        const prevDuration = prevSegment.end_timestamp - prevSegment.start_timestamp;
        // Land 1 second before the end (magnitude of directionSeconds), clamped to [0, duration].
        const seekOffset = Math.max(prevDuration + directionSeconds, 0);
        timelineState.setState({
          currentSegmentIndex: prevIndex,
          currentTime: prevSegment.start_timestamp + seekOffset,
          prevCurrentTime: timelineState.currentTime,
          isPlaying: timelineState.isPlaying,
          forceReload: true
        });
      }
      // Return true whether or not there is a previous segment, so the browser
      // does not scroll the page on the key press.
      return true;
    }

    const fallbackDuration = Math.max(activeSegment.end_timestamp - activeSegment.start_timestamp, 0);
    const effectiveDuration = Number.isFinite(videoPlayer.duration) && videoPlayer.duration > 0
      ? videoPlayer.duration
      : fallbackDuration;

    // Forward auto-advance: if seeking past the end of this segment, jump to
    // the beginning of the next segment (mirrors backward auto-advance above).
    if (directionSeconds > 0 && videoPlayer.currentTime + directionSeconds >= effectiveDuration) {
      const nextIndex = activeIndex + 1;
      const nextSegment = timelineState.timelineSegments?.[nextIndex];
      if (nextSegment) {
        timelineState.setState({
          currentSegmentIndex: nextIndex,
          currentTime: nextSegment.start_timestamp,
          prevCurrentTime: timelineState.currentTime,
          isPlaying: timelineState.isPlaying,
          forceReload: true
        });
      }
      // Return true whether or not there is a next segment, so the browser
      // does not scroll the page on the key press.
      return true;
    }

    const nextTime = getSteppedVideoTime(videoPlayer.currentTime, directionSeconds, effectiveDuration);

    try {
      videoPlayer.currentTime = nextTime;
    } catch (error) {
      console.warn('TimelinePage: unable to seek current video from keyboard navigation', error);
      return false;
    }

    timelineState.setState({
      currentSegmentIndex: activeIndex,
      currentTime: activeSegment.start_timestamp + nextTime,
      prevCurrentTime: timelineState.currentTime,
      isPlaying: timelineState.isPlaying
    });
    return true;
  }, []);

  useEffect(() => {
    const handlePointerDown = (event) => {
      const target = event.target instanceof Element ? event.target : null;
      // Controls marked with this attribute (speed buttons, action bar, etc.) should
      // not exit 'fine' mode — the user is adjusting playback settings, not leaving
      // the video player context.
      if (target?.closest('[data-keyboard-nav-preserve]')) {
        return;
      }
      const nextMode = target?.closest('#video-player') ? 'fine' : 'broad';
      setKeyboardNavigationMode((prevMode) => prevMode === nextMode ? prevMode : nextMode);
    };

    document.addEventListener('pointerdown', handlePointerDown, true);
    return () => {
      document.removeEventListener('pointerdown', handlePointerDown, true);
    };
  }, []);

  useEffect(() => {
    const handleKeyDown = (event) => {
      if (event.defaultPrevented || event.altKey || event.ctrlKey || event.metaKey) {
        return;
      }

      if (isEditableKeyboardTarget(event.target)) {
        return;
      }

      // Space: toggle play/pause on the active video regardless of which element
      // has focus. The native video controls also handle space, but only when the
      // video element itself is focused; this handler makes it global.
      if (event.key === ' ') {
        const videoPlayer = videoElementRef.current;
        if (videoPlayer) {
          // When the video element itself has focus the browser's native controls
          // already handle the space key — if we also toggle here the two actions
          // cancel each other out.  Prevent the default scroll but let the native
          // control do the toggling.
          if (event.target === videoPlayer) {
            event.preventDefault();
            return;
          }
          event.preventDefault();
          if (videoPlayer.paused) {
            videoPlayer.play().catch(e => {
              if (e.name !== 'AbortError') console.warn('Space play error:', e);
            });
          } else {
            videoPlayer.pause();
          }
        }
        return;
      }

      if (event.key !== 'ArrowLeft' && event.key !== 'ArrowRight') {
        return;
      }

      const direction = event.key === 'ArrowLeft' ? -1 : 1;
      const handled = keyboardNavigationMode === 'fine'
        ? seekCurrentVideo(direction * (timelineState.playbackSpeed || 1))
        : jumpToAdjacentSegment(direction);

      if (handled) {
        event.preventDefault();
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    return () => {
      document.removeEventListener('keydown', handleKeyDown);
    };
  }, [jumpToAdjacentSegment, keyboardNavigationMode, seekCurrentVideo, videoElementRef]);

  useEffect(() => {
    const container = timelineContainerRef.current;
    if (!container) {
      return undefined;
    }

    const handleWheel = (event) => {
      const startHour = timelineState.timelineStartHour ?? 0;
      const endHour = timelineState.timelineEndHour ?? getTimelineDayLengthHours(timelineState.selectedDate);
      const currentRange = endHour - startHour;
      if (currentRange <= 0) {
        return;
      }

      const rect = container.getBoundingClientRect();
      if (rect.width <= 0) {
        return;
      }

      if (event.ctrlKey || event.metaKey) {
        event.preventDefault();
        const pointerRatio = Math.min(Math.max((event.clientX - rect.left) / rect.width, 0), 1);
        const anchorHour = startHour + (pointerRatio * currentRange);
        const zoomFactor = event.deltaY < 0 ? 0.8 : 1.25;
        const nextRange = zoomTimelineRange(startHour, endHour, zoomFactor, anchorHour, getTimelineDayLengthHours(timelineState.selectedDate));
        timelineState.setState({
          timelineStartHour: nextRange.startHour,
          timelineEndHour: nextRange.endHour
        });
        return;
      }

      const horizontalDelta = Math.abs(event.deltaX) > Math.abs(event.deltaY)
        ? event.deltaX
        : (event.shiftKey ? event.deltaY : 0);

      if (horizontalDelta !== 0) {
        event.preventDefault();
        const deltaHours = (horizontalDelta / rect.width) * currentRange;
        const nextRange = panTimelineRange(startHour, endHour, deltaHours, getTimelineDayLengthHours(timelineState.selectedDate));
        timelineState.setState({
          timelineStartHour: nextRange.startHour,
          timelineEndHour: nextRange.endHour
        });
      }
    };

    container.addEventListener('wheel', handleWheel, { passive: false });
    return () => {
      container.removeEventListener('wheel', handleWheel);
    };
  }, [segments.length]);

  const idsAvailableDates = useMemo(() => (
    idsMode ? getAvailableDatesForSegments(idsTimelineSegments) : []
  ), [idsMode, idsTimelineSegments]);

  const idsSelectedDayIndex = idsAvailableDates.indexOf(selectedDate);
  const idsVisibleSegmentCount = useMemo(() => (
    idsMode ? countSegmentsForDate(idsTimelineSegments, selectedDate) : 0
  ), [idsMode, idsTimelineSegments, selectedDate]);
  const selectedIdsList = useMemo(() => (
    idsMode ? recordingIds.split(',').map(id => id.trim()).filter(Boolean) : []
  ), [idsMode, recordingIds]);
  const selectedRecordingMap = useMemo(() => (
    selectedIdsList.reduce((acc, id) => {
      acc[String(id)] = true;
      return acc;
    }, {})
  ), [selectedIdsList]);
  const selectedRecordingsForDownload = useMemo(() => {
    if (!idsMode) return [];

    const seen = new Set();
    return idsTimelineSegments.filter(segment => {
      const key = String(segment.id);
      if (!selectedRecordingMap[key] || seen.has(key)) {
        return false;
      }
      seen.add(key);
      return true;
    });
  }, [idsMode, idsTimelineSegments, selectedRecordingMap]);

  const loadSegmentsIntoTimeline = useCallback((rawSegments, effectiveDate, options = {}) => {
    const { successMessage = null } = options;
    const segmentsCopy = JSON.parse(JSON.stringify(rawSegments || []));

    if (segmentsCopy.length === 0) {
      setSegments([]);
      timelineState.setState({
        timelineSegments: [],
        currentSegmentIndex: -1,
        currentTime: null,
        prevCurrentTime: null,
        isPlaying: false,
        selectedDate: effectiveDate
      });
      return false;
    }

    setSegments(segmentsCopy);

    const dayBounds = getLocalDayBounds(effectiveDate);
    const dayLengthHours = getTimelineDayLengthHours(effectiveDate);
    const visibleIndices = [];

    // Compute auto-fit range from segments for the selected day
    let earliest = dayLengthHours;
    let latest = 0;
    segmentsCopy.forEach((seg, index) => {
      const visibleRange = getClippedSegmentHourRange(seg, effectiveDate);
      if (!visibleRange) return;
      visibleIndices.push(index);
      earliest = Math.min(earliest, visibleRange.startHour);
      latest = Math.max(latest, visibleRange.endHour);
    });

    if (visibleIndices.length === 0) {
      timelineState.setState({
        timelineSegments: segmentsCopy,
        currentSegmentIndex: -1,
        currentTime: null,
        prevCurrentTime: null,
        isPlaying: false,
        selectedDate: effectiveDate,
        timelineStartHour: 0,
        timelineEndHour: dayLengthHours,
        autoFitStartHour: 0,
        autoFitEndHour: dayLengthHours
      });
      return false;
    }

    const findNearestVisibleIndex = (targetTimestamp) => {
      let bestIndex = -1;
      let bestDistance = Infinity;
      let bestStart = Infinity;

      visibleIndices.forEach(index => {
        const seg = segmentsCopy[index];
        let distance = 0;
        if (targetTimestamp < seg.start_timestamp) {
          distance = seg.start_timestamp - targetTimestamp;
        } else if (targetTimestamp > seg.end_timestamp) {
          distance = targetTimestamp - seg.end_timestamp;
        }

        if (distance < bestDistance || (distance === bestDistance && seg.start_timestamp < bestStart)) {
          bestDistance = distance;
          bestStart = seg.start_timestamp;
          bestIndex = index;
        }
      });

      return bestIndex;
    };

    // Determine initial time/segment (honour ?time= URL param)
    let initialTime = null;
    let initialSegmentIndex = -1;
    let preserveExistingPlayback = false;

    if (timelineState.currentTime !== null && timelineState.currentTime !== undefined) {
      const containingIndex = findContainingSegmentIndex(segmentsCopy, timelineState.currentTime);
      if (containingIndex !== -1 && getClippedSegmentHourRange(segmentsCopy[containingIndex], effectiveDate)) {
        initialSegmentIndex = containingIndex;
        initialTime = timelineState.currentTime;
        preserveExistingPlayback = true;
      }
    }

    if (initialSegmentIndex === -1 && initialTimeRef.current) {
      const seekTs = localClockTimeToTimestamp(initialTimeRef.current, effectiveDate);
      if (seekTs !== null) {

        const containingIndex = findContainingSegmentIndex(segmentsCopy, seekTs);
        if (containingIndex !== -1 && getClippedSegmentHourRange(segmentsCopy[containingIndex], effectiveDate)) {
          initialSegmentIndex = containingIndex;
          initialTime = seekTs;
        } else {
          const nearestIndex = findNearestVisibleIndex(seekTs);
          if (nearestIndex !== -1) {
            initialSegmentIndex = nearestIndex;
            initialTime = Math.max(segmentsCopy[nearestIndex].start_timestamp, dayBounds.startTimestamp);
          }
        }
      }
      initialTimeRef.current = '';
    }

    if (initialSegmentIndex === -1) {
      initialSegmentIndex = findFirstVisibleSegmentIndex(segmentsCopy, effectiveDate);
      if (initialSegmentIndex !== -1) {
        initialTime = Math.max(segmentsCopy[initialSegmentIndex].start_timestamp, dayBounds.startTimestamp);
      }
    }

    if (initialSegmentIndex === -1 || initialTime === null) {
      return false;
    }

    let fitStart = 0;
    let fitEnd = dayLengthHours;
    if (earliest < dayLengthHours && latest > 0) {
      const span = latest - earliest;
      const pad = Math.max(0.5, Math.min(1, span * 0.1));
      fitStart = Math.max(0, Math.floor((earliest - pad) * 2) / 2);
      fitEnd = Math.min(dayLengthHours, Math.ceil((latest + pad) * 2) / 2);
      if (fitEnd - fitStart < 2) {
        const center = (earliest + latest) / 2;
        fitStart = Math.max(0, Math.floor((center - 1) * 2) / 2);
        fitEnd = Math.min(dayLengthHours, fitStart + 2);
      }
    }

    // Push to global state
    timelineState.timelineSegments = segmentsCopy;
    timelineState.currentSegmentIndex = initialSegmentIndex;
    timelineState.currentTime = initialTime;
    timelineState.prevCurrentTime = initialTime;
    if (!preserveExistingPlayback) {
      timelineState.isPlaying = false;
    }
    timelineState.forceReload = !preserveExistingPlayback;
    timelineState.autoFitStartHour = fitStart;
    timelineState.autoFitEndHour = fitEnd;
    timelineState.timelineStartHour = fitStart;
    timelineState.timelineEndHour = fitEnd;
    timelineState.selectedDate = effectiveDate;
    timelineState.setState({});

    if (!preserveExistingPlayback) {
      // Safety net: retry if state didn't stick
      setTimeout(() => {
        if (!timelineState.currentTime || timelineState.currentSegmentIndex === -1) {
          timelineState.setState({
            currentSegmentIndex: initialSegmentIndex,
            currentTime: initialTime,
            prevCurrentTime: initialTime,
            selectedDate: effectiveDate
          });
        }
      }, 100);

      // Preload the initial segment's video for this day
      const videoPlayer = videoElementRef.current;
      if (videoPlayer instanceof HTMLVideoElement) {
        videoPlayer.src = `/api/recordings/play/${segmentsCopy[initialSegmentIndex].id}?t=${nowMilliseconds()}`;
        videoPlayer.load();
      }
    }

    if (successMessage) {
      showStatusMessage(successMessage, 'success');
    }
    return true;
  }, []);

  // Set up periodic flush of pending updates
  useEffect(() => {
    // Set up interval to flush pending updates every 200ms
    flushIntervalRef.current = setInterval(() => {
      timelineState.flushPendingUpdates();
    }, 200);

    // Clean up interval on unmount
    return () => {
      if (flushIntervalRef.current) {
        clearInterval(flushIntervalRef.current);
      }
    };
  }, []);

  // Load streams using preact-query
  const {
    data: streamsData,
    error: streamsError
  } = useQuery('streams', '/api/streams', {
    timeout: 15000, // 15 second timeout
    retries: 2,     // Retry twice
    retryDelay: 1000 // 1 second between retries
  });

  // Handle initial data load when streams are available
  useEffect(() => {
    if (streamsData && Array.isArray(streamsData) && streamsData.length > 0 && !initialLoadRef.current) {
      console.log('TimelinePage: Streams loaded, initializing data');
      initialLoadRef.current = true;

      // Update streamsList state
      setStreamsList(streamsData);

      // Update global state for child components
      timelineState.setState({ streams: streamsData });

      // Check if the selected stream from URL exists
      const streamExists = streamsData.some(s => s.name === selectedStream);

      if (streamExists && selectedStream) {
        console.log(`TimelinePage: Using stream from URL: ${selectedStream}`);
      } else if (streamsData.length > 0) {
        // Use first stream if URL stream doesn't exist
        const firstStream = streamsData[0].name;
        console.log(`TimelinePage: Using first stream: ${firstStream}`);
        setSelectedStream(firstStream);
      }
    }
  }, [streamsData]);

  // Handle streams error
  useEffect(() => {
    if (streamsError) {
      console.error('TimelinePage: Error loading streams:', streamsError);
      showStatusMessage('Error loading streams: ' + streamsError.message, 'error');
    }
  }, [streamsError]);

  // IDs mode: fetch segments by recording IDs instead of stream+date
  useEffect(() => {
    if (!idsMode || !recordingIds) return;

    const fetchByIds = async () => {
      setIdsLoading(true);
      try {
        const resp = await fetch(`/api/timeline/segments-by-ids?ids=${encodeURIComponent(recordingIds)}`);
        if (!resp.ok) throw new Error(`HTTP ${resp.status}`);
        const data = await resp.json();
        setIdsSegmentInfo(data);

        const segs = (data.segments || []).sort((a, b) => a.start_timestamp - b.start_timestamp);
        if (segs.length > 0) {
          const availableDates = getAvailableDatesForSegments(segs);
          const anchorDate = availableDates.includes(selectedDateRef.current)
            ? selectedDateRef.current
            : (availableDates[0] || formatDateForInput(segs[0].start_timestamp));

          setIdsTimelineSegments(segs);
          if (anchorDate === selectedDateRef.current) {
            loadSegmentsIntoTimeline(segs, anchorDate);
          }
          if (anchorDate !== selectedDateRef.current) {
            setSelectedDate(anchorDate);
          }

          // Build a pseudo-stream name from unique streams
          const uniqueStreams = [...new Set(segs.map(s => s.stream))];
          setSelectedStream(uniqueStreams.join(', '));

          showStatusMessage(`Loaded ${segs.length} selected recording segments across ${availableDates.length} day${availableDates.length !== 1 ? 's' : ''}`, 'success');
        } else {
          setIdsTimelineSegments([]);
          loadSegmentsIntoTimeline([], selectedDate);
          showStatusMessage('No recordings found for the selected IDs', 'info');
        }
      } catch (err) {
        console.error('Error fetching segments by IDs:', err);
        setIdsTimelineSegments([]);
        showStatusMessage('Error loading selected recordings: ' + err.message, 'error');
      } finally {
        setIdsLoading(false);
      }
    };

    fetchByIds();
  }, [idsMode, recordingIds, loadSegmentsIntoTimeline]);

  useEffect(() => {
    if (!idsMode) return;
    if (idsTimelineSegments.length === 0) return;
    if (idsAvailableDates.length === 0) return;

    const effectiveDate = idsAvailableDates.includes(selectedDate)
      ? selectedDate
      : idsAvailableDates[0];

    if (effectiveDate !== selectedDate) {
      setSelectedDate(effectiveDate);
      return;
    }

    loadSegmentsIntoTimeline(idsTimelineSegments, effectiveDate);
  }, [idsMode, idsTimelineSegments, idsAvailableDates, selectedDate, loadSegmentsIntoTimeline]);

  // Calculate time range for timeline data
  const getTimeRange = (date) => {
    return getLocalDayIsoRange(date);
  };

  // Update URL and global state when stream or date changes
  useEffect(() => {
    if (idsMode) {
      const url = new URL(window.location.href);
      url.searchParams.set('date', selectedDate);
      url.searchParams.delete('time');
      window.history.replaceState({}, '', url);
      timelineState.setState({ selectedDate });
      return;
    }

    if (selectedStream) {
      updateUrlParams(selectedStream, selectedDate);
      timelineState.setState({ selectedStream, selectedDate });
    }
  }, [idsMode, selectedStream, selectedDate]);

  // Get time range for current date
  const { startTime, endTime } = getTimeRange(selectedDate);

  // Construct the URL for the API call
  const timelineUrl = selectedStream
    ? `/api/timeline/segments?stream=${encodeURIComponent(selectedStream)}&start=${encodeURIComponent(startTime)}&end=${encodeURIComponent(endTime)}`
    : null;

  // Fetch timeline segments using preact-query
  // NOTE: onSuccess/onError were removed in TanStack Query v5 (@preact-signals/query v2.x)
  // so we use a useEffect below to process the data instead.
  const {
    data: timelineData,
    isLoading: isLoadingTimeline,
    error: timelineError,
  } = useQuery(
    ['timeline-segments', selectedStream, selectedDate],
    timelineUrl,
    {
      timeout: 30000, // 30 second timeout
      retries: 2,     // Retry twice
      retryDelay: 1000 // 1 second between retries
    },
    {
      enabled: !!selectedStream && !idsMode // Only run query if we have a selected stream (and not in IDs mode)
    }
  );

  // Process timeline data when it arrives
  useEffect(() => {
    if (timelineError) {
      console.error('TimelinePage: Error loading timeline data:', timelineError.message);
      showStatusMessage('Error loading timeline data: ' + timelineError.message, 'error');
      setSegments([]);
      return;
    }

    if (!timelineData || timelineData === processedDataRef.current) return;
    processedDataRef.current = timelineData;

    const timelineSegments = timelineData.segments || [];

    if (timelineSegments.length === 0) {
      loadSegmentsIntoTimeline([], selectedDate);
      showStatusMessage('No recordings found for the selected date', 'warning');
      return;
    }

    loadSegmentsIntoTimeline(timelineSegments, selectedDate, {
      successMessage: `Loaded ${timelineSegments.length} recording segments`
    });
  }, [timelineData, timelineError, selectedDate, loadSegmentsIntoTimeline]);

  // Transparent polling: silently fetch for new recordings every 30 s (normal mode only)
  useEffect(() => {
    if (idsMode || !selectedStream || !timelineUrl) return;

    const POLL_INTERVAL_MS = 30000;

    const pollForNewRecordings = async () => {
      try {
        const response = await fetch(timelineUrl);
        if (!response.ok) return;
        const data = await response.json();
        const polledSegments = data.segments || [];
        if (polledSegments.length === 0) return;

        const currentSegs = timelineState.timelineSegments || [];
        const currentIds = new Set(currentSegs.map(s => String(s.id)));
        const addedSegs = polledSegments.filter(s => !currentIds.has(String(s.id)));

        if (addedSegs.length > 0) {
          const merged = [...currentSegs, ...addedSegs].sort((a, b) => a.start_timestamp - b.start_timestamp);
          setSegments(merged);
          timelineState.setState({ timelineSegments: merged });
          showStatusMessage(
            `${addedSegs.length} new recording${addedSegs.length !== 1 ? 's' : ''} added to timeline`,
            'info',
            3000
          );
        }
      } catch (err) {
        console.debug('Timeline poll error (suppressed):', err);
      }
    };

    const intervalId = setInterval(pollForNewRecordings, POLL_INTERVAL_MS);
    return () => clearInterval(intervalId);
  }, [idsMode, selectedStream, timelineUrl]);

  useEffect(() => {
    if (!idsMode || idsAvailableDates.length === 0) return undefined;

    return timelineState.subscribe(({ currentTime }) => {
      if (currentTime === null || currentTime === undefined) return;

      const playbackDate = formatTimestampAsLocalDate(currentTime);
      if (idsAvailableDates.includes(playbackDate) && playbackDate !== selectedDate) {
        setSelectedDate(prev => (prev === playbackDate ? prev : playbackDate));
      }
    });
  }, [idsMode, idsAvailableDates, selectedDate]);

  // When a recording is deleted from the timeline player, remove it from the
  // local segments list (creates the visual gap) and advance to the next segment.
  useEffect(() => {
    const handleTimelineDeleted = (e) => {
      const deletedId = e.detail.id;

      setSegments(prev => {
        const idx = prev.findIndex(s => s.id === deletedId);
        const updated = prev.filter(s => s.id !== deletedId);

        // Advance to the next segment (or the previous if it was the last)
        if (updated.length > 0) {
          const nextIdx = Math.min(idx, updated.length - 1);
          const nextSeg = updated[nextIdx];

          timelineState.setState({
            timelineSegments: updated,
            currentSegmentIndex: nextIdx,
            currentTime: nextSeg.start_timestamp,
            isPlaying: false,
            forceReload: true
          });

          // Load the next segment's video
          setTimeout(() => {
            const videoEl = videoElementRef.current;
            if (videoEl instanceof HTMLVideoElement) {
              videoEl.pause();
              videoEl.removeAttribute('src');
              videoEl.load();
              videoEl.src = `/api/recordings/play/${nextSeg.id}?t=${nowMilliseconds()}`;
              videoEl.load();
            }
          }, 100);
        } else {
          timelineState.setState({
            timelineSegments: [],
            currentSegmentIndex: -1,
            currentTime: null,
            isPlaying: false
          });
        }

        return updated;
      });
    };

    window.addEventListener('timeline-recording-deleted', handleTimelineDeleted);
    return () => window.removeEventListener('timeline-recording-deleted', handleTimelineDeleted);
  }, []);

  const handleStreamChange = (e) => {
    setSelectedStream(e.target.value);
    // Blur the select so it no longer intercepts keyboard events after the
    // selection is made, preserving the current keyboard-nav mode.
    e.target.blur();
  };

  const handleDateChange = (newDate) => {
    if (newDate && /^\d{4}-\d{2}-\d{2}$/.test(newDate)) {
      setSelectedDate(newDate);
    } else {
      setSelectedDate(currentDateInputValue());
    }
  };

  const handleIdsDayChange = useCallback((newDate) => {
    if (!idsAvailableDates.includes(newDate)) return;
    setSelectedDate(newDate);
  }, [idsAvailableDates]);

  const jumpIdsDay = useCallback((offset) => {
    if (idsSelectedDayIndex === -1) return;
    const nextDate = idsAvailableDates[idsSelectedDayIndex + offset];
    if (nextDate) {
      handleIdsDayChange(nextDate);
    }
  }, [handleIdsDayChange, idsAvailableDates, idsSelectedDayIndex]);

  // Render content based on state
  const renderContent = () => {
    const isContentLoading = idsMode ? idsLoading : isLoadingTimeline;

    if (isContentLoading) {
      return <LoadingIndicator message={idsMode ? "Loading selected recordings..." : "Loading timeline data..."} />;
    }

    if (segments.length === 0) {
      return (
        <div className="flex flex-col items-center justify-center py-12 text-center">
          <svg className="w-16 h-16 text-muted-foreground mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
          </svg>
          <p className="text-muted-foreground text-lg">
            {idsMode ? 'No selected recordings are visible for this day' : 'No recordings found for the selected date and stream'}
          </p>
        </div>
      );
    }

    return (
      <>
        {/* Video player */}
        <TimelinePlayer videoElementRef={videoElementRef} />

        {/* Playback controls (includes time display) */}
        <TimelineControls />

        <div className="mb-2 flex justify-end">
          <span
            data-testid="timeline-keyboard-nav-mode"
            className="rounded border border-border bg-secondary px-2 py-1 text-[11px] text-muted-foreground"
            title={t('timeline.arrowKeysHelpTitle')}
          >
            {t('timeline.arrowKeys')}: {keyboardNavigationMode === 'fine' ? t('timeline.seekOneSecond') : t('timeline.jumpRecordings')}
          </span>
        </div>

        {/* Timeline — clicking anywhere on it should not change the keyboard-nav mode */}
        <div
          id="timeline-container"
          data-keyboard-nav-preserve
          className="relative w-full h-24 bg-secondary border border-input rounded-lg mb-2 overflow-hidden"
          ref={timelineContainerRef}
        >
          <TimelineRuler />
          <TimelineSegments segments={segments} />
          <TimelineCursor />

          {/* Inline hint */}
          <div className="absolute bottom-1 right-2 text-[10px] text-muted-foreground bg-card/75 px-1.5 py-0.5 rounded">
            {t('timeline.inlineHint')}
          </div>
        </div>
      </>
    );
  };

  // Get return URL for "Refine Selections" link
  const returnUrl = idsMode ? (sessionStorage.getItem(RECORDINGS_RETURN_URL_KEY) || 'recordings.html') : null;

  return (
    <div className="timeline-page">
      <div className="flex items-center mb-4">
        <h1 className="text-2xl font-bold">
          {idsMode ? t('timeline.selectedRecordingsTimeline') : t('timeline.timelinePlayback')}
        </h1>
        <div className="ml-4 flex">
          <a
            href={returnUrl || 'recordings.html'}
            className="px-3 py-1 rounded-l-md text-sm"
            style={{
              backgroundColor: 'hsl(var(--secondary))',
              color: 'hsl(var(--secondary-foreground))'
            }}
            onMouseOver={(e) => e.currentTarget.style.backgroundColor = 'hsl(var(--secondary) / 0.8)'}
            onMouseOut={(e) => e.currentTarget.style.backgroundColor = 'hsl(var(--secondary))'}
            onClick={() => { try { localStorage.setItem('recordings_view_mode', 'table'); } catch(e) {} }}
          >
            {t('recordings.table')}
          </a>
          <a
            href="recordings.html"
            className="px-3 py-1 text-sm"
            style={{
              backgroundColor: 'hsl(var(--secondary))',
              color: 'hsl(var(--secondary-foreground))'
            }}
            onMouseOver={(e) => e.currentTarget.style.backgroundColor = 'hsl(var(--secondary) / 0.8)'}
            onMouseOut={(e) => e.currentTarget.style.backgroundColor = 'hsl(var(--secondary))'}
            onClick={() => { try { localStorage.setItem('recordings_view_mode', 'grid'); } catch(e) {} }}
          >
            {t('recordings.grid')}
          </a>
          <a
            href="timeline.html"
            className="px-3 py-1 rounded-r-md text-sm"
            style={{
              backgroundColor: idsMode ? 'hsl(var(--secondary))' : 'hsl(var(--primary))',
              color: idsMode ? 'hsl(var(--secondary-foreground))' : 'hsl(var(--primary-foreground))'
            }}
            onMouseOver={(e) => { if (idsMode) e.currentTarget.style.backgroundColor = 'hsl(var(--secondary) / 0.8)'; }}
            onMouseOut={(e) => { if (idsMode) e.currentTarget.style.backgroundColor = 'hsl(var(--secondary))'; }}
          >
            {t('nav.timeline')}
          </a>
        </div>
      </div>

      {idsMode ? (
        /* IDs mode: compact info bar */
        <div className="flex flex-wrap items-center gap-3 mb-3 px-3 py-2 bg-secondary rounded-lg text-sm">
          <span className="font-medium">
            {t('timeline.recordingsCount', { count: segments.length })}
            {idsSegmentInfo?.multi_stream && ` · ${t('timeline.streamsCount', { count: [...new Set(segments.map(s => s.stream))].length })}`}
          </span>
          {idsSegmentInfo && (
            <span className="text-xs text-muted-foreground">
              {idsSegmentInfo.start_time} — {idsSegmentInfo.end_time}
            </span>
          )}
          {idsAvailableDates.length > 0 && (
            <div className="flex flex-wrap items-center gap-2 rounded-md border border-border/60 bg-background/70 px-2 py-1">
              <span className="text-[11px] uppercase tracking-wide text-muted-foreground">{t('timeline.activeDay')}</span>
              <button
                type="button"
                data-keyboard-nav-preserve
                className="btn-secondary text-xs px-2 py-1 disabled:opacity-50"
                disabled={idsSelectedDayIndex <= 0}
                onClick={() => jumpIdsDay(-1)}
                title={t('timeline.previousSelectedDay')}
              >
                ←
              </button>
              <select
                data-keyboard-nav-preserve
                className="min-w-[180px] rounded border border-border bg-background px-2 py-1 text-xs"
                value={selectedDate}
                onChange={(e) => handleIdsDayChange(e.target.value)}
              >
                {idsAvailableDates.map((date, index) => (
                  <option key={date} value={date}>
                    {t('timeline.dayNumberWithDate', { number: index + 1, date: formatDisplayDate(date) })}
                  </option>
                ))}
              </select>
              <button
                type="button"
                data-keyboard-nav-preserve
                className="btn-secondary text-xs px-2 py-1 disabled:opacity-50"
                disabled={idsSelectedDayIndex === -1 || idsSelectedDayIndex >= idsAvailableDates.length - 1}
                onClick={() => jumpIdsDay(1)}
                title={t('timeline.nextSelectedDay')}
              >
                →
              </button>
              <span className="text-xs text-muted-foreground">
                {idsSelectedDayIndex === -1 ? '' : t('timeline.dayIndexOfTotal', { index: idsSelectedDayIndex + 1, total: idsAvailableDates.length })}
                {idsVisibleSegmentCount > 0 && ` · ${t('timeline.visibleCount', { count: idsVisibleSegmentCount })}`}
              </span>
            </div>
          )}
          <div className="ml-auto flex gap-2">
            <a href={returnUrl || 'recordings.html'} data-keyboard-nav-preserve className="btn-secondary text-xs px-2 py-1">
              ← {t('timeline.refineSelections')}
            </a>
            <button
              data-keyboard-nav-preserve
              className="btn-primary text-xs px-2 py-1"
              onClick={() => setIsDownloadModalOpen(true)}
              disabled={selectedRecordingsForDownload.length === 0}
            >
              ↓ {t('timeline.downloadAll', { count: selectedRecordingsForDownload.length })}
            </button>
          </div>
        </div>
      ) : (
        /* Normal mode: compact single-row stream + date selectors */
        <div className="flex flex-wrap items-end gap-3 mb-3">
          <div className="flex-grow min-w-[180px]">
            <label htmlFor="stream-selector" className="block text-xs text-muted-foreground mb-1">{t('nav.streams')}</label>
            <select
              id="stream-selector"
              data-keyboard-nav-preserve
              className="w-full p-1.5 text-sm border border-border rounded bg-background text-foreground"
              value={selectedStream || ''}
              onChange={handleStreamChange}
            >
              <option value="" disabled>{t('timeline.selectStreamCount', { count: streamsList.length })}</option>
              {streamsList.map(stream => (
                <option key={stream.name} value={stream.name}>{stream.name}</option>
              ))}
            </select>
          </div>
          <div className="min-w-[160px]" data-keyboard-nav-preserve>
            <label className="block text-xs text-muted-foreground mb-1">{t('timeline.date')}</label>
            <CalendarPicker value={selectedDate} onChange={handleDateChange} />
          </div>
          {isLoadingTimeline && (
            <span className="text-xs text-muted-foreground italic">{t('common.loading')}</span>
          )}
        </div>
      )}

      {/* Content */}
      {renderContent()}

      <BatchDownloadModal
        isOpen={isDownloadModalOpen}
        onClose={() => setIsDownloadModalOpen(false)}
        recordings={selectedRecordingsForDownload}
        selectedIds={selectedRecordingMap}
      />

      {/* Compact help — collapsible, replaces the old large instructions block */}
      <TimelineHelp idsMode={idsMode} t={t} />
    </div>
  );
}

// Export the timeline state for use in other components
export { timelineState };
