/**
 * Stream Configuration Modal Component
 * Expanded, responsive modal with accordion sections for stream configuration
 */

import { useState, useEffect, useRef, useCallback } from 'preact/hooks';
import { ZoneEditor } from './ZoneEditor.jsx';
import { obfuscateUrlCredentials } from '../../utils/url-utils.js';
import { useI18n } from '../../i18n.js';
import { showStatusMessage } from './ToastContainer.jsx';

const primaryAccentStyle = { accentColor: 'hsl(var(--primary))' };
const HOURS_PER_DAY = 24;
const HOURS_PER_WEEK = 7 * HOURS_PER_DAY;

const isCustomApiUrl = (url) =>
  typeof url === 'string' && (url.startsWith('http://') || url.startsWith('https://'));

const getDetectionModelSelectValue = (modelValue) => {
  if (!modelValue) {
    return '';
  }
  return isCustomApiUrl(modelValue) ? 'api-detection' : modelValue;
};

/**
 * Recording Schedule Grid Component
 * Interactive 7-day × 24-hour weekly grid for scheduling continuous recording
 */
function RecordingScheduleGrid({ schedule, onChange }) {
  const { t } = useI18n();
  const isDragging = useRef(false);
  const dragValue = useRef(true);

  const DAYS = [
    t('streamsConfig.sunShort'),
    t('streamsConfig.monShort'),
    t('streamsConfig.tueShort'),
    t('streamsConfig.wedShort'),
    t('streamsConfig.thuShort'),
    t('streamsConfig.friShort'),
    t('streamsConfig.satShort')
  ];
  const HOURS = Array.from({ length: HOURS_PER_DAY }, (_, i) => {
    if (i === 0) return '12am';
    if (i < 12) return `${i}am`;
    if (i === 12) return '12pm';
    return `${i - 12}pm`;
  });

  useEffect(() => {
    const handleMouseUp = () => { isDragging.current = false; };
    window.addEventListener('mouseup', handleMouseUp);
    return () => window.removeEventListener('mouseup', handleMouseUp);
  }, []);

  const handleCellMouseDown = useCallback((e, dayIdx, hourIdx) => {
    e.preventDefault();
    const newVal = !schedule[dayIdx * HOURS_PER_DAY + hourIdx];
    dragValue.current = newVal;
    isDragging.current = true;
    const s = schedule.slice();
    s[dayIdx * HOURS_PER_DAY + hourIdx] = newVal;
    onChange(s);
  }, [schedule, onChange]);

  const handleCellMouseEnter = useCallback((dayIdx, hourIdx) => {
    if (!isDragging.current) return;
    const s = schedule.slice();
    s[dayIdx * HOURS_PER_DAY + hourIdx] = dragValue.current;
    onChange(s);
  }, [schedule, onChange]);

  const toggleDay = useCallback((dayIdx) => {
    const allOn = Array.from({ length: HOURS_PER_DAY }).every(
      (_, h) => schedule[dayIdx * HOURS_PER_DAY + h]
    );
    const s = schedule.slice();
    for (let h = 0; h < HOURS_PER_DAY; h++) s[dayIdx * HOURS_PER_DAY + h] = !allOn;
    onChange(s);
  }, [schedule, onChange]);

  const toggleHour = useCallback((hourIdx) => {
    const allOn = Array.from({ length: 7 }).every((_, d) => schedule[d * HOURS_PER_DAY + hourIdx]);
    const s = schedule.slice();
    for (let d = 0; d < 7; d++) s[d * HOURS_PER_DAY + hourIdx] = !allOn;
    onChange(s);
  }, [schedule, onChange]);

  /** Build the boolean pattern array for a given preset key */
  const getPresetPattern = useCallback((preset) => {
    const p = Array(HOURS_PER_WEEK).fill(false);
    if (preset === 'weekdays') {
      for (let d = 1; d <= 5; d++) for (let h = 0; h < HOURS_PER_DAY; h++) p[d * HOURS_PER_DAY + h] = true;
    } else if (preset === 'business') {
      for (let d = 1; d <= 5; d++) for (let h = 8; h < 18; h++) p[d * HOURS_PER_DAY + h] = true;
    } else if (preset === 'nights') {
      for (let d = 0; d < 7; d++) {
        for (let h = 0; h < 6; h++) p[d * HOURS_PER_DAY + h] = true;
        for (let h = 20; h < HOURS_PER_DAY; h++) p[d * HOURS_PER_DAY + h] = true;
      }
    } else if (preset === 'weekends') {
      for (const d of [0, 6]) for (let h = 0; h < HOURS_PER_DAY; h++) p[d * HOURS_PER_DAY + h] = true;
    }
    return p;
  }, []);

  /** Check if every hour in a preset's pattern is currently enabled */
  const isPresetActive = useCallback((presetKey) => {
    const pattern = getPresetPattern(presetKey);
    return pattern.some(Boolean) && pattern.every((on, i) => !on || schedule[i]);
  }, [schedule, getPresetPattern]);

  const applyPreset = useCallback((preset) => {
    // Absolute presets — always replace the entire schedule
    if (preset === 'all') { onChange(Array(HOURS_PER_WEEK).fill(true)); return; }
    if (preset === 'none') { onChange(Array(HOURS_PER_WEEK).fill(false)); return; }

    // Toggleable presets — OR on / remove off
    const pattern = getPresetPattern(preset);
    const active = isPresetActive(preset);
    const s = schedule.slice();
    for (let i = 0; i < HOURS_PER_WEEK; i++) {
      if (pattern[i]) s[i] = !active;
    }
    onChange(s);
  }, [schedule, onChange, getPresetPattern, isPresetActive]);

  const scheduledHours = schedule.filter(Boolean).length;
  const PRESETS = [
    { key: 'all', label: '24/7' },
    { key: 'weekdays', label: t('streamsConfig.weekdays') },
    { key: 'business', label: t('streamsConfig.businessHours') },
    { key: 'nights', label: t('streamsConfig.nights') },
    { key: 'weekends', label: t('streamsConfig.weekends') },
    { key: 'none', label: t('streamsConfig.clearAll') },
  ];

  return (
    <div className="space-y-2 select-none" style={{ userSelect: 'none' }}>
      {/* Presets & counter */}
      <div className="flex flex-wrap items-center gap-1.5">
        <span className="text-xs text-muted-foreground font-medium">{t('streamsConfig.quickSelect')}:</span>
        {PRESETS.map(p => {
          const toggleable = p.key !== 'all' && p.key !== 'none';
          const active = toggleable && isPresetActive(p.key);
          return (
            <button
              key={p.key}
              type="button"
              onClick={() => applyPreset(p.key)}
              className={`px-2 py-0.5 text-xs rounded-full border transition-all duration-150 font-medium ${
                active
                  ? 'bg-primary text-primary-foreground border-primary shadow-sm'
                  : 'border-border hover:bg-primary/10 hover:border-primary/40 hover:text-primary'
              }`}
            >
              {p.label}
            </button>
          );
        })}
        <span className="ml-auto text-xs font-semibold text-primary tabular-nums">
          {t('streamsConfig.hoursPerWeek', { count: scheduledHours })}
        </span>
      </div>

      {/* Grid */}
      <div className="overflow-x-auto rounded-lg border border-border bg-card p-3">
        <div style={{ minWidth: '340px' }}>
          {/* Day header row */}
          <div className="flex mb-1">
            <div className="flex-shrink-0" style={{ width: '28px' }} />
            {DAYS.map((day, dayIdx) => (
              <button
                key={day}
                type="button"
                title={t('streamsConfig.toggleAllDay', { day })}
                onClick={() => toggleDay(dayIdx)}
                className="flex-1 text-center text-[10px] font-semibold text-muted-foreground hover:text-primary hover:bg-primary/10 rounded transition-colors py-0.5 leading-none"
              >
                {day}
              </button>
            ))}
          </div>

          {/* Hour rows */}
          {HOURS.map((hourLabel, hourIdx) => (
            <div key={hourIdx} className="flex items-center" style={{ height: '13px', marginBottom: '1px' }}>
              {/* Hour label — clickable to toggle row */}
              <button
                type="button"
                title={t('streamsConfig.toggleHourAllDays', { hour: hourLabel })}
                onClick={() => toggleHour(hourIdx)}
                className="flex-shrink-0 flex items-center justify-end pr-1.5 hover:text-primary transition-colors"
                style={{ width: '28px', height: '13px' }}
              >
                {hourIdx % 3 === 0 && (
                  <span className="text-[9px] text-muted-foreground leading-none">{hourLabel}</span>
                )}
              </button>
              {/* Day cells */}
              {DAYS.map((_, dayIdx) => {
                const isActive = !!schedule[dayIdx * HOURS_PER_DAY + hourIdx];
                return (
                  <div
                    key={dayIdx}
                    className={`flex-1 rounded-sm cursor-pointer transition-colors ${
                      isActive
                        ? 'bg-primary/70 hover:bg-primary'
                        : 'bg-muted/40 hover:bg-muted/80'
                    }`}
                    style={{ height: '12px', margin: '0 1px' }}
                    onMouseDown={(e) => handleCellMouseDown(e, dayIdx, hourIdx)}
                    onMouseEnter={() => handleCellMouseEnter(dayIdx, hourIdx)}
                  />
                );
              })}
            </div>
          ))}

          {/* Legend */}
          <div className="flex items-center justify-end gap-3 mt-2 pt-2 border-t border-border/50">
            <div className="flex items-center gap-1">
              <div className="w-3 h-2.5 rounded-sm bg-muted/40 border border-border/30" />
              <span className="text-[10px] text-muted-foreground">{t('streamsConfig.noRecording')}</span>
            </div>
            <div className="flex items-center gap-1">
              <div className="w-3 h-2.5 rounded-sm bg-primary/70" />
              <span className="text-[10px] text-muted-foreground">{t('streamsConfig.recordingActive')}</span>
            </div>
            <span className="text-[10px] text-muted-foreground italic">{t('streamsConfig.clickHeadersOrDragCells')}</span>
          </div>
        </div>
      </div>
    </div>
  );
}

/**
 * Accordion Section Component
 */
function AccordionSection({ title, isExpanded, onToggle, children, badge }) {
  return (
    <div className="border border-border rounded-lg mb-3">
      <button
        type="button"
        onClick={onToggle}
        className="w-full flex items-center justify-between p-4 text-left hover:bg-muted/50 transition-colors rounded-t-lg"
      >
        <div className="flex items-center space-x-2">
          <h4 className="text-md font-semibold text-foreground">{title}</h4>
          {badge && (
            <span className="px-2 py-0.5 text-xs rounded-full bg-primary/10 text-primary">
              {badge}
            </span>
          )}
        </div>
        <svg
          className={`w-5 h-5 transition-transform ${isExpanded ? 'transform rotate-180' : ''}`}
          fill="none"
          stroke="currentColor"
          viewBox="0 0 24 24"
        >
          <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M19 9l-7 7-7-7" />
        </svg>
      </button>
      {isExpanded && (
        <div className="p-4 pt-3 space-y-4 border-t border-border">
          {children}
        </div>
      )}
    </div>
  );
}

/**
 * Main Stream Configuration Modal
 */
export function StreamConfigModal({
  isEditing,
  isCloning = false,
  currentStream,
  detectionModels,
  expandedSections,
  onToggleSection,
  onInputChange,
  onThresholdChange,
  onTestConnection,
  onTestMotion,
  onSave,
  onClose,
  onRefreshModels,
  hideCredentials = false
}) {
  const { t } = useI18n();
  const [showZoneEditor, setShowZoneEditor] = useState(false);
  const [detectionZones, setDetectionZones] = useState(currentStream.detectionZones || []);

  // Keep a ref to onInputChange so the zones-load effect never needs it as a
  // dependency. If we put onInputChange in the dependency array, calling it
  // would trigger a parent re-render, which would create a new onInputChange
  // function, which would cause the effect to re-run and re-fetch zones again,
  // leading to an infinite (or at least very noisy) loop of fetches.
  const onInputChangeRef = useRef(onInputChange);
  // Always point the ref to the latest onInputChange so the effect below can
  // safely use onInputChangeRef.current without depending on onInputChange
  // itself. This gives us "latest callback" semantics without re-running the
  // effect whenever the parent recreates onInputChange.
  useEffect(() => {
    onInputChangeRef.current = onInputChange;
  }, [onInputChange]);

  // Load zones from API when the modal opens for an existing stream.
  //
  // IMPORTANT: This effect intentionally only depends on isEditing and
  // currentStream.name. It does NOT depend on onInputChange by design:
  // - We only want to fetch zones when the target stream changes or when we
  //   switch into/out of editing mode, not whenever the parent re-renders.
  // - onInputChange is typically recreated on each parent render; including it
  //   would cause this effect to re-run and re-fetch zones far more often than
  //   needed, potentially creating an infinite loop (fetch → onInputChange →
  //   parent re-render → new onInputChange → effect re-run → fetch ...).
  //
  // Using onInputChangeRef.current inside the effect lets us always call the
  // latest onInputChange implementation without adding it to the dependency
  // array, which is why the ref pattern is used here.
  useEffect(() => {
    const loadZones = async () => {
      if (!isEditing || !currentStream.name) {
        return;
      }

      try {
        const response = await fetch(`/api/streams/${encodeURIComponent(currentStream.name)}/zones`);
        if (response.ok) {
          const data = await response.json();
          if (data.zones && Array.isArray(data.zones)) {
            setDetectionZones(data.zones);
            // Use ref so we always call the latest callback without making it
            // a dependency of this effect.
            onInputChangeRef.current({ target: { name: 'detectionZones', value: data.zones } });
          }
        } else {
          console.error('Failed to load zones:', response.status, response.statusText);
          const statusCode = response.status;
          const statusText = response.statusText || '';
          showStatusMessage(
            t(
              'streamConfig.zonesLoadFailed',
              'Failed to load detection zones (HTTP {statusCode}{statusText}). Please try again or contact an administrator.',
              {
                statusCode,
                statusText: statusText ? `: ${statusText}` : '',
              }
            ),
            'error'
          );
        }
      } catch (error) {
        console.error('Error loading zones:', error);
        const errorMessage = (error && error.message) ? error.message : String(error || '');
        showStatusMessage(
          t(
            'streamConfig.zonesLoadError',
            'An error occurred while loading detection zones. Details: {errorMessage}',
            { errorMessage }
          ),
          'error'
        );
      }
    };

    loadZones();
  }, [isEditing, currentStream.name]);

  const handleZonesChange = (zones) => {
    setDetectionZones(zones);
    // Update currentStream with new zones
    onInputChange({ target: { name: 'detectionZones', value: zones } });
  };

  return (
    <>
      {showZoneEditor && (
        <ZoneEditor
          streamName={currentStream.name}
          zones={detectionZones}
          onZonesChange={handleZonesChange}
          onClose={() => setShowZoneEditor(false)}
        />
      )}
    <div id="stream-modal" className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
      <div className="bg-card text-card-foreground rounded-lg shadow-xl w-full max-w-5xl max-h-[95vh] overflow-hidden flex flex-col">
        {/* Header */}
        <div className="flex justify-between items-center p-6 border-b border-border flex-shrink-0">
          <div>
            <h3 className="text-2xl font-bold">{isEditing ? t('streams.editStream') : isCloning ? t('streams.cloneStream') : t('streams.addStream')}</h3>
            <p className="text-sm text-muted-foreground mt-1">
              {t('streamsConfig.subtitle')}
            </p>
          </div>
          <button
            onClick={onClose}
            className="text-muted-foreground hover:text-foreground transition-colors p-2 rounded-full hover:bg-muted"
          >
            <svg className="w-6 h-6" fill="none" stroke="currentColor" viewBox="0 0 24 24">
              <path strokeLinecap="round" strokeLinejoin="round" strokeWidth={2} d="M6 18L18 6M6 6l12 12" />
            </svg>
          </button>
        </div>

        {/* Scrollable Content */}
        <div className="flex-1 overflow-y-auto p-6">
          <form id="stream-form" className="space-y-3">

            {/* Basic Settings Section */}
            <AccordionSection
              title={t('streamsConfig.basicSettings')}
              isExpanded={expandedSections.basic}
              onToggle={() => onToggleSection('basic')}
            >
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div className="md:col-span-2">
                  <label htmlFor="stream-name" className="block text-sm font-medium mb-2">
                    {t('streamsConfig.streamName')} <span className="text-destructive">*</span>
                  </label>
                  <input
                    type="text"
                    id="stream-name"
                    name="name"
                    className={`w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground ${isEditing ? 'bg-muted/30' : ''}`}
                    value={currentStream.name}
                    onChange={onInputChange}
                    disabled={isEditing}
                    required
                    placeholder={t('streamsConfig.streamNamePlaceholder')}
                  />
                  {isEditing && (
                    <p className="mt-1 text-xs text-muted-foreground">{t('streamsConfig.streamNameImmutable')}</p>
                  )}
                </div>

                <div className="md:col-span-2">
                  <label htmlFor="stream-url" className="block text-sm font-medium mb-2">
                    {t('streamsConfig.streamUrl')} <span className="text-destructive">*</span>
                  </label>
                  <input
                    type="text"
                    id="stream-url"
                    name="url"
                    className={`w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground ${hideCredentials ? 'cursor-not-allowed opacity-75' : ''}`}
                    placeholder={t('streamsConfig.streamUrlPlaceholder')}
                    value={hideCredentials ? obfuscateUrlCredentials(currentStream.url) : currentStream.url}
                    onChange={onInputChange}
                    required
                    readOnly={hideCredentials}
                    title={hideCredentials ? t('streamsConfig.urlCredentialsHiddenTitle') : ''}
                  />
                  <p className="mt-1 text-xs text-muted-foreground">
                    {hideCredentials ? t('streamsConfig.urlCredentialsHidden') : t('streamsConfig.rtspUrlHelp')}
                  </p>
                </div>

                <div className="md:col-span-2">
                  <label htmlFor="stream-sub-url" className="block text-sm font-medium mb-2">
                    {t('streamsConfig.subStreamUrl')}
                  </label>
                  <input
                    type="text"
                    id="stream-sub-url"
                    name="subStreamUrl"
                    className={`w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground ${hideCredentials ? 'cursor-not-allowed opacity-75' : ''}`}
                    placeholder={t('streamsConfig.subStreamUrlPlaceholder')}
                    value={hideCredentials ? obfuscateUrlCredentials(currentStream.subStreamUrl || '') : (currentStream.subStreamUrl || '')}
                    onChange={onInputChange}
                    readOnly={hideCredentials}
                  />
                  <p className="mt-1 text-xs text-muted-foreground">
                    {t('streamsConfig.subStreamUrlHelp')}
                  </p>
                </div>

                <div className="md:col-span-2">
                  <label htmlFor="stream-admin-url" className="block text-sm font-medium mb-2">
                    {t('streamsConfig.cameraAdminUrl')}
                  </label>
                  <input
                    type="text"
                    id="stream-admin-url"
                    name="adminUrl"
                    className={`w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground ${hideCredentials ? 'cursor-not-allowed opacity-75' : ''}`}
                    placeholder="http://192.168.1.100/"
                    value={hideCredentials ? obfuscateUrlCredentials(currentStream.adminUrl || '') : (currentStream.adminUrl || '')}
                    onChange={onInputChange}
                    readOnly={hideCredentials}
                    title={hideCredentials ? t('streamsConfig.urlCredentialsHiddenTitle') : ''}
                  />
                  <p className="mt-1 text-xs text-muted-foreground">
                    {t('streamsConfig.cameraAdminUrlHelpBefore')} <span className="font-mono">http://</span> {t('streamsConfig.cameraAdminUrlHelpOr')} <span className="font-mono">https://</span> {t('streamsConfig.cameraAdminUrlHelpAfter')}
                  </p>
                </div>

                <div>
                  <label className="block text-sm font-medium mb-1" htmlFor="stream-tags">
                    {t('streamsConfig.streamTags')}
                  </label>
                  <input
                    type="text"
                    id="stream-tags"
                    name="tags"
                    className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                    placeholder={t('streamsConfig.streamTagsPlaceholder')}
                    value={currentStream.tags || ''}
                    onChange={onInputChange}
                    maxLength={255}
                  />
                  <p className="mt-1 text-xs text-muted-foreground">
                    {t('streamsConfig.streamTagsHelpBefore')} <span className="font-mono">outdoor,critical</span>. {t('streamsConfig.streamTagsHelpAfter')}
                  </p>
                  {(() => {
                    const tagsArray = (currentStream.tags || '').split(',').map(tag => tag.trim()).filter(Boolean);
                    return tagsArray.length > 0 && (
                      <div className="mt-2 flex flex-wrap gap-1">
                        {tagsArray.map(tag => (
                          <span key={tag} className="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium bg-primary/10 text-primary border border-primary/20">
                            #{tag}
                          </span>
                        ))}
                      </div>
                    );
                  })()}
                </div>

                <div className="flex items-center space-x-6">
                  <label className="flex items-center space-x-2 cursor-pointer">
                    <input
                      type="checkbox"
                      id="stream-enabled"
                      name="enabled"
                      className="h-4 w-4 rounded border-input"
                      style={primaryAccentStyle}
                      checked={currentStream.enabled}
                      onChange={onInputChange}
                    />
                    <span className="text-sm font-medium">{t('streamsConfig.streamActive')}</span>
                  </label>
                  <label className="flex items-center space-x-2 cursor-pointer">
                    <input
                      type="checkbox"
                      id="stream-streaming-enabled"
                      name="streamingEnabled"
                      className="h-4 w-4 rounded border-input"
                      style={primaryAccentStyle}
                      checked={currentStream.streamingEnabled}
                      onChange={onInputChange}
                    />
                    <span className="text-sm font-medium">{t('streamsConfig.liveViewEnabled')}</span>
                  </label>
                </div>

                <div className="flex items-center">
                  <label className="flex items-center space-x-2 cursor-pointer">
                    <input
                      type="checkbox"
                      id="stream-is-onvif"
                      name="isOnvif"
                      className="h-4 w-4 rounded border-input"
                      style={primaryAccentStyle}
                      checked={currentStream.isOnvif}
                      onChange={onInputChange}
                    />
                    <span className="text-sm font-medium">{t('streamsConfig.onvifCamera')}</span>
                  </label>
                  <span className="ml-2 text-xs text-muted-foreground">{t('streamsConfig.onvifCameraHelp')}</span>
                </div>

                {/* ONVIF Credentials - shown when ONVIF Camera is enabled */}
                {currentStream.isOnvif && (
                  <div className="col-span-2 p-4 bg-muted/50 rounded-lg border border-border">
                    <h4 className="text-sm font-medium mb-3">{t('streamsConfig.onvifSettings')}</h4>
                    <p className="text-xs text-muted-foreground mb-3">
                      {t('streamsConfig.onvifSettingsHelp')}
                    </p>
                    <div className="grid grid-cols-1 md:grid-cols-4 gap-4">
                      <div>
                        <label htmlFor="stream-onvif-username" className="block text-sm font-medium mb-1">{t('auth.username')}</label>
                        <input
                          type="text"
                          id="stream-onvif-username"
                          name="onvifUsername"
                          className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                          placeholder="admin"
                          value={currentStream.onvifUsername || ''}
                          onChange={onInputChange}
                        />
                      </div>
                      <div>
                        <label htmlFor="stream-onvif-password" className="block text-sm font-medium mb-1">{t('auth.password')}</label>
                        <input
                          type="password"
                          id="stream-onvif-password"
                          name="onvifPassword"
                          className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                          placeholder="••••••••"
                          value={currentStream.onvifPassword || ''}
                          onChange={onInputChange}
                        />
                      </div>
                      <div>
                        <label htmlFor="stream-onvif-profile" className="block text-sm font-medium mb-1">{t('streamsConfig.profileOptional')}</label>
                        <input
                          type="text"
                          id="stream-onvif-profile"
                          name="onvifProfile"
                          className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                          placeholder="Profile_1"
                          value={currentStream.onvifProfile || ''}
                          onChange={onInputChange}
                        />
                      </div>
                      <div>
                        <label htmlFor="stream-onvif-port" className="block text-sm font-medium mb-1">{t('streamsConfig.onvifPort')}</label>
                        <input
                          type="number"
                          id="stream-onvif-port"
                          name="onvifPort"
                          className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                          placeholder="80"
                          min="0"
                          max="65535"
                          value={currentStream.onvifPort || 0}
                          onChange={onInputChange}
                        />
                        <p className="text-xs text-muted-foreground mt-1">{t('streamsConfig.zeroAutoDetect')}</p>
                      </div>
                    </div>
                  </div>
                )}

                <div>
                  <label className="block text-sm font-medium mb-2">{t('streams.resolution')}</label>
                  <span className="block w-full px-3 py-2 border border-input rounded-md bg-muted/30 text-muted-foreground">
                    {currentStream.width > 0 && currentStream.height > 0
                      ? `${currentStream.width}×${currentStream.height}`
                      : t('streamsConfig.autoDetected')}
                  </span>
                  <span className="text-xs text-muted-foreground mt-1 block">{t('streamsConfig.detectedFromSource')}</span>
                </div>

                <div>
                  <label className="block text-sm font-medium mb-2">{t('streams.fps')}</label>
                  <span className="block w-full px-3 py-2 border border-input rounded-md bg-muted/30 text-muted-foreground">
                    {currentStream.fps > 0 ? currentStream.fps : t('streamsConfig.autoDetected')}
                  </span>
                  <span className="text-xs text-muted-foreground mt-1 block">{t('streamsConfig.detectedFromSource')}</span>
                </div>

                <div>
                  <label className="block text-sm font-medium mb-2">{t('streams.codec')}</label>
                  <span className="block w-full px-3 py-2 border border-input rounded-md bg-muted/30 text-muted-foreground">
                    {currentStream.codec ? currentStream.codec.toUpperCase() : t('streamsConfig.autoDetected')}
                  </span>
                  <span className="text-xs text-muted-foreground mt-1 block">{t('streamsConfig.detectedFromSource')}</span>
                </div>

                <div>
                  <label htmlFor="stream-protocol" className="block text-sm font-medium mb-2">{t('streamsConfig.protocol')}</label>
                  <select
                    id="stream-protocol"
                    name="protocol"
                    className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                    value={currentStream.protocol}
                    onChange={onInputChange}
                  >
                    <option value="0">TCP</option>
                    <option value="1">UDP</option>
                  </select>
                </div>
              </div>
            </AccordionSection>

            {/* Recording Settings Section */}
            <AccordionSection
              title={t('streamsConfig.recordingSettings')}
              isExpanded={expandedSections.recording}
              onToggle={() => onToggleSection('recording')}
            >
              <div className="space-y-4">
                {/* Recording Mode Selection */}
                <div className="space-y-3">
                  <h5 className="text-sm font-semibold">{t('streamsConfig.recordingMode')}</h5>
                  <div className="space-y-2">
                    {/* Continuous Recording block (with optional schedule sub-option) */}
                    <div className={`rounded-lg border transition-colors ${currentStream.record ? 'border-primary/40 bg-primary/5' : 'border-border'}`}>
                      <label className="flex items-start space-x-3 cursor-pointer p-3 hover:bg-muted/30 transition-colors rounded-lg">
                        <input
                          type="checkbox"
                          id="stream-record"
                          name="record"
                          className="h-4 w-4 mt-0.5 rounded border-input"
                          style={primaryAccentStyle}
                          checked={currentStream.record}
                          onChange={onInputChange}
                        />
                        <div>
                          <span className="text-sm font-medium">{t('streamsConfig.enableContinuousRecording')}</span>
                          <p className="text-xs text-muted-foreground mt-1">
                            {currentStream.record && currentStream.recordOnSchedule
                              ? t('streamsConfig.recordingScheduleOnly')
                              : t('streamsConfig.recordAllVideoContinuously')}
                          </p>
                        </div>
                      </label>

                      {/* On a Schedule sub-option — visible when continuous recording is enabled */}
                      {currentStream.record && (
                        <div className="px-3 pb-3 ml-7 space-y-2">
                          <label className="flex items-center gap-2 cursor-pointer px-2 py-1.5 rounded-md hover:bg-primary/10 transition-colors group w-fit border border-transparent hover:border-primary/20">
                            <input
                              type="checkbox"
                              id="stream-record-on-schedule"
                              name="recordOnSchedule"
                              className="h-3.5 w-3.5 rounded border-input"
                              style={primaryAccentStyle}
                              checked={currentStream.recordOnSchedule || false}
                              onChange={onInputChange}
                            />
                            <span className="text-xs font-semibold group-hover:text-primary transition-colors">
                              📅 {t('streamsConfig.onASchedule')}
                            </span>
                            <span className="text-xs text-muted-foreground">— {t('streamsConfig.restrictRecordingToSchedule')}</span>
                          </label>

                          {currentStream.recordOnSchedule && (
                            <div className="rounded-xl border border-primary/25 bg-background p-3 shadow-sm">
                              <p className="text-xs text-muted-foreground mb-3 flex items-center gap-1.5">
                                <span className="text-primary">●</span>
                                {t('streamsConfig.scheduleGridHelp')}
                              </p>
                              <RecordingScheduleGrid
                                schedule={currentStream.recordingSchedule && currentStream.recordingSchedule.length === HOURS_PER_WEEK
                                  ? currentStream.recordingSchedule
                                  : Array(HOURS_PER_WEEK).fill(true)}
                                onChange={(s) => onInputChange({ target: { name: 'recordingSchedule', value: s } })}
                              />
                            </div>
                          )}
                        </div>
                      )}
                    </div>
                    <label className="flex items-start space-x-3 cursor-pointer p-3 rounded-lg border border-border hover:bg-muted/50 transition-colors">
                      <input
                        type="checkbox"
                        id="stream-detection-enabled"
                        name="detectionEnabled"
                        className="h-4 w-4 mt-0.5 rounded border-input"
                        style={primaryAccentStyle}
                        checked={currentStream.detectionEnabled}
                        onChange={onInputChange}
                      />
                      <div>
                        <span className="text-sm font-medium">{t('streamsConfig.enableDetectionRecording')}</span>
                        <p className="text-xs text-muted-foreground mt-1">
                          {t('streamsConfig.enableDetectionRecordingHelp')}
                        </p>
                      </div>
                    </label>
                  </div>

                  {/* Show info box based on recording mode selection */}
                  {currentStream.record && currentStream.detectionEnabled && (
                    <div className="p-3 rounded-md bg-muted border border-border">
                      <p className="text-sm text-muted-foreground">
                        <strong className="text-foreground">{t('streamsConfig.bothModesEnabled')}</strong> {t('streamsConfig.bothModesEnabledHelp')}
                      </p>
                    </div>
                  )}
                  {!currentStream.record && currentStream.detectionEnabled && (
                    <div className="p-3 rounded-md bg-muted border border-border">
                      <p className="text-sm text-muted-foreground">
                        <strong className="text-foreground">{t('streamsConfig.detectionOnlyMode')}</strong> {t('streamsConfig.detectionOnlyModeHelp')}
                      </p>
                    </div>
                  )}
                  {!currentStream.record && !currentStream.detectionEnabled && (
                    <div className="p-3 bg-muted border border-border rounded-md">
                      <p className="text-sm text-muted-foreground">
                        {t('streamsConfig.noRecordingModeSelected')}
                      </p>
                    </div>
                  )}
                </div>

                {/* Audio Settings */}
                <div className="border-t border-border pt-4">
                  <h5 className="text-sm font-semibold mb-3">{t('streamsConfig.audioSettings')}</h5>
                  <div className="flex items-center space-x-4">
                    <label className="flex items-center space-x-2 cursor-pointer">
                      <input
                        type="checkbox"
                        id="stream-record-audio"
                        name="recordAudio"
                        className="h-4 w-4 rounded border-input"
                        style={primaryAccentStyle}
                        checked={currentStream.recordAudio}
                        onChange={onInputChange}
                      />
                      <span className="text-sm font-medium">{t('streamsConfig.recordAudio')}</span>
                    </label>
                    <label className="flex items-center space-x-2 cursor-pointer">
                      <input
                        type="checkbox"
                        id="stream-backchannel-enabled"
                        name="backchannelEnabled"
                        className="h-4 w-4 rounded border-input"
                        style={primaryAccentStyle}
                        checked={currentStream.backchannelEnabled}
                        onChange={onInputChange}
                      />
                      <span className="text-sm font-medium">{t('streamsConfig.twoWayAudio')}</span>
                    </label>
                  </div>
                  <p className="text-sm text-muted-foreground mt-2">
                    {t('streamsConfig.audioSettingsHelp')}
                  </p>
                </div>

                {/* AI Detection Settings - nested under recording */}
                {currentStream.detectionEnabled && (
                  <div className="border-t border-border pt-4">
                    <h5 className="text-sm font-semibold mb-3">{t('streamsConfig.aiDetectionSettings')}</h5>
                    <div className="space-y-4">
                      <div>
                        <label htmlFor="stream-detection-model" className="block text-sm font-medium mb-2">
                          {t('streamsConfig.detectionModel')}
                        </label>
                        <div className="flex space-x-2">
                          <select
                            id="stream-detection-model"
                            name="detectionModel"
                            className="flex-1 px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                            value={getDetectionModelSelectValue(currentStream.detectionModel)}
                            onChange={onInputChange}
                          >
                            <option value="">{t('streamsConfig.selectModel')}</option>
                            <option value="api-detection">
                              {t('streamsConfig.customApiModel')}
                            </option>
                            {detectionModels.map(model => (
                              <option key={model.id} value={model.id}>{model.name}</option>
                            ))}
                          </select>
                          <button
                            type="button"
                            onClick={onRefreshModels}
                            className="p-2 rounded-md bg-secondary hover:bg-secondary/80 text-secondary-foreground focus:outline-none"
                            title={t('streamsConfig.refreshModels')}
                          >
                            <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                              <path fillRule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clipRule="evenodd" />
                            </svg>
                          </button>
                        </div>

                        {/* Show info box and custom endpoint option for API detection */}
                        {(currentStream.detectionModel === 'api-detection' ||
                          isCustomApiUrl(currentStream.detectionModel)) && (
                          <div className="mt-3 space-y-3">
                            {/* Show info box only when using default endpoint */}
                            {currentStream.detectionModel === 'api-detection' && (
                              <div className="p-3 rounded-md bg-muted border border-border">
                                <p className="text-sm mb-2 text-muted-foreground">
                                  <strong className="text-foreground">ℹ️ {t('streamsConfig.usingDefaultApiEndpoint')}</strong>
                                </p>
                                <p className="text-xs font-mono px-2 py-1 rounded bg-background text-foreground">
                                  http://localhost:9001/detect
                                </p>
                                <p className="text-xs mt-2 text-muted-foreground">
                                  {t('streamsConfig.configuredInLightnvrIni')} <code className="px-1 rounded bg-background">[api_detection]</code>
                                </p>
                                <p className="text-xs mt-2 text-muted-foreground">
                                  ⚠️ {t('streamsConfig.ensureLightObjectDetectRunning')}
                                </p>
                              </div>
                            )}

                            {/* Show custom endpoint input when using custom URL */}
                            {isCustomApiUrl(currentStream.detectionModel) && (
                              <div className="space-y-2">
                                <div className="flex items-center justify-between">
                                  <label htmlFor="stream-custom-api-url" className="block text-sm font-medium">
                                    {t('streamsConfig.customApiEndpointUrl')}
                                  </label>
                                  <button
                                    type="button"
                                    onClick={() => {
                                      // Switch back to default API detection
                                      const event = {
                                        target: {
                                          name: 'detectionModel',
                                          value: 'api-detection'
                                        }
                                      };
                                      onInputChange(event);
                                    }}
                                    className="text-xs text-muted-foreground hover:text-foreground transition-colors"
                                  >
                                    {t('streamsConfig.useDefaultEndpoint')}
                                  </button>
                                </div>
                                <input
                                  type="text"
                                  id="stream-custom-api-url"
                                  name="detectionModel"
                                  className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground font-mono text-sm"
                                  placeholder={t('streamsConfig.customApiEndpointPlaceholder')}
                                  value={currentStream.detectionModel}
                                  onChange={onInputChange}
                                />
                                <p className="text-xs text-muted-foreground">
                                  {t('streamsConfig.customApiEndpointHelp')}
                                </p>
                              </div>
                            )}

                            {/* Override button - show only when using default endpoint */}
                            {currentStream.detectionModel === 'api-detection' && (
                              <button
                                type="button"
                                onClick={() => {
                                  // Switch to custom API mode by setting a placeholder URL
                                  const event = {
                                    target: {
                                      name: 'detectionModel',
                                      value: 'http://'
                                    }
                                  };
                                  onInputChange(event);
                                }}
                                className="w-full px-3 py-2 bg-secondary hover:bg-secondary/80 text-secondary-foreground rounded-md text-sm font-medium transition-colors"
                              >
                                {t('streamsConfig.overrideWithCustomEndpoint')}
                              </button>
                            )}
                          </div>
                        )}
                      </div>

                      <div>
                        <label htmlFor="stream-detection-threshold" className="block text-sm font-medium mb-2">
                          {t('streamsConfig.detectionThreshold')}: <span className="text-primary font-semibold">{currentStream.detectionThreshold}%</span>
                        </label>
                        <input
                          type="range"
                          id="stream-detection-threshold"
                          name="detectionThreshold"
                          className="w-full h-2 bg-muted rounded-lg appearance-none cursor-pointer"
                          min="0"
                          max="100"
                          step="1"
                          value={currentStream.detectionThreshold}
                          onInput={onThresholdChange}
                        />
                        <p className="mt-1 text-xs text-muted-foreground">
                          {t('streamsConfig.detectionThresholdHelp')}
                        </p>
                      </div>

                      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                        <div>
                          <label htmlFor="stream-detection-interval" className="block text-sm font-medium mb-2">
                            {t('streamsConfig.detectionIntervalFrames')}
                          </label>
                          <input
                            type="number"
                            id="stream-detection-interval"
                            name="detectionInterval"
                            className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                            min="1"
                            max="100"
                            value={currentStream.detectionInterval}
                            onChange={onInputChange}
                          />
                          <p className="mt-1 text-xs text-muted-foreground">{t('streamsConfig.detectionIntervalHelp')}</p>
                        </div>

                        <div>
                          <label htmlFor="stream-pre-buffer" className="block text-sm font-medium mb-2">
                            {t('streamsConfig.preDetectionBufferSec')}
                          </label>
                          <input
                            type="number"
                            id="stream-pre-buffer"
                            name="preBuffer"
                            className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                            min="0"
                            max="60"
                            value={currentStream.preBuffer}
                            onChange={onInputChange}
                          />
                          <p className="mt-1 text-xs text-muted-foreground">{t('streamsConfig.preDetectionBufferHelp')}</p>
                        </div>

                        <div>
                          <label htmlFor="stream-post-buffer" className="block text-sm font-medium mb-2">
                            {t('streamsConfig.postDetectionBufferSec')}
                          </label>
                          <input
                            type="number"
                            id="stream-post-buffer"
                            name="postBuffer"
                            className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                            min="0"
                            max="300"
                            value={currentStream.postBuffer}
                            onChange={onInputChange}
                          />
                          <p className="mt-1 text-xs text-muted-foreground">{t('streamsConfig.postDetectionBufferHelp')}</p>
                        </div>
                      </div>

                      {/* Detection Object Filter */}
                      <div>
                        <label htmlFor="stream-detection-object-filter" className="block text-sm font-medium mb-2">
                          {t('streamsConfig.objectFilterMode')}
                        </label>
                        <select
                          id="stream-detection-object-filter"
                          name="detectionObjectFilter"
                          className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                          value={currentStream.detectionObjectFilter || 'none'}
                          onChange={onInputChange}
                        >
                          <option value="none">{t('streamsConfig.objectFilterNone')}</option>
                          <option value="include">{t('streamsConfig.objectFilterInclude')}</option>
                          <option value="exclude">{t('streamsConfig.objectFilterExclude')}</option>
                        </select>
                        <p className="mt-1 text-xs text-muted-foreground">
                          {t('streamsConfig.objectFilterModeHelp')}
                        </p>
                      </div>

                      {currentStream.detectionObjectFilter && currentStream.detectionObjectFilter !== 'none' && (
                        <div>
                          <label htmlFor="stream-detection-object-filter-list" className="block text-sm font-medium mb-2">
                            {currentStream.detectionObjectFilter === 'include' ? t('streamsConfig.includeObjects') : t('streamsConfig.excludeObjects')}
                          </label>
                          <input
                            type="text"
                            id="stream-detection-object-filter-list"
                            name="detectionObjectFilterList"
                            className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                            placeholder={t('streamsConfig.objectFilterListPlaceholder')}
                            value={currentStream.detectionObjectFilterList || ''}
                            onChange={onInputChange}
                          />
                          <p className="mt-1 text-xs text-muted-foreground">
                            {currentStream.detectionObjectFilter === 'include'
                              ? t('streamsConfig.includeObjectsHelp')
                              : t('streamsConfig.excludeObjectsHelp')}
                          </p>
                        </div>
                      )}
                    </div>
                  </div>
                )}

                {/* Retention Policy Settings */}
                <div className="border-t border-border pt-4">
                  <h5 className="text-sm font-semibold mb-3">{t('streamsConfig.retentionPolicy')}</h5>
                  <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                    <div>
                      <label htmlFor="retention-days" className="block text-sm font-medium mb-2">
                        {t('streamsConfig.retentionDays')}
                      </label>
                      <input
                        type="number"
                        id="retention-days"
                        name="retentionDays"
                        className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                        min="0"
                        max="365"
                        value={currentStream.retentionDays || 0}
                        onChange={onInputChange}
                      />
                      <p className="mt-1 text-xs text-muted-foreground">{t('streamsConfig.retentionDaysHelp')}</p>
                    </div>
                    <div>
                      <label htmlFor="detection-retention-days" className="block text-sm font-medium mb-2">
                        {t('streamsConfig.detectionRetentionDays')}
                      </label>
                      <input
                        type="number"
                        id="detection-retention-days"
                        name="detectionRetentionDays"
                        className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                        min="0"
                        max="365"
                        value={currentStream.detectionRetentionDays || 0}
                        onChange={onInputChange}
                      />
                      <p className="mt-1 text-xs text-muted-foreground">{t('streamsConfig.detectionRetentionDaysHelp')}</p>
                    </div>
                    <div>
                      <label htmlFor="max-storage-mb" className="block text-sm font-medium mb-2">
                        {t('streamsConfig.maxStorageMb')}
                      </label>
                      <input
                        type="number"
                        id="max-storage-mb"
                        name="maxStorageMb"
                        className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                        min="0"
                        step="100"
                        value={currentStream.maxStorageMb || 0}
                        onChange={onInputChange}
                      />
                      <p className="mt-1 text-xs text-muted-foreground">{t('streamsConfig.maxStorageMbHelp')}</p>
                    </div>
                  </div>
                </div>
              </div>
            </AccordionSection>

            {/* Detection Zones Section — hidden when cloning (stream doesn't exist yet, so zones can't be saved) */}
            {currentStream.detectionEnabled && !isCloning && (
              <AccordionSection
                title={t('streamsConfig.detectionZones')}
                isExpanded={expandedSections.zones}
                onToggle={() => onToggleSection('zones')}
                badge={t('streamsConfig.optional')}
              >
                <div className="space-y-4">
                  <p className="text-sm text-muted-foreground">
                    {t('streamsConfig.detectionZonesHelp')}
                  </p>

                  <div className="flex items-center justify-between p-4 bg-muted rounded-lg">
                    <div>
                      <p className="font-medium">
                        {detectionZones.length === 0 ? t('streamsConfig.noZonesConfigured') : t('streamsConfig.zonesConfigured', { count: detectionZones.length })}
                      </p>
                      {detectionZones.length > 0 && (
                        <p className="text-sm text-muted-foreground mt-1">
                          {t('streamsConfig.enabledZonesCount', { count: detectionZones.filter(z => z.enabled).length })}
                        </p>
                      )}
                    </div>
                    <button
                      type="button"
                      onClick={() => setShowZoneEditor(true)}
                      className="btn-primary"
                    >
                      {detectionZones.length === 0 ? t('streamsConfig.configureZones') : t('streamsConfig.editZones')}
                    </button>
                  </div>

                  {detectionZones.length > 0 && (
                    <div className="space-y-2">
                      <p className="text-sm font-medium">{t('streamsConfig.configuredZones')}:</p>
                      {detectionZones.map((zone) => (
                        <div
                          key={zone.id}
                          className="flex items-center justify-between p-3 bg-background border border-border rounded"
                        >
                          <div className="flex items-center space-x-3">
                            <div
                              className="w-4 h-4 rounded"
                              style={{ backgroundColor: zone.color || 'hsl(var(--primary))' }}
                            />
                            <span className="font-medium">{zone.name}</span>
                            <span className="text-sm text-muted-foreground">
                              ({zone.polygon.length} points)
                            </span>
                          </div>
                          <span className={`text-sm px-2 py-1 rounded ${zone.enabled ? 'bg-[hsl(var(--success-muted))] text-[hsl(var(--success-muted-foreground))]' : 'bg-muted text-muted-foreground'}`}>
                            {zone.enabled ? t('common.enabled') : t('common.disabled')}
                          </span>
                        </div>
                      ))}
                    </div>
                  )}

                  <div className="rounded-lg p-4 bg-[hsl(var(--info-muted))] border border-[hsl(var(--info)_/_0.3)]">
                    <div className="flex">
                      <svg className="w-5 h-5 mr-2 flex-shrink-0 text-[hsl(var(--info))]" fill="currentColor" viewBox="0 0 20 20">
                        <path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clipRule="evenodd" />
                      </svg>
                      <div className="text-sm text-[hsl(var(--info-muted-foreground))]">
                        <p className="font-medium mb-1">{t('streamsConfig.zoneDetectionTips')}:</p>
                        <ul className="list-disc list-inside space-y-1 text-xs">
                          <li>{t('streamsConfig.zoneTipConfigure')}</li>
                          <li>{t('streamsConfig.zoneTipDrawPolygons')}</li>
                          <li>{t('streamsConfig.zoneTipIgnoreAreas')}</li>
                          <li>{t('streamsConfig.zoneTipMultipleZones')}</li>
                        </ul>
                      </div>
                    </div>
                  </div>
                </div>
              </AccordionSection>
            )}

            {/* Motion Recording Section (ONVIF only) */}
            {currentStream.isOnvif && (
              <AccordionSection
                title={t('streamsConfig.motionRecordingOnvif')}
                isExpanded={expandedSections.motion}
                onToggle={() => onToggleSection('motion')}
                badge={t('streamsConfig.onvifOnly')}
              >
                <div className="space-y-4">
                  <div className="flex items-center">
                    <label className="flex items-center space-x-2 cursor-pointer">
                      <input
                        type="checkbox"
                        id="motion-recording-enabled"
                        name="motionRecordingEnabled"
                        className="h-4 w-4 rounded border-input"
                        style={primaryAccentStyle}
                        checked={currentStream.motionRecordingEnabled}
                        onChange={onInputChange}
                      />
                      <span className="text-sm font-medium">{t('streamsConfig.enableOnvifMotionRecording')}</span>
                    </label>
                  </div>
                  <p className="text-sm text-muted-foreground">
                    {t('streamsConfig.onvifMotionRecordingHelp')}
                  </p>

                  {currentStream.motionRecordingEnabled && (
                    <>
                      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div>
                          <label htmlFor="motion-pre-buffer" className="block text-sm font-medium mb-2">
                            {t('streamsConfig.preEventBufferSeconds')}
                          </label>
                          <input
                            type="number"
                            id="motion-pre-buffer"
                            name="motionPreBuffer"
                            className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                            min="0"
                            max="30"
                            value={currentStream.motionPreBuffer}
                            onChange={onInputChange}
                          />
                          <p className="mt-1 text-xs text-muted-foreground">{t('streamsConfig.preEventBufferHelp')}</p>
                        </div>

                        <div>
                          <label htmlFor="motion-post-buffer" className="block text-sm font-medium mb-2">
                            {t('streamsConfig.postEventBufferSeconds')}
                          </label>
                          <input
                            type="number"
                            id="motion-post-buffer"
                            name="motionPostBuffer"
                            className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                            min="0"
                            max="60"
                            value={currentStream.motionPostBuffer}
                            onChange={onInputChange}
                          />
                          <p className="mt-1 text-xs text-muted-foreground">{t('streamsConfig.postEventBufferHelp')}</p>
                        </div>

                        <div>
                          <label htmlFor="motion-max-duration" className="block text-sm font-medium mb-2">
                            {t('streamsConfig.maxFileDurationSeconds')}
                          </label>
                          <input
                            type="number"
                            id="motion-max-duration"
                            name="motionMaxDuration"
                            className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                            min="60"
                            max="3600"
                            step="60"
                            value={currentStream.motionMaxDuration}
                            onChange={onInputChange}
                          />
                          <p className="mt-1 text-xs text-muted-foreground">{t('streamsConfig.maxFileDurationHelp')}</p>
                        </div>

                        <div>
                          <label htmlFor="motion-retention-days" className="block text-sm font-medium mb-2">
                            {t('streamsConfig.retentionPeriodDays')}
                          </label>
                          <input
                            type="number"
                            id="motion-retention-days"
                            name="motionRetentionDays"
                            className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                            min="1"
                            max="365"
                            value={currentStream.motionRetentionDays}
                            onChange={onInputChange}
                          />
                          <p className="mt-1 text-xs text-muted-foreground">{t('streamsConfig.retentionPeriodHelp')}</p>
                        </div>

                        <div>
                          <label htmlFor="motion-codec" className="block text-sm font-medium mb-2">
                            {t('streams.codec')}
                          </label>
                          <select
                            id="motion-codec"
                            name="motionCodec"
                            className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                            value={currentStream.motionCodec}
                            onChange={onInputChange}
                          >
                            <option value="h264">H.264</option>
                            <option value="h265">H.265 (HEVC)</option>
                          </select>
                        </div>

                        <div>
                          <label htmlFor="motion-quality" className="block text-sm font-medium mb-2">
                            {t('streamsConfig.recordingQuality')}
                          </label>
                          <select
                            id="motion-quality"
                            name="motionQuality"
                            className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                            value={currentStream.motionQuality}
                            onChange={onInputChange}
                          >
                            <option value="low">{t('common.low')}</option>
                            <option value="medium">{t('common.medium')}</option>
                            <option value="high">{t('common.high')}</option>
                          </select>
                        </div>

                        <div className="flex justify-end pt-2">
                          <button
                            type="button"
                            onClick={onTestMotion}
                            className="px-4 py-2 bg-secondary text-secondary-foreground rounded-md hover:bg-secondary/80 transition-colors font-medium"
                          >
                            {t('streamsConfig.triggerTestMotionEvent')}
                          </button>
                        </div>

                      </div>
                    </>
                  )}
                </div>
              </AccordionSection>
            )}

            {/* PTZ Settings Section (ONVIF only) */}
            {currentStream.isOnvif && (
              <AccordionSection
                title={t('live.ptzControl')}
                isExpanded={expandedSections.ptz}
                onToggle={() => onToggleSection('ptz')}
                badge={t('streamsConfig.onvifOnly')}
              >
                <div className="space-y-4">
                  {/* Enable PTZ */}
                  <div className="flex items-center space-x-3">
                    <input
                      type="checkbox"
                      id="ptz-enabled"
                      name="ptzEnabled"
                      className="h-4 w-4 text-primary focus:ring-primary border-input rounded"
                      checked={currentStream.ptzEnabled || false}
                      onChange={onInputChange}
                    />
                    <label htmlFor="ptz-enabled" className="text-sm font-medium">
                      {t('streamsConfig.enablePtzControl')}
                    </label>
                  </div>
                  <p className="text-xs text-muted-foreground">
                    {t('streamsConfig.enablePtzControlHelp')}
                  </p>

                  {currentStream.ptzEnabled && (
                    <>
                      {/* PTZ Limits */}
                      <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mt-4">
                        <div>
                          <label htmlFor="ptz-max-x" className="block text-sm font-medium mb-2">
                            {t('streamsConfig.maxPanX')}
                          </label>
                          <input
                            type="number"
                            id="ptz-max-x"
                            name="ptzMaxX"
                            className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                            min="0"
                            value={currentStream.ptzMaxX || 0}
                            onChange={onInputChange}
                          />
                          <p className="mt-1 text-xs text-muted-foreground">{t('streamsConfig.zeroNoLimit')}</p>
                        </div>

                        <div>
                          <label htmlFor="ptz-max-y" className="block text-sm font-medium mb-2">
                            {t('streamsConfig.maxTiltY')}
                          </label>
                          <input
                            type="number"
                            id="ptz-max-y"
                            name="ptzMaxY"
                            className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                            min="0"
                            value={currentStream.ptzMaxY || 0}
                            onChange={onInputChange}
                          />
                          <p className="mt-1 text-xs text-muted-foreground">{t('streamsConfig.zeroNoLimit')}</p>
                        </div>

                        <div>
                          <label htmlFor="ptz-max-z" className="block text-sm font-medium mb-2">
                            {t('streamsConfig.maxZoomZ')}
                          </label>
                          <input
                            type="number"
                            id="ptz-max-z"
                            name="ptzMaxZ"
                            className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                            min="0"
                            value={currentStream.ptzMaxZ || 0}
                            onChange={onInputChange}
                          />
                          <p className="mt-1 text-xs text-muted-foreground">{t('streamsConfig.zeroNoLimit')}</p>
                        </div>
                      </div>

                      {/* Home Position Support */}
                      <div className="flex items-center space-x-3 mt-4">
                        <input
                          type="checkbox"
                          id="ptz-has-home"
                          name="ptzHasHome"
                          className="h-4 w-4 text-primary focus:ring-primary border-input rounded"
                          checked={currentStream.ptzHasHome || false}
                          onChange={onInputChange}
                        />
                        <label htmlFor="ptz-has-home" className="text-sm font-medium">
                          {t('streamsConfig.cameraSupportsHomePosition')}
                        </label>
                      </div>
                    </>
                  )}
                </div>
              </AccordionSection>
            )}

            {/* Advanced Settings Section */}
            <AccordionSection
              title={t('streamsConfig.advancedSettings')}
              isExpanded={expandedSections.advanced}
              onToggle={() => onToggleSection('advanced')}
            >
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                  <label htmlFor="stream-priority" className="block text-sm font-medium mb-2">
                    {t('streamsConfig.streamPriority')}
                  </label>
                  <select
                    id="stream-priority"
                    name="priority"
                    className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                    value={currentStream.priority}
                    onChange={onInputChange}
                  >
                    <option value="1">{t('streamsConfig.priorityLow')}</option>
                    <option value="5">{t('streamsConfig.priorityMedium')}</option>
                    <option value="10">{t('streamsConfig.priorityHigh')}</option>
                  </select>
                  <p className="mt-1 text-xs text-muted-foreground">{t('streamsConfig.streamPriorityHelp')}</p>
                </div>

                <div>
                  <label htmlFor="stream-segment" className="block text-sm font-medium mb-2">
                    {t('streamsConfig.segmentDurationSeconds')}
                  </label>
                  <input
                    type="number"
                    id="stream-segment"
                    name="segment"
                    className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                    min="60"
                    max="3600"
                    value={currentStream.segment}
                    onChange={onInputChange}
                  />
                  <p className="mt-1 text-xs text-muted-foreground">{t('streamsConfig.segmentDurationHelp')}</p>
                </div>

                <div className="md:col-span-2">
                  <label htmlFor="motion-trigger-source" className="block text-sm font-medium mb-2">
                    {t('streamsConfig.motionTriggerSource')}
                  </label>
                  <input
                    type="text"
                    id="motion-trigger-source"
                    name="motionTriggerSource"
                    className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                    placeholder={t('streamsConfig.motionTriggerSourcePlaceholder')}
                    value={currentStream.motionTriggerSource || ''}
                    onChange={onInputChange}
                  />
                  <p className="mt-1 text-xs text-muted-foreground">{t('streamsConfig.motionTriggerSourceHelp')}</p>
                </div>
              </div>
            </AccordionSection>

            {/* go2rtc Source Override Section */}
            <AccordionSection
              title={t('streamsConfig.go2rtcSourceOverride')}
              isExpanded={expandedSections.go2rtcOverride}
              onToggle={() => onToggleSection('go2rtcOverride')}
            >
              <div>
                <label htmlFor="go2rtc-source-override" className="block text-sm font-medium mb-2">
                  {t('streamsConfig.go2rtcSourceOverrideLabel')}
                </label>
                <textarea
                  id="go2rtc-source-override"
                  name="go2rtcSourceOverride"
                  className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground font-mono text-sm"
                  rows="4"
                  placeholder={"rtsp://admin:pass@camera/stream#transport=tcp\n\nOr multi-source:\n- rtsp://admin:pass@camera/main#transport=tcp\n- ffmpeg:camera1#video=h264#hardware"}
                  value={currentStream.go2rtcSourceOverride || ''}
                  onChange={onInputChange}
                />
                <p className="mt-1 text-xs text-muted-foreground">
                  {t('streamsConfig.go2rtcSourceOverrideHelp')}
                </p>
              </div>
            </AccordionSection>

          </form>
        </div>

        {/* Footer */}
        <div className="flex justify-between items-center p-6 border-t border-border flex-shrink-0 bg-muted/20">
          <button
            id="stream-test-btn"
            type="button"
            onClick={onTestConnection}
            className="px-4 py-2 bg-secondary text-secondary-foreground rounded-md hover:bg-secondary/80 transition-colors font-medium"
          >
            {t('streamsConfig.testConnection')}
          </button>
          <div className="flex space-x-3">
            <button
              id="stream-cancel-btn"
              type="button"
              onClick={onClose}
              className="px-6 py-2 border border-input rounded-md shadow-sm text-sm font-medium text-foreground bg-background hover:bg-muted transition-colors"
            >
              {t('common.cancel')}
            </button>
            <button
              id="stream-save-btn"
              type="button"
              onClick={onSave}
              className="px-6 py-2 btn-primary font-medium"
            >
              {isEditing ? t('streamsConfig.updateStream') : t('streams.addStream')}
            </button>
          </div>
        </div>
      </div>
    </div>
    </>
  );
}

