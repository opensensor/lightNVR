/**
 * LightNVR Web Interface Navigation Utilities
 * Helper functions for reliable navigation from Preact event handlers.
 */

/**
 * Force navigation after the current event queue drains.
 *
 * @param {string} href Destination URL
 * @param {Event} [event] Event that triggered the navigation
 * @returns {boolean} Always false to simplify onClick usage
 */
export function forceNavigation(href, event) {
  if (event) {
    event.preventDefault();
    event.stopPropagation();
  }

  if (!href) {
    return false;
  }

  setTimeout(() => {
    window.location.href = href;
  }, 0);

  return false;
}