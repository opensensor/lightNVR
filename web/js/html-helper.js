/**
 * LightNVR Web Interface HTML Helper
 * Provides the html template literal function for Preact components
 */

import { h } from 'preact';
import htm from 'htm';

// Initialize htm with Preact's h
export const html = htm.bind(h);
