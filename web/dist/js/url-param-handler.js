/**
 * URL Parameter Handler
 * This script ensures that URL parameters are properly preserved when the page is reloaded
 */

// Function to run when the page loads
function handleUrlParams() {
  // Get current URL parameters
  const urlParams = new URLSearchParams(window.location.search);
  
  // Check if we have the detection parameter
  if (urlParams.has('detection') && urlParams.get('detection') === '1') {
    console.log('Detection parameter found in URL, ensuring it is preserved');
    
    // Create a function to check and update the detection filter
    const updateDetectionFilter = () => {
      // Try to find the detection filter dropdown
      const detectionFilter = document.getElementById('detection-filter');
      if (detectionFilter) {
        // Set the filter to 'detection' if it's not already
        if (detectionFilter.value !== 'detection') {
          console.log('Setting detection filter to "detection"');
          detectionFilter.value = 'detection';
          
          // Dispatch a change event to trigger any event listeners
          const event = new Event('change', { bubbles: true });
          detectionFilter.dispatchEvent(event);
        }
        return true;
      }
      return false;
    };
    
    // Try to update the filter immediately
    if (!updateDetectionFilter()) {
      // If the filter element doesn't exist yet, set up a mutation observer to watch for it
      console.log('Detection filter not found, setting up observer');
      
      // Create a mutation observer to watch for changes to the DOM
      const observer = new MutationObserver((mutations) => {
        // Check if the filter exists now
        if (updateDetectionFilter()) {
          // If we found and updated the filter, disconnect the observer
          observer.disconnect();
        }
      });
      
      // Start observing the document with the configured parameters
      observer.observe(document.body, { childList: true, subtree: true });
      
      // Also set a timeout to stop the observer after a reasonable time
      setTimeout(() => {
        observer.disconnect();
      }, 10000); // 10 seconds
    }
  }
}

// Run the function when the DOM is fully loaded
document.addEventListener('DOMContentLoaded', handleUrlParams);
