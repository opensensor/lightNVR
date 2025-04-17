/**
 * LightNVR Web Interface DOM Utilities
 * Provides abstracted methods for DOM manipulation to avoid direct DOM access
 */

/**
 * Get an element by its ID
 * @param {string} id - Element ID
 * @returns {HTMLElement|null} - The element or null if not found
 */
export function getElementById(id) {
  return document.getElementById(id);
}

/**
 * Query selector for a single element
 * @param {string} selector - CSS selector
 * @param {HTMLElement} [parent=document] - Parent element to query within
 * @returns {HTMLElement|null} - The element or null if not found
 */
export function querySelector(selector, parent = document) {
  return parent.querySelector(selector);
}

/**
 * Query selector for multiple elements
 * @param {string} selector - CSS selector
 * @param {HTMLElement} [parent=document] - Parent element to query within
 * @returns {NodeList} - List of matching elements
 */
export function querySelectorAll(selector, parent = document) {
  return parent.querySelectorAll(selector);
}

/**
 * Create an element with optional attributes, properties, and children
 * @param {string} tagName - Tag name of the element to create
 * @param {Object} [options={}] - Options for the element
 * @param {Object} [options.attributes={}] - Attributes to set on the element
 * @param {Object} [options.properties={}] - Properties to set on the element
 * @param {string} [options.textContent] - Text content to set on the element
 * @param {string} [options.innerHTML] - Inner HTML to set on the element
 * @param {string} [options.className] - Class name(s) to set on the element
 * @param {Array} [options.children=[]] - Child elements to append
 * @returns {HTMLElement} - The created element
 */
export function createElement(tagName, options = {}) {
  const element = document.createElement(tagName);
  
  // Set attributes
  if (options.attributes) {
    Object.entries(options.attributes).forEach(([key, value]) => {
      element.setAttribute(key, value);
    });
  }
  
  // Set properties
  if (options.properties) {
    Object.entries(options.properties).forEach(([key, value]) => {
      element[key] = value;
    });
  }
  
  // Set text content
  if (options.textContent !== undefined) {
    element.textContent = options.textContent;
  }
  
  // Set inner HTML
  if (options.innerHTML !== undefined) {
    element.innerHTML = options.innerHTML;
  }
  
  // Set class name
  if (options.className) {
    element.className = options.className;
  }
  
  // Append children
  if (options.children) {
    options.children.forEach(child => {
      element.appendChild(child);
    });
  }
  
  return element;
}

/**
 * Append a child element to a parent element
 * @param {HTMLElement} parent - Parent element
 * @param {HTMLElement} child - Child element to append
 * @returns {HTMLElement} - The appended child element
 */
export function appendChild(parent, child) {
  return parent.appendChild(child);
}

/**
 * Remove a child element from a parent element
 * @param {HTMLElement} parent - Parent element
 * @param {HTMLElement} child - Child element to remove
 * @returns {HTMLElement} - The removed child element
 */
export function removeChild(parent, child) {
  return parent.removeChild(child);
}

/**
 * Add an event listener to an element
 * @param {HTMLElement} element - Element to add listener to
 * @param {string} eventType - Type of event
 * @param {Function} listener - Event listener function
 * @param {Object} [options] - Event listener options
 */
export function addEventListener(element, eventType, listener, options) {
  element.addEventListener(eventType, listener, options);
}

/**
 * Remove an event listener from an element
 * @param {HTMLElement} element - Element to remove listener from
 * @param {string} eventType - Type of event
 * @param {Function} listener - Event listener function
 * @param {Object} [options] - Event listener options
 */
export function removeEventListener(element, eventType, listener, options) {
  element.removeEventListener(eventType, listener, options);
}

/**
 * Add a class to an element
 * @param {HTMLElement} element - Element to add class to
 * @param {string} className - Class name to add
 */
export function addClass(element, className) {
  element.classList.add(className);
}

/**
 * Remove a class from an element
 * @param {HTMLElement} element - Element to remove class from
 * @param {string} className - Class name to remove
 */
export function removeClass(element, className) {
  element.classList.remove(className);
}

/**
 * Toggle a class on an element
 * @param {HTMLElement} element - Element to toggle class on
 * @param {string} className - Class name to toggle
 * @param {boolean} [force] - Force add or remove
 * @returns {boolean} - Whether the class is now present
 */
export function toggleClass(element, className, force) {
  return element.classList.toggle(className, force);
}

/**
 * Check if an element has a class
 * @param {HTMLElement} element - Element to check
 * @param {string} className - Class name to check for
 * @returns {boolean} - Whether the element has the class
 */
export function hasClass(element, className) {
  return element.classList.contains(className);
}

/**
 * Set the text content of an element
 * @param {HTMLElement} element - Element to set text content on
 * @param {string} text - Text content to set
 */
export function setTextContent(element, text) {
  element.textContent = text;
}

/**
 * Set the inner HTML of an element
 * @param {HTMLElement} element - Element to set inner HTML on
 * @param {string} html - HTML content to set
 */
export function setInnerHTML(element, html) {
  element.innerHTML = html;
}

/**
 * Get the body element
 * @returns {HTMLElement} - The body element
 */
export function getBodyElement() {
  return document.body;
}

/**
 * Append an element to the body
 * @param {HTMLElement} element - Element to append to body
 * @returns {HTMLElement} - The appended element
 */
export function appendToBody(element) {
  return document.body.appendChild(element);
}

/**
 * Remove an element from the body
 * @param {HTMLElement} element - Element to remove from body
 * @returns {HTMLElement} - The removed element
 */
export function removeFromBody(element) {
  return document.body.removeChild(element);
}

/**
 * Create and append a link element for downloading
 * @param {string} url - URL to download
 * @param {string} [filename] - Filename for the download
 */
export function triggerDownload(url, filename) {
  const link = createElement('a', {
    attributes: {
      href: url,
      download: filename || ''
    }
  });
  
  appendToBody(link);
  link.click();
  removeFromBody(link);
}

/**
 * Show a loading indicator on an element
 * @param {HTMLElement} element - Element to show loading on
 */
export function showLoading(element) {
  if (!element) return;

  // Add loading class
  addClass(element, 'loading');

  // Create loading overlay if it doesn't exist
  let loadingOverlay = querySelector('.loading-overlay', element);
  if (!loadingOverlay) {
    loadingOverlay = createElement('div', {
      className: 'loading-overlay',
      innerHTML: '<div class="loading-spinner"></div>'
    });
    appendChild(element, loadingOverlay);
  }

  // Show loading overlay
  loadingOverlay.style.display = 'flex';
}

/**
 * Hide loading indicator on an element
 * @param {HTMLElement} element - Element to hide loading from
 */
export function hideLoading(element) {
  if (!element) return;

  // Remove loading class
  removeClass(element, 'loading');

  // Hide loading overlay
  const loadingOverlay = querySelector('.loading-overlay', element);
  if (loadingOverlay) {
    loadingOverlay.style.display = 'none';
  }
}
