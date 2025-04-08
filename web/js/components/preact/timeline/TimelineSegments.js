/**
 * LightNVR Timeline Segments Component
 * Displays recording segments on the timeline
 */

import { html } from '../../../html-helper.js';
import { useState, useEffect, useRef } from 'preact/hooks';
import { timelineState } from './TimelinePage.js';

/**
 * TimelineSegments component
 * @returns {JSX.Element} TimelineSegments component
 */
export function TimelineSegments() {
  // Local state
  const [segments, setSegments] = useState([]);
  const [startHour, setStartHour] = useState(0);
  const [endHour, setEndHour] = useState(24);
  const [currentSegmentIndex, setCurrentSegmentIndex] = useState(-1);
  
  // Refs
  const containerRef = useRef(null);
  const isDragging = useRef(false);

  // Subscribe to timeline state changes
  useEffect(() => {
    console.log('TimelineSegments: Setting up subscription to timelineState');
    
    const unsubscribe = timelineState.subscribe(state => {
      console.log('TimelineSegments: Received state update');
      
      // Update segments
      if (state.timelineSegments) {
        console.log(`TimelineSegments: Updating segments (${state.timelineSegments.length})`);
        setSegments(state.timelineSegments);
      }
      
      // Update other state
      setStartHour(state.timelineStartHour || 0);
      setEndHour(state.timelineEndHour || 24);
      setCurrentSegmentIndex(state.currentSegmentIndex || -1);
    });
    
    // Check if we already have segments in the timelineState
    if (timelineState.timelineSegments && timelineState.timelineSegments.length > 0) {
      console.log(`TimelineSegments: Initial segments available (${timelineState.timelineSegments.length})`);
      setSegments(timelineState.timelineSegments);
      setCurrentSegmentIndex(timelineState.currentSegmentIndex || 0);
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
    
    // Find segment that contains this timestamp
    let foundSegment = false;
    for (let i = 0; i < segments.length; i++) {
      const segment = segments[i];
      // Use local timestamps if available, otherwise fall back to regular timestamps
      const startTimestamp = segment.local_start_timestamp || segment.start_timestamp;
      const endTimestamp = segment.local_end_timestamp || segment.end_timestamp;
      
      if (clickTimestamp >= startTimestamp && clickTimestamp <= endTimestamp) {
        console.log(`TimelineSegments: Found segment ${i} containing timestamp`);
        
        // Calculate relative time within the segment
        const relativeTime = clickTimestamp - startTimestamp;
        
        // Update current segment index
        setCurrentSegmentIndex(i);
        
        // Play this segment starting at the clicked time
        playSegment(i, relativeTime);
        foundSegment = true;
        break;
      }
    }
    
    if (!foundSegment) {
      if (segments.length > 0) {
        console.log('TimelineSegments: No segment contains the timestamp, finding closest segment');
        // Find the closest segment
        let closestSegment = -1;
        let minDistance = Infinity;
        
        for (let i = 0; i < segments.length; i++) {
          const segment = segments[i];
          // Use local timestamps if available, otherwise fall back to regular timestamps
          const startTimestamp = segment.local_start_timestamp || segment.start_timestamp;
          const endTimestamp = segment.local_end_timestamp || segment.end_timestamp;
          
          const startDistance = Math.abs(startTimestamp - clickTimestamp);
          const endDistance = Math.abs(endTimestamp - clickTimestamp);
          const distance = Math.min(startDistance, endDistance);
          
          if (distance < minDistance) {
            minDistance = distance;
            closestSegment = i;
          }
        }
        
        if (closestSegment >= 0) {
          console.log(`TimelineSegments: Playing closest segment ${closestSegment}`);
          
          // Play the closest segment
          playSegment(closestSegment);
        }
      } else {
        // No segments found, just update the currentTime
        console.log('TimelineSegments: No segments found, just updating currentTime');
        timelineState.setState({ 
          currentTime: clickTimestamp,
          prevCurrentTime: timelineState.currentTime
        });
      }
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
    console.log(`TimelineSegments: Rendering ${segments.length} segments`);
    
    if (!segments || segments.length === 0) {
      return null;
    }
    
    const visibleSegments = [];
    const hourMap = new Map();
    
    // First pass: collect all segments by hour
    segments.forEach((segment, index) => {
      // Use local timestamps if available, otherwise fall back to regular timestamps
      const startTimestamp = segment.local_start_timestamp || segment.start_timestamp;
      const endTimestamp = segment.local_end_timestamp || segment.end_timestamp;
      
      // Convert timestamps to Date objects
      const startTime = new Date(startTimestamp * 1000);
      const endTime = new Date(endTimestamp * 1000);
      
      // Calculate position and width
      const startHourFloat = startTime.getHours() + (startTime.getMinutes() / 60) + (startTime.getSeconds() / 3600);
      const endHourFloat = endTime.getHours() + (endTime.getMinutes() / 60) + (endTime.getSeconds() / 3600);
      
      // Skip segments outside the visible range
      if (endHourFloat < startHour || startHourFloat > endHour) {
        return;
      }
      
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
    
    // Sort segments by start time (using local timestamps if available)
    const sortedSegments = [...segments].sort((a, b) => {
      const aStart = a.local_start_timestamp || a.start_timestamp;
      const bStart = b.local_start_timestamp || b.start_timestamp;
      return aStart - bStart;
    });
    
    // Merge adjacent segments (no gap or very small gap)
    sortedSegments.forEach((segment, index) => {
      if (!currentMergedSegment) {
        // First segment
        currentMergedSegment = { ...segment, originalIndices: [index] };
      } else {
        // Check if this segment is adjacent to the current merged segment
        // Use local timestamps if available, otherwise fall back to regular timestamps
        const segmentStart = segment.local_start_timestamp || segment.start_timestamp;
        const mergedEnd = currentMergedSegment.local_end_timestamp || currentMergedSegment.end_timestamp;
        
        // Allow a small gap (1 second) to account for rounding errors
        const gap = segmentStart - mergedEnd;
        
        if (gap <= 1) {
          // Merge with current segment
          // Update both regular and local timestamps
          currentMergedSegment.end_timestamp = segment.end_timestamp;
          if (segment.local_end_timestamp) {
            currentMergedSegment.local_end_timestamp = segment.local_end_timestamp;
          }
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
    
    console.log(`TimelineSegments: Merged ${segments.length} segments into ${mergedSegments.length} segments`);
    
    // Second pass: add visible segments
    mergedSegments.forEach((segment, mergedIndex) => {
      // Use local timestamps if available, otherwise fall back to regular timestamps
      const segStartTimestamp = segment.local_start_timestamp || segment.start_timestamp;
      const segEndTimestamp = segment.local_end_timestamp || segment.end_timestamp;
      
      // Convert timestamps to Date objects
      const startTime = new Date(segStartTimestamp * 1000);
      const endTime = new Date(segEndTimestamp * 1000);
      
      // Calculate position and width
      const startHourFloat = startTime.getHours() + (startTime.getMinutes() / 60) + (startTime.getSeconds() / 3600);
      const endHourFloat = endTime.getHours() + (endTime.getMinutes() / 60) + (endTime.getSeconds() / 3600);
      
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
      const startTimeStr = startTime.toLocaleTimeString();
      const endTimeStr = endTime.toLocaleTimeString();
      
      // Use a consistent height for all segments
      const heightPercent = 80; // 80% height for all segments
      
      visibleSegments.push(html`
        <div 
          key="segment-${mergedIndex}"
          class="timeline-segment absolute rounded-sm transition-all duration-200 ${segment.has_detection ? 'bg-red-500' : 'bg-blue-500'}"
          style="left: ${startPercent}%; width: ${widthPercent}%; height: ${heightPercent}%; top: 50%; transform: translateY(-50%);"
          title="${startTimeStr} - ${endTimeStr} (${durationStr})"
        ></div>
      `);
    });
    
    // Third pass: fill in gaps with clickable areas
    for (let hour = Math.floor(startHour); hour < Math.ceil(endHour); hour++) {
      if (!hourMap.has(hour)) {
        // No segments in this hour, create a clickable area
        const position = ((hour - startHour) / (endHour - startHour)) * 100;
        const width = 100 / (endHour - startHour);
        
        visibleSegments.push(html`
          <div 
            key="clickable-${hour}"
            class="timeline-clickable-area absolute h-full cursor-pointer"
            style="left: ${position}%; width: ${width}%;"
            data-hour=${hour}
          ></div>
        `);
      }
    }
    
    return visibleSegments;
  };

  return html`
    <div 
      class="timeline-segments relative w-full h-16 pt-2"
      ref=${containerRef}
    >
      ${renderSegments()}
    </div>
  `;
}
