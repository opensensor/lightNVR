/**
 * LightNVR Timeline Segments Component
 * Displays recording segments on the timeline
 */

import { useState, useEffect, useRef } from 'preact/hooks';
import { timelineState } from './TimelinePage.jsx';

/**
 * TimelineSegments component
 * @param {Object} props Component props
 * @param {Array} props.segments Array of timeline segments
 * @returns {JSX.Element} TimelineSegments component
 */
export function TimelineSegments({ segments: propSegments }) {
  // Local state
  const [segments, setSegments] = useState(propSegments || []);
  const [startHour, setStartHour] = useState(0);
  const [endHour, setEndHour] = useState(24);
  const [currentSegmentIndex, setCurrentSegmentIndex] = useState(-1);

  // Update segments when props change
  useEffect(() => {
    console.log(`TimelineSegments: Received segments from props: ${propSegments ? propSegments.length : 0}`);
    if (propSegments && propSegments.length > 0) {
      setSegments(propSegments);
    }
  }, [propSegments]);

  // Refs
  const containerRef = useRef(null);
  const isDragging = useRef(false);

  // Track the last time segments were updated to prevent too frequent updates
  const lastSegmentsUpdateRef = useRef(0);
  const lastSegmentsRef = useRef([]);

  // Subscribe to timeline state changes
  useEffect(() => {
    const unsubscribe = timelineState.subscribe(state => {
      console.log(`TimelineSegments: State update received, segments: ${state.timelineSegments ? state.timelineSegments.length : 0}`);

      // Always update segments when they change
      if (state.timelineSegments) {
        // Check if segments have changed
        const segmentsChanged = !lastSegmentsRef.current ||
                               state.timelineSegments.length !== lastSegmentsRef.current.length ||
                               JSON.stringify(state.timelineSegments) !== JSON.stringify(lastSegmentsRef.current) ||
                               state.forceReload;

        if (segmentsChanged) {
          console.log(`TimelineSegments: Updating segments (${state.timelineSegments.length})`);
          setSegments(state.timelineSegments);
          lastSegmentsRef.current = [...state.timelineSegments]; // Create a copy
          lastSegmentsUpdateRef.current = Date.now();
        }
      }

      // Always update these lightweight properties
      const newStartHour = state.timelineStartHour !== undefined ? state.timelineStartHour : 0;
      const newEndHour = state.timelineEndHour !== undefined ? state.timelineEndHour : 24;

      console.log(`TimelineSegments: Time range update - startHour: ${newStartHour}, endHour: ${newEndHour}`);

      setStartHour(newStartHour);
      setEndHour(newEndHour);
      setCurrentSegmentIndex(state.currentSegmentIndex || -1);
    });

    // Initial load of segments
    if (timelineState.timelineSegments && timelineState.timelineSegments.length > 0) {
      console.log(`TimelineSegments: Initial load of segments (${timelineState.timelineSegments.length})`);
      setSegments(timelineState.timelineSegments);
      lastSegmentsRef.current = [...timelineState.timelineSegments];
      setCurrentSegmentIndex(timelineState.currentSegmentIndex || 0);

      // Also set the time range
      if (timelineState.timelineStartHour !== undefined) {
        setStartHour(timelineState.timelineStartHour);
      }
      if (timelineState.timelineEndHour !== undefined) {
        setEndHour(timelineState.timelineEndHour);
      }

      lastSegmentsUpdateRef.current = Date.now();
    }

    return () => unsubscribe();
  }, []);

  // Set up drag handling
  useEffect(() => {
    const container = containerRef.current;
    if (!container) return;

    const handleMouseDown = (e) => {
      // Only handle clicks on the container itself, not on segments
      if (e.target === container || e.target.classList.contains('timeline-clickable-area')) {
        isDragging.current = true;
        handleTimelineClick(e);

        // Add event listeners for drag
        document.addEventListener('mousemove', handleMouseMove);
        document.addEventListener('mouseup', handleMouseUp);
      }
    };

    const handleMouseMove = (e) => {
      if (!isDragging.current) return;
      handleTimelineClick(e);
    };

    const handleMouseUp = () => {
      isDragging.current = false;
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };

    container.addEventListener('mousedown', handleMouseDown);

    return () => {
      container.removeEventListener('mousedown', handleMouseDown);
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
    };
  }, [startHour, endHour, segments]);

  // Handle click on timeline for seeking
  const handleTimelineClick = (event) => {
    const container = containerRef.current;
    if (!container) return;

    // Get click position relative to container
    const rect = container.getBoundingClientRect();
    const clickX = event.clientX - rect.left;
    const containerWidth = rect.width;

    // Calculate time based on click position
    const clickPercent = clickX / containerWidth;
    const clickHour = startHour + (clickPercent * (endHour - startHour));

    // Find the segment that contains this time
    const clickDate = new Date(timelineState.selectedDate);
    clickDate.setHours(Math.floor(clickHour));
    clickDate.setMinutes(Math.floor((clickHour % 1) * 60));
    clickDate.setSeconds(Math.floor(((clickHour % 1) * 60) % 1 * 60));

    const clickTimestamp = clickDate.getTime() / 1000;

    // Always update the current time to where the user clicked
    // This allows the user to position the cursor anywhere on the timeline
    timelineState.setState({
      currentTime: clickTimestamp,
      prevCurrentTime: timelineState.currentTime,
      // Don't automatically start playing
      isPlaying: false
    });

    // Find segment that contains this timestamp
    let foundSegment = false;
    for (let i = 0; i < segments.length; i++) {
      const segment = segments[i];
      // Use local timestamps if available, otherwise fall back to regular timestamps
      const startTimestamp = segment.local_start_timestamp || segment.start_timestamp;
      const endTimestamp = segment.local_end_timestamp || segment.end_timestamp;

      if (clickTimestamp >= startTimestamp && clickTimestamp <= endTimestamp) {
        console.log(`TimelineSegments: Found segment ${i} containing timestamp`);

        // Update current segment index without starting playback
        timelineState.setState({
          currentSegmentIndex: i
        });

        // Only if the user clicked directly on a segment (not the background),
        // play that segment starting at the clicked time
        if (event.target.classList.contains('timeline-segment')) {
          // Calculate relative time within the segment
          const relativeTime = clickTimestamp - startTimestamp;

          // Play this segment starting at the clicked time
          playSegment(i, relativeTime);
        }

        foundSegment = true;
        break;
      }
    }

    if (!foundSegment) {
      // If no segment found, don't automatically jump to a different segment
      // Just leave the cursor where the user clicked
      timelineState.setState({
        currentSegmentIndex: -1
      });
    }
  };

  // Play a specific segment
  const playSegment = (index, relativeTime = null) => {
    console.log(`TimelineSegments: playSegment(${index}, ${relativeTime})`);

    if (index < 0 || index >= segments.length) {
      console.warn(`TimelineSegments: Invalid segment index: ${index}`);
      return;
    }

    const segment = segments[index];

    // Use local timestamps if available, otherwise fall back to regular timestamps
    const startTimestamp = segment.local_start_timestamp || segment.start_timestamp;

    // Calculate absolute timestamp for currentTime
    const absoluteTime = relativeTime !== null
      ? startTimestamp + relativeTime
      : startTimestamp;

    // First, pause any current playback and reset the segment index
    timelineState.setState({
      isPlaying: false,
      currentSegmentIndex: -1
    });

    // Force a synchronous DOM update
    document.body.offsetHeight;

    // Now set the new segment index and start playing
    setTimeout(() => {
      timelineState.setState({
        currentSegmentIndex: index,
        currentTime: absoluteTime,
        isPlaying: true,
        forceReload: true
      });

      // Force the video player to reload
      setTimeout(() => {
        const videoElement = document.querySelector('#video-player video');
        if (videoElement) {
          // Pause any current playback
          videoElement.pause();

          // Clear the source and reload
          videoElement.removeAttribute('src');
          videoElement.load();

          // Set the new source
          videoElement.src = `/api/recordings/play/${segment.id}?t=${Date.now()}`;

          // Set the current time and play
          videoElement.onloadedmetadata = () => {
            const seekTime = relativeTime !== null ? relativeTime : 0;
            videoElement.currentTime = seekTime;
            videoElement.play().catch(e => console.error('Error playing video:', e));
          };
        }
      }, 50);
    }, 50);
  };

  // Render segments
  const renderSegments = () => {
    console.log('TimelineSegments: renderSegments called');
    console.log('TimelineSegments: segments:', segments);
    console.log('TimelineSegments: startHour:', startHour, 'endHour:', endHour);

    if (!segments || segments.length === 0) {
      console.log('TimelineSegments: No segments to render');
      return html`<div class="text-center text-red-500 font-bold">No segments to display</div>`;
    }

    console.log('TimelineSegments: Rendering segments:', segments.length);

    const visibleSegments = [];
    const hourMap = new Map();

    // First pass: collect all segments by hour
    console.log('TimelineSegments: Starting to process segments');
    let visibleCount = 0;
    let skippedCount = 0;

    segments.forEach((segment, index) => {
      // Always use regular timestamps for consistency
      const startTimestamp = segment.start_timestamp;
      const endTimestamp = segment.end_timestamp;

      // Convert timestamps to Date objects
      const startTime = new Date(startTimestamp * 1000);
      const endTime = new Date(endTimestamp * 1000);

      // Calculate position and width
      const startHourFloat = startTime.getHours() + (startTime.getMinutes() / 60) + (startTime.getSeconds() / 3600);
      const endHourFloat = endTime.getHours() + (endTime.getMinutes() / 60) + (endTime.getSeconds() / 3600);

      // Skip segments outside the visible range
      if (endHourFloat < startHour || startHourFloat > endHour) {
        skippedCount++;
        if (index < 5) {
          console.log(`TimelineSegments: Skipping segment ${index}, startHour=${startHourFloat}, endHour=${endHourFloat}, visible range=${startHour}-${endHour}`);
        }
        return;
      }
      visibleCount++;

      // Mark each hour that this segment spans
      const startFloorHour = Math.floor(startHourFloat);
      const endCeilHour = Math.min(Math.ceil(endHourFloat), 24);

      for (let h = startFloorHour; h < endCeilHour; h++) {
        if (h >= startHour && h <= endHour) {
          if (!hourMap.has(h)) {
            hourMap.set(h, []);
          }
          hourMap.get(h).push(index);
        }
      }
    });

    // Preprocess segments to merge adjacent ones
    const mergedSegments = [];
    let currentMergedSegment = null;

    // Sort segments by start time
    const sortedSegments = [...segments].sort((a, b) => {
      return a.start_timestamp - b.start_timestamp;
    });

    // Merge adjacent segments (no gap or very small gap)
    sortedSegments.forEach((segment, index) => {
      if (!currentMergedSegment) {
        // First segment
        currentMergedSegment = { ...segment, originalIndices: [index] };
      } else {
        // Check if this segment is adjacent to the current merged segment
        const segmentStart = segment.start_timestamp;
        const mergedEnd = currentMergedSegment.end_timestamp;

        // Allow a small gap (1 second) to account for rounding errors
        const gap = segmentStart - mergedEnd;

        if (gap <= 1) {
          // Merge with current segment
          currentMergedSegment.end_timestamp = segment.end_timestamp;
          currentMergedSegment.originalIndices.push(index);

          // If this segment has detection, mark the merged segment as having detection
          if (segment.has_detection) {
            currentMergedSegment.has_detection = true;
          }
        } else {
          // Gap is too large, start a new merged segment
          mergedSegments.push(currentMergedSegment);
          currentMergedSegment = { ...segment, originalIndices: [index] };
        }
      }
    });

    // Add the last merged segment
    if (currentMergedSegment) {
      mergedSegments.push(currentMergedSegment);
    }

    // Second pass: add visible segments
    mergedSegments.forEach((segment, mergedIndex) => {
      const segStartTimestamp = segment.start_timestamp;
      const segEndTimestamp = segment.end_timestamp;

      // Convert timestamps to timeline hours using the utility function
      const startHourFloat = timelineState.timestampToTimelineHour(segStartTimestamp);
      const endHourFloat = timelineState.timestampToTimelineHour(segEndTimestamp);

      // Log the timestamps and hours for debugging
      console.log('TimelineSegments: Segment timestamps', {
        segmentId: segment.id,
        startTimestamp: segStartTimestamp,
        endTimestamp: segEndTimestamp,
        startTimeLocal: new Date(segStartTimestamp * 1000).toLocaleString(),
        endTimeLocal: new Date(segEndTimestamp * 1000).toLocaleString(),
        startHourFloat,
        endHourFloat
      });

      // Skip segments outside the visible range
      if (endHourFloat < startHour || startHourFloat > endHour) {
        return;
      }

      // Adjust start and end to fit within visible range
      const visibleStartHour = Math.max(startHourFloat, startHour);
      const visibleEndHour = Math.min(endHourFloat, endHour);

      // Calculate position and width as percentages
      const startPercent = ((visibleStartHour - startHour) / (endHour - startHour)) * 100;
      const widthPercent = ((visibleEndHour - visibleStartHour) / (endHour - startHour)) * 100;

      // Format duration for tooltip
      const duration = Math.round(segEndTimestamp - segStartTimestamp);
      const durationStr = `${duration}s`;

      // Format times for tooltip
      const startTimeStr = new Date(segStartTimestamp * 1000).toLocaleTimeString();
      const endTimeStr = new Date(segEndTimestamp * 1000).toLocaleTimeString();

      // Use a consistent height for all segments
      const heightPercent = 80; // 80% height for all segments

      visibleSegments.push(
        <div
          key={`segment-${mergedIndex}`}
          className="timeline-segment absolute rounded-sm transition-all duration-200"
          style={{
            backgroundColor: segment.has_detection ? 'hsl(var(--danger))' : 'hsl(var(--primary))',
            left: `${startPercent}%`,
            width: `${widthPercent}%`,
            height: `${heightPercent}%`,
            top: '50%',
            transform: 'translateY(-50%)'
          }}
          title={`${startTimeStr} - ${endTimeStr} (${durationStr})`}
        ></div>
      );
    });

    // Third pass: fill in gaps with clickable areas
    for (let hour = Math.floor(startHour); hour < Math.ceil(endHour); hour++) {
      if (!hourMap.has(hour)) {
        // No segments in this hour, create a clickable area
        const position = ((hour - startHour) / (endHour - startHour)) * 100;
        const width = 100 / (endHour - startHour);

        visibleSegments.push(
          <div
            key={`clickable-${hour}`}
            className="timeline-clickable-area absolute h-full cursor-pointer"
            style={{
              left: `${position}%`,
              width: `${width}%`
            }}
            data-hour={hour}
          ></div>
        );
      }
    }

    console.log(`TimelineSegments: Rendering complete. Total: ${segments.length}, Visible: ${visibleCount}, Skipped: ${skippedCount}, Final rendered: ${visibleSegments.length}`);
    return visibleSegments;
  };

  return (
    <div
      className="timeline-segments relative w-full h-16 pt-2"
      ref={containerRef}
    >
      {renderSegments()}
    </div>
  );
}
