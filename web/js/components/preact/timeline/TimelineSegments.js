/**
 * LightNVR Timeline Segments Component
 * Displays recording segments on the timeline
 */

import { h } from '../../../preact.min.js';
import { html } from '../../../html-helper.js';
import { useState, useEffect, useRef } from '../../../preact.hooks.module.js';
import { timelineState } from './TimelinePage.js';
import { showStatusMessage } from '../UI.js';

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
      console.log('TimelineSegments: Received state update:', state);
      console.log('TimelineSegments: Segments in update:', state.timelineSegments);
      
      setSegments(state.timelineSegments || []);
      setStartHour(state.timelineStartHour);
      setEndHour(state.timelineEndHour);
      setCurrentSegmentIndex(state.currentSegmentIndex);
      
      console.log('TimelineSegments: Local state updated with segments:', state.timelineSegments?.length || 0);
    });
    
    // Log initial state
    console.log('TimelineSegments: Initial timelineState:', timelineState);
    console.log('TimelineSegments: Initial segments:', timelineState.timelineSegments);
    
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
    console.log('Timeline click event:', event);
    
    const container = containerRef.current;
    if (!container) {
      console.error('Container ref is null');
      return;
    }
    
    // Get click position relative to container
    const rect = container.getBoundingClientRect();
    const clickX = event.clientX - rect.left;
    const containerWidth = rect.width;
    
    console.log(`Click position: ${clickX}px / ${containerWidth}px`);
    
    // Calculate time based on click position
    const clickPercent = clickX / containerWidth;
    const clickHour = startHour + (clickPercent * (endHour - startHour));
    
    console.log(`Click hour: ${clickHour} (${Math.floor(clickHour)}:${Math.floor((clickHour % 1) * 60)})`);
    
    // Find the segment that contains this time
    const clickDate = new Date(timelineState.selectedDate);
    clickDate.setHours(Math.floor(clickHour));
    clickDate.setMinutes(Math.floor((clickHour % 1) * 60));
    clickDate.setSeconds(Math.floor(((clickHour % 1) * 60) % 1 * 60));
    
    const clickTimestamp = clickDate.getTime() / 1000;
    console.log(`Click timestamp: ${clickTimestamp}, date: ${clickDate.toLocaleString()}`);
    
    // Find segment that contains this timestamp
    let foundSegment = false;
    for (let i = 0; i < segments.length; i++) {
      const segment = segments[i];
      if (clickTimestamp >= segment.start_timestamp && clickTimestamp <= segment.end_timestamp) {
        console.log(`Found segment ${i} containing timestamp:`, segment);
        // Calculate relative time within the segment
        const relativeTime = clickTimestamp - segment.start_timestamp;
        console.log(`Relative time within segment: ${relativeTime}s`);
        
        // Always update the segment index, even if it's the same as the current one
        // This ensures the video player will reload the segment if needed
        console.log(`Setting currentSegmentIndex to ${i} (was ${currentSegmentIndex})`);
        setCurrentSegmentIndex(i);
        
        // Play this segment starting at the clicked time
        playSegment(i, relativeTime);
        foundSegment = true;
        break;
      }
    }
    
    if (!foundSegment) {
      if (segments.length > 0) {
        console.log('No segment contains the timestamp, finding closest segment');
        // Find the closest segment
        let closestSegment = -1;
        let minDistance = Infinity;
        
        for (let i = 0; i < segments.length; i++) {
          const segment = segments[i];
          const startDistance = Math.abs(segment.start_timestamp - clickTimestamp);
          const endDistance = Math.abs(segment.end_timestamp - clickTimestamp);
          const distance = Math.min(startDistance, endDistance);
          
          if (distance < minDistance) {
            minDistance = distance;
            closestSegment = i;
          }
        }
        
        if (closestSegment >= 0) {
          console.log(`Playing closest segment ${closestSegment}:`, segments[closestSegment]);
          // Always update the segment index, even if it's the same as the current one
          console.log(`Setting currentSegmentIndex to ${closestSegment} (was ${currentSegmentIndex})`);
          
          // Play the closest segment
          playSegment(closestSegment);
        }
      } else {
        // No segments found, just update the currentTime
        console.log('No segments found, just updating currentTime to:', clickTimestamp);
        timelineState.setState({ 
          currentTime: clickTimestamp,
          prevCurrentTime: timelineState.currentTime
        });
      }
    }
  };

  // Play a specific segment
  const playSegment = (index, relativeTime = null) => {
    console.log(`TimelineSegments.playSegment(${index}, ${relativeTime})`);
    
    if (index < 0 || index >= segments.length) {
      console.warn(`Invalid segment index: ${index}, segments length: ${segments.length}`);
      return;
    }
    
    const segment = segments[index];
    
    // Calculate absolute timestamp for currentTime
    const absoluteTime = relativeTime !== null 
      ? segment.start_timestamp + relativeTime 
      : segment.start_timestamp;
    
    // Force a complete reset of the player state to ensure immediate transition
    // First, pause any current playback and reset the segment index
    timelineState.setState({ 
      isPlaying: false,
      currentSegmentIndex: -1
    });
    
    // Force a synchronous DOM update by accessing a property that causes a reflow
    document.body.offsetHeight;
    
    // Now set the new segment index and start playing
    // This two-step process ensures the player detects a complete state change
    setTimeout(() => {
      console.log(`Setting new segment index to ${index} with time ${absoluteTime}`);
      timelineState.setState({ 
        currentSegmentIndex: index,
        currentTime: absoluteTime,
        isPlaying: true,
        forceReload: true
      });
      
      // Force the video player to reload by directly accessing the video element
      setTimeout(() => {
        const videoElement = document.querySelector('#video-player video');
        if (videoElement) {
          console.log('Directly manipulating video element to force reload');
          
          // Pause any current playback
          videoElement.pause();
          
          // Clear the source and reload
          videoElement.removeAttribute('src');
          videoElement.load();
          
          // Set the new source with a timestamp to prevent caching
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
    console.log('TimelineSegments.renderSegments() called');
    console.log('Segments:', segments);
    console.log('Start hour:', startHour);
    console.log('End hour:', endHour);
    
    if (!segments || segments.length === 0) {
      console.log('No segments to render');
      return null;
    }
    
    const visibleSegments = [];
    const hourMap = new Map();
    
    // First pass: collect all segments by hour
    segments.forEach((segment, index) => {
      console.log(`Processing segment ${index}:`, segment);
      
      // Convert timestamps to Date objects
      const startTime = new Date(segment.start_timestamp * 1000);
      const endTime = new Date(segment.end_timestamp * 1000);
      
      console.log(`Segment ${index} times:`, {
        startTime: startTime.toLocaleTimeString(),
        endTime: endTime.toLocaleTimeString()
      });
      
      // Calculate position and width
      const startHourFloat = startTime.getHours() + (startTime.getMinutes() / 60) + (startTime.getSeconds() / 3600);
      const endHourFloat = endTime.getHours() + (endTime.getMinutes() / 60) + (endTime.getSeconds() / 3600);
      
      console.log(`Segment ${index} hour range:`, {
        startHourFloat,
        endHourFloat,
        visibleRange: `${startHour} - ${endHour}`
      });
      
      // Skip segments outside the visible range
      if (endHourFloat < startHour || startHourFloat > endHour) {
        console.log(`Segment ${index} is outside visible range, skipping`);
        return;
      }
      
      // Mark each hour that this segment spans
      const startFloorHour = Math.floor(startHourFloat);
      const endCeilHour = Math.min(Math.ceil(endHourFloat), 24);
      
      console.log(`Segment ${index} spans hours:`, {
        startFloorHour,
        endCeilHour
      });
      
      for (let h = startFloorHour; h < endCeilHour; h++) {
        if (h >= startHour && h <= endHour) {
          if (!hourMap.has(h)) {
            hourMap.set(h, []);
          }
          hourMap.get(h).push(index);
          console.log(`Added segment ${index} to hour ${h}`);
        }
      }
    });
    
    console.log('Hour map after first pass:', Object.fromEntries([...hourMap.entries()]));
    
    // Second pass: add visible segments
    segments.forEach((segment, index) => {
      // Convert timestamps to Date objects
      const startTime = new Date(segment.start_timestamp * 1000);
      const endTime = new Date(segment.end_timestamp * 1000);
      
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
      const duration = Math.round(segment.end_timestamp - segment.start_timestamp);
      const durationStr = `${duration}s`;
      
      // Format times for tooltip
      const startTimeStr = startTime.toLocaleTimeString();
      const endTimeStr = endTime.toLocaleTimeString();
      
      // Calculate height based on duration (longer segments are taller)
      const heightPercent = Math.min(100, Math.max(60, (duration / 60) * 5)); // 5% height per minute, min 60%, max 100%
      
      visibleSegments.push(html`
        <div 
          key="segment-${index}"
          class="timeline-segment absolute rounded-sm cursor-pointer transition-all duration-200 ${segment.has_detection ? 'bg-red-500 hover:bg-red-600' : 'bg-blue-500 hover:bg-blue-600'} ${index === currentSegmentIndex ? 'border-2 border-yellow-400' : ''}"
          style="left: ${startPercent}%; width: ${widthPercent}%; height: ${heightPercent}%; top: 50%; transform: translateY(-50%);"
          title="${startTimeStr} - ${endTimeStr} (${durationStr})"
          onClick=${() => playSegment(index)}
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
