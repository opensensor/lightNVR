/**
 * LightNVR Web Interface Footer Component
 * Preact component for the site footer
 */

import { useState } from 'preact/hooks';
import { getCurrentYear } from '../../utils/date-utils.js';
import { useI18n } from '../../i18n.js';

/**
 * Footer component
 * @param {Object} props - Component props
 * @returns {JSX.Element} Footer component
 */
export function Footer() {
  const [year] = useState(getCurrentYear());
  const { t } = useI18n();

  return (
    <footer class="bg-card text-card-foreground py-3 px-4 mt-4 shadow-inner">
      <div class="container mx-auto flex justify-between items-center">
        <div class="text-sm text-muted-foreground">
          {t('footer.tagline')} © {year}
        </div>
        <div>
          <a href="https://github.com/opensensor/lightnvr" class="text-sm no-underline hover:underline"
             style={{color: 'hsl(var(--primary))'}}
             target="_blank" rel="noopener noreferrer">{t('footer.github')}</a>
        </div>
      </div>
    </footer>
  );
}

