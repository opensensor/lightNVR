/**
 * LightNVR Web Interface HTML Helper
 * Provides the html template literal function for Preact components
 */

// Import htm as a module
import htm from './htm.module.js';
import { h } from 'preact';

// Initialize htm with Preact's createElement (h)
export const html = htm.bind(h);
