import { useCallback, useEffect, useMemo, useRef, useState } from 'preact/hooks';

import { fetchJSON, useQuery } from '../../../query-client.js';
import { useI18n } from '../../../i18n.js';
import { Priority, queueThumbnailLoad } from '../../../request-queue.js';
import { LoadingIndicator } from '../LoadingIndicator.jsx';
import { formatUtils } from '../recordings/formatUtils.js';
import {
  MAX_ACTIVE_INVESTIGATION_PLAYERS,
  MAX_INVESTIGATION_CAMERAS,
  adjacentInvestigationResultIndex,
  advanceInvestigationCursor,
  findSegmentAt,
  formatCursorTime,
  formatDateTimeLocal,
  normalizedRegionRectangle,
  narrowThumbnailWindow,
  parseDateTimeLocal,
  parseInvestigationRegion,
  segmentTrackPosition,
  thumbnailWindowForResult,
  videoContentBox,
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
    eventType: params.get('event') || '',
    location: params.get('location') || '',
    label: params.get('label') || '',
    zone: params.get('zone') || '',
    source: params.get('source') || '',
    captureMethod: params.get('capture') || '',
    recordingTag: params.get('tag') || '',
    protection: params.get('protected') || '',
    minConfidence: params.get('confidence_min') || '',
    region: parseInvestigationRegion(params),
  };
}

function InvestigationPlayer({
  track,
  cursor,
  playing,
  speed,
  primary,
  region,
  drawingRegion,
  onRegionChange,
  onRegionComplete,
  onMakePrimary,
  t,
}) {
  const videoRef = useRef(null);
  const cursorRef = useRef(cursor);
  const regionAnchorRef = useRef(null);
  const segment = findSegmentAt(track.segments, cursor);
  const [status, setStatus] = useState(segment ? 'loading' : 'gap');
  const [videoDimensions, setVideoDimensions] = useState({ width: 16, height: 9 });

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
      if (video.videoWidth > 0 && video.videoHeight > 0) {
        setVideoDimensions({ width: video.videoWidth, height: video.videoHeight });
      }
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

  const regionContent = videoContentBox(
    16, 9, videoDimensions.width, videoDimensions.height,
  );
  const pointFromEvent = (event) => {
    const bounds = event.currentTarget.getBoundingClientRect();
    if (bounds.width <= 0 || bounds.height <= 0) return null;
    return {
      x: Math.max(0, Math.min(1, (event.clientX - bounds.left) / bounds.width)),
      y: Math.max(0, Math.min(1, (event.clientY - bounds.top) / bounds.height)),
    };
  };
  const finishRegion = (event) => {
    if (!regionAnchorRef.current) return;
    const point = pointFromEvent(event);
    const rectangle = normalizedRegionRectangle(regionAnchorRef.current, point);
    regionAnchorRef.current = null;
    if (!rectangle || rectangle.width < 0.01 || rectangle.height < 0.01) {
      onRegionChange(null);
    } else {
      onRegionChange(rectangle);
    }
    onRegionComplete();
  };

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
        {segment && (drawingRegion || region) && (
          <div
            className={`investigation-region-layer ${drawingRegion ? 'is-drawing' : ''}`}
            style={{
              left: `${regionContent.left * 100}%`,
              top: `${regionContent.top * 100}%`,
              width: `${regionContent.width * 100}%`,
              height: `${regionContent.height * 100}%`,
            }}
            aria-label={drawingRegion ? t('investigation.drawRegionPrompt') : undefined}
            onPointerDown={(event) => {
              if (!drawingRegion || (event.button !== undefined && event.button !== 0)) return;
              event.preventDefault();
              const point = pointFromEvent(event);
              if (!point) return;
              regionAnchorRef.current = point;
              event.currentTarget.setPointerCapture?.(event.pointerId);
            }}
            onPointerMove={(event) => {
              if (!drawingRegion || !regionAnchorRef.current) return;
              const rectangle = normalizedRegionRectangle(
                regionAnchorRef.current, pointFromEvent(event),
              );
              if (rectangle) onRegionChange(rectangle);
            }}
            onPointerUp={finishRegion}
            onPointerCancel={() => {
              regionAnchorRef.current = null;
              onRegionComplete();
            }}
          >
            {region && (
              <span
                className="investigation-region-rectangle"
                style={{
                  left: `${region.x * 100}%`,
                  top: `${region.y * 100}%`,
                  width: `${region.width * 100}%`,
                  height: `${region.height * 100}%`,
                }}
              />
            )}
            {drawingRegion && !region && (
              <span className="investigation-region-prompt">
                {t('investigation.drawRegionPrompt')}
              </span>
            )}
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
        {result.recording?.capture_method && (
          <small>
            {formatUtils.formatCaptureMethod(result.recording.capture_method, t)}
            {result.recording.protected
              ? ` · ${t('investigation.protected')}` : ''}
          </small>
        )}
      </span>
    </button>
  );
}

function InvestigationSampleThumbnail({ sample, onSelect, t }) {
  const url = sample.thumbnail?.url || '';
  const [loadState, setLoadState] = useState(url ? 'loading' : 'unavailable');
  const [retryCount, setRetryCount] = useState(0);

  useEffect(() => {
    let active = true;
    if (!url) {
      setLoadState(sample.media_status === 'gap' ? 'gap' : 'unavailable');
      return () => { active = false; };
    }
    setLoadState('loading');
    queueThumbnailLoad(
      url,
      retryCount > 0 ? Priority.HIGH : Priority.NORMAL,
      retryCount > 0 ? 1 : 3,
    ).then(() => {
      if (active) setLoadState('ready');
    }).catch(() => {
      if (active) setLoadState('error');
    });
    return () => { active = false; };
  }, [url, retryCount, sample.media_status]);

  const timeLabel = formatCursorTime(sample.timestamp);
  return (
    <article className={`investigation-sample-card is-${loadState}`}>
      <button
        type="button"
        className="investigation-sample-target"
        onClick={onSelect}
        aria-label={t('investigation.selectThumbnailSample', { time: timeLabel })}
      >
        <span className="investigation-sample-image">
          {loadState === 'ready' ? (
            <img src={url} alt="" />
          ) : loadState === 'loading' ? (
            <span>{t('investigation.loadingThumbnail')}</span>
          ) : loadState === 'gap' ? (
            <span>{t('investigation.noFootage')}</span>
          ) : (
            <span>{t('investigation.thumbnailUnavailable')}</span>
          )}
        </span>
        <time>{timeLabel}</time>
      </button>
      {loadState === 'error' && (
        <button
          type="button"
          className="investigation-thumbnail-retry"
          onClick={() => setRetryCount((count) => count + 1)}
        >
          {t('common.retry')}
        </button>
      )}
    </article>
  );
}

function InvestigationThumbnailDrilldown({
  drilldown,
  data,
  loading,
  error,
  onSelect,
  onBack,
  onClose,
  t,
}) {
  return (
    <section className="investigation-thumbnail-drilldown">
      <header>
        <div>
          <h3>{t('investigation.thumbnailDrilldown')}</h3>
          <p>
            {data?.camera?.name || ''}
            {' · '}{formatCursorTime(drilldown.startTime)}
            {' – '}{formatCursorTime(drilldown.endTime)}
          </p>
        </div>
        <div className="investigation-thumbnail-actions">
          <button
            type="button"
            className="btn-secondary"
            disabled={loading || drilldown.history.length === 0}
            onClick={onBack}
          >
            ← {t('investigation.backOneLevel')}
          </button>
          <button type="button" className="btn-secondary" onClick={onClose}>
            {t('common.close')}
          </button>
        </div>
      </header>
      <p className="investigation-thumbnail-help">
        {t('investigation.thumbnailDrilldownHelp')}
      </p>
      {loading && <LoadingIndicator message={t('investigation.loadingSamples')} />}
      {error && <div className="investigation-coverage-warning">{error}</div>}
      {data && !loading && (
        <>
          <div className="investigation-thumbnail-rail">
            {(data.samples || []).map((sample, index) => (
              <InvestigationSampleThumbnail
                key={`${sample.timestamp}-${sample.recording_id || 'gap'}`}
                sample={sample}
                onSelect={() => onSelect(sample, index)}
                t={t}
              />
            ))}
          </div>
          {data.coverage?.segments_truncated && (
            <div className="investigation-coverage-warning">
              {t('investigation.thumbnailCoverageTruncated')}
            </div>
          )}
        </>
      )}
    </section>
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
  const [drawingRegion, setDrawingRegion] = useState(false);
  const [thumbnailDrilldown, setThumbnailDrilldown] = useState(null);
  const [thumbnailData, setThumbnailData] = useState(null);
  const [thumbnailLoading, setThumbnailLoading] = useState(false);
  const [thumbnailError, setThumbnailError] = useState('');
  const initialSelectionApplied = useRef(false);
  const initialQueryLoaded = useRef(false);
  const requestController = useRef(null);
  const searchRequestController = useRef(null);
  const thumbnailRequestController = useRef(null);
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
      if (searchFilters.eventType) filters.event_types = [searchFilters.eventType];
      if (searchFilters.location) filters.locations = [searchFilters.location];
      if (searchFilters.label) filters.labels = [searchFilters.label];
      if (searchFilters.zone) filters.zones = [searchFilters.zone];
      if (searchFilters.source) filters.sources = [searchFilters.source];
      if (searchFilters.captureMethod) {
        filters.capture_methods = [searchFilters.captureMethod];
      }
      if (searchFilters.recordingTag) {
        filters.recording_tags = [searchFilters.recordingTag];
      }
      if (searchFilters.protection) {
        filters.protected = searchFilters.protection === 'protected';
      }
      if (minimumConfidence !== null) filters.min_confidence = minimumConfidence;
      const activeRegion = searchFilters.region &&
        searchFilters.region.width > 0 && searchFilters.region.height > 0
        ? {
          camera_uuid: searchFilters.region.cameraUuid,
          x: searchFilters.region.x,
          y: searchFilters.region.y,
          width: searchFilters.region.width,
          height: searchFilters.region.height,
          match: searchFilters.region.match,
          min_intersection: searchFilters.region.minIntersection,
        } : undefined;
      const data = await fetchJSON('/api/investigations/search', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          camera_uuids: cameraUuids,
          start_time: Math.floor(searchStart),
          end_time: Math.floor(searchEnd),
          filters,
          region: activeRegion,
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
        event: searchFilters.eventType,
        location: searchFilters.location,
        label: searchFilters.label,
        zone: searchFilters.zone,
        source: searchFilters.source,
        capture: searchFilters.captureMethod,
        tag: searchFilters.recordingTag,
        protected: searchFilters.protection,
        confidence_min: searchFilters.minConfidence,
      };
      Object.entries(filterParams).forEach(([name, value]) => {
        if (value) url.searchParams.set(name, value);
        else url.searchParams.delete(name);
      });
      if (activeRegion) {
        url.searchParams.set('region_camera', activeRegion.camera_uuid);
        url.searchParams.set('region_rect', [
          activeRegion.x,
          activeRegion.y,
          activeRegion.width,
          activeRegion.height,
        ].join(','));
        url.searchParams.set('region_match', activeRegion.match);
        url.searchParams.set('region_min', String(activeRegion.min_intersection));
      } else {
        ['region_camera', 'region_rect', 'region_match', 'region_min']
          .forEach((name) => url.searchParams.delete(name));
      }
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
    thumbnailRequestController.current?.abort();
    setThumbnailDrilldown(null);
    setThumbnailData(null);
    setThumbnailLoading(false);
    setThumbnailError('');
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
    thumbnailRequestController.current?.abort();
  }, []);

  const tracks = timeline?.tracks || [];
  const activeTracks = tracks.filter((track) =>
    activeCameraUuids.includes(track.camera_uuid));
  const primaryTrack = tracks.find((track) =>
    track.camera_uuid === primaryCameraUuid) || null;
  const regionTrack = tracks.find((track) =>
    track.camera_uuid === searchFilters.region?.cameraUuid) || null;
  const primaryHasFootage = Boolean(
    primaryTrack && findSegmentAt(primaryTrack.segments, cursor),
  );
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
    if (selectedCameraUuids.includes(cameraUuid) &&
        searchFilters.region?.cameraUuid === cameraUuid) {
      setDrawingRegion(false);
      setSearchFilter('region', null);
    }
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
  const selectedResult = selectedResultIndex >= 0
    ? searchResults[selectedResultIndex] : null;

  const selectResult = useCallback((result) => {
    if (!result || !timeline) return;
    setDrawingRegion(false);
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

  const setRegionRectangle = useCallback((rectangle) => {
    setSearchFilters((current) => ({
      ...current,
      region: rectangle ? {
        ...rectangle,
        cameraUuid: primaryCameraUuid,
        match: current.region?.match || 'center',
        minIntersection: current.region?.minIntersection || 0.25,
      } : null,
    }));
  }, [primaryCameraUuid]);

  const setRegionCoordinate = (name, percentValue) => {
    const value = Number(percentValue) / 100;
    if (!Number.isFinite(value)) return;
    setSearchFilters((current) => {
      if (!current.region) return current;
      const next = { ...current.region, [name]: value };
      if (next.x < 0 || next.y < 0 || next.width <= 0 || next.height <= 0 ||
          next.x + next.width > 1 || next.y + next.height > 1) return current;
      return { ...current, region: next };
    });
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

  const loadThumbnailSamples = useCallback(async (
    cameraUuid, sampleStart, sampleEnd, history = [],
  ) => {
    thumbnailRequestController.current?.abort();
    const controller = new AbortController();
    thumbnailRequestController.current = controller;
    const nextDrilldown = {
      cameraUuid,
      startTime: Math.floor(sampleStart),
      endTime: Math.ceil(sampleEnd),
      history,
    };
    setThumbnailDrilldown(nextDrilldown);
    setThumbnailData(null);
    setThumbnailLoading(true);
    setThumbnailError('');
    try {
      const data = await fetchJSON('/api/investigations/thumbnail-samples', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({
          camera_uuids: [cameraUuid],
          start_time: nextDrilldown.startTime,
          end_time: nextDrilldown.endTime,
          sample_count: 7,
        }),
        signal: controller.signal,
        timeout: 30000,
        retries: 0,
      });
      setThumbnailData(data);
      const url = new URL(window.location.href);
      url.searchParams.set('drill_camera', cameraUuid);
      url.searchParams.set('drill_start', String(nextDrilldown.startTime));
      url.searchParams.set('drill_end', String(nextDrilldown.endTime));
      window.history.replaceState({}, '', url);
    } catch (requestError) {
      if (!controller.signal.aborted) setThumbnailError(requestError.message);
    } finally {
      if (!controller.signal.aborted) setThumbnailLoading(false);
    }
  }, []);

  const startThumbnailDrilldown = () => {
    if (!timeline) return;
    const cameraUuid = selectedResult?.camera_uuid || primaryCameraUuid;
    if (!cameraUuid) return;
    const selectedWindow = selectedResult
      ? thumbnailWindowForResult(
        selectedResult, timeline.start_time, timeline.end_time,
      ) : null;
    const sampleWindow = selectedWindow || {
      startTime: timeline.start_time,
      endTime: timeline.end_time,
    };
    setPlaying(false);
    setPrimaryCameraUuid(cameraUuid);
    void loadThumbnailSamples(
      cameraUuid, sampleWindow.startTime, sampleWindow.endTime, [],
    );
  };

  const selectThumbnailSample = (sample, index) => {
    if (!thumbnailDrilldown || !thumbnailData || !timeline) return;
    setPlaying(false);
    setCursor(Math.max(
      timeline.start_time,
      Math.min(timeline.end_time, sample.timestamp),
    ));
    const nextWindow = narrowThumbnailWindow(
      thumbnailData.samples, index,
      thumbnailDrilldown.startTime, thumbnailDrilldown.endTime,
    );
    if (!nextWindow) {
      setThumbnailError(t('investigation.thumbnailMinimumWindow'));
      return;
    }
    void loadThumbnailSamples(
      thumbnailDrilldown.cameraUuid,
      nextWindow.startTime,
      nextWindow.endTime,
      [
        ...thumbnailDrilldown.history,
        {
          startTime: thumbnailDrilldown.startTime,
          endTime: thumbnailDrilldown.endTime,
        },
      ],
    );
  };

  const backThumbnailDrilldown = () => {
    if (!thumbnailDrilldown || thumbnailDrilldown.history.length === 0) return;
    const previous = thumbnailDrilldown.history[thumbnailDrilldown.history.length - 1];
    void loadThumbnailSamples(
      thumbnailDrilldown.cameraUuid,
      previous.startTime,
      previous.endTime,
      thumbnailDrilldown.history.slice(0, -1),
    );
  };

  const closeThumbnailDrilldown = () => {
    thumbnailRequestController.current?.abort();
    setThumbnailDrilldown(null);
    setThumbnailData(null);
    setThumbnailLoading(false);
    setThumbnailError('');
    const url = new URL(window.location.href);
    ['drill_camera', 'drill_start', 'drill_end']
      .forEach((name) => url.searchParams.delete(name));
    window.history.replaceState({}, '', url);
  };

  useEffect(() => {
    if (!timeline) return;
    const params = new URLSearchParams(window.location.search);
    const cameraUuid = params.get('drill_camera');
    const sampleStart = Number(params.get('drill_start'));
    const sampleEnd = Number(params.get('drill_end'));
    if (!cameraUuid && !params.has('drill_start') && !params.has('drill_end')) return;
    const validCamera = tracks.some((track) => track.camera_uuid === cameraUuid);
    if (validCamera && Number.isFinite(sampleStart) &&
        Number.isFinite(sampleEnd) && sampleEnd > sampleStart &&
        sampleStart >= timeline.start_time && sampleEnd <= timeline.end_time) {
      setPrimaryCameraUuid(cameraUuid);
      void loadThumbnailSamples(cameraUuid, sampleStart, sampleEnd, []);
      return;
    }
    const url = new URL(window.location.href);
    ['drill_camera', 'drill_start', 'drill_end']
      .forEach((name) => url.searchParams.delete(name));
    window.history.replaceState({}, '', url);
  }, [timeline]);

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
            <span>{t('investigation.eventType')}</span>
            <select
              value={searchFilters.eventType}
              onChange={(event) => setSearchFilter('eventType', event.target.value)}
            >
              <option value="">{t('investigation.anyEventType')}</option>
              {(searchData?.facets?.event_types || []).map((facet) => (
                <option key={facet.value} value={facet.value}>
                  {t(`investigation.eventType.${facet.value}`)} ({facet.count})
                </option>
              ))}
            </select>
          </label>
          <label>
            <span>{t('investigation.location')}</span>
            <select
              value={searchFilters.location}
              onChange={(event) => setSearchFilter('location', event.target.value)}
            >
              <option value="">{t('investigation.anyLocation')}</option>
              {(searchData?.facets?.locations || []).map((facet) => (
                <option key={facet.value} value={facet.value}>
                  {facet.label || facet.value} ({facet.count})
                </option>
              ))}
            </select>
          </label>
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
            <span>{t('investigation.captureMethod')}</span>
            <select
              value={searchFilters.captureMethod}
              onChange={(event) => setSearchFilter('captureMethod', event.target.value)}
            >
              <option value="">{t('investigation.anyCaptureMethod')}</option>
              {(searchData?.facets?.capture_methods || []).map((facet) => (
                <option key={facet.value} value={facet.value}>
                  {formatUtils.formatCaptureMethod(facet.value, t)} ({facet.count})
                </option>
              ))}
            </select>
          </label>
          <label>
            <span>{t('investigation.recordingTag')}</span>
            <select
              value={searchFilters.recordingTag}
              onChange={(event) => setSearchFilter('recordingTag', event.target.value)}
            >
              <option value="">{t('investigation.anyRecordingTag')}</option>
              {(searchData?.facets?.recording_tags || []).map((facet) => (
                <option key={facet.value} value={facet.value}>
                  {facet.value} ({facet.count})
                </option>
              ))}
            </select>
          </label>
          <label>
            <span>{t('investigation.protection')}</span>
            <select
              value={searchFilters.protection}
              onChange={(event) => setSearchFilter('protection', event.target.value)}
            >
              <option value="">{t('investigation.anyProtection')}</option>
              {(searchData?.facets?.protection || []).map((facet) => (
                <option key={facet.value} value={facet.value}>
                  {t(`investigation.${facet.value}`)} ({facet.count})
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
        {timeline && (
          <div className="investigation-region-controls">
            <div className="investigation-region-copy">
              <strong>{t('investigation.metadataRegionSearch')}</strong>
              <small>
                {searchFilters.region
                  ? t('investigation.regionActive', {
                    camera: regionTrack?.name || searchFilters.region.cameraUuid,
                  })
                  : t('investigation.regionSearchHelp')}
              </small>
            </div>
            {searchFilters.region && (
              <>
                {[
                  ['x', 'investigation.regionLeft', 0,
                    (1 - searchFilters.region.width) * 100],
                  ['y', 'investigation.regionTop', 0,
                    (1 - searchFilters.region.height) * 100],
                  ['width', 'investigation.regionWidth', 1,
                    (1 - searchFilters.region.x) * 100],
                  ['height', 'investigation.regionHeight', 1,
                    (1 - searchFilters.region.y) * 100],
                ].map(([name, label, minimum, maximum]) => (
                  <label className="investigation-region-coordinate" key={name}>
                    <span>{t(label)}</span>
                    <input
                      type="number"
                      min={minimum}
                      max={maximum}
                      step="0.1"
                      value={Math.round(searchFilters.region[name] * 1000) / 10}
                      onInput={(event) => setRegionCoordinate(name, event.target.value)}
                    />
                  </label>
                ))}
                <label>
                  <span>{t('investigation.regionMatch')}</span>
                  <select
                    value={searchFilters.region.match}
                    onChange={(event) => setSearchFilter('region', {
                      ...searchFilters.region,
                      match: event.target.value,
                    })}
                  >
                    <option value="center">{t('investigation.regionMatch.center')}</option>
                    <option value="intersects">{t('investigation.regionMatch.intersects')}</option>
                    <option value="minimum_intersection">
                      {t('investigation.regionMatch.minimumIntersection')}
                    </option>
                  </select>
                </label>
                {searchFilters.region.match === 'minimum_intersection' && (
                  <label>
                    <span>{t('investigation.regionMinimumOverlap')}</span>
                    <input
                      type="number"
                      min="1"
                      max="100"
                      step="1"
                      value={Math.round(searchFilters.region.minIntersection * 100)}
                      onInput={(event) => {
                        const value = Number(event.target.value);
                        if (Number.isFinite(value) && value >= 1 && value <= 100) {
                          setSearchFilter('region', {
                            ...searchFilters.region,
                            minIntersection: value / 100,
                          });
                        }
                      }}
                    />
                  </label>
                )}
              </>
            )}
            <button
              type="button"
              className="btn-secondary"
              disabled={!primaryTrack || !primaryHasFootage}
              onClick={() => {
                setPlaying(false);
                setDrawingRegion(true);
              }}
            >
              {drawingRegion
                ? t('investigation.drawingRegion')
                : t('investigation.drawRegion')}
            </button>
            <button
              type="button"
              className="btn-secondary"
              disabled={!primaryTrack}
              onClick={() => {
                setDrawingRegion(false);
                setRegionRectangle({ x: 0, y: 0, width: 1, height: 1 });
              }}
            >
              {t('investigation.useFullFrame')}
            </button>
            {searchFilters.region && (
              <button
                type="button"
                className="btn-secondary"
                onClick={() => {
                  setDrawingRegion(false);
                  setSearchFilter('region', null);
                }}
              >
                {t('investigation.clearRegion')}
              </button>
            )}
          </div>
        )}
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
                  disabled={!timeline || (!selectedResult && !primaryCameraUuid)}
                  onClick={startThumbnailDrilldown}
                >
                  {selectedResult
                    ? t('investigation.exploreSelectedThumbnails')
                    : t('investigation.exploreThumbnails')}
                </button>
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

            {thumbnailDrilldown && (
              <InvestigationThumbnailDrilldown
                drilldown={thumbnailDrilldown}
                data={thumbnailData}
                loading={thumbnailLoading}
                error={thumbnailError}
                onSelect={selectThumbnailSample}
                onBack={backThumbnailDrilldown}
                onClose={closeThumbnailDrilldown}
                t={t}
              />
            )}
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
                {(searchData.coverage?.unresolved_legacy_rows || 0) > 0 && (
                  <div className="investigation-coverage-warning">
                    {t('investigation.incompleteCoverage', {
                      count: searchData.coverage.unresolved_legacy_rows,
                    })}
                  </div>
                )}
                {(searchData.coverage?.spatial_metadata?.rows_without_boxes || 0) > 0 && (
                  <div className="investigation-coverage-warning">
                    {t('investigation.spatialCoverageGap', {
                      count: searchData.coverage.spatial_metadata.rows_without_boxes,
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
                    {searchData.coverage?.spatial_metadata?.requested &&
                      searchData.coverage.spatial_metadata.rows_with_boxes === 0
                      ? t('investigation.noSpatialMetadata')
                      : t('investigation.noResults')}
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
                  region={searchFilters.region?.cameraUuid === track.camera_uuid
                    ? searchFilters.region : null}
                  drawingRegion={drawingRegion &&
                    primaryCameraUuid === track.camera_uuid}
                  onRegionChange={setRegionRectangle}
                  onRegionComplete={() => setDrawingRegion(false)}
                  onMakePrimary={() => {
                    setDrawingRegion(false);
                    setPrimaryCameraUuid(track.camera_uuid);
                  }}
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
