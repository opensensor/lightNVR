/**
 * LightNVR Web Interface IndexView Component
 * Preact component for the index/dashboard page
 */

import { h } from '../../preact.min.js';
import { html } from '../../preact-app.js';
import { useState, useEffect } from '../../preact.hooks.module.js';
import { showStatusMessage } from './UI.js';

/**
 * IndexView component
 * @returns {JSX.Element} IndexView component
 */
export function IndexView() {
  const [streams, setStreams] = useState([]);
  const [systemInfo, setSystemInfo] = useState({
    cpu: { usage: 0 },
    memory: { total: 0, used: 0 },
    disk: { total: 0, used: 0 },
    streams: { active: 0, total: 0 },
    recordings: { count: 0, size: 0 }
  });
  const [isLoading, setIsLoading] = useState(true);
  
  // Load streams and system info on mount
  useEffect(() => {
    Promise.all([
      loadStreams(),
      loadSystemInfo()
    ]).finally(() => {
      setIsLoading(false);
    });
    
    // Set up interval to refresh system info
    const interval = setInterval(loadSystemInfo, 10000);
    
    // Clean up interval on unmount
    return () => clearInterval(interval);
  }, []);
  
  // Load streams from API
  const loadStreams = async () => {
    try {
      const response = await fetch('/api/streams');
      if (!response.ok) {
        throw new Error('Failed to load streams');
      }
      
      const data = await response.json();
      setStreams(data || []);
    } catch (error) {
      console.error('Error loading streams:', error);
      showStatusMessage('Error loading streams: ' + error.message);
    }
  };
  
  // Load system info from API
  const loadSystemInfo = async () => {
    try {
      const response = await fetch('/api/system/info');
      if (!response.ok) {
        throw new Error('Failed to load system info');
      }
      
      const data = await response.json();
      setSystemInfo(data);
    } catch (error) {
      console.error('Error loading system info:', error);
      // Don't show error message for this, just log it
    }
  };
  
  // Format bytes to human-readable size
  const formatBytes = (bytes, decimals = 1) => {
    if (bytes === 0) return '0 Bytes';
    
    const k = 1024;
    const dm = decimals < 0 ? 0 : decimals;
    const sizes = ['Bytes', 'KB', 'MB', 'GB', 'TB', 'PB', 'EB', 'ZB', 'YB'];
    
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    
    return parseFloat((bytes / Math.pow(k, i)).toFixed(dm)) + ' ' + sizes[i];
  };
  
  // Get stream thumbnail URL
  const getStreamThumbnail = (streamName) => {
    return `/api/streams/${encodeURIComponent(streamName)}/thumbnail?t=${Date.now()}`;
  };
  
  // Navigate to stream page
  const goToStream = (streamName) => {
    window.location.href = `/live.html?stream=${encodeURIComponent(streamName)}`;
  };
  
  return html`
    <section id="index-page" class="page">
      <div class="page-header flex justify-between items-center mb-4 p-4 bg-white dark:bg-gray-800 rounded-lg shadow">
        <h2 class="text-xl font-bold">Dashboard</h2>
      </div>
      
      ${isLoading ? html`
        <div class="flex justify-center items-center h-64">
          <div class="animate-spin rounded-full h-12 w-12 border-t-2 border-b-2 border-blue-500"></div>
        </div>
      ` : html`
        <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-4 mb-6">
          <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
            <h3 class="text-lg font-semibold mb-2">CPU Usage</h3>
            <div class="flex items-center">
              <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700 mr-2">
                <div class="bg-blue-600 h-2.5 rounded-full" style=${`width: ${systemInfo.cpu?.usage || 0}%`}></div>
              </div>
              <span class="text-sm font-medium">${systemInfo.cpu?.usage ? `${systemInfo.cpu.usage.toFixed(1)}%` : '0%'}</span>
            </div>
          </div>
          
          <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
            <h3 class="text-lg font-semibold mb-2">Memory</h3>
            <div class="flex items-center">
              <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700 mr-2">
                <div class="bg-green-600 h-2.5 rounded-full" style=${`width: ${systemInfo.memory?.total ? (systemInfo.memory.used / systemInfo.memory.total * 100).toFixed(1) : 0}%`}></div>
              </div>
              <span class="text-sm font-medium">${systemInfo.memory?.used ? formatBytes(systemInfo.memory.used) : '0'}</span>
            </div>
          </div>
          
          <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
            <h3 class="text-lg font-semibold mb-2">Storage</h3>
            <div class="flex items-center">
              <div class="w-full bg-gray-200 rounded-full h-2.5 dark:bg-gray-700 mr-2">
                <div class="bg-yellow-600 h-2.5 rounded-full" style=${`width: ${systemInfo.disk?.total ? (systemInfo.disk.used / systemInfo.disk.total * 100).toFixed(1) : 0}%`}></div>
              </div>
              <span class="text-sm font-medium">${systemInfo.disk?.used ? formatBytes(systemInfo.disk.used) : '0'}</span>
            </div>
          </div>
          
          <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
            <h3 class="text-lg font-semibold mb-2">Streams</h3>
            <div class="flex justify-between">
              <div>
                <span class="text-sm text-gray-600 dark:text-gray-400">Active:</span>
                <span class="ml-1 font-medium">${systemInfo.streams?.active || 0}</span>
              </div>
              <div>
                <span class="text-sm text-gray-600 dark:text-gray-400">Total:</span>
                <span class="ml-1 font-medium">${systemInfo.streams?.total || 0}</span>
              </div>
              <div>
                <span class="text-sm text-gray-600 dark:text-gray-400">Recordings:</span>
                <span class="ml-1 font-medium">${systemInfo.recordings?.count || 0}</span>
              </div>
            </div>
          </div>
        </div>
        
        <div class="mb-6">
          <div class="flex justify-between items-center mb-4">
            <h3 class="text-lg font-semibold">Streams</h3>
            <a href="/streams.html" class="text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300">Manage Streams</a>
          </div>
          
          ${streams.length === 0 ? html`
            <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-6 text-center">
              <p class="text-gray-600 dark:text-gray-400 mb-4">No streams configured yet</p>
              <a href="/streams.html" class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors">Add Stream</a>
            </div>
          ` : html`
            <div class="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-3 gap-4">
              ${streams.map(stream => html`
                <div key=${stream.name} class="bg-white dark:bg-gray-800 rounded-lg shadow overflow-hidden">
                  <div class="relative aspect-video bg-gray-900">
                    ${stream.enabled ? html`
                      <img 
                        src=${getStreamThumbnail(stream.name)} 
                        alt=${`${stream.name} thumbnail`}
                        class="w-full h-full object-contain"
                        onerror=${e => e.target.src = 'img/no-signal.png'}
                      />
                      <div class="absolute top-2 right-2 px-2 py-1 bg-green-500 text-white text-xs font-medium rounded-full">
                        LIVE
                      </div>
                    ` : html`
                      <div class="w-full h-full flex items-center justify-center">
                        <span class="text-gray-500 dark:text-gray-400">Stream Disabled</span>
                      </div>
                    `}
                  </div>
                  <div class="p-4">
                    <h4 class="font-medium text-lg mb-1">${stream.name}</h4>
                    <p class="text-sm text-gray-600 dark:text-gray-400 mb-2">${stream.url}</p>
                    <div class="flex justify-between text-sm">
                      <span>${stream.width}x${stream.height}</span>
                      <span>${stream.fps} FPS</span>
                      <span>${stream.codec.toUpperCase()}</span>
                    </div>
                    <div class="mt-4 flex justify-end">
                      <button 
                        class="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700 transition-colors"
                        onClick=${() => goToStream(stream.name)}
                        disabled=${!stream.enabled}
                      >
                        View Stream
                      </button>
                    </div>
                  </div>
                </div>
              `)}
            </div>
          `}
        </div>
        
        <div class="mb-6">
          <div class="flex justify-between items-center mb-4">
            <h3 class="text-lg font-semibold">Recent Recordings</h3>
            <a href="/recordings.html" class="text-blue-600 hover:text-blue-800 dark:text-blue-400 dark:hover:text-blue-300">View All Recordings</a>
          </div>
          
          <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4">
            <p class="text-center text-gray-600 dark:text-gray-400">
              View all recordings in the Recordings section
            </p>
          </div>
        </div>
      `}
    </section>
  `;
}

/**
 * Load IndexView component
 */
export function loadIndexView() {
  const mainContent = document.getElementById('main-content');
  if (!mainContent) return;
  
  // Render the IndexView component to the container
  import('../../preact.min.js').then(({ render }) => {
    render(html`<${IndexView} />`, mainContent);
  });
}
