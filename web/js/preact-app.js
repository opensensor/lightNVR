/**
 * LightNVR Web Interface Preact App
 * Main entry point for the Preact application
 */

import { h, render } from './preact.min.js';
import { html } from './html-helper.js';
import { loadHeader } from './components/preact/Header.js';
import { loadFooter } from './components/preact/Footer.js';
import { loadLiveView } from './components/preact/LiveView.js';
import { loadRecordingsView } from './components/preact/RecordingsView.js';
import { loadSettingsView } from './components/preact/SettingsView.js';
import { loadStreamsView } from './components/preact/StreamsView.js';
import { loadSystemView } from './components/preact/SystemView.js';
import { loadLoginView } from './components/preact/LoginView.js';
import { loadIndexView } from './components/preact/IndexView.js';
import { setupModals, addStatusMessageStyles, addModalStyles } from './components/preact/UI.js';
import './components/auth.js';  // Import authentication module

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
  timeout: null,
  type: 'success'
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
      activeNav = 'nav-live';
      break;
    case 'recordings.html':
      activeNav = 'nav-recordings';
      break;
    case 'timeline.html':
      activeNav = 'nav-timeline';
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
  if (currentPage === 'live.html') {
    loadLiveView();
  } else if (currentPage === 'recordings.html') {
    loadRecordingsView();
  } else if (currentPage === 'timeline.html') {
    // For timeline.html, we use the vanilla JS implementation
    // The initialization is handled by the script in timeline.html
    // Just load the header and footer
  } else if (currentPage === 'settings.html') {
    loadSettingsView();
  } else if (currentPage === 'streams.html') {
    loadStreamsView();
  } else if (currentPage === 'system.html') {
    loadSystemView();
  } else if (currentPage === 'login.html') {
    loadLoginView();
  } else if (currentPage === '' || currentPage === 'live.html') {
    loadLiveView();
  }
  
  // Setup UI components
  setupModals();
  addStatusMessageStyles();
  addModalStyles();
}

// Initialize the app when the DOM is loaded
document.addEventListener('DOMContentLoaded', initApp);

// Export for use in other modules
export { h, render, html };
