import { useEffect, useMemo, useRef, useState } from 'preact/hooks';
import { fetchJSON } from '../../fetch-utils.js';
import { fetchAllStreamSummaries } from '../../utils/stream-summaries.js';
import { useQuery, useQueryClient } from '../../query-client.js';
import { PlaybackTransportCell } from './PlaybackTransportCell.jsx';
import { requestNativeFullscreen, exitNativeFullscreen, getNativeFullscreenElement } from './fullscreenApi.js';
import { resolveForcedLiveTransport } from '../../utils/live-view-url.js';
import { compareFavoriteCameras, MOBILE_CAMERA_PAGE_SIZE, paginateMobileCameras } from '../../utils/mobile-ui.js';

function MobileCameraCard({ stream, favorite, canFavorite, onFavorite, busy, viewFlags, forcedTransport }) {
  const root = useRef(null);
  const [visible, setVisible] = useState(false);
  const [fullscreen, setFullscreen] = useState(false);
  useEffect(() => {
    if (!root.current) return undefined;
    if (!('IntersectionObserver' in window)) { setVisible(true); return undefined; }
    const observer = new IntersectionObserver(([entry]) => {
      setVisible(entry.isIntersecting || (fullscreen && Boolean(getNativeFullscreenElement())));
    }, { rootMargin: '160px' });
    observer.observe(root.current);
    return () => observer.disconnect();
  }, [fullscreen]);

  useEffect(() => {
    const syncFullscreen = () => {
      const element = getNativeFullscreenElement();
      const active = Boolean(element && root.current?.contains(element));
      setFullscreen(active);
    };
    document.addEventListener('fullscreenchange', syncFullscreen);
    document.addEventListener('webkitfullscreenchange', syncFullscreen);
    document.addEventListener('lightnvr:pseudo-fullscreenchange', syncFullscreen);
    return () => {
      document.removeEventListener('fullscreenchange', syncFullscreen);
      document.removeEventListener('webkitfullscreenchange', syncFullscreen);
      document.removeEventListener('lightnvr:pseudo-fullscreenchange', syncFullscreen);
      const element = getNativeFullscreenElement();
      if (element && root.current?.contains(element)) exitNativeFullscreen().catch(() => {});
    };
  }, []);

  async function toggleFullscreen(_streamName, event, element) {
    event?.stopPropagation?.();
    const target = element || root.current?.querySelector('.video-cell') || root.current;
    try {
      if (getNativeFullscreenElement()) await exitNativeFullscreen();
      else await requestNativeFullscreen(target);
    } finally { setFullscreen(Boolean(getNativeFullscreenElement())); }
  }

  return (
    <article className="mobile-camera-card" ref={root} aria-label={`Камера ${stream.name}`}>
      <header className="mobile-camera-heading">
        <h2>{stream.name}</h2>
        {canFavorite && <button
          type="button"
          className="mobile-favorite-button"
          aria-label={favorite ? `Убрать ${stream.name} из избранного` : `Добавить ${stream.name} в избранное`}
          aria-pressed={favorite}
          disabled={busy}
          onClick={() => onFavorite(stream.camera_uuid)}
        >{favorite ? '★' : '☆'}</button>}
      </header>
      <div className="mobile-camera-player">
        {visible ? <PlaybackTransportCell
          stream={stream}
          audioDisabled={viewFlags.audioDisabled}
          offerings={{ webrtc: !viewFlags.webrtcDisabled, mse: !viewFlags.mseDisabled, hls: !viewFlags.hlsDisabled }}
          defaultTransport="webrtc"
          forcedTransport={forcedTransport}
          streamId={stream.name}
          showLabels
          showControls
          onToggleFullscreen={toggleFullscreen}
          fullscreenUpgraded={fullscreen}
        /> : <div className="mobile-camera-placeholder" aria-label={`Поток камеры ${stream.name} загрузится при прокрутке`} />}
      </div>
    </article>
  );
}

export function MobileLiveView({ viewFlags }) {
  const accountKey = window._lightnvrUserKey || 'scoped';
  const queryClient = useQueryClient();
  const [search, setSearch] = useState('');
  const [section, setSection] = useState('all');
  const [page, setPage] = useState(0);
  const [busyUuid, setBusyUuid] = useState('');
  const [mutationError, setMutationError] = useState('');
  const streamsQuery = useQuery({
    queryKey: ['mobile-live-streams', accountKey],
    queryFn: ({ signal }) => fetchAllStreamSummaries({ surface: 'live', availability: 'all', signal }),
    staleTime: 30000,
    refetchInterval: 30000,
  });
  const favoritesQuery = useQuery({
    queryKey: ['ui-favorites', accountKey],
    queryFn: () => fetchJSON('/api/ui/favorites', { retries: 0 }),
    retry: false,
    refetchInterval: 30000,
  });
  const streams = streamsQuery.data || [];
  const favoritesData = favoritesQuery.data || { favorites: [], configurable: false };
  const favoriteIds = new Set((favoritesData.favorites || []).map((favorite) => favorite.camera_uuid));
  const filtered = useMemo(() => streams.filter((stream) => {
    if (stream.enabled === false || stream.streaming_enabled === false) return false;
    const matchesSearch = stream.name.toLocaleLowerCase().includes(search.trim().toLocaleLowerCase());
    return matchesSearch && (section === 'all' || favoriteIds.has(stream.camera_uuid));
  }).sort((a, b) => {
    if (section !== 'favorites') return 0;
    return compareFavoriteCameras(a, b, favoritesData.favorites);
  }), [streams, search, section, favoritesQuery.data]);
  const pageCount = Math.ceil(filtered.length / MOBILE_CAMERA_PAGE_SIZE);
  const visibleStreams = paginateMobileCameras(filtered, page);
  const forcedTransport = resolveForcedLiveTransport(window.location.pathname, window.location.search, {
    autoDisabled: viewFlags.autoDisabled,
    offerings: { webrtc: !viewFlags.webrtcDisabled, mse: !viewFlags.mseDisabled, hls: !viewFlags.hlsDisabled },
  });

  useEffect(() => { setPage(0); }, [section, search]);
  useEffect(() => { setPage((current) => Math.min(current, Math.max(0, pageCount - 1))); }, [pageCount]);

  async function toggleFavorite(cameraUuid) {
    if (!cameraUuid || busyUuid) return;
    setBusyUuid(cameraUuid);
    setMutationError('');
    try {
      if (favoriteIds.has(cameraUuid)) {
        await fetchJSON(`/api/ui/favorites/${encodeURIComponent(cameraUuid)}`, { method: 'DELETE', retries: 0 });
        queryClient.setQueryData(['ui-favorites', accountKey], {
          ...favoritesData,
          favorites: favoritesData.favorites.filter((favorite) => favorite.camera_uuid !== cameraUuid),
        });
      } else {
        const created = await fetchJSON('/api/ui/favorites', {
          method: 'POST', headers: { 'Content-Type': 'application/json' },
          body: JSON.stringify({ camera_uuid: cameraUuid }), retries: 0,
        });
        queryClient.setQueryData(['ui-favorites', accountKey], {
          ...favoritesData,
          favorites: [...favoritesData.favorites.filter((favorite) => favorite.camera_uuid !== cameraUuid), created],
        });
      }
    } catch (error) {
      setMutationError(error.status === 403
        ? 'Избранное недоступно для этой сессии.'
        : 'Не удалось обновить избранное. Повторите попытку.');
    } finally {
      setBusyUuid('');
    }
  }

  if (streamsQuery.isLoading) return <main className="mobile-live-page"><p role="status">Загружаем камеры…</p></main>;
  if (streamsQuery.isError) return <main className="mobile-live-page"><p role="alert">Не удалось загрузить камеры.</p><button onClick={() => streamsQuery.refetch()}>Повторить</button></main>;

  return <main className="mobile-live-page">
    <div className="mobile-live-toolbar">
      <label htmlFor="mobile-camera-search">Поиск камер</label>
      <input id="mobile-camera-search" type="search" value={search} onInput={(event) => setSearch(event.currentTarget.value)} placeholder="Название камеры" />
      <div className="mobile-live-tabs" role="tablist" aria-label="Список камер">
        <button role="tab" aria-selected={section === 'all'} onClick={() => setSection('all')}>Все камеры</button>
        <button role="tab" aria-selected={section === 'favorites'} onClick={() => setSection('favorites')}>Избранное</button>
      </div>
    </div>
    {favoritesQuery.isError && <p className="mobile-inline-error" role="status">Не удалось загрузить избранное. Доступны все камеры, изменение избранного временно недоступно.</p>}
    {mutationError && <p className="mobile-inline-error" role="alert">{mutationError}</p>}
    {streams.length === 0 ? <section className="mobile-empty-state"><h1>Камеры не найдены</h1><p>Добавьте камеру в настройках потоков.</p></section>
      : visibleStreams.length === 0 ? <section className="mobile-empty-state"><h1>{section === 'favorites' && !streams.some((stream) => favoriteIds.has(stream.camera_uuid)) ? 'В избранном пока пусто' : 'Ничего не найдено'}</h1><p>{section === 'favorites' && !streams.some((stream) => favoriteIds.has(stream.camera_uuid)) ? 'Нажмите на звезду у камеры, чтобы добавить её сюда.' : 'Измените поисковый запрос.'}</p></section>
        : <section className="mobile-camera-list" aria-label={section === 'favorites' ? 'Избранные камеры' : 'Все камеры'}>
          {visibleStreams.map((stream) => <MobileCameraCard key={stream.camera_uuid} stream={stream} favorite={favoriteIds.has(stream.camera_uuid)} canFavorite={favoritesData.configurable === true} busy={Boolean(busyUuid)} onFavorite={toggleFavorite} viewFlags={viewFlags} forcedTransport={forcedTransport} />)}
        </section>}
    {pageCount > 1 && <nav className="mobile-pagination" aria-label="Страницы камер">
      <button disabled={page === 0} onClick={() => { setPage((current) => Math.max(0, current - 1)); window.scrollTo({ top: 0, behavior: 'smooth' }); }}>Назад</button>
      <span>Страница {page + 1} из {pageCount}</span>
      <button disabled={page + 1 >= pageCount} onClick={() => { setPage((current) => Math.min(pageCount - 1, current + 1)); window.scrollTo({ top: 0, behavior: 'smooth' }); }}>Дальше</button>
    </nav>}
  </main>;
}
