/**
 * Utility functions for the LiveView component
 */

/**
 * Show loading state
 * @param {HTMLElement} element - Element to show loading state on
 */
export function showLoading(element) {
  if (!element) return;
  console.log('Showing loading for element:', element);
  
  // Add loading class to element
  element.classList.add('loading');
  
  // Optionally add a loading spinner
  const spinner = document.createElement('div');
  spinner.className = 'spinner';
  element.appendChild(spinner);
}

/**
 * Hide loading state
 * @param {HTMLElement} element - Element to hide loading state from
 */
export function hideLoading(element) {
  if (!element) return;
  console.log('Hiding loading for element:', element);
  
  // Remove loading class from element
  element.classList.remove('loading');
  
  // Remove loading spinner if exists
  const spinner = element.querySelector('.spinner');
  if (spinner) {
    spinner.remove();
  }
}
