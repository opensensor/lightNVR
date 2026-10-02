import { useEffect, useState } from 'preact/hooks';
import { fetchJSON } from '../../fetch-utils.js';
import { useQuery, useQueryClient } from '../../query-client.js';
import { resolveUiMode } from '../../utils/mobile-ui.js';

const VALID_MODES = ['auto', 'mobile', 'desktop'];
const VIEWPORT_QUERY = '(max-width: 760px), (max-width: 1024px) and (pointer: coarse)';

function viewportMode() {
  if (typeof window.matchMedia !== 'function') return 'desktop';
  return window.matchMedia(VIEWPORT_QUERY).matches ? 'mobile' : 'desktop';
}

export function UiModeControl({ children }) {
  const accountKey = window._lightnvrUserKey || 'scoped';
  const queryClient = useQueryClient();
  const preferencesQuery = useQuery({
    queryKey: ['ui-preferences', accountKey],
    queryFn: () => fetchJSON('/api/ui/preferences', { retries: 0 }),
    retry: false,
  });
  const [viewport, setViewport] = useState(viewportMode);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState('');
  const [notice, setNotice] = useState('');
  const label = 'Режим интерфейса';
  const devMode = import.meta.env.DEV
    ? new URLSearchParams(window.location.search).get('ui_mode')
    : null;
  const serverMode = VALID_MODES.includes(preferencesQuery.data?.ui_mode)
    ? preferencesQuery.data.ui_mode
    : 'auto';
  const mode = resolveUiMode(serverMode, viewport, devMode);
  const isMobile = mode === 'mobile';

  useEffect(() => {
    const media = window.matchMedia?.(VIEWPORT_QUERY);
    const update = () => setViewport(viewportMode());
    if (media?.addEventListener) media.addEventListener('change', update);
    else media?.addListener?.(update);
    window.addEventListener('resize', update);
    return () => {
      if (media?.removeEventListener) media.removeEventListener('change', update);
      else media?.removeListener?.(update);
      window.removeEventListener('resize', update);
    };
  }, []);

  async function changeMode(event) {
    const nextMode = event.currentTarget.value;
    if (!VALID_MODES.includes(nextMode) || nextMode === serverMode || saving) return;
    setSaving(true);
    setError('');
    setNotice('');
    try {
      const result = await fetchJSON('/api/ui/preferences', {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ ui_mode: nextMode }),
      });
      queryClient.setQueryData(['ui-preferences', accountKey], result);
    } catch (requestError) {
      setError(requestError.status === 403
        ? 'Настройка интерфейса недоступна для этой сессии.'
        : 'Не удалось сохранить режим интерфейса. Проверьте соединение и повторите попытку.');
    } finally {
      setSaving(false);
    }
  }

  useEffect(() => {
    if (preferencesQuery.isError) {
      setNotice('Не удалось загрузить сохранённый режим. Используется автоматический выбор.');
    }
  }, [preferencesQuery.isError]);

  return (
    <>
      {preferencesQuery.isLoading
        ? <main className="mobile-live-page" role="status">Загружаем настройки интерфейса…</main>
        : <div className="ui-mode-control" aria-label={label}>
        <label htmlFor="ui-mode-select">{label}</label>
        <select
          id="ui-mode-select"
          value={serverMode}
          onChange={changeMode}
          disabled={saving || preferencesQuery.data?.configurable !== true}
          aria-label={label}
        >
          <option value="auto">Автоматически</option>
          <option value="mobile">Мобильный интерфейс</option>
          <option value="desktop">Компьютерный интерфейс</option>
        </select>
        {(error || notice) && <span className="ui-mode-message" role={error ? 'alert' : 'status'}>{error || notice}</span>}
        </div>}
      {!preferencesQuery.isLoading && (typeof children === 'function' ? children({ isMobile, uiMode: mode }) : children)}
    </>
  );
}

export { viewportMode };
