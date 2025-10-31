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
    <footer class="bg-card text-card-foreground py-3 px-4 mt-4 shadow-inner">
      <div class="container mx-auto flex justify-between items-center">
        <div class="text-sm text-muted-foreground">
          Lightweight Network Video Recorder© {year}
        </div>
        <div>
          <a href="https://github.com/opensensor/lightnvr" class="text-sm no-underline hover:underline"
             style={{color: 'hsl(var(--primary))'}}
             target="_blank" rel="noopener noreferrer">GitHub</a>
        </div>
      </div>
    </footer>
  );
}

