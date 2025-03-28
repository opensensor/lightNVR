/**
 * LightNVR Timeline Cursor Component
 * Displays the playback cursor on the timeline
 */

import { h } from '../../../preact.min.js';
import { html } from '../../../html-helper.js';
import { useState, useEffect, useRef } from '../../../preact.hooks.module.js';
import { timelineState } from './TimelinePage.js';

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

  // Subscribe to timeline state changes
  useEffect(() => {
    const unsubscribe = timelineState.subscribe(state => {
      // Update local state
      setStartHour(state.timelineStartHour || 0);
      setEndHour(state.timelineEndHour || 24);
      setCurrentTime(state.currentTime);
      
      // Update time display
      updateTimeDisplay(state.currentTime);
      
      // Update cursor position (only if not dragging)
      if (!isDragging) {
        updateCursorPosition(state.currentTime, state.timelineStartHour || 0, state.timelineEndHour || 24);
      }
    });
    
    return () => unsubscribe();
  }, [isDragging]);

  // Set up drag handling
  useEffect(() => {
    const cursor = cursorRef.current;
    if (!cursor) return;
    
    const handleMouseDown = (e) => {
      e.preventDefault();
      e.stopPropagation();
      
      // Store the starting X position
      dragStartXRef.current = e.clientX;
      
      setIsDragging(true);
      
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
      
      // Convert hour to timestamp
      const date = new Date(timelineState.selectedDate);
      date.setHours(Math.floor(hour));
      date.setMinutes(Math.floor((hour % 1) * 60));
      date.setSeconds(Math.floor(((hour % 1) * 60) % 1 * 60));
      
      const timestamp = date.getTime() / 1000;
      
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
      
      // Calculate time based on position
      const hourRange = endHour - startHour;
      const hour = startHour + (positionPercent / 100) * hourRange;
      
      // Convert hour to timestamp
      const date = new Date(timelineState.selectedDate);
      date.setHours(Math.floor(hour));
      date.setMinutes(Math.floor((hour % 1) * 60));
      date.setSeconds(Math.floor(((hour % 1) * 60) % 1 * 60));
      
      const timestamp = date.getTime() / 1000;
      
      // Reset dragging state FIRST
      setIsDragging(false);
      
      // Remove event listeners
      document.removeEventListener('mousemove', handleMouseMove);
      document.removeEventListener('mouseup', handleMouseUp);
      
      // Find segment that contains this timestamp
      const segments = timelineState.timelineSegments || [];
      let foundSegment = false;
      
      for (let i = 0; i < segments.length; i++) {
        const segment = segments[i];
        if (timestamp >= segment.start_timestamp && timestamp <= segment.end_timestamp) {
          // Calculate relative time within the segment
          const relativeTime = timestamp - segment.start_timestamp;
          
          // Update timeline state
          timelineState.setState({ 
            currentSegmentIndex: i,
            currentTime: timestamp,
            prevCurrentTime: timelineState.currentTime,
            isPlaying: true
          });
          
          foundSegment = true;
          break;
        }
      }
      
      // If no segment found, find the closest one
      if (!foundSegment && segments.length > 0) {
        let closestSegment = 0;
        let minDistance = Infinity;
        
        for (let i = 0; i < segments.length; i++) {
          const segment = segments[i];
          const startDistance = Math.abs(segment.start_timestamp - timestamp);
          const endDistance = Math.abs(segment.end_timestamp - timestamp);
          const distance = Math.min(startDistance, endDistance);
          
          if (distance < minDistance) {
            minDistance = distance;
            closestSegment = i;
          }
        }
        
        // Update timeline state
        timelineState.setState({ 
          currentSegmentIndex: closestSegment,
          currentTime: segments[closestSegment].start_timestamp,
          prevCurrentTime: timelineState.currentTime,
          isPlaying: true
        });
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
    if (time === null) {
      setVisible(false);
      return;
    }
    
    // Calculate cursor position
    const date = new Date(time * 1000);
    const hour = date.getHours() + (date.getMinutes() / 60) + (date.getSeconds() / 3600);
    
    // Check if the current time is within the visible range
    if (hour < startHr || hour > endHr) {
      setVisible(false);
      return;
    }
    
    // Calculate position as percentage
    const position = ((hour - startHr) / (endHr - startHr)) * 100;
    
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
    const year = date.getFullYear();
    const month = (date.getMonth() + 1).toString().padStart(2, '0');
    const day = date.getDate().toString().padStart(2, '0');
    const hours = date.getHours().toString().padStart(2, '0');
    const minutes = date.getMinutes().toString().padStart(2, '0');
    const seconds = date.getSeconds().toString().padStart(2, '0');
    
    // Display date and time
    timeDisplay.textContent = `${year}-${month}-${day} ${hours}:${minutes}:${seconds}`;
  };

  // Force the cursor to be visible on initial render
  useEffect(() => {
    // Set visible to true after a short delay
    setTimeout(() => {
      if (timelineState.currentTime) {
        setVisible(true);
        updateCursorPosition(
          timelineState.currentTime, 
          timelineState.timelineStartHour || 0, 
          timelineState.timelineEndHour || 24
        );
      }
    }, 500);
  }, []);

  return html`
    <div 
      ref=${cursorRef}
      class="timeline-cursor absolute top-0 h-full z-50 transition-all duration-100 cursor-ew-resize"
      style="left: ${position}%; display: ${visible ? 'block' : 'none'}; pointer-events: auto; width: 7px; margin-left: -3.5px;"
    >
      <!-- Invisible wider clickable area -->
      <div class="absolute top-0 bottom-0 left-0 w-full"></div>
      
      <!-- Skinnier needle with no middle chunk -->
      <div class="absolute top-0 bottom-0 w-0.5 bg-orange-500 left-1/2 transform -translate-x-1/2 pointer-events-none"></div>
      
      <!-- Top handle (black) -->
      <div class="absolute top-0 left-1/2 w-4 h-4 bg-black rounded-full transform -translate-x-1/2 -translate-y-1/2 shadow-md pointer-events-none"></div>
      
      <!-- Bottom handle (black) -->
      <div class="absolute bottom-0 left-1/2 w-4 h-4 bg-black rounded-full transform -translate-x-1/2 translate-y-1/2 shadow-md pointer-events-none"></div>
    </div>
  `;
}
