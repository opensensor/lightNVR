/**
 * LightNVR Web Interface Footer Component
 * Preact component for the site footer
 */

import { h } from 'preact';
import { useState } from 'preact/hooks';

/**
 * Footer component
 * @param {Object} props - Component props
 * @returns {JSX.Element} Footer component
 */
export function Footer() {
  const [year] = useState(new Date().getFullYear());

  return (
    <footer class="bg-gray-800 text-white py-3 px-4 mt-4 shadow-inner">
      <div class="container mx-auto flex justify-between items-center">
        <div class="text-sm text-gray-400">
          Lightweight Network Video Recorder© {year}
        </div>
        <div>
          <a href="https://github.com/opensensor/lightnvr" class="text-blue-300 hover:text-blue-100 text-sm"
             target="_blank" rel="noopener noreferrer">GitHub</a>
        </div>
      </div>
    </footer>
  );
}

