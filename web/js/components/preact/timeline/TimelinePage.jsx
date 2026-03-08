/**
 * LightNVR Timeline Page Component
 * Main component for the timeline view
 */

import React, { useState, useEffect, useRef, useCallback, useMemo } from 'react';
import { TimelineControls } from './TimelineControls.jsx';
import { TimelineRuler } from './TimelineRuler.jsx';
import { TimelineSegments } from './TimelineSegments.jsx';
import { TimelineCursor } from './TimelineCursor.jsx';
import { TimelinePlayer } from './TimelinePlayer.jsx';
import { CalendarPicker } from './CalendarPicker.jsx';
import { showStatusMessage } from '../ToastContainer.jsx';
import { LoadingIndicator } from '../LoadingIndicator.jsx';
import { useQuery } from '../../../query-client.js';
import {
  countSegmentsForDate,
  findFirstVisibleSegmentIndex,
  findContainingSegmentIndex,
  formatTimestampAsLocalDate,
  getAvailableDatesForSegments,
  getClippedSegmentHourRange,
  getLocalDayBounds
} from './timelineUtils.js';

// Convert fractional hour (0–24) → Unix timestamp (seconds) for the given date
function timelineHourToTimestamp(hour, selectedDate) {
  let date;
  if (selectedDate && typeof selectedDate === 'string' && selectedDate.includes('-')) {
    const [year, month, day] = selectedDate.split('-').map(Number);
    date = new Date(year, month - 1, day, 0, 0, 0, 0);
  } else {
    date = new Date();
    date.setHours(0, 0, 0, 0);
  }
  return Math.floor((date.getTime() + hour * 3600000) / 1000);
}

// Utility function to convert timestamp to timeline hour
function timestampToTimelineHour(timestamp) {
  const date = new Date(timestamp * 1000);
  return date.getHours() + (date.getMinutes() / 60) + (date.getSeconds() / 3600);
}

// Global timeline state for child components
const timelineState = {
  streams: [],
  timelineSegments: [],
  selectedStream: null,
  selectedDate: null,
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

  // Update state and notify listeners
  setState(newState) {
    const now = Date.now();

    // Batch frequent time-only updates (≤250 ms apart)
    if (newState.currentTime !== undefined &&
        !newState.currentSegmentIndex &&
        !newState.isPlaying &&
        now - this.lastUpdateTime < 250) {
      return;
    }

    Object.assign(this, newState);

    if (newState.forceReload) {
      this.forceReload = false;
    }

    this.lastUpdateTime = now;
    this.notifyListeners();
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
    if (Object.keys(this.pendingUpdates).length > 0) {
      Object.assign(this, this.pendingUpdates);
      this.pendingUpdates = {};
      this.lastUpdateTime = Date.now();
      this.notifyListeners();
    }
  }
};

/**
 * Format date for input element
 */
function formatDateForInput(date) {
  const year = date.getFullYear();
  const month = String(date.getMonth() + 1).padStart(2, '0');
  const day = String(date.getDate()).padStart(2, '0');
  return `${year}-${month}-${day}`;
}

function formatDisplayDate(dateString) {
  const [year, month, day] = dateString.split('-').map(Number);
  return new Date(year, month - 1, day).toLocaleDateString(undefined, {
    weekday: 'short',
    month: 'short',
    day: 'numeric',
    year: 'numeric'
  });
}

/**
 * Parse URL parameters
 */
function parseUrlParams() {
  const params = new URLSearchParams(window.location.search);
  return {
    stream: params.get('stream') || '',
    date: params.get('date') || formatDateForInput(new Date()),
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
function TimelineHelp({ idsMode }) {
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
        {open ? 'Hide help' : 'How to use the timeline'}
      </button>
      {open && (
        <ul className="mt-1.5 ml-5 text-xs text-muted-foreground list-disc space-y-0.5">
          {!idsMode && <li>Select a stream and date — recordings load automatically</li>}
          {idsMode && <li>Use <strong>Refine Selections</strong> to go back and adjust your selection</li>}
          <li>Click on the timeline to position the cursor at a specific time</li>
          <li>Drag the playhead to navigate precisely</li>
          <li>Click a segment (coloured bar) to play that recording</li>
          <li>Use the green play button to start playback from the current cursor position</li>
          <li>Use the <strong>Fit</strong>, <strong>+</strong> and <strong>−</strong> buttons to zoom the timeline</li>
          <li>Use <strong>Snapshot</strong>, <strong>Download</strong>, <strong>Protect</strong> or <strong>Delete</strong> on the currently playing recording</li>
        </ul>
      )}
    </div>
  );
}

/**
 * TimelinePage component
 */
export function TimelinePage() {
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

  // Refs
  const timelineContainerRef = useRef(null);
  const initialLoadRef = useRef(false);
  const flushIntervalRef = useRef(null);
  const initialTimeRef = useRef(urlParams.time);  // Store initial time param for auto-seek
  const processedDataRef = useRef(null);  // Track last processed timeline data
  const selectedDateRef = useRef(urlParams.date);

  useEffect(() => {
    selectedDateRef.current = selectedDate;
  }, [selectedDate]);

  const idsAvailableDates = useMemo(() => (
    idsMode ? getAvailableDatesForSegments(idsTimelineSegments) : []
  ), [idsMode, idsTimelineSegments]);

  const idsSelectedDayIndex = idsAvailableDates.indexOf(selectedDate);
  const idsVisibleSegmentCount = useMemo(() => (
    idsMode ? countSegmentsForDate(idsTimelineSegments, selectedDate) : 0
  ), [idsMode, idsTimelineSegments, selectedDate]);

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
    const visibleIndices = [];

    // Compute auto-fit range from segments for the selected day
    let earliest = 24;
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
        timelineEndHour: 24,
        autoFitStartHour: 0,
        autoFitEndHour: 24
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
      const timeParts = initialTimeRef.current.match(/^(\d{2}):(\d{2}):(\d{2})$/);
      if (timeParts) {
        const [, h, m, s] = timeParts.map(Number);
        const seekTs = timelineHourToTimestamp(h + m / 60 + s / 3600, effectiveDate);

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
    let fitEnd = 24;
    if (earliest < 24 && latest > 0) {
      const span = latest - earliest;
      const pad = Math.max(0.5, Math.min(1, span * 0.1));
      fitStart = Math.max(0, Math.floor((earliest - pad) * 2) / 2);
      fitEnd = Math.min(24, Math.ceil((latest + pad) * 2) / 2);
      if (fitEnd - fitStart < 2) {
        const center = (earliest + latest) / 2;
        fitStart = Math.max(0, Math.floor((center - 1) * 2) / 2);
        fitEnd = Math.min(24, fitStart + 2);
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
      const videoPlayer = document.querySelector('#video-player video');
      if (videoPlayer) {
        videoPlayer.src = `/api/recordings/play/${segmentsCopy[initialSegmentIndex].id}?t=${Date.now()}`;
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
            : (availableDates[0] || formatDateForInput(new Date(segs[0].start_timestamp * 1000)));

          setIdsTimelineSegments(segs);
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
        showStatusMessage('Error loading selected recordings: ' + err.message, 'error');
      }
      setIdsLoading(false);
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
    // Create a date object at midnight for the selected date
    // The date string from the input is in YYYY-MM-DD format (local date)
    const [year, month, day] = date.split('-').map(num => parseInt(num, 10));

    // Create date objects using local date components to avoid timezone issues
    // Month is 0-indexed in JavaScript Date
    const startDate = new Date(year, month - 1, day, 0, 0, 0, 0);
    const endDate = new Date(year, month - 1, day, 23, 59, 59, 999);

    // Format dates for API in ISO format
    const startTime = startDate.toISOString();
    const endTime = endDate.toISOString();

    return { startTime, endTime };
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
    refetch: refetchTimeline
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
      loadSegmentsIntoTimeline([], selectedDate, '');
      showStatusMessage('No recordings found for the selected date', 'warning');
      return;
    }

    loadSegmentsIntoTimeline(timelineSegments, selectedDate, {
      successMessage: `Loaded ${timelineSegments.length} recording segments`
    });
  }, [timelineData, timelineError, selectedDate, loadSegmentsIntoTimeline]);

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
            const videoEl = document.querySelector('#video-player video');
            if (videoEl) {
              videoEl.pause();
              videoEl.removeAttribute('src');
              videoEl.load();
              videoEl.src = `/api/recordings/play/${nextSeg.id}?t=${Date.now()}`;
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

  const handleStreamChange = (e) => setSelectedStream(e.target.value);

  const handleDateChange = (newDate) => {
    if (newDate && /^\d{4}-\d{2}-\d{2}$/.test(newDate)) {
      setSelectedDate(newDate);
    } else {
      setSelectedDate(formatDateForInput(new Date()));
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
    if (isLoadingTimeline || idsLoading) {
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
        <TimelinePlayer />

        {/* Playback controls (includes time display) */}
        <TimelineControls />

        {/* Timeline */}
        <div
          id="timeline-container"
          className="relative w-full h-24 bg-secondary border border-input rounded-lg mb-2 overflow-hidden"
          ref={timelineContainerRef}
        >
          <TimelineRuler />
          <TimelineSegments segments={segments} />
          <TimelineCursor />

          {/* Inline hint */}
          <div className="absolute bottom-1 right-2 text-[10px] text-muted-foreground bg-card/75 px-1.5 py-0.5 rounded">
            Click segment to play · Drag playhead to navigate
          </div>
        </div>
      </>
    );
  };

  // Get return URL for "Refine Selections" link
  const returnUrl = idsMode ? (sessionStorage.getItem('lightnvr_recordings_return_url') || 'recordings.html') : null;

  // IDs for download
  const downloadSelectedRecordings = () => {
    if (!idsMode || !recordingIds) return;
    const idsList = recordingIds.split(',');
    // Use the same batch download mechanism
    idsList.forEach(id => {
      const seg = segments.find(s => String(s.id) === String(id));
      if (seg) {
        const a = document.createElement('a');
        a.href = `/api/recordings/${id}/download`;
        a.download = '';
        document.body.appendChild(a);
        a.click();
        document.body.removeChild(a);
      }
    });
    showStatusMessage(`Downloading ${idsList.length} recordings...`, 'success');
  };

  return (
    <div className="timeline-page">
      <div className="flex items-center mb-4">
        <h1 className="text-2xl font-bold">
          {idsMode ? 'Selected Recordings Timeline' : 'Timeline Playback'}
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
            Table
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
            Grid
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
            Timeline
          </a>
        </div>
      </div>

      {idsMode ? (
        /* IDs mode: compact info bar */
        <div className="flex flex-wrap items-center gap-3 mb-3 px-3 py-2 bg-secondary rounded-lg text-sm">
          <span className="font-medium">
            {segments.length} recording{segments.length !== 1 ? 's' : ''}
            {idsSegmentInfo?.multi_stream && ` · ${[...new Set(segments.map(s => s.stream))].length} stream(s)`}
          </span>
          {idsSegmentInfo && (
            <span className="text-xs text-muted-foreground">
              {idsSegmentInfo.start_time} — {idsSegmentInfo.end_time}
            </span>
          )}
          {idsAvailableDates.length > 0 && (
            <div className="flex flex-wrap items-center gap-2 rounded-md border border-border/60 bg-background/70 px-2 py-1">
              <span className="text-[11px] uppercase tracking-wide text-muted-foreground">Active day</span>
              <button
                type="button"
                className="btn-secondary text-xs px-2 py-1 disabled:opacity-50"
                disabled={idsSelectedDayIndex <= 0}
                onClick={() => jumpIdsDay(-1)}
                title="Previous selected day"
              >
                ←
              </button>
              <select
                className="min-w-[180px] rounded border border-border bg-background px-2 py-1 text-xs"
                value={selectedDate}
                onChange={(e) => handleIdsDayChange(e.target.value)}
              >
                {idsAvailableDates.map((date, index) => (
                  <option key={date} value={date}>
                    {`Day ${index + 1} · ${formatDisplayDate(date)}`}
                  </option>
                ))}
              </select>
              <button
                type="button"
                className="btn-secondary text-xs px-2 py-1 disabled:opacity-50"
                disabled={idsSelectedDayIndex === -1 || idsSelectedDayIndex >= idsAvailableDates.length - 1}
                onClick={() => jumpIdsDay(1)}
                title="Next selected day"
              >
                →
              </button>
              <span className="text-xs text-muted-foreground">
                {idsSelectedDayIndex === -1 ? '' : `${idsSelectedDayIndex + 1} of ${idsAvailableDates.length}`}
                {idsVisibleSegmentCount > 0 && ` · ${idsVisibleSegmentCount} visible`}
              </span>
            </div>
          )}
          <div className="ml-auto flex gap-2">
            <a href={returnUrl || 'recordings.html'} className="btn-secondary text-xs px-2 py-1">
              ← Refine Selections
            </a>
            <button
              className="btn-primary text-xs px-2 py-1"
              onClick={downloadSelectedRecordings}
              disabled={segments.length === 0}
            >
              ↓ Download All ({segments.length})
            </button>
          </div>
        </div>
      ) : (
        /* Normal mode: compact single-row stream + date selectors */
        <div className="flex flex-wrap items-end gap-3 mb-3">
          <div className="flex-grow min-w-[180px]">
            <label htmlFor="stream-selector" className="block text-xs text-muted-foreground mb-1">Stream</label>
            <select
              id="stream-selector"
              className="w-full p-1.5 text-sm border border-border rounded bg-background text-foreground"
              value={selectedStream || ''}
              onChange={handleStreamChange}
            >
              <option value="" disabled>Select a stream ({streamsList.length})</option>
              {streamsList.map(stream => (
                <option key={stream.name} value={stream.name}>{stream.name}</option>
              ))}
            </select>
          </div>
          <div className="min-w-[160px]">
            <label className="block text-xs text-muted-foreground mb-1">Date</label>
            <CalendarPicker value={selectedDate} onChange={handleDateChange} />
          </div>
          <button
            className="text-xs bg-secondary text-secondary-foreground hover:bg-secondary/80 px-2 py-1.5 rounded"
            onClick={() => refetchTimeline()}
            title="Reload timeline data"
          >
            ↻ Reload
          </button>
          {isLoadingTimeline && (
            <span className="text-xs text-muted-foreground italic">Loading...</span>
          )}
        </div>
      )}

      {/* Content */}
      {renderContent()}

      {/* Compact help — collapsible, replaces the old large instructions block */}
      <TimelineHelp idsMode={idsMode} />
    </div>
  );
}

// Export the timeline state for use in other components
export { timelineState };
