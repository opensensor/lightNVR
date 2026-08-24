import { useCallback, useEffect, useMemo, useRef, useState } from 'preact/hooks';

import { fetchJSON, useQuery } from '../../../query-client.js';
import { useI18n } from '../../../i18n.js';
import { LoadingIndicator } from '../LoadingIndicator.jsx';
import {
  MAX_ACTIVE_INVESTIGATION_PLAYERS,
  MAX_INVESTIGATION_CAMERAS,
  adjacentInvestigationResultIndex,
  advanceInvestigationCursor,
  findSegmentAt,
  formatCursorTime,
  formatDateTimeLocal,
  parseDateTimeLocal,
  segmentTrackPosition,
} from './investigationUtils.js';

function initialTimeState() {
  const params = new URLSearchParams(window.location.search);
  const now = Math.floor(Date.now() / 1000);
  const parsedStart = Number(params.get('start'));
  const parsedEnd = Number(params.get('end'));
  const start = Number.isFinite(parsedStart) && parsedStart > 0
    ? parsedStart : now - 5 * 60;
  const end = Number.isFinite(parsedEnd) && parsedEnd > start
    ? parsedEnd : now;
  return { start, end };
}

function initialSearchFilters() {
  const params = new URLSearchParams(window.location.search);
  return {
    label: params.get('label') || '',
    zone: params.get('zone') || '',
    source: params.get('source') || '',
    minConfidence: params.get('confidence_min') || '',
  };
}

function InvestigationPlayer({
  track,
  cursor,
  playing,
  speed,
  primary,
  onMakePrimary,
  t,
}) {
  const videoRef = useRef(null);
  const cursorRef = useRef(cursor);
  const segment = findSegmentAt(track.segments, cursor);
  const [status, setStatus] = useState(segment ? 'loading' : 'gap');

  useEffect(() => {
    cursorRef.current = cursor;
  }, [cursor]);

  const seekToCursor = useCallback(() => {
    const video = videoRef.current;
    if (!video || !segment || video.readyState < 1) return;
    const expected = Math.max(
      0,
      Math.min(cursorRef.current - segment.start_time, video.duration || Infinity),
    );
    const drift = Math.abs(video.currentTime - expected);
    if (drift > 0.5) {
      setStatus('synchronizing');
      try {
        video.currentTime = expected;
      } catch (_error) {
        setStatus('late');
        return;
      }
    } else {
      setStatus((current) => current === 'error' ? current : 'ready');
    }
  }, [segment?.id, segment?.start_time]);

  useEffect(() => {
    const video = videoRef.current;
    if (!video) return undefined;
    if (!segment) {
      video.pause();
      video.removeAttribute('src');
      video.load();
      setStatus('gap');
      return undefined;
    }

    setStatus('loading');
    const loaded = () => {
      seekToCursor();
      video.playbackRate = speed;
      if (playing) {
        video.play().catch(() => setStatus('paused-by-browser'));
      }
    };
    video.addEventListener('loadedmetadata', loaded);
    video.src = `/api/recordings/play/${segment.id}`;
    video.load();
    return () => video.removeEventListener('loadedmetadata', loaded);
  }, [segment?.id]);

  useEffect(() => {
    seekToCursor();
  }, [cursor, seekToCursor]);

  useEffect(() => {
    const video = videoRef.current;
    if (!video || !segment) return;
    video.playbackRate = speed;
    if (playing) {
      video.play().catch(() => setStatus('paused-by-browser'));
    } else {
      video.pause();
    }
  }, [playing, speed, segment?.id]);

  return (
    <article className="investigation-player" data-status={segment ? status : 'gap'}>
      <header>
        <div>
          <strong>{track.name}</strong>
          <span className="investigation-player-status">
            {status === 'gap' ? t('investigation.noFootage') : t(`investigation.status.${status}`)}
          </span>
        </div>
        <button
          type="button"
          className={primary ? 'btn-primary' : 'btn-secondary'}
          onClick={onMakePrimary}
          aria-pressed={primary}
        >
          {primary ? t('investigation.primaryAudio') : t('investigation.makePrimary')}
        </button>
      </header>
      <div className="investigation-video-frame">
        {segment ? (
          <video
            ref={videoRef}
            muted={!primary}
            playsInline
            preload="metadata"
            controls
            onWaiting={() => setStatus('late')}
            onPlaying={() => setStatus('ready')}
            onCanPlay={() => setStatus('ready')}
            onError={() => setStatus('error')}
          />
        ) : (
          <div className="investigation-gap-state">
            <span>{t('investigation.noFootageAtTime')}</span>
            <time>{formatCursorTime(cursor)}</time>
          </div>
        )}
      </div>
    </article>
  );
}

function InvestigationTrack({
  track,
  startTime,
  endTime,
  cursor,
  active,
  onToggleActive,
  onSeek,
  t,
}) {
  const cursorPercent = Math.max(
    0,
    Math.min(100, ((cursor - startTime) / Math.max(endTime - startTime, 1)) * 100),
  );
  const hasFootage = Boolean(findSegmentAt(track.segments, cursor));

  const seekFromPointer = (event) => {
    const bounds = event.currentTarget.getBoundingClientRect();
    const ratio = Math.max(0, Math.min(1, (event.clientX - bounds.left) / bounds.width));
    onSeek(startTime + ratio * (endTime - startTime));
  };

  return (
    <div className={`investigation-track ${active ? 'is-active' : ''}`}>
      <div className="investigation-track-label">
        <label>
          <input type="checkbox" checked={active} onChange={onToggleActive} />
          <span>{track.name}</span>
        </label>
        <small>
          {track.segment_count} {t('investigation.segments')}
          {track.truncated ? ` · ${t('investigation.truncated')}` : ''}
        </small>
      </div>
      <div
        className="investigation-track-bar"
        role="slider"
        tabIndex="0"
        aria-label={`${track.name} ${t('investigation.timeline')}`}
        aria-valuemin={startTime}
        aria-valuemax={endTime}
        aria-valuenow={Math.round(cursor)}
        onClick={seekFromPointer}
        onKeyDown={(event) => {
          if (event.key === 'ArrowLeft') onSeek(Math.max(startTime, cursor - 1));
          if (event.key === 'ArrowRight') onSeek(Math.min(endTime, cursor + 1));
        }}
      >
        {(track.segments || []).map((segment) => (
          <span
            key={segment.id}
            className={`investigation-track-segment ${segment.has_detection ? 'has-detection' : ''}`}
            style={segmentTrackPosition(segment, startTime, endTime)}
            title={`${track.name}: ${formatCursorTime(segment.start_time)} – ${formatCursorTime(segment.end_time)}`}
          />
        ))}
        <span
          className="investigation-track-cursor"
          style={{ left: `${cursorPercent}%` }}
        />
        {!hasFootage && <span className="investigation-track-gap-label">{t('investigation.gap')}</span>}
      </div>
    </div>
  );
}

function InvestigationHistogram({ histogram, startTime, endTime, onSeek, t }) {
  const buckets = histogram?.buckets || [];
  const duration = Math.max(endTime - startTime, 1);
  const maximum = Math.max(1, ...buckets.map((bucket) => bucket.count));
  return (
    <div className="investigation-histogram">
      <div className="investigation-histogram-heading">
        <span>{t('investigation.resultHistogram')}</span>
        <small>{t('investigation.histogramHelp')}</small>
      </div>
      <div className="investigation-histogram-plot" aria-label={t('investigation.resultHistogram')}>
        {buckets.map((bucket) => {
          const left = Math.max(0, ((bucket.start_time - startTime) / duration) * 100);
          const width = Math.max(
            0.35,
            ((Math.min(bucket.end_time, endTime) - bucket.start_time) / duration) * 100,
          );
          return (
            <button
              key={`${bucket.start_time}-${bucket.end_time}`}
              type="button"
              style={{
                left: `${left}%`,
                width: `${width}%`,
                height: `${Math.max(10, (bucket.count / maximum) * 100)}%`,
              }}
              title={`${bucket.count} · ${formatCursorTime(bucket.start_time)}`}
              aria-label={`${bucket.count} ${t('investigation.results')} · ${formatCursorTime(bucket.start_time)}`}
              onClick={() => onSeek(bucket.start_time)}
            />
          );
        })}
      </div>
    </div>
  );
}

function InvestigationResultCard({ result, selected, onSelect, t }) {
  const [thumbnailFailed, setThumbnailFailed] = useState(false);
  const thumbnailAvailable = result.thumbnail?.status === 'available' &&
    result.thumbnail?.url && !thumbnailFailed;
  const durationSeconds = Math.max(
    0, Math.round((result.end_time || result.start_time) - result.start_time),
  );
  return (
    <button
      type="button"
      className={`investigation-result-card ${selected ? 'is-selected' : ''}`}
      aria-pressed={selected}
      onClick={onSelect}
    >
      <span className="investigation-result-thumbnail">
        {thumbnailAvailable ? (
          <img
            src={result.thumbnail.url}
            alt=""
            loading="lazy"
            onError={() => setThumbnailFailed(true)}
          />
        ) : (
          <span>{t('investigation.noThumbnail')}</span>
        )}
      </span>
      <span className="investigation-result-copy">
        <strong>{result.detection?.label || result.event_type}</strong>
        <span>{result.camera?.name}</span>
        <time>{formatCursorTime(result.start_time)}</time>
        <small>
          {Math.round((result.detection?.confidence || 0) * 100)}%
          {' · '}{result.detection?.source || t('investigation.localSource')}
          {result.detection?.zone_uuid ? ` · ${result.detection.zone_uuid}` : ''}
        </small>
        <small>
          {result.known_gap
            ? t('investigation.noFootage') : t('investigation.mediaAvailable')}
          {' · '}{t('investigation.durationSeconds', { count: durationSeconds })}
        </small>
      </span>
    </button>
  );
}

export function InvestigationView() {
  const { t } = useI18n();
  const initialTimes = useMemo(initialTimeState, []);
  const initialFilters = useMemo(initialSearchFilters, []);
  const [startTime, setStartTime] = useState(initialTimes.start);
  const [endTime, setEndTime] = useState(initialTimes.end);
  const [selectedCameraUuids, setSelectedCameraUuids] = useState([]);
  const [timeline, setTimeline] = useState(null);
  const [cursor, setCursor] = useState(initialTimes.start);
  const [activeCameraUuids, setActiveCameraUuids] = useState([]);
  const [primaryCameraUuid, setPrimaryCameraUuid] = useState(null);
  const [playing, setPlaying] = useState(false);
  const [speed, setSpeed] = useState(1);
  const [playbackMode, setPlaybackMode] = useState('wall-clock');
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [searchFilters, setSearchFilters] = useState(initialFilters);
  const [searchData, setSearchData] = useState(null);
  const [searchLoading, setSearchLoading] = useState(false);
  const [searchError, setSearchError] = useState('');
  const [searchPageCursors, setSearchPageCursors] = useState([null]);
  const [searchPageIndex, setSearchPageIndex] = useState(0);
  const [selectedResultId, setSelectedResultId] = useState(null);
  const initialSelectionApplied = useRef(false);
  const initialQueryLoaded = useRef(false);
  const requestController = useRef(null);
  const searchRequestController = useRef(null);
  const lastUrlCursor = useRef(null);

  const { data: streamData, isLoading: streamsLoading, error: streamsError } =
    useQuery('investigation-streams', '/api/streams', {
      timeout: 15000,
      retries: 1,
    });
  const streams = Array.isArray(streamData) ? streamData : [];

  useEffect(() => {
    if (initialSelectionApplied.current || streams.length === 0) return;
    const params = new URLSearchParams(window.location.search);
    const requestedUuids = (params.get('cameras') || '')
      .split(',')
      .map((value) => value.trim())
      .filter(Boolean);
    const requestedNames = (params.get('stream') || '')
      .split(',')
      .map((value) => value.trim())
      .filter(Boolean);
    const valid = streams.filter((stream) =>
      requestedUuids.includes(stream.camera_uuid) || requestedNames.includes(stream.name));
    const initial = (valid.length > 0 ? valid : streams.slice(0, 1))
      .slice(0, MAX_INVESTIGATION_CAMERAS)
      .map((stream) => stream.camera_uuid);
    setSelectedCameraUuids(initial);
    initialSelectionApplied.current = true;
  }, [streams]);

  const loadSearchPage = useCallback(async (
    pageCursor = null,
    pageIndex = 0,
    pageCursors = [null],
    searchContext = null,
  ) => {
    const cameraUuids = searchContext?.cameraUuids ||
      (timeline?.tracks || []).map((track) => track.camera_uuid);
    const searchStart = searchContext?.startTime ?? timeline?.start_time;
    const searchEnd = searchContext?.endTime ?? timeline?.end_time;
    if (cameraUuids.length === 0 || !Number.isFinite(searchStart) ||
        !Number.isFinite(searchEnd) || searchEnd <= searchStart) return;
    const minimumConfidence = searchFilters.minConfidence === ''
      ? null : Number(searchFilters.minConfidence);
    if (minimumConfidence !== null &&
        (!Number.isFinite(minimumConfidence) || minimumConfidence < 0 ||
         minimumConfidence > 1)) {
      setSearchError(t('investigation.confidenceError'));
      return;
    }

    searchRequestController.current?.abort();
    const controller = new AbortController();
    searchRequestController.current = controller;
    setSearchLoading(true);
    setSearchError('');
    try {
      const filters = {};
      if (searchFilters.label) filters.labels = [searchFilters.label];
      if (searchFilters.zone) filters.zones = [searchFilters.zone];
      if (searchFilters.source) filters.sources = [searchFilters.source];
      if (minimumConfidence !== null) filters.min_confidence = minimumConfidence;
      const data = await fetchJSON('/api/investigations/search', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          camera_uuids: cameraUuids,
          start_time: Math.floor(searchStart),
          end_time: Math.floor(searchEnd),
          filters,
          cursor: pageCursor,
          limit: 24,
        }),
        signal: controller.signal,
        timeout: 30000,
        retries: 0,
      });
      setSearchData(data);
      setSearchPageIndex(pageIndex);
      setSearchPageCursors(pageCursors);
      setSelectedResultId(null);

      const url = new URL(window.location.href);
      const filterParams = {
        label: searchFilters.label,
        zone: searchFilters.zone,
        source: searchFilters.source,
        confidence_min: searchFilters.minConfidence,
      };
      Object.entries(filterParams).forEach(([name, value]) => {
        if (value) url.searchParams.set(name, value);
        else url.searchParams.delete(name);
      });
      window.history.replaceState({}, '', url);
    } catch (requestError) {
      if (!controller.signal.aborted) setSearchError(requestError.message);
    } finally {
      if (!controller.signal.aborted) setSearchLoading(false);
    }
  }, [timeline, searchFilters, t]);

  const loadTimeline = useCallback(async () => {
    if (selectedCameraUuids.length === 0) {
      setError(t('investigation.selectCameraError'));
      return;
    }
    if (endTime <= startTime) {
      setError(t('investigation.timeRangeError'));
      return;
    }
    requestController.current?.abort();
    const controller = new AbortController();
    requestController.current = controller;
    setLoading(true);
    setError('');
    setPlaying(false);
    try {
      const data = await fetchJSON('/api/investigations/timeline', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          camera_uuids: selectedCameraUuids,
          start_time: Math.floor(startTime),
          end_time: Math.floor(endTime),
        }),
        signal: controller.signal,
        timeout: 30000,
        retries: 0,
      });
      setTimeline(data);
      const tracks = data.tracks || [];
      const nextActive = tracks
        .slice(0, data.max_active_decoders || MAX_ACTIVE_INVESTIGATION_PLAYERS)
        .map((track) => track.camera_uuid);
      setActiveCameraUuids(nextActive);
      setPrimaryCameraUuid(nextActive[0] || null);
      const params = new URLSearchParams(window.location.search);
      const requestedCursor = Number(params.get('cursor'));
      const firstSegmentStart = tracks
        .flatMap((track) => track.segments || [])
        .reduce((earliest, segment) =>
          earliest === null || segment.start_time < earliest
            ? segment.start_time : earliest, null);
      const nextCursor = Number.isFinite(requestedCursor) &&
        requestedCursor >= data.start_time && requestedCursor <= data.end_time
        ? requestedCursor
        : firstSegmentStart ?? data.start_time;
      setCursor(nextCursor);

      const url = new URL(window.location.href);
      url.searchParams.set('cameras', selectedCameraUuids.join(','));
      url.searchParams.set('start', String(Math.floor(startTime)));
      url.searchParams.set('end', String(Math.floor(endTime)));
      url.searchParams.set('cursor', String(Math.floor(nextCursor)));
      url.searchParams.delete('stream');
      window.history.replaceState({}, '', url);
      void loadSearchPage(null, 0, [null], {
        cameraUuids: selectedCameraUuids,
        startTime: data.start_time,
        endTime: data.end_time,
      });
    } catch (requestError) {
      if (!controller.signal.aborted) setError(requestError.message);
    } finally {
      if (!controller.signal.aborted) setLoading(false);
    }
  }, [selectedCameraUuids, startTime, endTime, t, loadSearchPage]);

  useEffect(() => {
    if (!initialSelectionApplied.current || initialQueryLoaded.current ||
        selectedCameraUuids.length === 0) return;
    initialQueryLoaded.current = true;
    void loadTimeline();
  }, [selectedCameraUuids, loadTimeline]);

  useEffect(() => () => {
    requestController.current?.abort();
    searchRequestController.current?.abort();
  }, []);

  const tracks = timeline?.tracks || [];
  const activeTracks = tracks.filter((track) =>
    activeCameraUuids.includes(track.camera_uuid));
  const activeKey = activeCameraUuids.join(',');

  useEffect(() => {
    if (!playing || !timeline) return undefined;
    let previous = performance.now();
    const timer = window.setInterval(() => {
      const current = performance.now();
      const elapsedSeconds = (current - previous) / 1000;
      previous = current;
      setCursor((value) => advanceInvestigationCursor({
        cursor: value,
        elapsedSeconds,
        speed,
        endTime: timeline.end_time,
        mode: playbackMode,
        tracks,
        activeCameraUuids,
      }));
    }, 100);
    return () => window.clearInterval(timer);
  }, [playing, speed, playbackMode, timeline, activeKey]);

  useEffect(() => {
    if (timeline && cursor >= timeline.end_time) setPlaying(false);
    if (!timeline) return;
    const roundedCursor = Math.floor(cursor);
    if (lastUrlCursor.current === roundedCursor) return;
    lastUrlCursor.current = roundedCursor;
    const url = new URL(window.location.href);
    url.searchParams.set('cursor', String(roundedCursor));
    window.history.replaceState({}, '', url);
  }, [cursor, timeline]);

  const toggleSelectedCamera = (cameraUuid) => {
    setSelectedCameraUuids((current) => {
      if (current.includes(cameraUuid)) {
        return current.filter((uuid) => uuid !== cameraUuid);
      }
      if (current.length >= MAX_INVESTIGATION_CAMERAS) {
        setError(t('investigation.cameraLimit', { count: MAX_INVESTIGATION_CAMERAS }));
        return current;
      }
      setError('');
      return [...current, cameraUuid];
    });
  };

  const toggleActiveCamera = (cameraUuid) => {
    setActiveCameraUuids((current) => {
      if (current.includes(cameraUuid)) {
        const next = current.filter((uuid) => uuid !== cameraUuid);
        if (primaryCameraUuid === cameraUuid) setPrimaryCameraUuid(next[0] || null);
        return next;
      }
      const limit = timeline?.max_active_decoders || MAX_ACTIVE_INVESTIGATION_PLAYERS;
      if (current.length >= limit) {
        setError(t('investigation.decoderLimit', { count: limit }));
        return current;
      }
      setError('');
      return [...current, cameraUuid];
    });
  };

  const searchResults = searchData?.results || [];
  const selectedResultIndex = searchResults.findIndex((result) =>
    result.result_id === selectedResultId);

  const selectResult = useCallback((result) => {
    if (!result || !timeline) return;
    setSelectedResultId(result.result_id);
    setPlaying(false);
    setCursor(Math.max(
      timeline.start_time,
      Math.min(timeline.end_time, result.start_time),
    ));
    const cameraUuid = result.camera_uuid;
    const limit = timeline.max_active_decoders || MAX_ACTIVE_INVESTIGATION_PLAYERS;
    setActiveCameraUuids((current) => {
      if (current.includes(cameraUuid)) return current;
      return current.length >= limit
        ? [...current.slice(0, Math.max(0, limit - 1)), cameraUuid]
        : [...current, cameraUuid];
    });
    setPrimaryCameraUuid(cameraUuid);
  }, [timeline]);

  const selectAdjacentResult = useCallback((direction) => {
    const index = adjacentInvestigationResultIndex(
      searchResults, selectedResultId, direction,
    );
    if (index >= 0) selectResult(searchResults[index]);
  }, [searchResults, selectedResultId, selectResult]);

  useEffect(() => {
    const handleKeyDown = (event) => {
      const target = event.target;
      if (target?.matches?.('input, select, textarea, button') ||
          target?.isContentEditable) return;
      if (event.key === '[') {
        event.preventDefault();
        selectAdjacentResult(-1);
      } else if (event.key === ']') {
        event.preventDefault();
        selectAdjacentResult(1);
      }
    };
    window.addEventListener('keydown', handleKeyDown);
    return () => window.removeEventListener('keydown', handleKeyDown);
  }, [selectAdjacentResult]);

  const setSearchFilter = (name, value) => {
    setSearchFilters((current) => ({ ...current, [name]: value }));
  };

  const loadNextSearchPage = () => {
    const nextCursor = searchData?.page?.next_cursor;
    if (!nextCursor) return;
    const cursors = [
      ...searchPageCursors.slice(0, searchPageIndex + 1),
      nextCursor,
    ];
    void loadSearchPage(nextCursor, searchPageIndex + 1, cursors);
  };

  const loadPreviousSearchPage = () => {
    if (searchPageIndex < 1) return;
    const previousIndex = searchPageIndex - 1;
    void loadSearchPage(
      searchPageCursors[previousIndex], previousIndex, searchPageCursors,
    );
  };

  return (
    <div className="investigation-page">
      <div className="investigation-heading">
        <div>
          <span className="investigation-eyebrow">{t('investigation.experimental')}</span>
          <h1>{t('investigation.title')}</h1>
          <p>{t('investigation.description')}</p>
        </div>
        <nav className="investigation-subnav" aria-label={t('recordings.views')}>
          <a href="recordings.html">{t('nav.recordings')}</a>
          <a href="timeline.html">{t('nav.timeline')}</a>
          <a className="is-active" href="investigation.html" aria-current="page">
            {t('nav.investigation')}
          </a>
        </nav>
      </div>

      <section className="investigation-query-card" aria-labelledby="investigation-query-heading">
        <div className="investigation-query-header">
          <div>
            <h2 id="investigation-query-heading">{t('investigation.query')}</h2>
            <p>{t('investigation.queryHelp')}</p>
          </div>
          <span>{selectedCameraUuids.length}/{MAX_INVESTIGATION_CAMERAS} {t('investigation.cameras')}</span>
        </div>
        <div className="investigation-query-grid">
          <fieldset className="investigation-camera-picker">
            <legend>{t('investigation.cameraSelection')}</legend>
            {streamsLoading && <LoadingIndicator message={t('common.loading')} />}
            {streamsError && <p className="text-destructive">{streamsError.message}</p>}
            {!streamsLoading && streams.map((stream) => (
              <label key={stream.camera_uuid}>
                <input
                  type="checkbox"
                  checked={selectedCameraUuids.includes(stream.camera_uuid)}
                  disabled={!selectedCameraUuids.includes(stream.camera_uuid) &&
                    selectedCameraUuids.length >= MAX_INVESTIGATION_CAMERAS}
                  onChange={() => toggleSelectedCamera(stream.camera_uuid)}
                />
                <span>{stream.name}</span>
              </label>
            ))}
          </fieldset>
          <div className="investigation-time-fields">
            <label>
              <span>{t('investigation.start')}</span>
              <input
                type="datetime-local"
                value={formatDateTimeLocal(startTime)}
                onChange={(event) => {
                  const value = parseDateTimeLocal(event.target.value);
                  if (value !== null) setStartTime(value);
                }}
              />
            </label>
            <label>
              <span>{t('investigation.end')}</span>
              <input
                type="datetime-local"
                value={formatDateTimeLocal(endTime)}
                onChange={(event) => {
                  const value = parseDateTimeLocal(event.target.value);
                  if (value !== null) setEndTime(value);
                }}
              />
            </label>
            <button type="button" className="btn-primary" disabled={loading} onClick={loadTimeline}>
              {loading ? t('investigation.loading') : t('investigation.load')}
            </button>
          </div>
        </div>
        <div className="investigation-search-filters">
          <label>
            <span>{t('investigation.objectLabel')}</span>
            <select
              value={searchFilters.label}
              onChange={(event) => setSearchFilter('label', event.target.value)}
            >
              <option value="">{t('investigation.anyLabel')}</option>
              {(searchData?.facets?.labels || []).map((facet) => (
                <option key={facet.value} value={facet.value}>
                  {facet.value} ({facet.count})
                </option>
              ))}
            </select>
          </label>
          <label>
            <span>{t('investigation.zone')}</span>
            <select
              value={searchFilters.zone}
              onChange={(event) => setSearchFilter('zone', event.target.value)}
            >
              <option value="">{t('investigation.anyZone')}</option>
              {(searchData?.facets?.zones || []).map((facet) => (
                <option key={facet.value} value={facet.value}>
                  {facet.value} ({facet.count})
                </option>
              ))}
            </select>
          </label>
          <label>
            <span>{t('investigation.source')}</span>
            <select
              value={searchFilters.source}
              onChange={(event) => setSearchFilter('source', event.target.value)}
            >
              <option value="">{t('investigation.anySource')}</option>
              {(searchData?.facets?.sources || []).map((facet) => (
                <option key={facet.value} value={facet.value}>
                  {facet.value} ({facet.count})
                </option>
              ))}
            </select>
          </label>
          <label>
            <span>{t('investigation.minimumConfidence')}</span>
            <input
              type="number"
              min="0"
              max="1"
              step="0.05"
              placeholder="0.00"
              value={searchFilters.minConfidence}
              onInput={(event) => setSearchFilter('minConfidence', event.target.value)}
            />
          </label>
          <button
            type="button"
            className="btn-secondary"
            disabled={!timeline || searchLoading}
            onClick={() => loadSearchPage(null, 0, [null])}
          >
            {searchLoading ? t('investigation.searching') : t('investigation.applyFilters')}
          </button>
        </div>
        {error && <div className="investigation-error" role="alert">{error}</div>}
        {searchError && <div className="investigation-error" role="alert">{searchError}</div>}
      </section>

      {loading && <LoadingIndicator message={t('investigation.loading')} />}

      {timeline && !loading && (
        <>
          <section className="investigation-controls" aria-label={t('investigation.playbackControls')}>
            <button
              type="button"
              className="btn-primary investigation-play-button"
              onClick={() => setPlaying((value) => !value)}
              disabled={activeTracks.length === 0}
            >
              {playing ? '❚❚' : '▶'}
              <span>{playing ? t('investigation.pause') : t('investigation.play')}</span>
            </button>
            <label>
              <span>{t('investigation.mode')}</span>
              <select value={playbackMode} onChange={(event) => setPlaybackMode(event.target.value)}>
                <option value="wall-clock">{t('investigation.wallClock')}</option>
                <option value="skip-common-gaps">{t('investigation.skipCommonGaps')}</option>
              </select>
            </label>
            <label>
              <span>{t('investigation.speed')}</span>
              <select value={speed} onChange={(event) => setSpeed(Number(event.target.value))}>
                <option value="0.5">0.5×</option>
                <option value="1">1×</option>
                <option value="2">2×</option>
                <option value="4">4×</option>
              </select>
            </label>
            <div className="investigation-cursor-time">
              <span>{t('investigation.sharedCursor')}</span>
              <strong>{formatCursorTime(cursor)}</strong>
            </div>
            <span className="investigation-decoder-count">
              {activeTracks.length}/{timeline.max_active_decoders} {t('investigation.activePlayers')}
            </span>
          </section>

          <section className="investigation-search-panel" aria-label={t('investigation.searchResults')}>
            <div className="investigation-results-heading">
              <div>
                <h2>{t('investigation.searchResults')}</h2>
                <p>
                  {searchData
                    ? t('investigation.resultCount', { count: searchData.page?.total || 0 })
                    : t('investigation.searchHelp')}
                </p>
              </div>
              <div className="investigation-result-navigation">
                <button
                  type="button"
                  className="btn-secondary"
                  disabled={searchResults.length === 0 || selectedResultIndex === 0}
                  onClick={() => selectAdjacentResult(-1)}
                  title={t('investigation.previousResultShortcut')}
                >
                  ← {t('investigation.previousResult')}
                </button>
                <button
                  type="button"
                  className="btn-secondary"
                  disabled={searchResults.length === 0 ||
                    selectedResultIndex === searchResults.length - 1}
                  onClick={() => selectAdjacentResult(1)}
                  title={t('investigation.nextResultShortcut')}
                >
                  {t('investigation.nextResult')} →
                </button>
              </div>
            </div>

            {searchLoading && <LoadingIndicator message={t('investigation.searching')} />}
            {searchData && !searchLoading && (
              <>
                <InvestigationHistogram
                  histogram={searchData.histogram}
                  startTime={timeline.start_time}
                  endTime={timeline.end_time}
                  onSeek={(value) => {
                    setPlaying(false);
                    setCursor(value);
                  }}
                  t={t}
                />
                {searchData.coverage && !searchData.coverage.complete && (
                  <div className="investigation-coverage-warning">
                    {t('investigation.incompleteCoverage', {
                      count: searchData.coverage.unresolved_legacy_rows,
                    })}
                  </div>
                )}
                {searchResults.length > 0 ? (
                  <div className="investigation-result-rail">
                    {searchResults.map((result) => (
                      <InvestigationResultCard
                        key={result.result_id}
                        result={result}
                        selected={result.result_id === selectedResultId}
                        onSelect={() => selectResult(result)}
                        t={t}
                      />
                    ))}
                  </div>
                ) : (
                  <div className="investigation-results-empty">
                    {t('investigation.noResults')}
                  </div>
                )}
                <div className="investigation-page-navigation">
                  <button
                    type="button"
                    className="btn-secondary"
                    disabled={searchPageIndex === 0}
                    onClick={loadPreviousSearchPage}
                  >
                    ← {t('common.previous')}
                  </button>
                  <span>{t('investigation.pageNumber', { count: searchPageIndex + 1 })}</span>
                  <button
                    type="button"
                    className="btn-secondary"
                    disabled={!searchData.page?.has_more}
                    onClick={loadNextSearchPage}
                  >
                    {t('common.next')} →
                  </button>
                </div>
              </>
            )}
          </section>

          <input
            className="investigation-scrubber"
            type="range"
            min={timeline.start_time}
            max={timeline.end_time}
            step="1"
            value={cursor}
            aria-label={t('investigation.sharedCursor')}
            onInput={(event) => {
              setPlaying(false);
              setCursor(Number(event.target.value));
            }}
          />

          <section className="investigation-timeline" aria-label={t('investigation.timeline')}>
            {tracks.map((track) => (
              <InvestigationTrack
                key={track.camera_uuid}
                track={track}
                startTime={timeline.start_time}
                endTime={timeline.end_time}
                cursor={cursor}
                active={activeCameraUuids.includes(track.camera_uuid)}
                onToggleActive={() => toggleActiveCamera(track.camera_uuid)}
                onSeek={(value) => {
                  setPlaying(false);
                  setCursor(value);
                }}
                t={t}
              />
            ))}
          </section>

          {activeTracks.length > 0 ? (
            <section className="investigation-player-grid" aria-label={t('investigation.players')}>
              {activeTracks.map((track) => (
                <InvestigationPlayer
                  key={track.camera_uuid}
                  track={track}
                  cursor={cursor}
                  playing={playing}
                  speed={speed}
                  primary={primaryCameraUuid === track.camera_uuid}
                  onMakePrimary={() => setPrimaryCameraUuid(track.camera_uuid)}
                  t={t}
                />
              ))}
            </section>
          ) : (
            <div className="investigation-empty">{t('investigation.activateCamera')}</div>
          )}
        </>
      )}
    </div>
  );
}
