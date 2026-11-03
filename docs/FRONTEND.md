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
│   │   ├── index-page.tsx            # Live view
│   │   ├── recordings-page.tsx       # Recording browser
│   │   ├── streams-page.tsx          # Stream management
│   │   ├── settings-page.tsx         # System settings
│   │   ├── system-page.tsx           # System info and logs
│   │   ├── users-page.tsx            # User management
│   │   ├── timeline-page.tsx         # Timeline playback
│   │   ├── hls-page.tsx              # Direct HLS viewer
│   │   └── login-page.tsx            # Authentication
│   ├── components/
│   │   └── preact/                   # Preact components
│   │       ├── LiveView.tsx          # Live camera grid
│   │       ├── WebRTCVideoCell.tsx   # WebRTC video player
│   │       ├── HLSVideoCell.tsx      # HLS video player
│   │       ├── MSEVideoCell.tsx      # MSE video player
│   │       ├── RecordingsView.tsx    # Recording browser
│   │       ├── StreamsView.tsx       # Stream list
│   │       ├── StreamConfigModal.tsx # Stream config with zones
│   │       ├── StreamDeleteModal.tsx # Stream deletion
│   │       ├── SettingsView.tsx      # System settings
│   │       ├── SystemView.tsx        # System info dashboard
│   │       ├── UsersView.tsx         # User management
│   │       ├── LoginView.tsx         # Login form
│   │       ├── ZoneEditor.tsx        # Detection zone editor
│   │       ├── DetectionOverlay.tsx  # Detection overlay
│   │       ├── PTZControls.tsx       # Pan-Tilt-Zoom controls
│   │       ├── ThemeCustomizer.tsx   # Theme/dark mode
│   │       ├── SnapshotManager.tsx   # Camera snapshots
│   │       ├── FullscreenManager.tsx # Fullscreen video
│   │       ├── BatchDeleteModal.tsx  # Batch recording delete
│   │       ├── Toast.tsx / ToastContainer.tsx  # Notifications
│   │       ├── Header.tsx / Footer.tsx         # Layout
│   │       ├── LoadingIndicator.tsx  # Loading spinner
│   │       ├── UI.tsx                # Shared UI primitives
│   │       ├── recordings/           # Recording sub-components
│   │       │   ├── ActiveFilters.tsx
│   │       │   ├── FiltersSidebar.tsx
│   │       │   ├── PaginationControls.tsx
│   │       │   ├── RecordingsTable.tsx
│   │       │   ├── formatUtils.ts
│   │       │   ├── recordingsAPI.tsx
│   │       │   └── urlUtils.ts
│   │       ├── system/               # System sub-components
│   │       │   ├── LogsView.tsx / LogsPoller.tsx
│   │       │   ├── MemoryStorage.tsx
│   │       │   ├── NetworkInfo.tsx
│   │       │   ├── StreamStorage.tsx / StreamsInfo.tsx
│   │       │   ├── SystemControls.tsx / SystemInfo.tsx
│   │       │   ├── RestartModal.tsx
│   │       │   └── SystemUtils.ts
│   │       ├── timeline/             # Timeline sub-components
│   │       │   ├── TimelinePage.tsx
│   │       │   ├── TimelinePlayer.tsx
│   │       │   ├── TimelineSegments.tsx
│   │       │   ├── TimelineRuler.tsx
│   │       │   ├── TimelineCursor.tsx
│   │       │   ├── TimelineControls.tsx
│   │       │   └── SpeedControls.tsx
│   │       └── users/                # User sub-components
│   │           ├── UsersTable.tsx
│   │           ├── AddUserModal.tsx
│   │           ├── EditUserModal.tsx
│   │           ├── DeleteUserModal.tsx
│   │           ├── ApiKeyModal.tsx
│   │           ├── TotpSetupModal.tsx
│   │           └── UserRoles.ts
│   ├── utils/                        # Utility modules
│   │   ├── auth-utils.ts
│   │   ├── dom-utils.ts
│   │   ├── settings-utils.ts
│   │   ├── theme-init.js
│   │   └── url-utils.ts
│   ├── lib/                          # Third-party libraries
│   ├── fetch-utils.ts                # HTTP fetch helpers
│   ├── query-client.ts               # @tanstack/query client
│   ├── url-param-handler.ts          # URL parameter management
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

- **LiveView**: Live camera grid with WebRTC, HLS, and MSE video cells
- **RecordingsView**: Recording browser with filtering, pagination, batch operations, and protection
- **StreamsView**: Stream list with CRUD, ONVIF discovery integration, and connection testing
- **StreamConfigModal**: Stream configuration with detection zone editor and PTZ settings
- **SettingsView**: System configuration including go2rtc, MQTT, and ONVIF settings
- **SystemView**: System dashboard with logs, memory/storage info, and restart controls
- **UsersView**: User management with roles, API keys, TOTP/MFA setup, and password controls
- **TimelinePage**: Timeline-based recording playback with segments, ruler, cursor, and speed controls
- **FisheyeEptzCanvas**: WebGL2 equidistant fisheye dewarping over an existing video decoder
- **PTZControls**: Pan-Tilt-Zoom camera controls with presets
- **ZoneEditor**: Detection zone polygon editor
- **DetectionOverlay**: Real-time detection result overlay on video
- **ThemeCustomizer**: Theme selection and dark mode toggle
- **UI**: Shared UI primitives (modals, buttons, form elements)

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

State management uses a combination of Preact hooks and @tanstack/query:

- **@tanstack/query (Preact Query)**: Primary data-fetching and server-state caching layer. Handles loading states, error handling, retries, cache invalidation, and stale-while-revalidate patterns.
- **useState / useEffect**: For component-local UI state (modals, forms, toggles)
- **useCallback / useMemo**: For memoized callbacks and computed values
- **useRef**: For DOM element references and persistent values across renders

### Data Fetching with Preact Query

The `query-client.ts` module provides custom hooks that wrap @tanstack/query with `enhancedFetch` (automatic timeout, retries, auth redirect on 401):

- `useQuery(key, url, options)` — Declarative data fetching with caching
- `useMutation(options)` — For POST/PUT/DELETE operations
- `usePostMutation(url)` / `usePutMutation(url)` — Convenience mutation hooks
- `useQueryClient()` — Access query client for cache invalidation
- `prefetchQuery(key, url)` — Prefetch data into cache

### Example: Fetching Streams

```javascript
import { useQuery, useQueryClient } from '../../query-client.ts';

const { data, isLoading, error } = useQuery(
  ['streams'], '/api/streams',
  { timeout: 10000, retries: 2, retryDelay: 1000 }
);
```

### Example: Mutation with Cache Invalidation

```javascript
const queryClient = useQueryClient();
const saveMutation = useMutation({
  mutationFn: async (streamData) => {
    const response = await fetch(`/api/streams/${id}`, {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify(streamData)
    });
    return response.json();
  },
  onSuccess: () => {
    queryClient.invalidateQueries({ queryKey: ['streams'] });
  }
});
```

## API Integration

The frontend communicates with the backend through a layered fetch architecture:

1. **`fetch-utils.ts`** — `enhancedFetch()` wraps the native Fetch API with timeouts, retries, abort controller support, and automatic 401 → login redirect. `fetchJSON()` adds JSON parsing.
2. **`query-client.ts`** — Wraps `enhancedFetch` in @tanstack/query hooks for declarative data fetching with caching and automatic background refetching.
3. **Components** — Use `useQuery` / `useMutation` hooks for all server communication. Direct `fetch()` calls are avoided in favor of the query hooks.

Session cookies (`credentials: 'same-origin'`) are automatically included in all requests.

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

- **Preact over React**: ~3KB virtual DOM library with identical API, significantly smaller than React
- **@tanstack/query Caching**: Server data cached with configurable stale times (5 min default), reducing redundant API calls
- **Vite Build**: Tree-shaking, code splitting, and minification for optimized production bundles
- **Memoization**: Using `useCallback` and `useMemo` to prevent unnecessary re-renders
- **Request Deduplication**: @tanstack/query automatically deduplicates concurrent requests for the same data
- **Abort Controllers**: Fetch requests are cancellable via AbortController to prevent stale responses
- **Efficient Rendering**: Using keys for list items and optimizing render cycles

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

Internet Explorer is not supported. WebRTC features require browser support for RTCPeerConnection.

## Future Enhancements

Planned enhancements for the frontend:

1. **Progressive Web App (PWA)** capabilities for offline access
2. **Customizable Dashboard** with drag-and-drop widgets
3. **Internationalization** support for multiple languages

**Note:** WebSocket support has been removed in favor of HTTP polling and @tanstack/query's background refetching. WebRTC live viewing is available via go2rtc integration.
