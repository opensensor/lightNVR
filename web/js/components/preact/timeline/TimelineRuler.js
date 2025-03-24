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
      const newStartHour = Math.max(0, Math.min(24 - hoursPerView, state.timelineStartHour));
      const newEndHour = Math.min(24, newStartHour + hoursPerView);
      
      setStartHour(newStartHour);
      setEndHour(newEndHour);
      setZoomLevel(state.zoomLevel);
      
      // Update global state with calculated values
      if (newStartHour !== state.timelineStartHour || newEndHour !== state.timelineEndHour) {
        timelineState.setState({
          timelineStartHour: newStartHour,
          timelineEndHour: newEndHour
        });
      }
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
