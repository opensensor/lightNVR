/**
 * LightNVR Timeline Page Component
 * Main component for the timeline view
 */

import React, { useState, useEffect, useRef } from 'react';
import { TimelineControls } from './TimelineControls.jsx';
import { TimelineRuler } from './TimelineRuler.jsx';
import { TimelineSegments } from './TimelineSegments.jsx';
import { TimelineCursor } from './TimelineCursor.jsx';
import { TimelinePlayer } from './TimelinePlayer.jsx';
import { CalendarPicker } from './CalendarPicker.jsx';
import { showStatusMessage } from '../ToastContainer.jsx';
import { LoadingIndicator } from '../LoadingIndicator.jsx';
import { useQuery } from '../../../query-client.js';

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

/**
 * Parse URL parameters
 */
function parseUrlParams() {
  const params = new URLSearchParams(window.location.search);
  return {
    stream: params.get('stream') || '',
    date: params.get('date') || formatDateForInput(new Date()),
    time: params.get('time') || ''  // Optional HH:MM:SS to auto-seek on load
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
 * TimelinePage component
 */
export function TimelinePage() {
  // Get initial values from URL parameters
  const urlParams = parseUrlParams();

  // State
  const [isLoading, setIsLoading] = useState(false);
  const [streamsList, setStreamsList] = useState([]);
  const [selectedStream, setSelectedStream] = useState(urlParams.stream);
  const [selectedDate, setSelectedDate] = useState(urlParams.date);
  const [segments, setSegments] = useState([]);

  // Refs
  const timelineContainerRef = useRef(null);
  const initialLoadRef = useRef(false);
  const flushIntervalRef = useRef(null);
  const initialTimeRef = useRef(urlParams.time);  // Store initial time param for auto-seek
  const processedDataRef = useRef(null);  // Track last processed timeline data

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
    isLoading: isLoadingStreams,
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
    if (selectedStream) {
      updateUrlParams(selectedStream, selectedDate);
      timelineState.setState({ selectedStream, selectedDate });
    }
  }, [selectedStream, selectedDate]);

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
      enabled: !!selectedStream // Only run query if we have a selected stream
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
      setSegments([]);
      timelineState.setState({
        timelineSegments: [],
        currentSegmentIndex: -1,
        currentTime: null,
        isPlaying: false
      });
      showStatusMessage('No recordings found for the selected date', 'warning');
      return;
    }

    const segmentsCopy = JSON.parse(JSON.stringify(timelineSegments));
    setSegments(segmentsCopy);

    // Determine initial time/segment (honour ?time= URL param)
    let initialTime = segmentsCopy[0].start_timestamp;
    let initialSegmentIndex = 0;

    if (initialTimeRef.current) {
      const timeParts = initialTimeRef.current.match(/^(\d{2}):(\d{2}):(\d{2})$/);
      if (timeParts) {
        const [, h, m, s] = timeParts.map(Number);
        const seekTs = timelineHourToTimestamp(h + m / 60 + s / 3600, selectedDate);

        let found = false;
        for (let i = 0; i < segmentsCopy.length; i++) {
          if (seekTs >= segmentsCopy[i].start_timestamp && seekTs <= segmentsCopy[i].end_timestamp) {
            initialSegmentIndex = i;
            initialTime = seekTs;
            found = true;
            break;
          }
        }
        if (!found) {
          let best = 0, bestDist = Infinity;
          for (let i = 0; i < segmentsCopy.length; i++) {
            const d = Math.abs(segmentsCopy[i].start_timestamp - seekTs);
            if (d < bestDist) { bestDist = d; best = i; }
          }
          initialSegmentIndex = best;
          initialTime = segmentsCopy[best].start_timestamp;
        }
      }
      initialTimeRef.current = '';
    }

    // Compute auto-fit range from segments
    let earliest = 24, latest = 0;
    segmentsCopy.forEach(seg => {
      const s = new Date(seg.start_timestamp * 1000);
      const e = new Date(seg.end_timestamp * 1000);
      earliest = Math.min(earliest, s.getHours() + s.getMinutes() / 60 + s.getSeconds() / 3600);
      latest   = Math.max(latest,   e.getHours() + e.getMinutes() / 60 + e.getSeconds() / 3600);
    });
    const span = latest - earliest;
    const pad  = Math.max(0.5, Math.min(1, span * 0.1));
    let fitStart = Math.max(0, Math.floor((earliest - pad) * 2) / 2);
    let fitEnd   = Math.min(24, Math.ceil((latest + pad) * 2) / 2);
    if (fitEnd - fitStart < 2) {
      const center = (earliest + latest) / 2;
      fitStart = Math.max(0, Math.floor((center - 1) * 2) / 2);
      fitEnd   = Math.min(24, fitStart + 2);
    }

    // Push to global state
    timelineState.timelineSegments = segmentsCopy;
    timelineState.currentSegmentIndex = initialSegmentIndex;
    timelineState.currentTime = initialTime;
    timelineState.prevCurrentTime = initialTime;
    timelineState.isPlaying = false;
    timelineState.forceReload = true;
    timelineState.autoFitStartHour = fitStart;
    timelineState.autoFitEndHour = fitEnd;
    timelineState.timelineStartHour = fitStart;
    timelineState.timelineEndHour = fitEnd;
    timelineState.selectedDate = selectedDate;
    timelineState.setState({});

    // Safety net: retry if state didn't stick
    setTimeout(() => {
      if (!timelineState.currentTime || timelineState.currentSegmentIndex === -1) {
        timelineState.setState({
          currentSegmentIndex: initialSegmentIndex,
          currentTime: initialTime,
          prevCurrentTime: initialTime
        });
      }
    }, 100);

    // Preload the initial segment's video
    const videoPlayer = document.querySelector('#video-player video');
    if (videoPlayer) {
      videoPlayer.src = `/api/recordings/play/${segmentsCopy[initialSegmentIndex].id}?t=${Date.now()}`;
      videoPlayer.load();
    }

    showStatusMessage(`Loaded ${segmentsCopy.length} recording segments`, 'success');
  }, [timelineData, timelineError, selectedDate]);

  const handleStreamChange = (e) => setSelectedStream(e.target.value);

  const handleDateChange = (newDate) => {
    if (newDate && /^\d{4}-\d{2}-\d{2}$/.test(newDate)) {
      setSelectedDate(newDate);
    } else {
      setSelectedDate(formatDateForInput(new Date()));
    }
  };

  // Render content based on state
  const renderContent = () => {
    if (isLoadingTimeline) {
      return <LoadingIndicator message="Loading timeline data..." />;
    }

    if (segments.length === 0) {
      return (
        <div className="flex flex-col items-center justify-center py-12 text-center">
          <svg className="w-16 h-16 text-muted-foreground mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
            <path strokeLinecap="round" strokeLinejoin="round" strokeWidth="2" d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
          </svg>
          <p className="text-muted-foreground text-lg">No recordings found for the selected date and stream</p>
        </div>
      );
    }

    return (
      <>
        {/* Video player */}
        <TimelinePlayer />

        {/* Playback controls */}
        <TimelineControls />

        {/* Timeline */}
        <div
          id="timeline-container"
          className="relative w-full h-24 bg-secondary border border-input rounded-lg mb-6 overflow-hidden"
          ref={timelineContainerRef}
        >
          <TimelineRuler />
          <TimelineSegments segments={segments} />
          <TimelineCursor />

          {/* Instructions for cursor */}
          <div className="absolute bottom-1 right-2 text-xs text-muted-foreground bg-card text-card-foreground bg-opacity-75 dark:bg-opacity-75 px-2 py-1 rounded">
            Click a segment to play · Drag playhead to navigate
          </div>
        </div>
      </>
    );
  };

  return (
    <div className="timeline-page">
      <div className="flex items-center mb-4">
        <h1 className="text-2xl font-bold">Timeline Playback</h1>
        <div className="ml-4 flex">
          <a
            href="recordings.html"
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
              backgroundColor: 'hsl(var(--primary))',
              color: 'hsl(var(--primary-foreground))'
            }}
          >
            Timeline
          </a>
        </div>
      </div>

      {/* Stream selector and date picker */}
      <div className="flex flex-wrap gap-4 mb-2">
        <div className="stream-selector flex-grow">
          <div className="flex justify-between items-center mb-2">
            <label htmlFor="stream-selector">Stream</label>
            <button
              className="text-xs bg-secondary text-secondary-foreground hover:bg-secondary/80 px-2 py-1 rounded"
              onClick={() => refetchTimeline()}
            >
              Reload Data
            </button>
          </div>
          <select
            id="stream-selector"
            className="w-full p-2 border border-border rounded bg-background text-foreground"
            value={selectedStream || ''}
            onChange={handleStreamChange}
          >
            <option value="" disabled>Select a stream ({streamsList.length} available)</option>
            {streamsList.map(stream => (
              <option key={stream.name} value={stream.name}>{stream.name}</option>
            ))}
          </select>
        </div>

        <div className="date-selector flex-grow">
          <label className="block mb-2">Date</label>
          <CalendarPicker value={selectedDate} onChange={handleDateChange} />
        </div>
      </div>

      {/* Auto-load message */}
      <div className="mb-4 text-sm text-muted-foreground italic">
        {isLoadingTimeline ? 'Loading...' : 'Recordings auto-load when stream or date changes'}
      </div>

      {/* Current time display */}
      <div className="flex justify-between items-center mb-2">
        <div id="time-display" className="timeline-time-display bg-secondary text-foreground px-3 py-1 rounded font-mono text-base">00:00:00</div>
      </div>

      {/* Debug info */}
      <div className="mb-2 text-xs text-muted-foreground">
        Debug - isLoading: {isLoadingTimeline ? 'true' : 'false'},
        Streams: {streamsList.length},
        Segments: {segments.length}
      </div>

      {/* Content */}
      {renderContent()}

      {/* Instructions */}
      <div className="mt-6 p-4 bg-secondary rounded">
        <h3 className="text-lg font-semibold mb-2">How to use the timeline:</h3>
        <ul className="list-disc pl-5">
          <li>Select a stream and date to load recordings</li>
          <li>Click on the timeline to position the cursor at a specific time</li>
          <li>Drag the playhead to navigate precisely</li>
          <li>Click on a segment (blue bar) to play that recording</li>
          <li>Use the play button to start playback from the current cursor position</li>
          <li>Use the zoom buttons to adjust the timeline scale</li>
        </ul>
      </div>
    </div>
  );
}

// Export the timeline state for use in other components
export { timelineState };
