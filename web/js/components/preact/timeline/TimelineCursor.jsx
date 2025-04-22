/**
 * LightNVR Timeline Cursor Component
 * Displays the playback cursor on the timeline
 */

import { useState, useEffect, useRef } from 'preact/hooks';
import { timelineState } from './TimelinePage.jsx';

/**
 * TimelineCursor component
 * @returns {JSX.Element} TimelineCursor component
 */
export function TimelineCursor() {
  // Local state
  const [position, setPosition] = useState(0);
  const [visible, setVisible] = useState(false);
  const [startHour, setStartHour] = useState(0);
  const [endHour, setEndHour] = useState(24);
  const [currentTime, setCurrentTime] = useState(null);
  const [isDragging, setIsDragging] = useState(false);

  // Refs
  const cursorRef = useRef(null);
  const containerRef = useRef(null);
  const dragStartXRef = useRef(0);

  // Debounce function to limit how often a function can be called
  const debounce = (func, delay) => {
    let timeoutId;
    return function(...args) {
      if (timeoutId) {
        clearTimeout(timeoutId);
      }
      timeoutId = setTimeout(() => {
        func.apply(this, args);
      }, delay);
    };
  };

  // Create debounced version of updateCursorPosition
  const debouncedUpdateCursorPosition = useRef(
    debounce((time, startHr, endHr) => {
      updateCursorPosition(time, startHr, endHr);
    }, 100)
  ).current;

  // Subscribe to timeline state changes
  useEffect(() => {
    const unsubscribe = timelineState.subscribe(state => {
      console.log('TimelineCursor: State update received', {
        currentTime: state.currentTime,
        startHour: state.timelineStartHour,
        endHour: state.timelineEndHour,
        segmentsCount: state.timelineSegments ? state.timelineSegments.length : 0,
        isDragging: isDragging,
        userControllingCursor: state.userControllingCursor
      });

      // Update local state
      setStartHour(state.timelineStartHour || 0);
      setEndHour(state.timelineEndHour || 24);

      // Only update current time if not dragging
      if (!isDragging && !state.userControllingCursor) {
        setCurrentTime(state.currentTime);
        updateTimeDisplay(state.currentTime);

        // Use debounced update for smoother performance
        debouncedUpdateCursorPosition(state.currentTime, state.timelineStartHour || 0, state.timelineEndHour || 24);
      }
    });

    return () => unsubscribe();
  }, [isDragging, debouncedUpdateCursorPosition]);

  // Set up drag handling
  useEffect(() => {
    const cursor = cursorRef.current;
    if (!cursor) return;

    const handleMouseDown = (e) => {
      e.preventDefault();
      e.stopPropagation();

      console.log('TimelineCursor: Mouse down event');

      // Store the starting X position
      dragStartXRef.current = e.clientX;

      // Set dragging state
      setIsDragging(true);

      // Set global flags to prevent other components from updating cursor
      timelineState.userControllingCursor = true;
      timelineState.preserveCursorPosition = true;
      timelineState.cursorPositionLocked = true;
      timelineState.setState({});

      // Add event listeners for drag
      document.addEventListener('mousemove', handleMouseMove);
      document.addEventListener('mouseup', handleMouseUp);
    };

    const handleMouseMove = (e) => {
      if (!isDragging) return;

      // Get container dimensions
      const container = cursor.parentElement;
      if (!container) return;

      const rect = container.getBoundingClientRect();
      const clickX = Math.max(0, Math.min(e.clientX - rect.left, rect.width));
      const containerWidth = rect.width;

      // Calculate position as percentage
      const positionPercent = (clickX / containerWidth) * 100;
      setPosition(positionPercent);

      // Calculate time based on position
      const hourRange = endHour - startHour;
      const hour = startHour + (positionPercent / 100) * hourRange;

      // Convert hour to timestamp using the utility function
      const timestamp = timelineState.timelineHourToTimestamp(hour, timelineState.selectedDate);

      // Store the current time locally but don't update the global state yet
      setCurrentTime(timestamp);

      // Update time display
      updateTimeDisplay(timestamp);
    };

    const handleMouseUp = (e) => {
      if (!isDragging) return;

      // Get container dimensions
      const container = cursor.parentElement;
      if (!container) return;

      const rect = container.getBoundingClientRect();
      const clickX = Math.max(0, Math.min(e.clientX - rect.left, rect.width));
      const containerWidth = rect.width;

      // Calculate position as percentage
      const positionPercent = (clickX / containerWidth) * 100;
      console.log('TimelineCursor: Mouse up at position', { positionPercent, clickX, containerWidth });

      // Calculate time based on position
      const hourRange = endHour - startHour;
      const hour = startHour + (positionPercent / 100) * hourRange;
      console.log('TimelineCursor: Calculated hour', { hour, startHour, endHour, hourRange });

      // Convert hour to timestamp using the utility function
      const timestamp = timelineState.timelineHourToTimestamp(hour, timelineState.selectedDate);

      console.log('TimelineCursor: Converted hour to timestamp', {
        hour,
        localDate: new Date(timestamp * 1000).toLocaleString(),
        timestamp
      });
      console.log('TimelineCursor: Converted to timestamp', {
        timestamp,
        dateTime: new Date(timestamp * 1000).toLocaleString(),
        selectedDate: timelineState.selectedDate
      });

      console.log('TimelineCursor: Mouse up event');

      // Reset dragging state FIRST
      setIsDragging(false);

      // Remove event listeners
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);

      // Reset some flags but keep preserveCursorPosition true
      // This allows the current update to complete before other components can update the cursor
      setTimeout(() => {
        console.log('TimelineCursor: Releasing cursor control but preserving position');
        timelineState.userControllingCursor = false;
        // Keep preserveCursorPosition true to prevent position resets
        // Keep cursorPositionLocked true to prevent automatic updates
        timelineState.setState({});
      }, 100);

      // Always update the current time to where the user placed the cursor
      // This allows the user to position the cursor anywhere on the timeline
      timelineState.currentTime = timestamp;

      // Add a small buffer to prevent cursor from snapping to segment start
      // This is especially important for positions near the beginning of segments
      if (timelineState.timelineSegments && timelineState.timelineSegments.length > 0) {
        // Find the segment that contains this timestamp
        const segment = timelineState.timelineSegments.find(seg =>
          timestamp >= seg.start_timestamp && timestamp <= seg.end_timestamp
        );

        if (segment) {
          // If we're very close to the start of the segment (within 1 second),
          // add a small offset to prevent snapping to the start
          const distanceFromStart = timestamp - segment.start_timestamp;
          if (distanceFromStart < 1.0) {
            // Add a small offset (0.5 seconds) to prevent snapping to start
            const adjustedTime = segment.start_timestamp + 1.0;
            console.log(`TimelineCursor: Adjusting cursor position from ${timestamp} to ${adjustedTime} to prevent snapping to segment start`);
            timelineState.currentTime = adjustedTime;
          }
        }
      }
      timelineState.prevCurrentTime = timelineState.currentTime;
      timelineState.isPlaying = false;

      // Notify listeners
      timelineState.setState({});

      // Find segment that contains this timestamp
      const segments = timelineState.timelineSegments || [];
      console.log('TimelineCursor: Searching for segment containing timestamp', {
        timestamp,
        segmentsCount: segments.length
      });

      let foundSegment = false;
      let closestSegment = -1;
      let minDistance = Infinity;

      // First try to find an exact match
      for (let i = 0; i < segments.length; i++) {
        const segment = segments[i];
        // Use local timestamps if available, otherwise fall back to regular timestamps
        const startTimestamp = segment.local_start_timestamp || segment.start_timestamp;
        const endTimestamp = segment.local_end_timestamp || segment.end_timestamp;

        // Log the first few segments for debugging
        if (i < 3) {
          console.log(`TimelineCursor: Segment ${i}`, {
            startTimestamp,
            endTimestamp,
            startTime: new Date(startTimestamp * 1000).toLocaleTimeString(),
            endTime: new Date(endTimestamp * 1000).toLocaleTimeString()
          });
        }

        // Check if timestamp is within this segment
        if (timestamp >= startTimestamp && timestamp <= endTimestamp) {
          console.log(`TimelineCursor: Found exact match at segment ${i}`);
          // Update current segment index without changing the time or starting playback
          timelineState.currentSegmentIndex = i;
          timelineState.setState({});
          foundSegment = true;
          break;
        }

        // Calculate distance to this segment (for finding closest if no exact match)
        const midpoint = (startTimestamp + endTimestamp) / 2;
        const distance = Math.abs(timestamp - midpoint);
        if (distance < minDistance) {
          minDistance = distance;
          closestSegment = i;
        }
      }

      // If no exact match found, use the closest segment
      if (!foundSegment) {
        if (closestSegment >= 0) {
          console.log(`TimelineCursor: No exact match, using closest segment ${closestSegment}`);
          timelineState.currentSegmentIndex = closestSegment;
          timelineState.setState({});
        } else {
          console.log('TimelineCursor: No segments found at all');
          // Reset current segment index
          timelineState.currentSegmentIndex = -1;
          timelineState.setState({});
        }
      }
    };

    // Add event listeners
    cursor.addEventListener('mousedown', handleMouseDown);

    return () => {
      cursor.removeEventListener('mousedown', handleMouseDown);
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };
  }, [cursorRef.current, startHour, endHour, isDragging]);

  // Update cursor position
  const updateCursorPosition = (time, startHr, endHr) => {
    console.log('TimelineCursor: updateCursorPosition called', { time, startHr, endHr });

    if (time === null) {
      console.log('TimelineCursor: No current time, hiding cursor');
      setVisible(false);
      return;
    }

    // Calculate cursor position
    const date = new Date(time * 1000);
    const hour = date.getHours() + (date.getMinutes() / 60) + (date.getSeconds() / 3600);
    console.log('TimelineCursor: Calculated hour', { hour, timeString: date.toLocaleTimeString() });

    // Check if the current time is within the visible range
    if (hour < startHr || hour > endHr) {
      console.log('TimelineCursor: Time outside visible range, hiding cursor');
      setVisible(false);
      return;
    }

    // Calculate position as percentage
    const position = ((hour - startHr) / (endHr - startHr)) * 100;
    console.log('TimelineCursor: Calculated position', { position, hour, startHr, endHr });

    // Update cursor position
    setPosition(position);
    setVisible(true);
  };

  // Update time display
  const updateTimeDisplay = (time) => {
    if (time === null) return;

    const timeDisplay = document.getElementById('time-display');
    if (!timeDisplay) return;

    const date = new Date(time * 1000);

    // Format date and time
    const hours = date.getHours().toString().padStart(2, '0');
    const minutes = date.getMinutes().toString().padStart(2, '0');
    const seconds = date.getSeconds().toString().padStart(2, '0');

    // Display time only (HH:MM:SS)
    timeDisplay.textContent = `${hours}:${minutes}:${seconds}`;
  };

  // Force the cursor to be visible on initial render
  useEffect(() => {
    console.log('TimelineCursor: Initializing cursor position');
    console.log('TimelineCursor: Initial state', {
      currentTime: timelineState.currentTime,
      startHour: timelineState.timelineStartHour,
      endHour: timelineState.timelineEndHour,
      segments: timelineState.timelineSegments ? timelineState.timelineSegments.length : 0
    });

    // Function to initialize cursor
    const initCursor = () => {
      console.log('TimelineCursor: Checking timelineState directly', {
        currentTime: timelineState.currentTime,
        segmentsLength: timelineState.timelineSegments ? timelineState.timelineSegments.length : 0,
        currentSegmentIndex: timelineState.currentSegmentIndex
      });

      if (timelineState.currentTime) {
        console.log('TimelineCursor: Setting initial cursor position with current time');
        setVisible(true);
        updateCursorPosition(
          timelineState.currentTime,
          timelineState.timelineStartHour || 0,
          timelineState.timelineEndHour || 24
        );
        return true;
      } else if (timelineState.timelineSegments && timelineState.timelineSegments.length > 0) {
        // If no current time but we have segments, use the first segment's start time
        console.log('TimelineCursor: Using first segment start time for cursor');
        const firstSegment = timelineState.timelineSegments[0];
        const segmentStartTime = firstSegment.start_timestamp;

        // Update the timeline state with this time - DIRECT ASSIGNMENT
        console.log('TimelineCursor: Directly setting timelineState properties');
        timelineState.currentTime = segmentStartTime;
        timelineState.currentSegmentIndex = 0;

        // Now call setState to notify listeners
        timelineState.setState({
          // Empty object just to trigger notification
        });

        setVisible(true);
        updateCursorPosition(
          segmentStartTime,
          timelineState.timelineStartHour || 0,
          timelineState.timelineEndHour || 24
        );
        return true;
      }
      return false;
    };

    // Try to initialize immediately
    const initialized = initCursor();

    // If not initialized, try again after a delay
    if (!initialized) {
      console.log('TimelineCursor: Initial initialization failed, will retry after delay');

      // Set up multiple attempts with increasing delays
      const delays = [100, 300, 500, 1000];

      delays.forEach((delay, index) => {
        setTimeout(() => {
          if (!visible) {
            console.log(`TimelineCursor: Retry initialization attempt ${index + 1}`);
            initCursor();
          }
        }, delay);
      });
    }
  }, []);

  return (
    <div
      ref={cursorRef}
      className="timeline-cursor absolute top-0 h-full z-50 transition-all duration-100 cursor-ew-resize"
      style={{
        left: `${position}%`,
        display: visible ? 'block' : 'none',
        pointerEvents: 'auto',
        width: '7px',
        marginLeft: '-3.5px'
      }}
    >
      {/* Invisible wider clickable area */}
      <div className="absolute top-0 bottom-0 left-0 w-full"></div>

      {/* Skinnier needle with no middle chunk - perfectly centered */}
      <div className="absolute top-0 bottom-0 w-0.5 bg-orange-500 left-0 right-0 mx-auto pointer-events-none"></div>

      {/* Top handle (black) - perfectly centered */}
      <div className="absolute top-0 left-0 right-0 mx-auto w-4 h-4 bg-black rounded-full transform -translate-y-1/2 shadow-md pointer-events-none"></div>

      {/* Bottom handle (black) - perfectly centered */}
      <div className="absolute bottom-0 left-0 right-0 mx-auto w-4 h-4 bg-black rounded-full transform translate-y-1/2 shadow-md pointer-events-none"></div>
    </div>
  );
}
