/**
 * LightNVR Timeline Ruler Component
 * Displays the time ruler with hour markers
 */

import { h } from '../../../preact.min.js';
import { html } from '../../../html-helper.js';
import { useState, useEffect } from '../../../preact.hooks.module.js';
import { timelineState } from './TimelinePage.js';

/**
 * TimelineRuler component
 * @returns {JSX.Element} TimelineRuler component
 */
export function TimelineRuler() {
  // Local state
  const [startHour, setStartHour] = useState(0);
  const [endHour, setEndHour] = useState(24);
  const [zoomLevel, setZoomLevel] = useState(1);

  // Subscribe to timeline state changes
  useEffect(() => {
    const unsubscribe = timelineState.subscribe(state => {
      // Calculate time range based on zoom level
      const hoursPerView = 24 / state.zoomLevel;
      
      // If we're showing only segments with recordings, adjust the view
      if (state.showOnlySegments && state.timelineSegments && state.timelineSegments.length > 0) {
        // Find the earliest and latest segments
        let earliestHour = 24;
        let latestHour = 0;
        
        state.timelineSegments.forEach(segment => {
          const startTime = new Date(segment.start_timestamp * 1000);
          const endTime = new Date(segment.end_timestamp * 1000);
          
          const startHour = startTime.getHours() + (startTime.getMinutes() / 60) + (startTime.getSeconds() / 3600);
          const endHour = endTime.getHours() + (endTime.getMinutes() / 60) + (endTime.getSeconds() / 3600);
          
          earliestHour = Math.min(earliestHour, startHour);
          latestHour = Math.max(latestHour, endHour);
        });
        
        // Add some padding
        earliestHour = Math.max(0, earliestHour - 0.5);
        latestHour = Math.min(24, latestHour + 0.5);
        
        // Ensure we show at least 1 hour
        if (latestHour - earliestHour < 1) {
          const midpoint = (earliestHour + latestHour) / 2;
          earliestHour = Math.max(0, midpoint - 0.5);
          latestHour = Math.min(24, midpoint + 0.5);
        }
        
        setStartHour(earliestHour);
        setEndHour(latestHour);
        
        // Update global state
        timelineState.setState({
          timelineStartHour: earliestHour,
          timelineEndHour: latestHour
        });
      } else {
        // If we have a current time, center the view around it
        let newStartHour = state.timelineStartHour;
        let newEndHour = state.timelineEndHour;
        
        if (state.currentTime !== null) {
          const currentDate = new Date(state.currentTime * 1000);
          const currentHour = currentDate.getHours() + (currentDate.getMinutes() / 60) + (currentDate.getSeconds() / 3600);
          
          // Center the view around the current time
          newStartHour = Math.max(0, Math.min(24 - hoursPerView, currentHour - (hoursPerView / 2)));
          newEndHour = Math.min(24, newStartHour + hoursPerView);
          
          // Adjust start hour if end hour is at the limit
          if (newEndHour === 24) {
            newStartHour = Math.max(0, 24 - hoursPerView);
          }
        } else {
          // No current time, just adjust the view based on zoom level
          newStartHour = Math.max(0, Math.min(24 - hoursPerView, state.timelineStartHour));
          newEndHour = Math.min(24, newStartHour + hoursPerView);
        }
        
        setStartHour(newStartHour);
        setEndHour(newEndHour);
        
        // Update global state with calculated values
        if (newStartHour !== state.timelineStartHour || newEndHour !== state.timelineEndHour) {
          timelineState.setState({
            timelineStartHour: newStartHour,
            timelineEndHour: newEndHour
          });
        }
      }
      
      setZoomLevel(state.zoomLevel);
    });
    
    return () => unsubscribe();
  }, []);

  // Generate hour markers and labels
  const generateHourMarkers = () => {
    const markers = [];
    const hourWidth = 100 / (endHour - startHour);
    
    // Add hour markers and labels
    for (let hour = Math.floor(startHour); hour <= Math.ceil(endHour); hour++) {
      if (hour >= 0 && hour <= 24) {
        const position = (hour - startHour) * hourWidth;
        
        // Add hour marker
        markers.push(html`
          <div 
            key="tick-${hour}" 
            class="absolute top-0 w-px h-5 bg-gray-500 dark:bg-gray-400" 
            style="left: ${position}%;"
          ></div>
        `);
        
        // Add hour label
        markers.push(html`
          <div 
            key="label-${hour}" 
            class="absolute top-0 text-xs text-gray-600 dark:text-gray-300 transform -translate-x-1/2" 
            style="left: ${position}%;"
          >
            ${hour}:00
          </div>
        `);
        
        // Add half-hour marker if not the last hour
        if (hour < 24) {
          const halfHourPosition = position + (hourWidth / 2);
          markers.push(html`
            <div 
              key="tick-${hour}-30" 
              class="absolute top-2 w-px h-3 bg-gray-400 dark:bg-gray-500" 
              style="left: ${halfHourPosition}%;"
            ></div>
          `);
        }
      }
    }
    
    return markers;
  };

  return html`
    <div class="timeline-ruler relative w-full h-8 bg-gray-300 dark:bg-gray-800 border-b border-gray-400 dark:border-gray-600">
      ${generateHourMarkers()}
    </div>
  `;
}
