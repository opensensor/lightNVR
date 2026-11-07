import { useI18n } from '../../../i18n.ts';
import { recordingViewLinks, type RecordingNavigationContext, type RecordingView } from './recordingNavigation.ts';

interface Props extends RecordingNavigationContext {
  active: RecordingView;
  onViewChange?: (view: 'table' | 'grid') => void;
}

/** One navigation landmark and visual treatment for all footage workspaces. */
export function RecordingsNavigation({ active, onViewChange, ...context }: Props) {
  const { t } = useI18n();
  const links = recordingViewLinks(context);
  return (
    <nav className="recordings-navigation" aria-label={t('recordings.views')}>
      {(['table', 'grid', 'timeline', 'investigation', 'plates'] as const).map(view => (
        <a key={view} href={links[view]} aria-current={active === view ? 'page' : undefined}
          onClick={event => {
            if (onViewChange && (view === 'table' || view === 'grid') && event.button === 0 &&
                !event.metaKey && !event.ctrlKey && !event.shiftKey && !event.altKey) {
              event.preventDefault();
              onViewChange(view);
            }
          }}>
          {t(view === 'table' || view === 'grid' ? `recordings.${view}` : `nav.${view}`)}
        </a>
      ))}
    </nav>
  );
}
