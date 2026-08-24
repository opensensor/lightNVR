/**
 * LightNVR Timeline Cursor Component
 * Displays the playback cursor on the timeline
 */

import { useState, useEffect, useRef } from 'preact/hooks';
import { timelineState } from './TimelinePage.jsx';
import { useI18n } from '../../../i18n.js';
import {
  findContainingSegmentIndex,
  findNearestSegmentIndex,
  formatPlaybackTimeLabel,
  getLocalDayBounds,
  getTimelineDayLengthHours,
  resolvePlaybackStreamName,
  snapTimestampToRecordingEdge,
  timestampToTimelineOffset
} from './timelineUtils.js';

/**
 * TimelineCursor component
 * @returns {JSX.Element} TimelineCursor component
 */
export function TimelineCursor() {
  const { t } = useI18n();
  // Local state
  const [position, setPosition] = useState(0);
  const [visible, setVisible] = useState(false);
  const [startHour, setStartHour] = useState(0);
  const [endHour, setEndHour] = useState(24);
  const [currentTime, setCurrentTime] = useState(timelineState.currentTime);
  const [selectedDate, setSelectedDate] = useState(timelineState.selectedDate);

  // Refs — use refs for values read inside event-handler closures so they
  // always see the latest value without needing to re-attach listeners.
  const cursorRef = useRef(null);
  const isDraggingRef = useRef(false);
  const activePointerIdRef = useRef(null);
  const pointerGrabOffsetRef = useRef(0);
  // Capture playback state at drag-start so we can resume after the seek
  // (standard scrubber UX: dragging while playing should keep playing).
  const wasPlayingAtDragStartRef = useRef(false);
  const startHourRef = useRef(startHour);
  const endHourRef = useRef(endHour);
  const positionRef = useRef(position);
  startHourRef.current = startHour;
  endHourRef.current = endHour;
  positionRef.current = position;

  // Subscribe to timeline state changes
  useEffect(() => {
    const unsubscribe = timelineState.subscribe(state => {
      setStartHour(state.timelineStartHour || 0);
      setEndHour(state.timelineEndHour || getTimelineDayLengthHours(state.selectedDate));
      setCurrentTime(state.currentTime);
      setSelectedDate(state.selectedDate);

      // Only update current time if not dragging
      if (!isDraggingRef.current && !state.userControllingCursor) {
        updateTimeDisplay(state.currentTime);
        updateCursorPosition(
          state.currentTime,
          state.timelineStartHour || 0,
          state.timelineEndHour || getTimelineDayLengthHours(state.selectedDate)
        );
      }
    });

    return () => unsubscribe();
  }, []);

  // Set up drag handling
  useEffect(() => {
    const cursor = cursorRef.current;
    if (!cursor) return;

    const resetCursorControl = () => {
      timelineState.setState({
        userControllingCursor: false,
        preserveCursorPosition: false,
        cursorPositionLocked: false
      });
    };

    const releaseActivePointer = (pointerId) => {
      if (pointerId === null || pointerId === undefined) {
        return;
      }
      try {
        if (cursor.hasPointerCapture(pointerId)) {
          cursor.releasePointerCapture(pointerId);
        }
      } catch (error) {
        // Pointer capture is optional on older embedded WebViews.
      }
    };

    const handlePointerDown = (e) => {
      if (
        isDraggingRef.current ||
        !e.isPrimary ||
        (e.pointerType === 'mouse' && e.button !== 0)
      ) {
        return;
      }
      e.preventDefault();
      e.stopPropagation();
      // Capture playback state at drag-start so we can resume after release.
      wasPlayingAtDragStartRef.current = !!timelineState.isPlaying;
      isDraggingRef.current = true;
      activePointerIdRef.current = e.pointerId;

      // The 36 px hit target is clamped inside the track at either edge while
      // the visible line remains on the exact timestamp. Preserve where the
      // pointer grabbed that line so a down/up without movement cannot seek.
      const container = cursor.parentElement;
      if (container) {
        const rect = container.getBoundingClientRect();
        const lineX = rect.left + (rect.width * positionRef.current / 100);
        pointerGrabOffsetRef.current = e.clientX - lineX;
      } else {
        pointerGrabOffsetRef.current = 0;
      }

      try {
        cursor.setPointerCapture(e.pointerId);
      } catch (error) {
        // Pointer capture is best-effort on older embedded WebViews. The
        // document listeners below keep the drag working when it is unavailable.
      }

      timelineState.setState({
        userControllingCursor: true,
        preserveCursorPosition: true,
        cursorPositionLocked: true
      });

      try {
        cursor.focus({ preventScroll: true });
      } catch (error) {
        cursor.focus();
      }

      document.addEventListener('pointermove', handlePointerMove);
      document.addEventListener('pointerup', handlePointerUp);
      document.addEventListener('pointercancel', handlePointerCancel);
    };

    const handlePointerMove = (e) => {
      if (!isDraggingRef.current || e.pointerId !== activePointerIdRef.current) return;
      e.preventDefault();

      // Get container dimensions
      const container = cursor.parentElement;
      if (!container) return;

      const rect = container.getBoundingClientRect();
      const clickX = Math.max(
        0,
        Math.min(e.clientX - pointerGrabOffsetRef.current - rect.left, rect.width)
      );
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

    const finishPointerDrag = (e, cancelled = false) => {
      if (!isDraggingRef.current || e.pointerId !== activePointerIdRef.current) return;

      e.preventDefault();
      e.stopPropagation();

      const pointerId = activePointerIdRef.current;
      isDraggingRef.current = false;
      activePointerIdRef.current = null;
      document.removeEventListener('pointermove', handlePointerMove);
      document.removeEventListener('pointerup', handlePointerUp);
      document.removeEventListener('pointercancel', handlePointerCancel);
      releaseActivePointer(pointerId);

      if (cancelled) {
        wasPlayingAtDragStartRef.current = false;
        pointerGrabOffsetRef.current = 0;
        resetCursorControl();
        return;
      }

      const container = cursor.parentElement;
      if (!container) {
        wasPlayingAtDragStartRef.current = false;
        pointerGrabOffsetRef.current = 0;
        resetCursorControl();
        return;
      }

      const rect = container.getBoundingClientRect();
      const clickX = Math.max(
        0,
        Math.min(e.clientX - pointerGrabOffsetRef.current - rect.left, rect.width)
      );
      pointerGrabOffsetRef.current = 0;
      const positionPercent = (clickX / rect.width) * 100;

      const hourRange = endHourRef.current - startHourRef.current;
      const hour = startHourRef.current + (positionPercent / 100) * hourRange;
      const timestamp = timelineState.timelineHourToTimestamp(hour, timelineState.selectedDate);

      const edgeSnap = snapTimestampToRecordingEdge(
        timestamp,
        timelineState.timelineSegments,
        { selectedDate: timelineState.selectedDate }
      );
      let targetTimestamp = edgeSnap.timestamp;

      // Preserve the existing start guard when this was not an intentional
      // edge snap. An actual snap must remain exactly on the recording edge.
      if (!edgeSnap.snapped && timelineState.timelineSegments && timelineState.timelineSegments.length > 0) {
        const segIndex = findContainingSegmentIndex(timelineState.timelineSegments, timestamp);
        const seg = segIndex !== -1 ? timelineState.timelineSegments[segIndex] : null;
        if (seg && (timestamp - seg.start_timestamp) < 1.0) {
          targetTimestamp = seg.start_timestamp + 1.0;
        }
      }

      // Find the segment at the drop position (exact match or closest)
      const segs = timelineState.timelineSegments || [];
      const containingIndex = findContainingSegmentIndex(segs, targetTimestamp);
      const targetSegmentIndex = containingIndex !== -1
        ? containingIndex
        : findNearestSegmentIndex(segs, targetTimestamp);

      const shouldResume = wasPlayingAtDragStartRef.current;
      wasPlayingAtDragStartRef.current = false;

      timelineState.setState({
        currentTime: targetTimestamp,
        prevCurrentTime: timelineState.currentTime,
        currentSegmentIndex: targetSegmentIndex,
        isPlaying: shouldResume && targetSegmentIndex >= 0,
        userControllingCursor: false,
        preserveCursorPosition: false,
        cursorPositionLocked: false,
        forceReload: targetSegmentIndex >= 0
      });
    };

    const handlePointerUp = (e) => finishPointerDrag(e, false);
    const handlePointerCancel = (e) => finishPointerDrag(e, true);

    // Add event listeners
    cursor.addEventListener('pointerdown', handlePointerDown);

    return () => {
      const pointerId = activePointerIdRef.current;
      cursor.removeEventListener('pointerdown', handlePointerDown);
      document.removeEventListener('pointermove', handlePointerMove);
      document.removeEventListener('pointerup', handlePointerUp);
      document.removeEventListener('pointercancel', handlePointerCancel);
      if (isDraggingRef.current) {
        isDraggingRef.current = false;
        activePointerIdRef.current = null;
        pointerGrabOffsetRef.current = 0;
        wasPlayingAtDragStartRef.current = false;
        releaseActivePointer(pointerId);
        resetCursorControl();
      }
    };
  }, []);

  // Update cursor position
  const updateCursorPosition = (time, startHr, endHr) => {
    if (time === null) {
      setVisible(false);
      return;
    }

    const hour = timestampToTimelineOffset(time, timelineState.selectedDate);
    if (hour === null) {
      setVisible(false);
      return;
    }

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

    const streamName = resolvePlaybackStreamName(
      timelineState.timelineSegments,
      timelineState.currentSegmentIndex,
      time
    );
    timeDisplay.textContent = formatPlaybackTimeLabel(time, streamName) || '00:00:00';
  };

  // Initialise cursor on mount (with retries for async data)
  useEffect(() => {
    const initCursor = () => {
      if (timelineState.currentTime) {
        setVisible(true);
        updateCursorPosition(
          timelineState.currentTime,
          timelineState.timelineStartHour || 0,
          timelineState.timelineEndHour || getTimelineDayLengthHours(timelineState.selectedDate)
        );
        return true;
      }
      if (timelineState.timelineSegments && timelineState.timelineSegments.length > 0) {
        const t = timelineState.timelineSegments[0].start_timestamp;
        timelineState.currentTime = t;
        timelineState.currentSegmentIndex = 0;
        timelineState.setState({});
        setVisible(true);
        updateCursorPosition(
          t,
          timelineState.timelineStartHour || 0,
          timelineState.timelineEndHour || getTimelineDayLengthHours(timelineState.selectedDate)
        );
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

  const dayBounds = getLocalDayBounds(selectedDate);
  const ariaMinimum = dayBounds?.startTimestamp ?? 0;
  const ariaMaximum = dayBounds ? dayBounds.endTimestamp - 0.001 : 0;
  const ariaValue = Number.isFinite(currentTime)
    ? Math.min(Math.max(currentTime, ariaMinimum), ariaMaximum)
    : ariaMinimum;
  const streamName = resolvePlaybackStreamName(
    timelineState.timelineSegments,
    timelineState.currentSegmentIndex,
    currentTime
  );
  const ariaValueText = formatPlaybackTimeLabel(currentTime, streamName) || '00:00:00';

  return (
    <>
      <div
        ref={cursorRef}
        className="timeline-cursor absolute top-0 z-50 cursor-ew-resize focus:outline-none focus-visible:ring-2 focus-visible:ring-primary focus-visible:ring-offset-1"
        data-testid="timeline-cursor-hit-area"
        data-keyboard-nav-preserve
        role="slider"
        tabIndex={visible ? 0 : -1}
        aria-label={t('timeline.help.dragPlayhead')}
        aria-orientation="horizontal"
        aria-valuemin={ariaMinimum}
        aria-valuemax={ariaMaximum}
        aria-valuenow={ariaValue}
        aria-valuetext={ariaValueText}
        aria-keyshortcuts="ArrowLeft ArrowRight Space"
        style={{
          left: `clamp(0px, calc(${position}% - 18px), calc(100% - 36px))`,
          display: visible ? 'block' : 'none',
          pointerEvents: 'auto',
          width: '36px',
          height: '36px',
          touchAction: 'none'
        }}
      />

      {/* Visible line stays on the exact timestamp while the hit target above
          clamps inside the container at the start and end of the day. */}
      <div
        className="pointer-events-none absolute top-0 z-40"
        data-testid="timeline-cursor-line"
        aria-hidden="true"
        style={{
          left: `${position}%`,
          display: visible ? 'block' : 'none',
          width: '4px',
          height: '6rem',
          transform: 'translateX(-50%)',
          background: '#ef6c00'
        }}
      />

      {/* Thumb — small rounded pill pinned to top of ruler */}
      <div
        className="pointer-events-none absolute z-40"
        data-testid="timeline-cursor-thumb"
        aria-hidden="true"
        style={{
          left: `${position}%`,
          display: visible ? 'block' : 'none',
          top: '0px',
          transform: 'translateX(-50%)',
          width: '10px',
          height: '18px',
          borderRadius: '3px',
          background: '#ef6c00',
          boxShadow: '0 1px 3px rgba(0,0,0,0.35)'
        }}
      />
    </>
  );
}
