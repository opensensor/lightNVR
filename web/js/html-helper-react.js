/**
 * LightNVR Web Interface HTML Helper
 * Provides the html template literal function for React components
 */

// Import htm as a module
import htm from './htm.module.js';

// React is loaded as a UMD module and is available as a global variable
// Initialize htm with React's createElement
export const html = htm.bind(React.createElement);
