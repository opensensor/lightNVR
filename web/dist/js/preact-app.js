/**
 * LightNVR Web Interface Preact App
 * Main entry point for the Preact application
 */

import { h, render } from './preact.min.js';
import { html } from './html-helper.js';
import { loadHeader } from './components/preact/Header.js';
import { loadFooter } from './components/preact/Footer.js';
import { setupModals, addStatusMessageStyles, addModalStyles } from './components/preact/UI.js';

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
 * Initialize the application
 */
function initApp() {
  // Get current page
  const currentPage = window.location.pathname.split('/').pop();
  
  // Load header with active navigation
  let activeNav = '';
  
  switch (currentPage) {
    case 'index.html':
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
  
  // Load header and footer
  loadHeader(activeNav);
  loadFooter();
  
  // Load page-specific content using dynamic imports
  if (currentPage === 'index.html') {
    import("./components/preact/WebRTCView.js").then(module => {
      module.loadWebRTCView();
    });
  } else if (currentPage === 'hls.html') {
    import("./components/preact/LiveView.js").then(module => {
      module.loadLiveView();
    });
  } else if (currentPage === 'recordings.html') {
    import("./components/preact/RecordingsView.js").then(module => {
      module.loadRecordingsView();
    });
  } else if (currentPage === 'timeline.html') {
    import("./components/preact/timeline/TimelineView.js").then(module => {
      module.loadTimelineView();
    });
  } else if (currentPage === 'settings.html') {
    import("./components/preact/SettingsView.js").then(module => {
      module.loadSettingsView();
    });
  } else if (currentPage === 'streams.html') {
    import("./components/preact/StreamsView.js").then(module => {
      module.loadStreamsView();
    });
  } else if (currentPage === 'system.html') {
    import("./components/preact/SystemView.js").then(module => {
      module.loadSystemView();
    });
  } else if (currentPage === 'login.html') {
    import("./components/preact/LoginView.js").then(module => {
      module.loadLoginView();
    });
  } else if (currentPage === 'users.html') {
    import("./components/preact/UsersView.js").then(module => {
      module.loadUsersView();
    });
  } else if (currentPage === '') {
    import("./components/preact/WebRTCView.js").then(module => {
      module.loadWebRTCView();
    });
  }
  
  // Setup UI components
  setupModals();
  addStatusMessageStyles();
  addModalStyles();
  
  // Initialize toast container
  import("./components/preact/toast.js").then(({ initToastContainer }) => {
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
