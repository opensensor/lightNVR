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
import { showStatusMessage } from '../ToastContainer.jsx';
import { LoadingIndicator } from '../LoadingIndicator.jsx';
import { useQuery } from '../../../query-client.js';

// Utility function to convert between timeline hour and timestamp
function timelineHourToTimestamp(hour, selectedDate) {
  // Get the selected date or today
  // If we have a selectedDate in YYYY-MM-DD format, parse it correctly
  let date;

  console.log(`timelineHourToTimestamp: Converting hour ${hour} for date ${selectedDate}`);

  if (selectedDate && typeof selectedDate === 'string' && selectedDate.includes('-')) {
    // Parse the date string to avoid timezone issues
    const [year, month, day] = selectedDate.split('-').map(num => parseInt(num, 10));
    // Create a date at midnight for the selected date (month is 0-indexed)
    date = new Date(year, month - 1, day, 0, 0, 0, 0);
    console.log(`timelineHourToTimestamp: Parsed date - Year: ${year}, Month: ${month}, Day: ${day}`);
    console.log(`timelineHourToTimestamp: Created date object: ${date.toString()}`);
  } else {
    // Fallback to current date
    date = new Date();
    date.setHours(0, 0, 0, 0);
    console.log(`timelineHourToTimestamp: Using current date: ${date.toString()}`);
  }

  // Add the hour component (this is in local time)
  const milliseconds = date.getTime() + (hour * 60 * 60 * 1000);
  const resultDate = new Date(milliseconds);

  console.log(`timelineHourToTimestamp: Result date for hour ${hour}: ${resultDate.toString()}`);

  // Convert to timestamp (seconds)
  const timestamp = Math.floor(milliseconds / 1000);
  return timestamp;
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
  zoomLevel: 1, // 1 = 1 hour, 2 = 30 minutes, 4 = 15 minutes
  timelineStartHour: 0,
  timelineEndHour: 24,
  currentTime: null,
  prevCurrentTime: null,
  playbackSpeed: 1.0,
  showOnlySegments: true,
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

    console.log('timelineState: setState called with', newState);
    console.log('timelineState: current state before update', {
      currentTime: this.currentTime,
      currentSegmentIndex: this.currentSegmentIndex,
      segmentsLength: this.timelineSegments.length
    });

    // For time-sensitive updates (like currentTime), we want to batch them
    // to prevent too many updates in a short period
    if (newState.currentTime !== undefined &&
        !newState.currentSegmentIndex &&
        !newState.isPlaying &&
        now - this.lastUpdateTime < 250) {
      // Skip frequent time updates that don't change playback state
      console.log('timelineState: Skipping frequent time update');
      return;
    }

    // Apply the new state
    Object.assign(this, newState);

    // Reset forceReload flag immediately
    if (newState.forceReload) {
      this.forceReload = false;
    }

    this.lastUpdateTime = now;

    console.log('timelineState: state after update', {
      currentTime: this.currentTime,
      currentSegmentIndex: this.currentSegmentIndex,
      segmentsLength: this.timelineSegments.length
    });

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
    date: params.get('date') || formatDateForInput(new Date())
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

    console.log('TimelinePage: Generated time range:', {
      date,
      startDate: startDate.toString(),
      endDate: endDate.toString(),
      startTime,
      endTime,
      year, month, day
    });

    return {
      startTime,
      endTime
    };
  };

  // Update URL and global state when stream or date changes
  useEffect(() => {
    if (selectedStream) {
      // Update URL
      updateUrlParams(selectedStream, selectedDate);

      // Update global state
      timelineState.setState({
        selectedStream,
        selectedDate
      });
    }
  }, [selectedStream, selectedDate]);

  // Get time range for current date
  const timeRange = getTimeRange(selectedDate);
  const startTime = timeRange.startTime;
  const endTime = timeRange.endTime;

  // Construct the URL for the API call
  const timelineUrl = selectedStream ?
    `/api/timeline/segments?stream=${encodeURIComponent(selectedStream)}&start=${encodeURIComponent(startTime)}&end=${encodeURIComponent(endTime)}` :
    null;

  // Debug the URL being used
  console.log('TimelinePage: Timeline URL:', timelineUrl);

  // Fetch timeline segments using preact-query
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
      enabled: !!selectedStream, // Only run query if we have a selected stream
      onSuccess: (data) => {
        console.log('TimelinePage: Timeline data received:', data);
        const timelineSegments = data.segments || [];
        console.log(`TimelinePage: Received ${timelineSegments.length} segments`);

        if (timelineSegments.length === 0) {
          console.log('TimelinePage: No segments found');
          setSegments([]);

          // Update global state
          timelineState.setState({
            timelineSegments: [],
            currentSegmentIndex: -1,
            currentTime: null,
            isPlaying: false
          });

          showStatusMessage('No recordings found for the selected date', 'warning');
          return;
        }

        // IMPORTANT: Make a deep copy of the segments to avoid reference issues
        const segmentsCopy = JSON.parse(JSON.stringify(timelineSegments));

        // Log the first few segments for debugging
        segmentsCopy.slice(0, 3).forEach((segment, i) => {
          const startTime = new Date(segment.start_timestamp * 1000);
          const endTime = new Date(segment.end_timestamp * 1000);
          console.log(`TimelinePage: Segment ${i} - Start: ${startTime.toLocaleTimeString()}, End: ${endTime.toLocaleTimeString()}`);
        });

        console.log('TimelinePage: Setting segments');
        setSegments(segmentsCopy);

        // Force a synchronous DOM update
        document.body.offsetHeight;

        // Directly update the global state with the segments
        const firstSegmentStartTime = segmentsCopy[0].start_timestamp;

        console.log('TimelinePage: Setting initial segment and time', {
          firstSegmentId: segmentsCopy[0].id,
          startTime: new Date(firstSegmentStartTime * 1000).toLocaleTimeString()
        });

        // DIRECT ASSIGNMENT to ensure state is properly set
        console.log('TimelinePage: Directly setting timelineState properties');
        timelineState.timelineSegments = segmentsCopy;
        timelineState.currentSegmentIndex = 0;
        timelineState.currentTime = firstSegmentStartTime;
        timelineState.prevCurrentTime = firstSegmentStartTime;
        timelineState.isPlaying = false;
        timelineState.forceReload = true;
        timelineState.zoomLevel = 1;
        timelineState.selectedDate = selectedDate; // Make sure the date is set

        // Now call setState to notify listeners
        timelineState.setState({
          // Empty object just to trigger notification
        });

        console.log('TimelinePage: Updated timelineState with segments');

        // Wait a moment to ensure state is updated, then log the current state
        setTimeout(() => {
          console.log('TimelinePage: State after update (delayed check):', {
            segmentsLength: timelineState.timelineSegments.length,
            currentSegmentIndex: timelineState.currentSegmentIndex,
            currentTime: timelineState.currentTime
          });

          // Force a state update if the state wasn't properly updated
          if (!timelineState.currentTime || timelineState.currentSegmentIndex === -1) {
            console.log('TimelinePage: State not properly updated, forcing update');
            timelineState.setState({
              currentSegmentIndex: 0,
              currentTime: firstSegmentStartTime,
              prevCurrentTime: firstSegmentStartTime
            });
          }
        }, 100);

        // Preload the first segment's video
        const videoPlayer = document.querySelector('#video-player video');
        if (videoPlayer) {
          videoPlayer.src = `/api/recordings/play/${segmentsCopy[0].id}?t=${Date.now()}`;
          videoPlayer.load();
        }

        showStatusMessage(`Loaded ${segmentsCopy.length} recording segments`, 'success');
      },
      onError: (error) => {
        console.error('TimelinePage: Error loading timeline data:', error);
        console.error('TimelinePage: Error details:', {
          message: error.message,
          status: error.status,
          statusText: error.statusText,
          response: error.response
        });
        showStatusMessage('Error loading timeline data: ' + error.message, 'error');
        setSegments([]);
      }
    }
  );

  // Handle stream selection change
  const handleStreamChange = (e) => {
    const newStream = e.target.value;
    console.log(`TimelinePage: Stream changed to ${newStream}`);
    setSelectedStream(newStream);
  };

  // Handle date selection change
  const handleDateChange = (e) => {
    const newDate = e.target.value;
    console.log(`TimelinePage: Date changed to ${newDate}`);

    // Validate the date format (should be YYYY-MM-DD)
    if (newDate && /^\d{4}-\d{2}-\d{2}$/.test(newDate)) {
      // Parse the date to ensure it's valid
      const [year, month, day] = newDate.split('-').map(num => parseInt(num, 10));
      console.log(`TimelinePage: Parsed date - Year: ${year}, Month: ${month}, Day: ${day}`);

      // Update the selected date
      setSelectedDate(newDate);
    } else {
      console.error(`TimelinePage: Invalid date format: ${newDate}`);
      // Fallback to today's date in YYYY-MM-DD format
      const today = formatDateForInput(new Date());
      console.log(`TimelinePage: Falling back to today's date: ${today}`);
      setSelectedDate(today);
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
            Drag the orange dial to navigate
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
          <a href="recordings.html" className="px-3 py-1 bg-secondary text-secondary-foreground hover:bg-secondary/80 rounded-l-md">Table View</a>
          <a href="timeline.html" className="px-3 py-1 rounded-r-md bg-primary text-primary-foreground">Timeline View</a>
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
          <label htmlFor="timeline-date" className="block mb-2">Date</label>
          <input
            type="date"
            id="timeline-date"
            className="w-full p-2 border border-border rounded bg-background text-foreground"
            value={selectedDate}
            onChange={handleDateChange}
          />
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
          <li>Drag the orange cursor to navigate precisely</li>
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
