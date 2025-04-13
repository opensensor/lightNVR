/**
 * LightNVR Web Interface Preact App
 * Main entry point for the Preact application
 */

// Import from preact for better compatibility
import { createElement as h, render } from 'preact';
import { html } from './html-helper.js';
// Import WebSocketClient class directly
import { WebSocketClient, BatchDeleteRecordingsClient } from './websocket-client.js';
// Add React compatibility layer
import * as React from '@preact/compat';
window.React = React; // Make React available globally

import { loadHeader } from './components/preact/Header.js';
import { loadFooter } from './components/preact/Footer.js';
import { setupModals, addStatusMessageStyles, addModalStyles } from './components/preact/UI.js';
import { QueryClientProvider, queryClient } from './query-client.js';

// Initialize WebSocket client at the parent level
// This ensures a single WebSocket connection is shared across all components
if (typeof WebSocketClient !== 'undefined') {
  // Create a global WebSocket client instance
  window.wsClient = new WebSocketClient();
  console.log('WebSocket client initialized at application level');

  // Add additional event listeners for debugging
  if (window.wsClient) {
    // Log initial connection state
    console.log('Initial WebSocket connection state:', {
      connected: window.wsClient.isConnected(),
      clientId: window.wsClient.getClientId()
    });

    // Add socket event listeners when socket is created
    const originalConnect = window.wsClient.connect;
    window.wsClient.connect = function() {
      const result = originalConnect.apply(this, arguments);

      // Add event listeners to the new socket
      if (this.socket) {
        const originalOnOpen = this.socket.onopen;
        this.socket.onopen = (event) => {
          console.log('WebSocket connection opened at application level');
          if (originalOnOpen) originalOnOpen.call(this, event);
        };

        const originalOnError = this.socket.onerror;
        this.socket.onerror = (error) => {
          console.error('WebSocket error at application level:', error);
          if (originalOnError) originalOnError.call(this, error);
        };

        const originalOnClose = this.socket.onclose;
        this.socket.onclose = (event) => {
          console.log(`WebSocket connection closed at application level: ${event.code} ${event.reason}`);
          if (originalOnClose) originalOnClose.call(this, event);
        };

        const originalOnMessage = this.socket.onmessage;
        this.socket.onmessage = (event) => {
          // Only log non-welcome messages at application level to reduce noise
          if (!event.data.includes('"type":"welcome"')) {
            console.log('WebSocket message received at application level');
          }
          if (originalOnMessage) originalOnMessage.call(this, event);
        };
      }

      return result;
    };

    // Override handleMessage to log when client ID is set
    const originalHandleMessage = window.wsClient.handleMessage;
    window.wsClient.handleMessage = function(data) {
      const clientIdBefore = this.clientId;
      originalHandleMessage.call(this, data);
      const clientIdAfter = this.clientId;

      // Log when client ID changes
      if (clientIdBefore !== clientIdAfter && clientIdAfter) {
        console.log(`WebSocket client ID changed at application level: ${clientIdAfter}`);
      }
    };
  }

  // Initialize batch delete client if needed
  if (typeof BatchDeleteRecordingsClient !== 'undefined') {
    window.batchDeleteClient = new BatchDeleteRecordingsClient(window.wsClient);
    console.log('Batch delete client initialized');
  }
}

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

// Create a store for WebSocket state
export const websocketStore = createStore({
  client: window.wsClient || null,
  connected: window.wsClient ? window.wsClient.isConnected() : false,
  clientId: window.wsClient ? window.wsClient.getClientId() : null
});

// Update websocket store when connection status changes
if (window.wsClient) {
  // Set up a periodic check for WebSocket status
  setInterval(() => {
    if (window.wsClient) {
      const connected = window.wsClient.isConnected();
      const clientId = window.wsClient.getClientId();

      websocketStore.setState({
        connected,
        clientId
      });

      // If not connected and not already reconnecting, try to reconnect
      if (!connected && !window.wsClient.connecting && window.wsClient.reconnectAttempts < window.wsClient.maxReconnectAttempts) {
        console.log('WebSocket not connected, attempting to reconnect from application level');
        window.wsClient.connect();
      }
    }
  }, 5000); // Check every 5 seconds
}

/**
 * Patch for the setupGlobalAuthHandler function in preact-app.js
 * Add this at the beginning of your setupGlobalAuthHandler function
 */
function setupGlobalAuthHandler() {
  // Create a flag to prevent redirect loops
  window.preventAuthRedirects = false;

  // Check for auth pending flag on page load
  if (localStorage.getItem('authPending')) {
    console.log('Auth pending detected, skipping auth redirect check');
    window.preventAuthRedirects = true;
    // Remove the flag after a short delay
    setTimeout(() => {
      localStorage.removeItem('authPending');
      window.preventAuthRedirects = false;
    }, 5000);
  }

  // Override fetch to handle 401 responses globally
  const originalFetch = window.fetch;
  window.fetch = async function(url, options) {
    // Check if this is a POST request with auth_token
    const isAuthForm = options &&
        options.body instanceof FormData &&
        options.body.has('auth_token');

    // Skip auth checking during form submission redirect
    if (isAuthForm || window.preventAuthRedirects) {
      console.log('Skipping auth check for auth form submission');
      return originalFetch.apply(this, arguments);
    }

    // Normal fetch with response check
    const response = await originalFetch.apply(this, arguments);

    // Only process 401 if we're not in the login page and not currently preventing redirects
    if (response.status === 401 &&
        !window.location.pathname.includes('login.html') &&
        !window.preventAuthRedirects) {

      console.log('401 detected, redirecting to login');

      // Save current path for redirect after login
      const currentPath = window.location.pathname + window.location.search;

      // Simple redirect to login page
      window.location.href = '/login.html?logout=true&redirect=' + encodeURIComponent(currentPath);
    }

    return response;
  };
}

/**
 * Initialize the application
 */
function initApp() {
  // Set up global authentication handler
  setupGlobalAuthHandler();

  // Get current page and query parameters
  const currentPage = window.location.pathname.split('/').pop();
  const currentQuery = window.location.search;

  // Load header with active navigation
  let activeNav = '';

  switch (currentPage) {
    case 'index.html':
    case '': // Handle root URL (/) as index.html
      activeNav = 'nav-live';
      break;
    case 'recordings.html':
    case 'timeline.html':
      activeNav = 'nav-recordings';
      break;
    case 'streams.html':
      activeNav = 'nav-streams';
      break;
    case 'settings.html':
      activeNav = 'nav-settings';
      break;
    case 'users.html':
      activeNav = 'nav-users';
      break;
    case 'system.html':
      activeNav = 'nav-system';
      break;
  }

  // Ensure we have an app-root element
  let appRoot = document.getElementById('app-root');
  if (!appRoot) {
    // Create app-root if it doesn't exist
    appRoot = document.createElement('div');
    appRoot.id = 'app-root';
    const mainContent = document.getElementById('main-content');
    if (mainContent) {
      mainContent.appendChild(appRoot);
    } else {
      document.body.appendChild(appRoot);
    }
    console.log('Created app-root element for Preact Query provider');
  }

  // Wrap the app with QueryClientProvider
  render(
    h(QueryClientProvider, { client: queryClient },
      h('div', { id: 'app-content' })
    ),
    appRoot
  );

  // Load header and footer
  loadHeader(activeNav);
  loadFooter();

  // Load page-specific content using dynamic imports
  if (currentPage === 'index.html' || currentPage === '') {
    // Check if WebRTC is disabled before loading the view
    fetch('/api/settings')
      .then(response => response.json())
      .then(settings => {
        if (settings.webrtc_disabled) {
          // Load HLS view if WebRTC is disabled
          import('./components/preact/LiveViewQuery.js').then(module => {
            module.loadLiveView();
          }).catch(error => {
            console.error('Error loading LiveViewQuery:', error);
          });
        } else {
          // Load WebRTC view if enabled
          import('./components/preact/WebRTCView.js').then(module => {
            module.loadWebRTCView();
          }).catch(error => {
            console.error('Error loading WebRTCView:', error);
          });
        }
      })
      .catch(error => {
        console.error('Error checking WebRTC status:', error);
        // Fall back to WebRTC view on error
        import('./components/preact/WebRTCView.js').then(module => {
          module.loadWebRTCView();
        });
      });
  } else if (currentPage === 'hls.html') {
    import('./components/preact/LiveViewQuery.js').then(module => {
      module.loadLiveView();
    });
  } else if (currentPage === 'recordings.html') {
    import('./components/preact/RecordingsView.js').then(module => {
      module.loadRecordingsView();
    });
  } else if (currentPage === 'timeline.html') {
    import('./components/preact/timeline/TimelineView.js').then(module => {
      module.loadTimelineView();
    });
  } else if (currentPage === 'settings.html') {
    import('./components/preact/SettingsView.js').then(module => {
      module.loadSettingsView();
    });
  } else if (currentPage === 'streams.html') {
    import('./components/preact/StreamsView.js').then(module => {
      module.loadStreamsView();
    });
  } else if (currentPage === 'system.html') {
    import('./components/preact/SystemView.js').then(module => {
      module.loadSystemView();
    });
  } else if (currentPage === 'login.html') {
    import('./components/preact/LoginView.js').then(module => {
      module.loadLoginView();
    });
  } else if (currentPage === 'users.html') {
    import('./components/preact/UsersView.js').then(module => {
      module.loadUsersView();
    });
  } else if (currentPage === '') {
    import('./components/preact/WebRTCView.js').then(module => {
      module.loadWebRTCView();
    });
  }

  // Setup UI components
  setupModals();
  addStatusMessageStyles();
  addModalStyles();

  // Initialize toast container
  import('./components/preact/toast.js').then(({ initToastContainer }) => {
    // Initialize the toast container without showing a test message
    initToastContainer(false);

    // Log success
    console.log('Toast container initialized from preact-app.js');

    // Add a global function to test toasts
    window.testToastFromApp = (type = 'info') => {
      const message = `App test ${type} toast at ${new Date().toLocaleTimeString()}`;

      if (window.showToast) {
        window.showToast(message, type);
      }

      return 'App toast triggered';
    };
  }).catch(error => {
    console.error('Error initializing toast container:', error);
  });
}

// Initialize the app when the DOM is loaded
document.addEventListener('DOMContentLoaded', initApp);

// Export for use in other modules
export { h, render, html };
