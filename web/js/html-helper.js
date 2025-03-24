/**
 * LightNVR Web Interface HTML Helper
 * Provides the html template literal function for Preact components
 */

import { h } from './preact.min.js';
import htm from './htm.module.js';

// Initialize htm with Preact's h
export const html = htm.bind(h);
