/**
 * AppearanceTab — ThemeCustomizer + the UXD T4 Reduce Motion toggle.
 *
 * Preserves the T4 Reduce Motion control verbatim; the parent owns the
 * preference state and handler and passes them in as props so there is a
 * single source of truth.
 *
 * Part of PRD UXD_01 §5.2 / T2 settings restructure (#399).
 */

import { ThemeCustomizer } from '../ThemeCustomizer.jsx';
import { REDUCE_MOTION_VALUES } from '../../../utils/reduceMotion.js';

export function AppearanceTab({ reduceMotionPref, handleReduceMotionChange, t }) {
  return (
    <div class="space-y-6">
      <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4">
        <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('settings.appearance')}</h3>
        <div>
          <ThemeCustomizer />

          {/* UXD T4 — Reduce Motion preference (§5.4).
              Auto respects prefers-reduced-motion; On/Off force-override. */}
          <div data-setting-label="Reduce motion" class="mt-6 pt-4 border-t border-border">
            <div class="flex flex-col gap-2 md:flex-row md:items-center md:justify-between">
              <div>
                <div class="font-medium">Reduce motion</div>
                <p class="text-sm text-muted-foreground mt-0.5">
                  Disables non-essential animations and transitions.
                  Auto follows your operating system preference.
                </p>
              </div>
              <div
                role="radiogroup"
                aria-label="Reduce motion preference"
                class="inline-flex rounded-md border border-border bg-background p-0.5 self-start md:self-auto"
              >
                {REDUCE_MOTION_VALUES.map((value) => {
                  const labelMap = { auto: 'Auto', on: 'On', off: 'Off' };
                  const isActive = reduceMotionPref === value;
                  return (
                    <button
                      key={value}
                      type="button"
                      role="radio"
                      aria-checked={isActive}
                      onClick={() => handleReduceMotionChange(value)}
                      class={`px-3 py-1.5 text-sm rounded transition-colors ${
                        isActive
                          ? 'bg-primary text-primary-foreground'
                          : 'text-foreground hover:bg-muted'
                      }`}
                    >
                      {labelMap[value]}
                    </button>
                  );
                })}
              </div>
            </div>
          </div>
        </div>
      </div>
    </div>
  );
}

export default AppearanceTab;
