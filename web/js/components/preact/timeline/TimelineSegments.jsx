/**
 * LightNVR Timeline Segments Component
 * Displays recording segments on the timeline
 */

import { useState, useEffect, useRef } from 'preact/hooks';
import { timelineState } from './TimelinePage.jsx';
import {
  findContainingSegmentIndex,
  findNearestSegmentIndex,
  formatPlaybackTimeLabel,
  getTimelinePreviewFrameIndex,
  getClippedSegmentHourRange
} from './timelineUtils.js';
import { formatLocalTime } from '../../../utils/date-utils.js';
import { useI18n } from '../../../i18n.js';

/**
 * TimelineSegments component
 * @param {Object} props Component props
 * @param {Array} props.segments Array of timeline segments
 * @param {Array} props.detectionIntervals Exact external-motion ranges
 * @returns {JSX.Element} TimelineSegments component
 */
export function TimelineSegments({ segments: propSegments, detectionIntervals: propIntervals = [] }) {
  const { t } = useI18n();
  // Local state
  const [segments, setSegments] = useState(propSegments || []);
  const [detectionIntervals, setDetectionIntervals] = useState(propIntervals || []);
  const [startHour, setStartHour] = useState(0);
  const [endHour, setEndHour] = useState(24);
  const [dragPreview, setDragPreview] = useState(null);
  const currentSegmentIndexRef = useRef(-1);

  // Update segments when props change (including when cleared to empty on deletion)
  useEffect(() => {
    if (Array.isArray(propSegments)) {
      setSegments(propSegments);
    }
  }, [propSegments]);

  useEffect(() => {
    setDetectionIntervals(Array.isArray(propIntervals) ? propIntervals : []);
  }, [propIntervals]);

  // Refs
  const containerRef = useRef(null);
  const isDragging = useRef(false);
  // Capture playback state at drag-start so we can preserve it across the seek
  // (standard scrubber UX: clicking/dragging while playing keeps playing).
  const wasPlayingAtDragStartRef = useRef(false);
  const pendingSeekRef = useRef(null);
  const lastSegmentsRef = useRef([]);

  // Subscribe to timeline state changes
  useEffect(() => {
    const unsubscribe = timelineState.subscribe(state => {
      // Update segments when they change
      if (state.timelineSegments) {
        const changed = state.forceReload
          || state.timelineSegments !== lastSegmentsRef.current;
        if (changed) {
          setSegments(state.timelineSegments);
          lastSegmentsRef.current = state.timelineSegments;
        }
      }

      setStartHour(state.timelineStartHour ?? 0);
      setEndHour(state.timelineEndHour ?? 24);
      currentSegmentIndexRef.current = state.currentSegmentIndex ?? -1;
    });

    // Hydrate from global state on mount
    if (timelineState.timelineSegments && timelineState.timelineSegments.length > 0) {
      setSegments(timelineState.timelineSegments);
      lastSegmentsRef.current = timelineState.timelineSegments;
      currentSegmentIndexRef.current = timelineState.currentSegmentIndex ?? -1;
      if (timelineState.timelineStartHour !== undefined) setStartHour(timelineState.timelineStartHour);
      if (timelineState.timelineEndHour !== undefined)   setEndHour(timelineState.timelineEndHour);
    }

    return () => unsubscribe();
  }, []);

  // Set up drag handling
  useEffect(() => {
    const container = containerRef.current;
    if (!container) return;

    const handleMouseDown = (e) => {
      // Handle clicks on the container, clickable area, or directly on segments
      const target = e.target;
      const isElementTarget = target instanceof Element;
      if (
        target === container ||
        (isElementTarget &&
          (target.classList.contains('timeline-clickable-area') ||
            target.classList.contains('timeline-segment') ||
            target.classList.contains('timeline-detection-interval')))
      ) {
        // Remember whether we were playing so we can resume after the seek.
        wasPlayingAtDragStartRef.current = !!timelineState.isPlaying;
        const pendingSeek = getTimelineClickTarget(e);
        if (!pendingSeek) return;
        isDragging.current = true;
        pendingSeekRef.current = pendingSeek;
        showDragPreview(pendingSeek);
        timelineState.setState({
          userControllingCursor: true,
          preserveCursorPosition: true,
          cursorPositionLocked: true,
        });

        // Add event listeners for drag
        document.addEventListener('mousemove', handleMouseMove);
        document.addEventListener('mouseup', handleMouseUp);
      }
    };

    const handleMouseMove = (e) => {
      if (!isDragging.current) return;
      const pendingSeek = getTimelineClickTarget(e);
      if (!pendingSeek) return;
      pendingSeekRef.current = pendingSeek;
      showDragPreview(pendingSeek);
    };

    const handleMouseUp = (event) => {
      if (!isDragging.current) {
        return;
      }
      const finalSeek = getTimelineClickTarget(event) || pendingSeekRef.current;
      isDragging.current = false;
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
      const shouldResume = wasPlayingAtDragStartRef.current;
      wasPlayingAtDragStartRef.current = false;
      pendingSeekRef.current = null;
      setDragPreview(null);

      if (!finalSeek) {
        timelineState.setState({
          userControllingCursor: false,
          preserveCursorPosition: false,
          cursorPositionLocked: false,
        });
        return;
      }

      timelineState.setState({
        currentTime: finalSeek.timestamp,
        prevCurrentTime: timelineState.currentTime,
        isPlaying: shouldResume && finalSeek.targetIndex >= 0,
        currentSegmentIndex: finalSeek.targetIndex,
        forceReload: finalSeek.targetIndex >= 0,
        userControllingCursor: false,
        preserveCursorPosition: false,
        cursorPositionLocked: false,
      });
    };

    container.addEventListener('mousedown', handleMouseDown);

    return () => {
      container.removeEventListener('mousedown', handleMouseDown);
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
      if (isDragging.current) {
        isDragging.current = false;
        pendingSeekRef.current = null;
        wasPlayingAtDragStartRef.current = false;
        timelineState.setState({
          userControllingCursor: false,
          preserveCursorPosition: false,
          cursorPositionLocked: false,
        });
      }
    };
  }, [startHour, endHour, segments]);

  const getTimelineClickTarget = (event) => {
    const container = containerRef.current;
    if (!container) return null;

    const rect = container.getBoundingClientRect();
    if (!rect.width) return null;
    const clickPercent = Math.min(1, Math.max(0, (event.clientX - rect.left) / rect.width));
    const clickHour = startHour + clickPercent * (endHour - startHour);

    // Convert fractional hour → timestamp using the shared utility
    const clickTimestamp = timelineState.timelineHourToTimestamp(clickHour, timelineState.selectedDate);

    // Find segment that contains this timestamp
    const foundIndex = findContainingSegmentIndex(segments, clickTimestamp);
    const targetIndex = foundIndex !== -1
      ? foundIndex
      : findNearestSegmentIndex(segments, clickTimestamp);

    return { timestamp: clickTimestamp, targetIndex, positionPercent: clickPercent * 100 };
  };

  const showDragPreview = ({ timestamp, targetIndex, positionPercent }) => {
    const segment = segments[targetIndex];
    if (!segment || segment.id === null || segment.id === undefined) {
      setDragPreview(null);
      return;
    }
    const frameIndex = getTimelinePreviewFrameIndex(segment, timestamp, 3);
    setDragPreview({
      frameIndex,
      label: formatPlaybackTimeLabel(timestamp, segment.stream_name || segment.stream || ''),
      positionPercent,
      recordingId: segment.id,
      url: `/api/recordings/thumbnail/${encodeURIComponent(segment.id)}/${frameIndex}`,
    });
  };

  // ── Merge adjacent segments (gap ≤ 1 s) and render ──
  const renderSegments = () => {
    if (!segments || segments.length === 0) {
      return (
        <div className="absolute inset-0 flex items-center justify-center text-muted-foreground text-sm">
          No segments to display
        </div>
      );
    }

    const hourRange = endHour - startHour;
    if (hourRange <= 0) return null;

    // Sort + merge adjacent segments
    const sorted = [...segments].sort((a, b) => a.start_timestamp - b.start_timestamp);
    const merged = [];
    let cur = { ...sorted[0] };

    for (let i = 1; i < sorted.length; i++) {
      const seg = sorted[i];
      // Only merge adjacent segments that share the same detection state, otherwise
      // a continuous recording collapses into one bar and the has-detection colour is
      // OR-ed across the whole span — hiding *which* periods actually had detections
      // (issue #454). Keeping detection and non-detection runs separate preserves the
      // per-period highlighting the timeline is meant to show.
      if (seg.start_timestamp - cur.end_timestamp <= 1 &&
          !!seg.has_detection === !!cur.has_detection &&
          !seg._removing && !cur._removing) {
        // extend current merged segment
        cur.end_timestamp = Math.max(cur.end_timestamp, seg.end_timestamp);
      } else {
        merged.push(cur);
        cur = { ...seg };
      }
    }
    merged.push(cur);

    // Render each merged segment as a positioned bar
    const rendered = [];
    merged.forEach((seg, i) => {
      const visibleRange = getClippedSegmentHourRange(seg, timelineState.selectedDate);
      if (!visibleRange) return;

      const sh = visibleRange.startHour;
      const eh = visibleRange.endHour;

      // Clip to visible range
      if (eh <= startHour || sh >= endHour) return;
      const vStart = Math.max(sh, startHour);
      const vEnd   = Math.min(eh, endHour);

      const leftPct  = ((vStart - startHour) / hourRange) * 100;
      const widthPct = ((vEnd - vStart) / hourRange) * 100;

      // Tooltip
      const t0 = formatLocalTime(seg.start_timestamp);
      const t1 = formatLocalTime(seg.end_timestamp);
      const dur = Math.round(seg.end_timestamp - seg.start_timestamp);
      const durLabel = dur >= 3600
        ? `${Math.floor(dur / 3600)}h ${Math.floor((dur % 3600) / 60)}m`
        : dur >= 60
          ? `${Math.floor(dur / 60)}m ${dur % 60}s`
          : `${dur}s`;

      const hasExactInterval = detectionIntervals.some(interval =>
        interval.start_timestamp <= seg.end_timestamp &&
        interval.end_timestamp >= seg.start_timestamp
      );

      rendered.push(
        <div
          key={`seg-${i}`}
          className={`timeline-segment ${seg.has_detection && !hasExactInterval ? 'has-detection' : ''} ${seg._removing ? 'removing' : ''}`}
          style={{
            left: `${leftPct}%`,
            width: `${Math.max(widthPct, 0.15)}%`,   // min width so tiny segments stay visible
          }}
          title={`${t0} – ${t1}  (${durLabel})${seg.has_detection ? `  • ${t('timeline.detectionEvent')}` : ''}`}
        />
      );
    });

    detectionIntervals.forEach((interval, i) => {
      const visibleRange = getClippedSegmentHourRange(interval, timelineState.selectedDate);
      if (!visibleRange) return;
      const vStart = Math.max(visibleRange.startHour, startHour);
      const vEnd = Math.min(visibleRange.endHour, endHour);
      if (vEnd < startHour || vStart > endHour) return;
      const leftPct = ((vStart - startHour) / hourRange) * 100;
      const widthPct = ((vEnd - vStart) / hourRange) * 100;
      const t0 = formatLocalTime(interval.start_timestamp);
      const t1 = formatLocalTime(interval.end_timestamp);
      rendered.push(
        <div
          key={`detection-interval-${i}`}
          className="timeline-detection-interval"
          style={{
            left: `${leftPct}%`,
            width: `${Math.max(widthPct, 0.15)}%`
          }}
          title={`${t0} – ${t1} • ${t('timeline.detectionEvent')}`}
        />
      );
    });

    return rendered;
  };

  return (
    <div
      className="timeline-segments relative w-full h-16"
      ref={containerRef}
      aria-label="Recording timeline"
    >
      {renderSegments()}
      {dragPreview && (
        <div
          className="pointer-events-none absolute top-1 z-[60] rounded bg-black/85 p-1 text-center text-white shadow-lg"
          style={{
            left: `clamp(0px, calc(${dragPreview.positionPercent}% - 36px), calc(100% - 72px))`,
            width: '72px',
          }}
          aria-hidden="true"
        >
          <img
            src={dragPreview.url}
            alt=""
            className="h-9 w-16 rounded object-cover"
            onError={(event) => {
              if (dragPreview.frameIndex !== 0) {
                setDragPreview((current) => current && ({
                  ...current,
                  frameIndex: 0,
                  url: `/api/recordings/thumbnail/${encodeURIComponent(current.recordingId)}/0`,
                }));
              } else {
                event.currentTarget.style.visibility = 'hidden';
              }
            }}
          />
          <span className="block truncate text-[9px] leading-3">{dragPreview.label}</span>
        </div>
      )}
    </div>
  );
}
