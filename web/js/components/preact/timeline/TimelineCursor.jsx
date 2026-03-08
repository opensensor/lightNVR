/**
 * LightNVR Timeline Cursor Component
 * Displays the playback cursor on the timeline
 */

import { useState, useEffect, useRef } from 'preact/hooks';
import { timelineState } from './TimelinePage.jsx';
import {
  findContainingSegmentIndex,
  findNearestSegmentIndex,
  formatTimestampAsClock
} from './timelineUtils.js';

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

  // Refs — use refs for values read inside event-handler closures so they
  // always see the latest value without needing to re-attach listeners.
  const cursorRef = useRef(null);
  const isDraggingRef = useRef(false);
  const startHourRef = useRef(startHour);
  const endHourRef = useRef(endHour);
  startHourRef.current = startHour;
  endHourRef.current = endHour;

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
      setStartHour(state.timelineStartHour || 0);
      setEndHour(state.timelineEndHour || 24);

      // Only update current time if not dragging
      if (!isDraggingRef.current && !state.userControllingCursor) {
        updateTimeDisplay(state.currentTime);
        debouncedUpdateCursorPosition(state.currentTime, state.timelineStartHour || 0, state.timelineEndHour || 24);
      }
    });

    return () => unsubscribe();
  }, [debouncedUpdateCursorPosition]);

  // Set up drag handling
  useEffect(() => {
    const cursor = cursorRef.current;
    if (!cursor) return;

    const handleMouseDown = (e) => {
      e.preventDefault();
      e.stopPropagation();
      isDraggingRef.current = true;

      timelineState.userControllingCursor = true;
      timelineState.preserveCursorPosition = true;
      timelineState.cursorPositionLocked = true;
      timelineState.setState({});

      document.addEventListener('mousemove', handleMouseMove);
      document.addEventListener('mouseup', handleMouseUp);
    };

    const handleMouseMove = (e) => {
      if (!isDraggingRef.current) return;

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
      const hourRange = endHourRef.current - startHourRef.current;
      const hour = startHourRef.current + (positionPercent / 100) * hourRange;

      // Convert hour to timestamp using the utility function
      const timestamp = timelineState.timelineHourToTimestamp(hour, timelineState.selectedDate);

      // Update time display
      updateTimeDisplay(timestamp);
    };

    const handleMouseUp = (e) => {
      if (!isDraggingRef.current) return;

      const container = cursor.parentElement;
      if (!container) return;

      const rect = container.getBoundingClientRect();
      const clickX = Math.max(0, Math.min(e.clientX - rect.left, rect.width));
      const positionPercent = (clickX / rect.width) * 100;

      const hourRange = endHourRef.current - startHourRef.current;
      const hour = startHourRef.current + (positionPercent / 100) * hourRange;
      const timestamp = timelineState.timelineHourToTimestamp(hour, timelineState.selectedDate);

      // Reset dragging state
      isDraggingRef.current = false;
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);

      // Release cursor control after a short delay
      setTimeout(() => {
        timelineState.userControllingCursor = false;
        timelineState.setState({});
      }, 100);

      // Set the cursor time
      timelineState.currentTime = timestamp;

      // Snap-guard: nudge away from segment start to prevent snap-back
      if (timelineState.timelineSegments && timelineState.timelineSegments.length > 0) {
        const segIndex = findContainingSegmentIndex(timelineState.timelineSegments, timestamp);
        const seg = segIndex !== -1 ? timelineState.timelineSegments[segIndex] : null;
        if (seg && (timestamp - seg.start_timestamp) < 1.0) {
          timelineState.currentTime = seg.start_timestamp + 1.0;
        }
      }

      timelineState.prevCurrentTime = timelineState.currentTime;
      timelineState.isPlaying = false;
      timelineState.setState({});

      // Find the segment at the drop position (exact match or closest)
      const segs = timelineState.timelineSegments || [];
      const containingIndex = findContainingSegmentIndex(segs, timestamp);
      timelineState.currentSegmentIndex = containingIndex !== -1
        ? containingIndex
        : findNearestSegmentIndex(segs, timestamp);
      timelineState.setState({});
    };

    // Add event listeners
    cursor.addEventListener('mousedown', handleMouseDown);

    return () => {
      cursor.removeEventListener('mousedown', handleMouseDown);
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };
  }, []);

  // Update cursor position
  const updateCursorPosition = (time, startHr, endHr) => {
    if (time === null) {
      setVisible(false);
      return;
    }

    const date = new Date(time * 1000);
    const hour = date.getHours() + date.getMinutes() / 60 + date.getSeconds() / 3600;

    if (hour < startHr || hour > endHr) {
      setVisible(false);
      return;
    }

    setPosition(((hour - startHr) / (endHr - startHr)) * 100);
    setVisible(true);
  };

  // Update time display
  const updateTimeDisplay = (time) => {
    if (time === null) return;

    const timeDisplay = document.getElementById('time-display');
    if (!timeDisplay) return;

    timeDisplay.textContent = formatTimestampAsClock(time);
  };

  // Initialise cursor on mount (with retries for async data)
  useEffect(() => {
    const initCursor = () => {
      if (timelineState.currentTime) {
        setVisible(true);
        updateCursorPosition(
          timelineState.currentTime,
          timelineState.timelineStartHour || 0,
          timelineState.timelineEndHour || 24
        );
        return true;
      }
      if (timelineState.timelineSegments && timelineState.timelineSegments.length > 0) {
        const t = timelineState.timelineSegments[0].start_timestamp;
        timelineState.currentTime = t;
        timelineState.currentSegmentIndex = 0;
        timelineState.setState({});
        setVisible(true);
        updateCursorPosition(t, timelineState.timelineStartHour || 0, timelineState.timelineEndHour || 24);
        return true;
      }
      return false;
    };

    if (!initCursor()) {
      // Retry a few times for async data arrival
      [100, 300, 500, 1000].forEach(delay => {
        setTimeout(() => { if (!visible) initCursor(); }, delay);
      });
    }
  }, []);

  return (
    <div
      ref={cursorRef}
      className="timeline-cursor absolute top-0 h-full z-50 cursor-ew-resize"
      style={{
        left: `${position}%`,
        display: visible ? 'block' : 'none',
        pointerEvents: 'auto',
        width: '18px',
        marginLeft: '-9px'
      }}
    >
      {/* Invisible hit-area */}
      <div className="absolute inset-0" />

      {/* Thin vertical line — full height */}
      <div
        className="pointer-events-none absolute top-0 bottom-0"
        style={{
          left: '50%',
          width: '1.5px',
          marginLeft: '-0.75px',
          background: '#ef6c00'
        }}
      />

      {/* Thumb — small rounded pill pinned to top of ruler */}
      <div
        className="pointer-events-none absolute"
        style={{
          left: '50%',
          top: '0px',
          transform: 'translateX(-50%)',
          width: '10px',
          height: '18px',
          borderRadius: '3px',
          background: '#ef6c00',
          boxShadow: '0 1px 3px rgba(0,0,0,0.35)'
        }}
      />
    </div>
  );
}
