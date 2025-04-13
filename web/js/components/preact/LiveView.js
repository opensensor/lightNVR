/**
 * LightNVR Web Interface LiveView Component
 * Preact component for the live view page
 */

import { h } from '../../preact.min.js';
import { html } from '../../html-helper.js';
import { useState, useEffect, useRef } from '../../preact.hooks.module.js';
import { LoadingIndicator } from './LoadingIndicator.js';

// Import modular components
import { loadStreams, updateVideoGrid } from './StreamGrid.js';
import { stopAllStreams } from './VideoPlayer.js';
import { toggleFullscreen, exitFullscreenMode } from './FullscreenManager.js';

/**
 * LiveView component
 * @returns {JSX.Element} LiveView component
 */
export function LiveView() {
  const [streams, setStreams] = useState([]);
  const [layout, setLayout] = useState('4');
  const [selectedStream, setSelectedStream] = useState('');
  const [isFullscreen, setIsFullscreen] = useState(false);
  const [isLoading, setIsLoading] = useState(true);
  const videoGridRef = useRef(null);
  const videoPlayers = useRef({});
  const detectionIntervals = useRef({});
  
  // Set up event listeners
  useEffect(() => {
    // Set up Escape key to exit fullscreen mode
    const handleEscape = (e) => {
      if (e.key === 'Escape') {
        console.log("Escape key pressed, current fullscreen state:", isFullscreen);
        // Check if we're in fullscreen mode by checking the DOM directly
        const livePage = document.getElementById('live-page');
        if (livePage && livePage.classList.contains('fullscreen-mode')) {
          console.log("Detected fullscreen mode via DOM, exiting fullscreen");
          exitFullscreenMode(null, setIsFullscreen);
        }
      }
    };
    
    document.addEventListener('keydown', handleEscape);
    
    // Add event listener to stop streams when leaving the page
    const handleBeforeUnload = () => {
      stopAllStreams(streams, videoPlayers.current, detectionIntervals.current);
    };
    
    window.addEventListener('beforeunload', handleBeforeUnload);
    
    // Cleanup
    return () => {
      document.removeEventListener('keydown', handleEscape);
      window.removeEventListener('beforeunload', handleBeforeUnload);
      stopAllStreams(streams, videoPlayers.current, detectionIntervals.current);
    };
  }, []);
  
  // Load streams after the component has rendered and videoGridRef is available
  useEffect(() => {
    if (videoGridRef.current) {
      // Set loading state
      setIsLoading(true);
      
      // Load streams from API
      loadStreams(setStreams, setSelectedStream, videoGridRef.current)
        .then(() => {
          // Hide loading indicator when done
          setIsLoading(false);
        })
        .catch(() => {
          // Hide loading indicator on error too
          setIsLoading(false);
        });
    }
  }, [videoGridRef.current]);
  
  // Update video grid when layout or streams change
  useEffect(() => {
    updateVideoGrid(
      videoGridRef.current, 
      streams, 
      layout, 
      selectedStream, 
      videoPlayers.current, 
      detectionIntervals.current
    );
  }, [layout, selectedStream, streams]);
  
  return html`
    <section id="live-page" class="page">
      <div class="page-header flex justify-between items-center mb-4 p-4 bg-white dark:bg-gray-800 rounded-lg shadow">
        <div class="flex items-center space-x-2">
          <h2 class="text-xl font-bold mr-4">Live View</h2>
          <div class="flex space-x-2">
            <button 
              id="webrtc-toggle-btn" 
              class="px-3 py-2 bg-green-600 text-white rounded-md hover:bg-green-700 transition-colors focus:outline-none focus:ring-2 focus:ring-green-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
              onClick=${() => window.location.href = '/index.html'}
            >
              Live View (WebRTC)
            </button>
            <button 
              id="fullscreen-btn" 
              class="px-3 py-2 bg-blue-600 text-white rounded-md hover:bg-blue-700 transition-colors focus:outline-none focus:ring-2 focus:ring-blue-500 focus:ring-offset-2 dark:focus:ring-offset-gray-800"
              onClick=${() => toggleFullscreen(isFullscreen, setIsFullscreen)}
            >
              Fullscreen
            </button>
          </div>
        </div>
        <div class="controls flex items-center space-x-2">
          <select 
            id="layout-selector" 
            class="px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600"
            value=${layout}
            onChange=${(e) => setLayout(e.target.value)}
          >
            <option value="1">Single View</option>
            <option value="4" selected>2x2 Grid</option>
            <option value="9">3x3 Grid</option>
            <option value="16">4x4 Grid</option>
          </select>
          
          ${layout === '1' && html`
            <select 
              id="stream-selector" 
              class="px-3 py-2 border border-gray-300 rounded-md shadow-sm focus:outline-none focus:ring-blue-500 focus:border-blue-500 dark:bg-gray-700 dark:border-gray-600"
              value=${selectedStream}
              onChange=${(e) => setSelectedStream(e.target.value)}
            >
              ${streams.map(stream => html`
                <option key=${stream.name} value=${stream.name}>${stream.name}</option>
              `)}
            </select>
          `}
        </div>
      </div>
      
      <div 
        id="video-grid" 
        class=${`video-container layout-${layout}`}
        ref=${videoGridRef}
      >
        ${isLoading ? html`
          <div class="flex justify-center items-center col-span-full row-span-full h-64 w-full">
            <${LoadingIndicator} message="Loading streams..." size="lg" />
          </div>
        ` : streams.length === 0 ? html`
          <div class="placeholder flex flex-col justify-center items-center col-span-full row-span-full bg-white dark:bg-gray-800 rounded-lg shadow-md text-center p-8">
            <p class="mb-6 text-gray-600 dark:text-gray-300 text-lg">No streams configured</p>
            <a href="streams.html" class="btn-primary px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors">Configure Streams</a>
          </div>
        ` : null}
        <!-- Video cells will be dynamically added by the updateVideoGrid function -->
      </div>
    </section>
  `;
}

/**
 * Load LiveView component
 */
export function loadLiveView() {
  const mainContent = document.getElementById('main-content');
  if (!mainContent) return;
  
  // Render the LiveView component to the container
  import('../../preact.min.js').then(({ render }) => {
    render(html`<${LiveView} />`, mainContent);
  });
}
