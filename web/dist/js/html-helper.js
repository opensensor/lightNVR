/**
 * LightNVR Web Interface HTML Helper
 * Provides the html template literal function for Preact components
 */

import { h } from '../_snowpack/pkg/preact.js';
import htm from '../_snowpack/pkg/htm.js';

// Initialize htm with Preact's h
export const html = htm.bind(h);
