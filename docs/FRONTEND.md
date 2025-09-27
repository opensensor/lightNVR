# LightNVR Frontend Architecture

This document describes the frontend architecture of the LightNVR web interface, which has been modernized to use Tailwind CSS and Preact.

## Overview

The LightNVR frontend has been redesigned to use modern web technologies:

- **Tailwind CSS** for utility-first styling
- **Preact** for lightweight component-based UI
- **Modular JavaScript** for better organization and maintainability

This approach provides several benefits:

- Improved performance and responsiveness
- Better code organization and maintainability
- Consistent styling across the application
- Smaller bundle size compared to larger frameworks

## Directory Structure

```
web/
├── css/
│   ├── base.css          # Base styles and Tailwind imports
│   ├── components.css    # Component-specific styles
│   ├── layout.css        # Layout styles
│   ├── live.css          # Live view specific styles
│   └── recordings.css    # Recordings view specific styles
├── js/
│   ├── components/       # JavaScript components
│   │   └── preact/       # Preact components
│   │       ├── LiveView.jsx
│   │       ├── RecordingsView.jsx
│   │       ├── SettingsView.jsx
│   │       ├── StreamsView.js
│   │       └── UI.js     # Shared UI components
│   ├── preact.min.js     # Preact library
│   ├── preact.hooks.module.js # Preact hooks
│   └── preact-app.js     # Preact application setup
└── *.html                # HTML entry points
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
