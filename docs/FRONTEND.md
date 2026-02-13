# LightNVR Frontend Architecture

This document describes the frontend architecture of the LightNVR web interface.

## Overview

The LightNVR frontend is a multi-page application built with modern web technologies:

- **Preact** for lightweight component-based UI (React-compatible)
- **Tailwind CSS** for utility-first styling with dark mode and theme customization
- **Vite** for build tooling, bundling, and development server
- **@tanstack/query** for data fetching and caching
- **HLS.js** for HLS video playback
- **WebRTC/MSE** for low-latency live viewing via go2rtc

## Directory Structure

```
web/
├── css/                              # Stylesheets
├── js/
│   ├── pages/                        # Page-level entry points
│   │   ├── index-page.jsx            # Live view
│   │   ├── recordings-page.jsx       # Recording browser
│   │   ├── streams-page.jsx          # Stream management
│   │   ├── settings-page.jsx         # System settings
│   │   ├── system-page.jsx           # System info and logs
│   │   ├── users-page.jsx            # User management
│   │   ├── timeline-page.jsx         # Timeline playback
│   │   ├── hls-page.jsx              # Direct HLS viewer
│   │   └── login-page.jsx            # Authentication
│   ├── components/
│   │   └── preact/                   # Preact components
│   │       ├── LiveView.jsx          # Live camera grid
│   │       ├── WebRTCVideoCell.jsx   # WebRTC video player
│   │       ├── HLSVideoCell.jsx      # HLS video player
│   │       ├── MSEVideoCell.jsx      # MSE video player
│   │       ├── RecordingsView.jsx    # Recording browser
│   │       ├── StreamsView.jsx       # Stream list
│   │       ├── StreamConfigModal.jsx # Stream config with zones
│   │       ├── StreamDeleteModal.jsx # Stream deletion
│   │       ├── SettingsView.jsx      # System settings
│   │       ├── SystemView.jsx        # System info dashboard
│   │       ├── UsersView.jsx         # User management
│   │       ├── LoginView.jsx         # Login form
│   │       ├── ZoneEditor.jsx        # Detection zone editor
│   │       ├── DetectionOverlay.jsx  # Detection overlay
│   │       ├── PTZControls.jsx       # Pan-Tilt-Zoom controls
│   │       ├── ThemeCustomizer.jsx   # Theme/dark mode
│   │       ├── SnapshotManager.jsx   # Camera snapshots
│   │       ├── FullscreenManager.jsx # Fullscreen video
│   │       ├── BatchDeleteModal.jsx  # Batch recording delete
│   │       ├── Toast.jsx / ToastContainer.jsx  # Notifications
│   │       ├── Header.jsx / Footer.jsx         # Layout
│   │       ├── LoadingIndicator.jsx  # Loading spinner
│   │       ├── UI.jsx                # Shared UI primitives
│   │       ├── recordings/           # Recording sub-components
│   │       │   ├── ActiveFilters.jsx
│   │       │   ├── FiltersSidebar.jsx
│   │       │   ├── PaginationControls.jsx
│   │       │   ├── RecordingsTable.jsx
│   │       │   ├── formatUtils.js
│   │       │   ├── recordingsAPI.jsx
│   │       │   └── urlUtils.js
│   │       ├── system/               # System sub-components
│   │       │   ├── LogsView.jsx / LogsPoller.jsx
│   │       │   ├── MemoryStorage.jsx
│   │       │   ├── NetworkInfo.jsx
│   │       │   ├── StreamStorage.jsx / StreamsInfo.jsx
│   │       │   ├── SystemControls.jsx / SystemInfo.jsx
│   │       │   ├── RestartModal.jsx
│   │       │   └── SystemUtils.js
│   │       ├── timeline/             # Timeline sub-components
│   │       │   ├── TimelinePage.jsx
│   │       │   ├── TimelinePlayer.jsx
│   │       │   ├── TimelineSegments.jsx
│   │       │   ├── TimelineRuler.jsx
│   │       │   ├── TimelineCursor.jsx
│   │       │   ├── TimelineControls.jsx
│   │       │   └── SpeedControls.jsx
│   │       └── users/                # User sub-components
│   │           ├── UsersTable.jsx
│   │           ├── AddUserModal.jsx
│   │           ├── EditUserModal.jsx
│   │           ├── DeleteUserModal.jsx
│   │           ├── ApiKeyModal.jsx
│   │           ├── TotpSetupModal.jsx
│   │           └── UserRoles.js
│   ├── utils/                        # Utility modules
│   │   ├── auth-utils.js
│   │   ├── dom-utils.js
│   │   ├── settings-utils.js
│   │   ├── theme-init.js
│   │   └── url-utils.js
│   ├── lib/                          # Third-party libraries
│   ├── fetch-utils.js                # HTTP fetch helpers
│   ├── query-client.js               # @tanstack/query client
│   ├── url-param-handler.js          # URL parameter management
│   └── version.js                    # Version info
└── *.html                            # HTML entry points
```

## Tailwind CSS Integration

Tailwind CSS is a utility-first CSS framework that allows for rapid UI development with predefined utility classes. In LightNVR, Tailwind is used for all styling needs.

### Key Features

- **Utility Classes**: Using classes like `flex`, `p-4`, `text-lg` directly in HTML
- **Responsive Design**: Built-in responsive utilities like `md:flex-row`
- **Dark Mode Support**: Using `dark:` variant for dark mode styles
- **Custom Components**: Consistent styling for buttons, cards, and other UI elements

### Example

```html
<div class="p-4 bg-white dark:bg-gray-800 rounded-lg shadow">
  <h2 class="text-xl font-bold mb-4">Recordings</h2>
  <div class="flex flex-col md:flex-row gap-4">
    <!-- Content here -->
  </div>
</div>
```

## Preact Components

Preact is a lightweight alternative to React with the same modern API. It's used to create reusable UI components throughout the application.

### Key Components

- **LiveView**: Displays live camera feeds
- **RecordingsView**: Manages recording playback and filtering
- **SettingsView**: Handles system configuration
- **StreamsView**: Manages camera streams
- **UI**: Shared UI components like modals, notifications, and form elements

### Component Example

```javascript
import { h } from 'preact';
import { useState, useEffect } from 'preact/hooks';

export function StreamCard({ stream, onToggle, onEdit, onDelete }) {
  const [isExpanded, setIsExpanded] = useState(false);
  
  return (
    <div class="bg-white dark:bg-gray-800 rounded-lg shadow p-4 mb-4">
      <div class="flex justify-between items-center">
        <h3 class="text-lg font-medium">{stream.name}</h3>
        <div class="flex space-x-2">
          <button 
            onClick={() => onToggle(stream.id)}
            class={`px-3 py-1 rounded ${stream.enabled 
              ? 'bg-green-600 text-white' 
              : 'bg-gray-300 dark:bg-gray-600'}`}
          >
            {stream.enabled ? 'Enabled' : 'Disabled'}
          </button>
          <button 
            onClick={() => setIsExpanded(!isExpanded)}
            class="p-1 rounded-full hover:bg-gray-200 dark:hover:bg-gray-700"
          >
            <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
              {/* SVG path */}
            </svg>
          </button>
        </div>
      </div>
      
      {isExpanded && (
        <div class="mt-4 border-t pt-4 dark:border-gray-700">
          <p>URL: {stream.url}</p>
          <p>Resolution: {stream.width}x{stream.height} @ {stream.fps}fps</p>
          <div class="mt-4 flex space-x-2">
            <button 
              onClick={() => onEdit(stream)}
              class="px-3 py-1 bg-blue-600 text-white rounded"
            >
              Edit
            </button>
            <button 
              onClick={() => onDelete(stream.id)}
              class="px-3 py-1 bg-red-600 text-white rounded"
            >
              Delete
            </button>
          </div>
        </div>
      )}
    </div>
  );
}
```

## State Management

State management in the LightNVR frontend is handled using Preact's built-in hooks:

- **useState**: For component-local state
- **useEffect**: For side effects like data fetching
- **useCallback**: For memoized callbacks
- **useRef**: For persistent references to DOM elements

For more complex state management, custom hooks are used to encapsulate related logic.

### Example: Custom Hook for API Data

```javascript
function useApiData(endpoint, defaultValue = []) {
  const [data, setData] = useState(defaultValue);
  const [loading, setLoading] = useState(true);
  const [error, setError] = useState(null);
  
  useEffect(() => {
    fetch(endpoint)
      .then(response => {
        if (!response.ok) {
          throw new Error('Network response was not ok');
        }
        return response.json();
      })
      .then(data => {
        setData(data);
        setLoading(false);
      })
      .catch(error => {
        setError(error.message);
        setLoading(false);
      });
  }, [endpoint]);
  
  return { data, loading, error };
}
```

## API Integration

The frontend communicates with the backend API using the Fetch API. API calls are typically made in the following scenarios:

- When a component mounts (via `useEffect`)
- In response to user actions (button clicks, form submissions)
- On a timer for data that needs to be periodically refreshed

### Example: Fetching Streams

```javascript
useEffect(() => {
  // Fetch streams when component mounts
  fetch('/api/streams')
    .then(response => {
      if (!response.ok) {
        throw new Error('Network response was not ok');
      }
      return response.json();
    })
    .then(data => {
      setStreams(data.streams);
      setLoading(false);
    })
    .catch(error => {
      setError(error.message);
      setLoading(false);
    });
}, []);
```

## Responsive Design

The LightNVR frontend is designed to be responsive and work well on devices of all sizes, from mobile phones to desktop monitors. This is achieved through:

- Tailwind's responsive utilities (`sm:`, `md:`, `lg:` prefixes)
- Flexible layouts using CSS Grid and Flexbox
- Mobile-first design approach
- Media queries for specific breakpoints

### Example: Responsive Layout

```html
<div class="recordings-layout flex flex-col md:flex-row gap-4">
  <!-- Sidebar for filters -->
  <aside class="w-full md:w-64 bg-white dark:bg-gray-800 rounded-lg shadow p-4">
    <!-- Filters content -->
  </aside>
  
  <!-- Main content area -->
  <div class="flex-1">
    <!-- Recordings content -->
  </div>
</div>
```

## Dark Mode Support

The LightNVR frontend supports both light and dark modes, using Tailwind's dark mode utilities. The mode can be toggled by the user and is persisted across sessions.

### Example: Dark Mode Toggle

```javascript
function DarkModeToggle() {
  const [isDarkMode, setIsDarkMode] = useState(
    document.documentElement.classList.contains('dark')
  );
  
  const toggleDarkMode = () => {
    if (isDarkMode) {
      document.documentElement.classList.remove('dark');
      localStorage.theme = 'light';
    } else {
      document.documentElement.classList.add('dark');
      localStorage.theme = 'dark';
    }
    setIsDarkMode(!isDarkMode);
  };
  
  return (
    <button 
      onClick={toggleDarkMode}
      class="p-2 rounded-full bg-gray-200 dark:bg-gray-700"
      title="Toggle dark mode"
    >
      {isDarkMode ? (
        <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
          {/* Sun icon */}
        </svg>
      ) : (
        <svg class="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
          {/* Moon icon */}
        </svg>
      )}
    </button>
  );
}
```

## Performance Optimizations

Several performance optimizations are implemented in the frontend:

- **Code Splitting**: Loading components only when needed
- **Memoization**: Using `useCallback` and `useMemo` to prevent unnecessary re-renders
- **Lazy Loading**: Images and non-critical resources are loaded lazily
- **Efficient Rendering**: Using keys for list items and optimizing render cycles
- **Minimal Dependencies**: Using lightweight libraries and avoiding bloat

## Accessibility

The LightNVR frontend is designed with accessibility in mind:

- Proper semantic HTML elements
- ARIA attributes where appropriate
- Keyboard navigation support
- Sufficient color contrast
- Focus management for modals and dialogs

## Browser Compatibility

The frontend is compatible with modern browsers:

- Chrome/Edge (latest 2 versions)
- Firefox (latest 2 versions)
- Safari (latest 2 versions)

Internet Explorer is not supported.

## Future Enhancements

Planned enhancements for the frontend:

1. **Progressive Web App (PWA)** capabilities for offline access
2. **Enhanced WebRTC Support** for lower-latency live viewing (via go2rtc integration)
3. **Advanced Filtering** for recordings and events
4. **Customizable Dashboard** with drag-and-drop widgets
5. **Internationalization** support for multiple languages

**Note:** WebSocket support has been removed as of version 0.11.22 to simplify the architecture. Real-time updates are now handled via HTTP polling.
