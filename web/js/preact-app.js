/**
 * LightNVR Web Interface Preact App
 * Main entry point for the Preact application
 */

import { h, render } from './preact.min.js';
import htm from './htm.module.js';
import { loadHeader } from './components/preact/Header.js';
import { loadFooter } from './components/preact/Footer.js';
import { loadLiveView } from './components/preact/LiveView.js';
import { setupModals, addStatusMessageStyles } from './components/preact/UI.js';

// Initialize htm with Preact's h
export const html = htm.bind(h);

/**
 * Simple store implementation for state management
 * @param {Object} initialState - Initial state
 * @returns {Object} Store object with getState, setState, and subscribe methods
 */
function createStore(initialState = {}) {
  let state = { ...initialState };
  const listeners = new Set();
  
  return {
    getState: () => state,
    setState: (newState) => {
      if (typeof newState === 'function') {
        state = newState(state);
      } else {
        state = { ...state, ...newState };
      }
      
      // Notify listeners
      listeners.forEach(listener => listener(state));
    },
    subscribe: (listener) => {
      listeners.add(listener);
      
      // Return unsubscribe function
      return () => {
        listeners.delete(listener);
      };
    }
  };
}

// Create stores for UI state
export const statusMessageStore = createStore({
  message: '',
  visible: false,
  timeout: null
});

export const snapshotModalStore = createStore({
  imageUrl: '',
  title: ''
});

export const videoModalStore = createStore({
  videoUrl: '',
  title: '',
  downloadUrl: ''
});

/**
 * Initialize the application
 */
function initApp() {
  // Get current page
  const currentPage = window.location.pathname.split('/').pop();
  
  // Load header with active navigation
  let activeNav = '';
  
  switch (currentPage) {
    case 'live.html':
    case 'live.preact.html':
      activeNav = 'nav-live';
      break;
    case 'recordings.html':
      activeNav = 'nav-recordings';
      break;
    case 'streams.html':
      activeNav = 'nav-streams';
      break;
    case 'settings.html':
      activeNav = 'nav-settings';
      break;
    case 'system.html':
      activeNav = 'nav-system';
      break;
  }
  
  // Load header and footer
  loadHeader(activeNav);
  loadFooter();
  
  // Load page-specific content
  if (currentPage === 'live.html' || currentPage === 'live.preact.html') {
    loadLiveView();
  }
  
  // Setup UI components
  setupModals();
  addStatusMessageStyles();
}

// Initialize the app when the DOM is loaded
document.addEventListener('DOMContentLoaded', initApp);

// Export for use in other modules
export { h, render };
