/**
 * Stream Configuration Modal Component
 * Expanded, responsive modal with accordion sections for stream configuration
 */

import { useState, useEffect, useRef, useCallback } from 'react';
import { ZoneEditor } from './ZoneEditor.jsx';
import { obfuscateUrlCredentials } from '../../utils/url-utils.js';

/**
 * Recording Schedule Grid Component
 * Interactive 7-day × 24-hour weekly grid for scheduling continuous recording
 */
function RecordingScheduleGrid({ schedule, onChange }) {
  const isDragging = useRef(false);
  const dragValue = useRef(true);

  const DAYS = ['Sun', 'Mon', 'Tue', 'Wed', 'Thu', 'Fri', 'Sat'];
  const HOURS = Array.from({ length: 24 }, (_, i) => {
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
    const newVal = !schedule[dayIdx * 24 + hourIdx];
    dragValue.current = newVal;
    isDragging.current = true;
    const s = schedule.slice();
    s[dayIdx * 24 + hourIdx] = newVal;
    onChange(s);
  }, [schedule, onChange]);

  const handleCellMouseEnter = useCallback((dayIdx, hourIdx) => {
    if (!isDragging.current) return;
    const s = schedule.slice();
    s[dayIdx * 24 + hourIdx] = dragValue.current;
    onChange(s);
  }, [schedule, onChange]);

  const toggleDay = useCallback((dayIdx) => {
    const allOn = HOURS.every((_, h) => schedule[dayIdx * 24 + h]);
    const s = schedule.slice();
    for (let h = 0; h < 24; h++) s[dayIdx * 24 + h] = !allOn;
    onChange(s);
  }, [schedule, onChange]);

  const toggleHour = useCallback((hourIdx) => {
    const allOn = DAYS.every((_, d) => schedule[d * 24 + hourIdx]);
    const s = schedule.slice();
    for (let d = 0; d < 7; d++) s[d * 24 + hourIdx] = !allOn;
    onChange(s);
  }, [schedule, onChange]);

  /** Build the boolean pattern array for a given preset key */
  const getPresetPattern = useCallback((preset) => {
    const p = Array(168).fill(false);
    if (preset === 'weekdays') {
      for (let d = 1; d <= 5; d++) for (let h = 0; h < 24; h++) p[d * 24 + h] = true;
    } else if (preset === 'business') {
      for (let d = 1; d <= 5; d++) for (let h = 8; h < 18; h++) p[d * 24 + h] = true;
    } else if (preset === 'nights') {
      for (let d = 0; d < 7; d++) {
        for (let h = 0; h < 6; h++) p[d * 24 + h] = true;
        for (let h = 20; h < 24; h++) p[d * 24 + h] = true;
      }
    } else if (preset === 'weekends') {
      for (const d of [0, 6]) for (let h = 0; h < 24; h++) p[d * 24 + h] = true;
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
    if (preset === 'all') { onChange(Array(168).fill(true)); return; }
    if (preset === 'none') { onChange(Array(168).fill(false)); return; }

    // Toggleable presets — OR on / remove off
    const pattern = getPresetPattern(preset);
    const active = isPresetActive(preset);
    const s = schedule.slice();
    for (let i = 0; i < 168; i++) {
      if (pattern[i]) s[i] = !active;
    }
    onChange(s);
  }, [schedule, onChange, getPresetPattern, isPresetActive]);

  const scheduledHours = schedule.filter(Boolean).length;
  const PRESETS = [
    { key: 'all', label: '24/7' },
    { key: 'weekdays', label: 'Weekdays' },
    { key: 'business', label: 'Business Hours' },
    { key: 'nights', label: 'Nights' },
    { key: 'weekends', label: 'Weekends' },
    { key: 'none', label: 'Clear All' },
  ];

  return (
    <div className="space-y-2 select-none" style={{ userSelect: 'none' }}>
      {/* Presets & counter */}
      <div className="flex flex-wrap items-center gap-1.5">
        <span className="text-xs text-muted-foreground font-medium">Quick select:</span>
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
          {scheduledHours}h<span className="text-muted-foreground font-normal">/week</span>
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
                title={`Toggle all ${day}`}
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
                title={`Toggle ${hourLabel} for all days`}
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
                const isActive = !!schedule[dayIdx * 24 + hourIdx];
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
              <span className="text-[10px] text-muted-foreground">No recording</span>
            </div>
            <div className="flex items-center gap-1">
              <div className="w-3 h-2.5 rounded-sm bg-primary/70" />
              <span className="text-[10px] text-muted-foreground">Recording active</span>
            </div>
            <span className="text-[10px] text-muted-foreground italic">Click headers or drag cells</span>
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
  const [showZoneEditor, setShowZoneEditor] = useState(false);
  const [detectionZones, setDetectionZones] = useState(currentStream.detectionZones || []);

  // Keep a ref to onInputChange so the zones-load effect never needs it as a
  // dependency (avoids an infinite fetch loop: calling onInputChange triggers a
  // parent re-render which produces a new function reference → effect re-fires).
  const onInputChangeRef = useRef(onInputChange);
  onInputChangeRef.current = onInputChange;

  // Load zones from API when modal opens for existing stream.
  // Depends only on isEditing / currentStream.name – NOT on onInputChange.
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
            console.log('Loaded zones for stream config modal:', data.zones);
            setDetectionZones(data.zones);
            // Use ref so we always call the latest callback without making it
            // a dependency of this effect.
            onInputChangeRef.current({ target: { name: 'detectionZones', value: data.zones } });
          }
        } else {
          console.warn('Failed to load zones:', response.status);
        }
      } catch (error) {
        console.error('Error loading zones:', error);
      }
    };

    loadZones();
  }, [isEditing, currentStream.name]); // eslint-disable-line react-hooks/exhaustive-deps

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
    <div id="stream-config-modal" className="fixed inset-0 bg-black bg-opacity-50 flex items-center justify-center z-50 p-4">
      <div className="bg-card text-card-foreground rounded-lg shadow-xl w-full max-w-5xl max-h-[95vh] overflow-hidden flex flex-col">
        {/* Header */}
        <div className="flex justify-between items-center p-6 border-b border-border flex-shrink-0">
          <div>
            <h3 className="text-2xl font-bold">{isEditing ? 'Edit Stream' : 'Add Stream'}</h3>
            <p className="text-sm text-muted-foreground mt-1">
              Configure stream settings, recording options, and detection parameters
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
              title="Basic Settings"
              isExpanded={expandedSections.basic}
              onToggle={() => onToggleSection('basic')}
            >
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div className="md:col-span-2">
                  <label htmlFor="stream-name" className="block text-sm font-medium mb-2">
                    Stream Name <span className="text-danger">*</span>
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
                    placeholder="e.g., front_door, backyard"
                  />
                  {isEditing && (
                    <p className="mt-1 text-xs text-muted-foreground">Stream name cannot be changed after creation</p>
                  )}
                </div>

                <div className="md:col-span-2">
                  <label htmlFor="stream-url" className="block text-sm font-medium mb-2">
                    Stream URL <span className="text-danger">*</span>
                  </label>
                  <input
                    type="text"
                    id="stream-url"
                    name="url"
                    className={`w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground ${hideCredentials ? 'cursor-not-allowed opacity-75' : ''}`}
                    placeholder="rtsp://username:password@192.168.1.100:554/stream"
                    value={hideCredentials ? obfuscateUrlCredentials(currentStream.url) : currentStream.url}
                    onChange={onInputChange}
                    required
                    readOnly={hideCredentials}
                    title={hideCredentials ? 'URL credentials are hidden in demo/viewer mode' : ''}
                  />
                  <p className="mt-1 text-xs text-muted-foreground">
                    {hideCredentials ? 'URL credentials hidden for security' : 'RTSP URL for the camera stream'}
                  </p>
                </div>

                <div className="flex items-center space-x-6">
                  <label className="flex items-center space-x-2 cursor-pointer">
                    <input
                      type="checkbox"
                      id="stream-enabled"
                      name="enabled"
                      className="h-4 w-4 rounded border-gray-300"
                      style={{accentColor: 'hsl(var(--primary))'}}
                      checked={currentStream.enabled}
                      onChange={onInputChange}
                    />
                    <span className="text-sm font-medium">Stream Active</span>
                  </label>
                  <label className="flex items-center space-x-2 cursor-pointer">
                    <input
                      type="checkbox"
                      id="stream-streaming-enabled"
                      name="streamingEnabled"
                      className="h-4 w-4 rounded border-gray-300"
                      style={{accentColor: 'hsl(var(--primary))'}}
                      checked={currentStream.streamingEnabled}
                      onChange={onInputChange}
                    />
                    <span className="text-sm font-medium">Live View Enabled</span>
                  </label>
                </div>

                <div className="flex items-center">
                  <label className="flex items-center space-x-2 cursor-pointer">
                    <input
                      type="checkbox"
                      id="stream-is-onvif"
                      name="isOnvif"
                      className="h-4 w-4 rounded border-gray-300"
                      style={{accentColor: 'hsl(var(--primary))'}}
                      checked={currentStream.isOnvif}
                      onChange={onInputChange}
                    />
                    <span className="text-sm font-medium">ONVIF Camera</span>
                  </label>
                  <span className="ml-2 text-xs text-muted-foreground">Enables motion detection and PTZ features</span>
                </div>

                {/* ONVIF Credentials - shown when ONVIF Camera is enabled */}
                {currentStream.isOnvif && (
                  <div className="col-span-2 p-4 bg-muted/50 rounded-lg border border-border">
                    <h4 className="text-sm font-medium mb-3">ONVIF Credentials</h4>
                    <p className="text-xs text-muted-foreground mb-3">
                      Enter credentials for ONVIF features (motion detection, PTZ control). Leave empty if your camera doesn't require authentication.
                    </p>
                    <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                      <div>
                        <label htmlFor="stream-onvif-username" className="block text-sm font-medium mb-1">Username</label>
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
                        <label htmlFor="stream-onvif-password" className="block text-sm font-medium mb-1">Password</label>
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
                        <label htmlFor="stream-onvif-profile" className="block text-sm font-medium mb-1">Profile (optional)</label>
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
                    </div>
                  </div>
                )}

                <div>
                  <label className="block text-sm font-medium mb-2">Resolution</label>
                  <span className="block w-full px-3 py-2 border border-input rounded-md bg-muted/30 text-muted-foreground">
                    {currentStream.width > 0 && currentStream.height > 0
                      ? `${currentStream.width}×${currentStream.height}`
                      : 'Auto-detected'}
                  </span>
                  <span className="text-xs text-muted-foreground mt-1 block">Detected from stream source</span>
                </div>

                <div>
                  <label className="block text-sm font-medium mb-2">FPS</label>
                  <span className="block w-full px-3 py-2 border border-input rounded-md bg-muted/30 text-muted-foreground">
                    {currentStream.fps > 0 ? currentStream.fps : 'Auto-detected'}
                  </span>
                  <span className="text-xs text-muted-foreground mt-1 block">Detected from stream source</span>
                </div>

                <div>
                  <label className="block text-sm font-medium mb-2">Codec</label>
                  <span className="block w-full px-3 py-2 border border-input rounded-md bg-muted/30 text-muted-foreground">
                    {currentStream.codec ? currentStream.codec.toUpperCase() : 'Auto-detected'}
                  </span>
                  <span className="text-xs text-muted-foreground mt-1 block">Detected from stream source</span>
                </div>

                <div>
                  <label htmlFor="stream-protocol" className="block text-sm font-medium mb-2">Protocol</label>
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
              title="Recording Settings"
              isExpanded={expandedSections.recording}
              onToggle={() => onToggleSection('recording')}
            >
              <div className="space-y-4">
                {/* Recording Mode Selection */}
                <div className="space-y-3">
                  <h5 className="text-sm font-semibold">Recording Mode</h5>
                  <div className="space-y-2">
                    {/* Continuous Recording block (with optional schedule sub-option) */}
                    <div className={`rounded-lg border transition-colors ${currentStream.record ? 'border-primary/40 bg-primary/5' : 'border-border'}`}>
                      <label className="flex items-start space-x-3 cursor-pointer p-3 hover:bg-muted/30 transition-colors rounded-lg">
                        <input
                          type="checkbox"
                          id="stream-record"
                          name="record"
                          className="h-4 w-4 mt-0.5 rounded border-gray-300"
                          style={{accentColor: 'hsl(var(--primary))'}}
                          checked={currentStream.record}
                          onChange={onInputChange}
                        />
                        <div>
                          <span className="text-sm font-medium">Enable Continuous Recording</span>
                          <p className="text-xs text-muted-foreground mt-1">
                            {currentStream.record && currentStream.recordOnSchedule
                              ? 'Recording only during configured schedule windows'
                              : 'Record all video continuously to disk'}
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
                              className="h-3.5 w-3.5 rounded border-gray-300"
                              style={{accentColor: 'hsl(var(--primary))'}}
                              checked={currentStream.recordOnSchedule || false}
                              onChange={onInputChange}
                            />
                            <span className="text-xs font-semibold group-hover:text-primary transition-colors">
                              📅 On a Schedule
                            </span>
                            <span className="text-xs text-muted-foreground">— restrict recording to specific days &amp; hours</span>
                          </label>

                          {currentStream.recordOnSchedule && (
                            <div className="rounded-xl border border-primary/25 bg-background p-3 shadow-sm">
                              <p className="text-xs text-muted-foreground mb-3 flex items-center gap-1.5">
                                <span className="text-primary">●</span>
                                Click day/hour headers to toggle columns or rows. Drag across cells to paint a range.
                              </p>
                              <RecordingScheduleGrid
                                schedule={currentStream.recordingSchedule && currentStream.recordingSchedule.length === 168
                                  ? currentStream.recordingSchedule
                                  : Array(168).fill(true)}
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
                        className="h-4 w-4 mt-0.5 rounded border-gray-300"
                        style={{accentColor: 'hsl(var(--primary))'}}
                        checked={currentStream.detectionEnabled}
                        onChange={onInputChange}
                      />
                      <div>
                        <span className="text-sm font-medium">Enable AI Detection-Based Recording</span>
                        <p className="text-xs text-muted-foreground mt-1">
                          Only record when AI detects objects (person, car, etc.)
                        </p>
                      </div>
                    </label>
                  </div>

                  {/* Show info box based on recording mode selection */}
                  {currentStream.record && currentStream.detectionEnabled && (
                    <div className="p-3 rounded-md bg-muted border border-border">
                      <p className="text-sm text-muted-foreground">
                        <strong className="text-foreground">{'\u2139\uFE0F Both modes enabled:'}</strong> Continuous recording will run, and detection events will be logged and associated with the recordings.
                      </p>
                    </div>
                  )}
                  {!currentStream.record && currentStream.detectionEnabled && (
                    <div className="p-3 rounded-md bg-muted border border-border">
                      <p className="text-sm text-muted-foreground">
                        <strong className="text-foreground">{'\u26A1 Detection-only mode:'}</strong> Video will only be saved when detections occur, with pre and post buffers.
                      </p>
                    </div>
                  )}
                  {!currentStream.record && !currentStream.detectionEnabled && (
                    <div className="p-3 bg-muted border border-border rounded-md">
                      <p className="text-sm text-muted-foreground">
                        {'\u26A0\uFE0F No recording mode selected. Video will not be saved to disk.'}
                      </p>
                    </div>
                  )}
                </div>

                {/* Audio Settings */}
                <div className="border-t border-border pt-4">
                  <h5 className="text-sm font-semibold mb-3">Audio Settings</h5>
                  <div className="flex items-center space-x-4">
                    <label className="flex items-center space-x-2 cursor-pointer">
                      <input
                        type="checkbox"
                        id="stream-record-audio"
                        name="recordAudio"
                        className="h-4 w-4 rounded border-gray-300"
                        style={{accentColor: 'hsl(var(--primary))'}}
                        checked={currentStream.recordAudio}
                        onChange={onInputChange}
                      />
                      <span className="text-sm font-medium">Record Audio</span>
                    </label>
                    <label className="flex items-center space-x-2 cursor-pointer">
                      <input
                        type="checkbox"
                        id="stream-backchannel-enabled"
                        name="backchannelEnabled"
                        className="h-4 w-4 rounded border-gray-300"
                        style={{accentColor: 'hsl(var(--primary))'}}
                        checked={currentStream.backchannelEnabled}
                        onChange={onInputChange}
                      />
                      <span className="text-sm font-medium">Two-Way Audio</span>
                    </label>
                  </div>
                  <p className="text-sm text-muted-foreground mt-2">
                    Audio recording applies to both continuous and detection-based recordings (requires audio track in stream).
                  </p>
                </div>

                {/* AI Detection Settings - nested under recording */}
                {currentStream.detectionEnabled && (
                  <div className="border-t border-border pt-4">
                    <h5 className="text-sm font-semibold mb-3">AI Detection Settings</h5>
                    <div className="space-y-4">
                      <div>
                        <label htmlFor="stream-detection-model" className="block text-sm font-medium mb-2">
                          Detection Model
                        </label>
                        <div className="flex space-x-2">
                          <select
                            id="stream-detection-model"
                            name="detectionModel"
                            className="flex-1 px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                            value={currentStream.detectionModel && (currentStream.detectionModel.startsWith('http://') || currentStream.detectionModel.startsWith('https://')) ? 'api-detection' : currentStream.detectionModel}
                            onChange={onInputChange}
                          >
                            <option value="">Select a model</option>
                            {detectionModels.map(model => (
                              <option key={model.id} value={model.id}>{model.name}</option>
                            ))}
                          </select>
                          <button
                            type="button"
                            onClick={onRefreshModels}
                            className="p-2 rounded-md bg-secondary hover:bg-secondary/80 text-secondary-foreground focus:outline-none"
                            title="Refresh Models"
                          >
                            <svg className="w-5 h-5" fill="currentColor" viewBox="0 0 20 20">
                              <path fillRule="evenodd" d="M4 2a1 1 0 011 1v2.101a7.002 7.002 0 0111.601 2.566 1 1 0 11-1.885.666A5.002 5.002 0 005.999 7H9a1 1 0 010 2H4a1 1 0 01-1-1V3a1 1 0 011-1zm.008 9.057a1 1 0 011.276.61A5.002 5.002 0 0014.001 13H11a1 1 0 110-2h5a1 1 0 011 1v5a1 1 0 11-2 0v-2.101a7.002 7.002 0 01-11.601-2.566 1 1 0 01.61-1.276z" clipRule="evenodd" />
                            </svg>
                          </button>
                        </div>

                        {/* Show info box and custom endpoint option for API detection */}
                        {(currentStream.detectionModel === 'api-detection' ||
                          (currentStream.detectionModel && (currentStream.detectionModel.startsWith('http://') || currentStream.detectionModel.startsWith('https://')))) && (
                          <div className="mt-3 space-y-3">
                            {/* Show info box only when using default endpoint */}
                            {currentStream.detectionModel === 'api-detection' && (
                              <div className="p-3 rounded-md bg-muted border border-border">
                                <p className="text-sm mb-2 text-muted-foreground">
                                  <strong className="text-foreground">ℹ️ Using Default API Endpoint:</strong>
                                </p>
                                <p className="text-xs font-mono px-2 py-1 rounded bg-background text-foreground">
                                  http://localhost:9001/detect
                                </p>
                                <p className="text-xs mt-2 text-muted-foreground">
                                  Configured in <code className="px-1 rounded bg-background">lightnvr.ini</code> under <code className="px-1 rounded bg-background">[api_detection]</code>
                                </p>
                                <p className="text-xs mt-2 text-muted-foreground">
                                  ⚠️ Make sure light-object-detect is running on this endpoint
                                </p>
                              </div>
                            )}

                            {/* Show custom endpoint input when using custom URL */}
                            {currentStream.detectionModel && (currentStream.detectionModel.startsWith('http://') || currentStream.detectionModel.startsWith('https://')) && (
                              <div className="space-y-2">
                                <div className="flex items-center justify-between">
                                  <label htmlFor="stream-custom-api-url" className="block text-sm font-medium">
                                    Custom API Endpoint URL
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
                                    Use Default Endpoint
                                  </button>
                                </div>
                                <input
                                  type="text"
                                  id="stream-custom-api-url"
                                  name="detectionModel"
                                  className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground font-mono text-sm"
                                  placeholder="http://192.168.1.100:9001/detect"
                                  value={currentStream.detectionModel}
                                  onChange={onInputChange}
                                />
                                <p className="text-xs text-muted-foreground">
                                  Enter the full URL to your custom detection API endpoint
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
                                Override with Custom Endpoint
                              </button>
                            )}
                          </div>
                        )}
                      </div>

                      <div>
                        <label htmlFor="stream-detection-threshold" className="block text-sm font-medium mb-2">
                          Detection Threshold: <span className="text-primary font-semibold">{currentStream.detectionThreshold}%</span>
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
                          Minimum confidence level to trigger recording (0-100%)
                        </p>
                      </div>

                      <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                        <div>
                          <label htmlFor="stream-detection-interval" className="block text-sm font-medium mb-2">
                            Detection Interval (frames)
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
                          <p className="mt-1 text-xs text-muted-foreground">Run detection every N frames</p>
                        </div>

                        <div>
                          <label htmlFor="stream-pre-buffer" className="block text-sm font-medium mb-2">
                            Pre-Detection Buffer (sec)
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
                          <p className="mt-1 text-xs text-muted-foreground">Seconds before detection</p>
                        </div>

                        <div>
                          <label htmlFor="stream-post-buffer" className="block text-sm font-medium mb-2">
                            Post-Detection Buffer (sec)
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
                          <p className="mt-1 text-xs text-muted-foreground">Seconds after detection</p>
                        </div>
                      </div>

                      {/* Detection Object Filter */}
                      <div>
                        <label htmlFor="stream-detection-object-filter" className="block text-sm font-medium mb-2">
                          Object Filter Mode
                        </label>
                        <select
                          id="stream-detection-object-filter"
                          name="detectionObjectFilter"
                          className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                          value={currentStream.detectionObjectFilter || 'none'}
                          onChange={onInputChange}
                        >
                          <option value="none">None (detect all objects)</option>
                          <option value="include">Include only (whitelist)</option>
                          <option value="exclude">Exclude (blacklist)</option>
                        </select>
                        <p className="mt-1 text-xs text-muted-foreground">
                          Filter which object types trigger detections for this stream
                        </p>
                      </div>

                      {currentStream.detectionObjectFilter && currentStream.detectionObjectFilter !== 'none' && (
                        <div>
                          <label htmlFor="stream-detection-object-filter-list" className="block text-sm font-medium mb-2">
                            {currentStream.detectionObjectFilter === 'include' ? 'Include Objects' : 'Exclude Objects'}
                          </label>
                          <input
                            type="text"
                            id="stream-detection-object-filter-list"
                            name="detectionObjectFilterList"
                            className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                            placeholder="person, car, bicycle, dog"
                            value={currentStream.detectionObjectFilterList || ''}
                            onChange={onInputChange}
                          />
                          <p className="mt-1 text-xs text-muted-foreground">
                            {currentStream.detectionObjectFilter === 'include'
                              ? 'Comma-separated list of object classes to detect (all others ignored)'
                              : 'Comma-separated list of object classes to ignore (all others detected)'}
                          </p>
                        </div>
                      )}
                    </div>
                  </div>
                )}

                {/* Retention Policy Settings */}
                <div className="border-t border-border pt-4">
                  <h5 className="text-sm font-semibold mb-3">Retention Policy</h5>
                  <div className="grid grid-cols-1 md:grid-cols-3 gap-4">
                    <div>
                      <label htmlFor="retention-days" className="block text-sm font-medium mb-2">
                        Retention Days
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
                      <p className="mt-1 text-xs text-muted-foreground">Days to keep regular recordings (0 = unlimited)</p>
                    </div>
                    <div>
                      <label htmlFor="detection-retention-days" className="block text-sm font-medium mb-2">
                        Detection Retention Days
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
                      <p className="mt-1 text-xs text-muted-foreground">Days to keep detection recordings (0 = unlimited)</p>
                    </div>
                    <div>
                      <label htmlFor="max-storage-mb" className="block text-sm font-medium mb-2">
                        Max Storage (MB)
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
                      <p className="mt-1 text-xs text-muted-foreground">Storage quota for this stream (0 = unlimited)</p>
                    </div>
                  </div>
                </div>
              </div>
            </AccordionSection>

            {/* Detection Zones Section */}
            {currentStream.detectionEnabled && (
              <AccordionSection
                title="Detection Zones"
                isExpanded={expandedSections.zones}
                onToggle={() => onToggleSection('zones')}
                badge="Optional"
              >
                <div className="space-y-4">
                  <p className="text-sm text-muted-foreground">
                    Define specific regions where object detection should be active. This helps reduce false positives and focus on areas of interest.
                  </p>

                  <div className="flex items-center justify-between p-4 bg-muted rounded-lg">
                    <div>
                      <p className="font-medium">
                        {detectionZones.length === 0 ? 'No zones configured' : `${detectionZones.length} zone(s) configured`}
                      </p>
                      {detectionZones.length > 0 && (
                        <p className="text-sm text-muted-foreground mt-1">
                          {detectionZones.filter(z => z.enabled).length} enabled
                        </p>
                      )}
                    </div>
                    <button
                      type="button"
                      onClick={() => setShowZoneEditor(true)}
                      className="btn-primary"
                    >
                      {detectionZones.length === 0 ? 'Configure Zones' : 'Edit Zones'}
                    </button>
                  </div>

                  {detectionZones.length > 0 && (
                    <div className="space-y-2">
                      <p className="text-sm font-medium">Configured Zones:</p>
                      {detectionZones.map((zone, index) => (
                        <div
                          key={zone.id}
                          className="flex items-center justify-between p-3 bg-background border border-border rounded"
                        >
                          <div className="flex items-center space-x-3">
                            <div
                              className="w-4 h-4 rounded"
                              style={{ backgroundColor: zone.color || '#3b82f6' }}
                            />
                            <span className="font-medium">{zone.name}</span>
                            <span className="text-sm text-muted-foreground">
                              ({zone.polygon.length} points)
                            </span>
                          </div>
                          <span className={`text-sm px-2 py-1 rounded ${zone.enabled ? 'bg-success/20 text-success' : 'bg-muted text-muted-foreground'}`}>
                            {zone.enabled ? 'Enabled' : 'Disabled'}
                          </span>
                        </div>
                      ))}
                    </div>
                  )}

                  <div className="rounded-lg p-4" style={{backgroundColor: 'hsl(var(--info-muted))', borderWidth: '1px', borderStyle: 'solid', borderColor: 'hsl(var(--info) / 0.3)'}}>
                    <div className="flex">
                      <svg className="w-5 h-5 mr-2 flex-shrink-0" style={{color: 'hsl(var(--info))'}} fill="currentColor" viewBox="0 0 20 20">
                        <path fillRule="evenodd" d="M18 10a8 8 0 11-16 0 8 8 0 0116 0zm-7-4a1 1 0 11-2 0 1 1 0 012 0zM9 9a1 1 0 000 2v3a1 1 0 001 1h1a1 1 0 100-2v-3a1 1 0 00-1-1H9z" clipRule="evenodd" />
                      </svg>
                      <div className="text-sm" style={{color: 'hsl(var(--info-muted-foreground))'}}>
                        <p className="font-medium mb-1">Zone Detection Tips:</p>
                        <ul className="list-disc list-inside space-y-1 text-xs">
                          <li>Click "Configure Zones" to draw detection regions on your camera view</li>
                          <li>Draw polygons by clicking points on the image</li>
                          <li>Use zones to ignore areas like trees, roads, or sky</li>
                          <li>Multiple zones can be configured for different areas</li>
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
                title="Motion Recording (ONVIF)"
                isExpanded={expandedSections.motion}
                onToggle={() => onToggleSection('motion')}
                badge="ONVIF Only"
              >
                <div className="space-y-4">
                  <div className="flex items-center">
                    <label className="flex items-center space-x-2 cursor-pointer">
                      <input
                        type="checkbox"
                        id="motion-recording-enabled"
                        name="motionRecordingEnabled"
                        className="h-4 w-4 rounded border-gray-300"
                        style={{accentColor: 'hsl(var(--primary))'}}
                        checked={currentStream.motionRecordingEnabled}
                        onChange={onInputChange}
                      />
                      <span className="text-sm font-medium">Enable ONVIF Motion Recording</span>
                    </label>
                  </div>
                  <p className="text-sm text-muted-foreground">
                    Record when the camera's built-in motion detection triggers. Uses ONVIF events from the camera.
                  </p>

                  {currentStream.motionRecordingEnabled && (
                    <>
                      <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                        <div>
                          <label htmlFor="motion-pre-buffer" className="block text-sm font-medium mb-2">
                            Pre-Event Buffer (seconds)
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
                          <p className="mt-1 text-xs text-muted-foreground">Video before motion event (0-30s)</p>
                        </div>

                        <div>
                          <label htmlFor="motion-post-buffer" className="block text-sm font-medium mb-2">
                            Post-Event Buffer (seconds)
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
                          <p className="mt-1 text-xs text-muted-foreground">Continue recording after motion (0-60s)</p>
                        </div>

                        <div>
                          <label htmlFor="motion-max-duration" className="block text-sm font-medium mb-2">
                            Max File Duration (seconds)
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
                          <p className="mt-1 text-xs text-muted-foreground">Max duration per file (60-3600s)</p>
                        </div>

                        <div>
                          <label htmlFor="motion-retention-days" className="block text-sm font-medium mb-2">
                            Retention Period (days)
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
                          <p className="mt-1 text-xs text-muted-foreground">Auto-delete after N days (1-365)</p>
                        </div>

                        <div>
                          <label htmlFor="motion-codec" className="block text-sm font-medium mb-2">
                            Video Codec
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
                            Recording Quality
                          </label>
                          <select
                            id="motion-quality"
                            name="motionQuality"
                            className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                            value={currentStream.motionQuality}
                            onChange={onInputChange}
                          >
                            <option value="low">Low</option>
                            <option value="medium">Medium</option>
                            <option value="high">High</option>
                          </select>
                        </div>

                        <div className="flex justify-end pt-2">
                          <button
                            type="button"
                            onClick={onTestMotion}
                            className="px-4 py-2 bg-secondary text-secondary-foreground rounded-md hover:bg-secondary/80 transition-colors font-medium"
                          >
                            Trigger Test Motion Event
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
                title="PTZ Control"
                isExpanded={expandedSections.ptz}
                onToggle={() => onToggleSection('ptz')}
                badge="ONVIF Only"
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
                      Enable PTZ Control
                    </label>
                  </div>
                  <p className="text-xs text-muted-foreground">
                    Enable pan-tilt-zoom controls for this camera via ONVIF PTZ protocol
                  </p>

                  {currentStream.ptzEnabled && (
                    <>
                      {/* PTZ Limits */}
                      <div className="grid grid-cols-1 md:grid-cols-3 gap-4 mt-4">
                        <div>
                          <label htmlFor="ptz-max-x" className="block text-sm font-medium mb-2">
                            Max Pan (X)
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
                          <p className="mt-1 text-xs text-muted-foreground">0 = no limit</p>
                        </div>

                        <div>
                          <label htmlFor="ptz-max-y" className="block text-sm font-medium mb-2">
                            Max Tilt (Y)
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
                          <p className="mt-1 text-xs text-muted-foreground">0 = no limit</p>
                        </div>

                        <div>
                          <label htmlFor="ptz-max-z" className="block text-sm font-medium mb-2">
                            Max Zoom (Z)
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
                          <p className="mt-1 text-xs text-muted-foreground">0 = no limit</p>
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
                          Camera supports Home Position
                        </label>
                      </div>
                    </>
                  )}
                </div>
              </AccordionSection>
            )}

            {/* Advanced Settings Section */}
            <AccordionSection
              title="Advanced Settings"
              isExpanded={expandedSections.advanced}
              onToggle={() => onToggleSection('advanced')}
            >
              <div className="grid grid-cols-1 md:grid-cols-2 gap-4">
                <div>
                  <label htmlFor="stream-priority" className="block text-sm font-medium mb-2">
                    Stream Priority
                  </label>
                  <select
                    id="stream-priority"
                    name="priority"
                    className="w-full px-3 py-2 border border-input rounded-md shadow-sm focus:outline-none focus:ring-2 focus:ring-primary bg-background text-foreground"
                    value={currentStream.priority}
                    onChange={onInputChange}
                  >
                    <option value="1">Low (1)</option>
                    <option value="5">Medium (5)</option>
                    <option value="10">High (10)</option>
                  </select>
                  <p className="mt-1 text-xs text-muted-foreground">Processing priority for this stream</p>
                </div>

                <div>
                  <label htmlFor="stream-segment" className="block text-sm font-medium mb-2">
                    Segment Duration (seconds)
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
                  <p className="mt-1 text-xs text-muted-foreground">Duration of each recording segment</p>
                </div>
              </div>
            </AccordionSection>

          </form>
        </div>

        {/* Footer */}
        <div className="flex justify-between items-center p-6 border-t border-border flex-shrink-0 bg-muted/20">
          <button
            type="button"
            onClick={onTestConnection}
            className="px-4 py-2 bg-secondary text-secondary-foreground rounded-md hover:bg-secondary/80 transition-colors font-medium"
          >
            Test Connection
          </button>
          <div className="flex space-x-3">
            <button
              type="button"
              onClick={onClose}
              className="px-6 py-2 border border-input rounded-md shadow-sm text-sm font-medium text-foreground bg-background hover:bg-muted transition-colors"
            >
              Cancel
            </button>
            <button
              type="button"
              onClick={onSave}
              className="px-6 py-2 btn-primary font-medium"
            >
              {isEditing ? 'Update Stream' : 'Add Stream'}
            </button>
          </div>
        </div>
      </div>
    </div>
    </>
  );
}

