/**
 * LightNVR Loading Indicator Component
 * Displays a loading spinner and optional message
 */


import { html } from '../../html-helper.js';

/**
 * LoadingIndicator component
 * @param {Object} props - Component props
 * @param {string} props.message - Optional message to display
 * @param {string} props.size - Size of the spinner (sm, md, lg)
 * @param {boolean} props.fullPage - Whether to display as a full page overlay
 * @returns {JSX.Element} LoadingIndicator component
 */
export function LoadingIndicator({ message = 'Loading...', size = 'md', fullPage = false }) {
  // Determine spinner size classes
  const sizeClasses = {
    sm: 'w-6 h-6',
    md: 'w-10 h-10',
    lg: 'w-16 h-16'
  };
  
  const spinnerSize = sizeClasses[size] || sizeClasses.md;
  
  // If fullPage is true, display as a full page overlay
  if (fullPage) {
    return html`
      <div class="fixed inset-0 bg-white bg-opacity-75 dark:bg-gray-900 dark:bg-opacity-75 flex items-center justify-center z-50">
        <div class="text-center">
          <div class="inline-block animate-spin rounded-full border-4 border-gray-300 dark:border-gray-600 border-t-blue-600 dark:border-t-blue-500 ${spinnerSize}"></div>
          ${message && html`<p class="mt-4 text-gray-700 dark:text-gray-300 text-lg">${message}</p>`}
        </div>
      </div>
    `;
  }
  
  // Default display as an inline element
  return html`
    <div class="flex flex-col items-center justify-center py-8">
      <div class="inline-block animate-spin rounded-full border-4 border-gray-300 dark:border-gray-600 border-t-blue-600 dark:border-t-blue-500 ${spinnerSize}"></div>
      ${message && html`<p class="mt-4 text-gray-700 dark:text-gray-300">${message}</p>`}
    </div>
  `;
}

/**
 * ContentLoader component - displays a loading indicator or content based on loading state
 * @param {Object} props - Component props
 * @param {boolean} props.isLoading - Whether content is loading
 * @param {boolean} props.hasData - Whether there is data to display
 * @param {JSX.Element} props.children - Content to display when not loading
 * @param {string} props.loadingMessage - Message to display while loading
 * @param {JSX.Element} props.emptyMessage - Message to display when there is no data
 * @returns {JSX.Element} ContentLoader component
 */
export function ContentLoader({ 
  isLoading, 
  hasData, 
  children, 
  loadingMessage = 'Loading data...', 
  emptyMessage = 'No data available' 
}) {
  if (isLoading) {
    return html`<${LoadingIndicator} message=${loadingMessage} />`;
  }
  
  if (!hasData) {
    return html`
      <div class="flex flex-col items-center justify-center py-12 text-center">
        <svg class="w-16 h-16 text-gray-400 dark:text-gray-600 mb-4" fill="none" stroke="currentColor" viewBox="0 0 24 24" xmlns="http://www.w3.org/2000/svg">
          <path stroke-linecap="round" stroke-linejoin="round" stroke-width="2" d="M9.172 16.172a4 4 0 015.656 0M9 10h.01M15 10h.01M21 12a9 9 0 11-18 0 9 9 0 0118 0z"></path>
        </svg>
        <p class="text-gray-600 dark:text-gray-400 text-lg">${emptyMessage}</p>
      </div>
    `;
  }
  
  return children;
}
