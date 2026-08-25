import { useI18n } from '../../i18n.js';

export function AlwaysFullscreenToggle({ enabled, onChange }) {
  const { t } = useI18n();
  const label = enabled
    ? t('live.alwaysFullscreenOnTapDisable')
    : t('live.alwaysFullscreenOnTapEnable');
  return (
    <button
      type="button"
      className={`rounded-full p-2 focus:outline-none focus:ring-2 focus:ring-primary ${enabled ? 'bg-primary text-primary-foreground hover:bg-primary/90' : 'bg-secondary text-secondary-foreground hover:bg-secondary/80'}`}
      onClick={() => onChange(!enabled)}
      title={label}
      aria-label={label}
      aria-pressed={enabled}
    >
      <svg viewBox="0 0 24 24" width="20" height="20" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
        <rect x="6" y="2.5" width="12" height="19" rx="2" />
        <path d="M9 8V6h2M15 8V6h-2M9 14v2h2M15 14v2h-2" />
      </svg>
    </button>
  );
}

export default AlwaysFullscreenToggle;
