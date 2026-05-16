/**
 * Go2rtcTab — go2rtc integration settings + the override editor.
 *
 * Owns the T9 (PR #398) override editor surface: quarantine banner,
 * size indicator, validate-on-blur, effective-config modal, curated
 * presets, and the supported-sections collapsible. This tab also owns
 * the hooks that back that editor (validation state, effective-config
 * state, preset insertion) because nothing outside this tab needs them.
 *
 * Part of PRD UXD_01 §5.2 / T2 settings restructure (#399).
 */

import { useEffect, useRef, useState } from 'preact/hooks';
import { fetchJSON } from '../../../query-client.js';

// Max UTF-8 byte length for the override — matches the server cap.
const GO2RTC_OVERRIDE_MAX_BYTES = 65535;

// Curated example presets (T9 scope). Inserts at cursor / end — never
// overwrites existing content.
const OVERRIDE_PRESETS = [
  {
    label: 'ffmpeg copy mode (no transcode)',
    yaml: 'ffmpeg:\n  h264: "-codec:v copy -codec:a copy"\n  h265: "-codec:v copy -codec:a copy"\n',
  },
  {
    label: 'Trace logging',
    yaml: 'log:\n  level: trace\n',
  },
  {
    label: 'Custom STUN server',
    yaml: 'webrtc:\n  ice_servers:\n    - urls: [stun:stun.example.com:3478]\n',
  },
  {
    label: 'TURN server with credentials',
    yaml: 'webrtc:\n  ice_servers:\n    - urls: [turn:turn.example.com:3478]\n      username: "USER"\n      credential: "PASS"\n',
  },
  {
    label: 'MQTT bridge',
    yaml: 'mqtt:\n  host: mqtt.example.com\n  port: 1883\n  username: "USER"\n  password: "PASS"\n',
  },
  {
    label: 'HomeKit accessory',
    yaml: 'homekit:\n  cam1:\n    pin: "12345678"\n    name: "Front Camera"\n',
  },
  {
    label: 'Preload streams at start',
    yaml: 'preload:\n  cam1: video=h264&audio=copy\n',
  },
];

const KNOWN_GO2RTC_SECTIONS = [
  ['api', 'HTTP/WS API server (listen, password, base_path, tls)'],
  ['rtsp', 'RTSP server (listen, default_query, credentials)'],
  ['webrtc', 'WebRTC (listen, candidates, ice_servers, filters)'],
  ['ffmpeg', 'Transcoding presets (bin, h264, h265, opus, custom templates)'],
  ['log', 'Logging (level, format, output, per-module overrides)'],
  ['streams', 'Stream definitions — overrides clash with lightNVR auto-streams'],
  ['publish', 'RTMP/S push destinations'],
  ['hass', 'Home Assistant integration'],
  ['mqtt', 'MQTT bridge (host, port, credentials, topic)'],
  ['hls', 'HLS output server'],
  ['srtp', 'SRTP server'],
  ['homekit', 'HomeKit accessory bridge'],
  ['ngrok', 'Ngrok tunnel'],
  ['echo', 'Dynamic URL via shell expansion'],
  ['preload', 'Auto-start streams (name → filter spec)'],
  ['app', 'Module enable/disable (modules list)'],
];

function formatBytes(n) {
  if (n < 1024) return `${n} B`;
  return `${(n / 1024).toFixed(1)} KB`;
}

// Count UTF-8 bytes (NOT JS string length, which is UTF-16 code units).
// The server enforces a byte cap, so non-ASCII in the override would
// otherwise let the UI report "within limit" while the save is rejected
// with HTTP 413. TextEncoder is universally available in modern browsers.
const TEXT_ENCODER = (typeof TextEncoder !== 'undefined') ? new TextEncoder() : null;
function utf8ByteLength(s) {
  if (!s) return 0;
  if (TEXT_ENCODER) return TEXT_ENCODER.encode(s).length;
  return unescape(encodeURIComponent(s)).length;
}

export function Go2rtcTab({ settings, setSettings, handleInputChange, canModifySettings, t }) {
  // ---------- T9 — go2rtc override validation, preview, presets ----------
  const [overrideValidation, setOverrideValidation] = useState(null); // null | { valid, error?, warnings, libyaml_available, skipped }
  const [overrideValidating, setOverrideValidating] = useState(false);
  const [effectiveConfig, setEffectiveConfig] = useState(null);
  const [effectiveConfigOpen, setEffectiveConfigOpen] = useState(false);
  const [effectiveConfigLoading, setEffectiveConfigLoading] = useState(false);
  const [overrideStatus, setOverrideStatus] = useState(null);
  const overrideValidateTimerRef = useRef(null);
  const isMountedRef = useRef(true);

  useEffect(() => {
    isMountedRef.current = true;
    return () => {
      isMountedRef.current = false;
      if (overrideValidateTimerRef.current) {
        clearTimeout(overrideValidateTimerRef.current);
        overrideValidateTimerRef.current = null;
      }
    };
  }, []);

  const validateOverride = async (yaml) => {
    if (isMountedRef.current) setOverrideValidating(true);
    try {
      const result = await fetchJSON('/api/settings/go2rtc/validate', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ override: yaml || '' }),
      });
      if (isMountedRef.current) setOverrideValidation(result);
    } catch (_err) {
      // Network/HTTP failure — don't block the user, just clear stale state.
      // The save-side validator will catch any real problem on POST /api/settings.
      if (isMountedRef.current) setOverrideValidation(null);
    } finally {
      if (isMountedRef.current) setOverrideValidating(false);
    }
  };

  const handleOverrideBlur = (e) => {
    if (overrideValidateTimerRef.current) {
      clearTimeout(overrideValidateTimerRef.current);
    }
    const liveValue = (e && e.target && typeof e.target.value === 'string')
      ? e.target.value
      : settings.go2rtcConfigOverride;
    overrideValidateTimerRef.current = setTimeout(
      () => validateOverride(liveValue),
      200
    );
  };

  const handleOverrideChange = (e) => {
    setOverrideValidation(null);
    handleInputChange(e);
  };

  const loadEffectiveConfig = async () => {
    setEffectiveConfigOpen(true);
    setEffectiveConfigLoading(true);
    setOverrideStatus(null);
    try {
      /* Fetch the YAML previews and the override-pipeline status in
       * parallel.  The status surfaces DB-bytes / file-bytes /
       * quarantine state so that when override.yaml is empty, the user
       * can see WHY instead of just seeing "(none)" (see #394). */
      const [cfg, status] = await Promise.all([
        fetchJSON('/api/system/go2rtc/effective-config'),
        fetchJSON('/api/system/go2rtc/override-status').catch(() => null),
      ]);
      if (isMountedRef.current) {
        setEffectiveConfig(cfg);
        setOverrideStatus(status);
      }
    } catch (err) {
      if (isMountedRef.current) setEffectiveConfig({ error: String(err) });
    } finally {
      if (isMountedRef.current) setEffectiveConfigLoading(false);
    }
  };

  /* Summarize the override-status response into a single, actionable
   * line.  Priorities are in order of "most likely to explain why the
   * user's override isn't applying". */
  const describeOverrideStatus = (s) => {
    if (!s) return null;
    if (s.quarantine_active) {
      return {
        kind: 'error',
        text: `Override is QUARANTINED — go2rtc refused to load it. ` +
              `Reason: ${s.quarantine_reason || '(unknown)'}. ` +
              `Save a new value to clear the quarantine.`,
      };
    }
    if (s.db_bytes > 0 && !s.file_present) {
      return {
        kind: 'error',
        text: `${s.db_bytes} bytes saved in DB, but ${s.expected_file_path || 'override.yaml'} ` +
              `is NOT present on disk. go2rtc is running WITHOUT your override. ` +
              `Check server logs for a "Failed to open go2rtc override file" error; ` +
              `likely a permissions or filesystem problem on the config dir.`,
      };
    }
    if (s.db_bytes > 0 && s.file_present && !s.content_matches) {
      return {
        kind: 'warn',
        text: `DB has ${s.db_bytes} bytes but override.yaml on disk is ${s.file_bytes} bytes ` +
              `and the contents differ — go2rtc may be using a stale override. ` +
              `A go2rtc restart will regenerate the file from the DB.`,
      };
    }
    if (s.db_bytes === 0 && s.file_present) {
      return {
        kind: 'warn',
        text: `override.yaml exists on disk but the DB setting is empty. ` +
              `The file will be removed on the next go2rtc restart.`,
      };
    }
    if (s.db_bytes > 0 && s.content_matches && !s.process_running) {
      return {
        kind: 'warn',
        text: `Override is saved and on disk, but go2rtc is not running — ` +
              `the override is not live. Enable go2rtc or check the service logs.`,
      };
    }
    if (s.db_bytes === 0 && !s.file_present) {
      return {
        kind: 'info',
        text: 'No override configured (DB and disk both empty). ' +
              'Nothing to apply — save a value in the textarea above first.',
      };
    }
    if (s.db_bytes > 0 && s.content_matches && s.process_running) {
      return {
        kind: 'ok',
        text: `Override is active: ${s.db_bytes} bytes on disk, matching DB, go2rtc PID ${s.process_pid}.`,
      };
    }
    return null;
  };

  const handleInsertPreset = (e) => {
    const idx = parseInt(e.target.value, 10);
    e.target.value = '';
    if (Number.isNaN(idx) || idx < 0 || idx >= OVERRIDE_PRESETS.length) return;
    const preset = OVERRIDE_PRESETS[idx];
    setSettings(prev => {
      const cur = prev.go2rtcConfigOverride || '';
      const sep = (cur && !cur.endsWith('\n')) ? '\n' : '';
      return { ...prev, go2rtcConfigOverride: cur + sep + preset.yaml };
    });
    setOverrideValidation(null);
  };

  return (
    <div class="space-y-6">
      <div class="settings-group bg-card rounded-lg shadow p-6 mb-6">
        <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('settings.go2rtcIntegration')}</h3>
        <p class="text-sm text-muted-foreground mb-4">
          {t('settings.go2rtcIntegrationDescription')}
        </p>
        <div data-setting-label={t('settings.enableGo2rtc')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-go2rtc-enabled" class="font-medium">{t('settings.enableGo2rtc')}</label>
          <div class="col-span-2">
            <input
              type="checkbox"
              id="setting-go2rtc-enabled"
              name="go2rtcEnabled"
              class="h-4 w-4 text-primary focus:ring-primary border-gray-300 rounded disabled:opacity-60 disabled:cursor-not-allowed"
              checked={settings.go2rtcEnabled}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.enableGo2rtcHelp')}</span>
          </div>
        </div>
        {settings.go2rtcEnabled && (
          <>
            <div data-setting-label={t('settings.forceNativeHls')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
              <label for="setting-go2rtc-force-native-hls" class="font-medium">{t('settings.forceNativeHls')}</label>
              <div class="col-span-2">
                <input
                  type="checkbox"
                  id="setting-go2rtc-force-native-hls"
                  name="go2rtcForceNativeHls"
                  class="h-4 w-4 text-primary focus:ring-primary border-gray-300 rounded disabled:opacity-60 disabled:cursor-not-allowed"
                  checked={settings.go2rtcForceNativeHls}
                  onChange={handleInputChange}
                  disabled={!canModifySettings}
                />
                <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.forceNativeHlsHelp')}</span>
              </div>
            </div>
            <div data-setting-label={t('settings.binaryPath')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
              <label for="setting-go2rtc-binary-path" class="font-medium">{t('settings.binaryPath')}</label>
              <div class="col-span-2">
                <input
                  type="text"
                  id="setting-go2rtc-binary-path"
                  name="go2rtcBinaryPath"
                  class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
                  value={settings.go2rtcBinaryPath}
                  onChange={handleInputChange}
                  disabled={!canModifySettings}
                  placeholder="/usr/local/bin/go2rtc"
                />
                <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.go2rtcBinaryPathHelp')}</span>
              </div>
            </div>
            <div data-setting-label={t('settings.configDirectory')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
              <label for="setting-go2rtc-config-dir" class="font-medium">{t('settings.configDirectory')}</label>
              <div class="col-span-2">
                <input
                  type="text"
                  id="setting-go2rtc-config-dir"
                  name="go2rtcConfigDir"
                  class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
                  value={settings.go2rtcConfigDir}
                  onChange={handleInputChange}
                  disabled={!canModifySettings}
                  placeholder="/etc/lightnvr/go2rtc"
                />
                <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.go2rtcConfigDirectoryHelp')}</span>
              </div>
            </div>
            <div data-setting-label={t('settings.apiPort')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
              <label for="setting-go2rtc-api-port" class="font-medium">{t('settings.apiPort')}</label>
              <div class="col-span-2">
                <input
                  type="number"
                  id="setting-go2rtc-api-port"
                  name="go2rtcApiPort"
                  min="1"
                  max="65535"
                  class="p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
                  value={settings.go2rtcApiPort}
                  onChange={handleInputChange}
                  disabled={!canModifySettings}
                />
                <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.go2rtcApiPortHelp')}</span>
              </div>
            </div>
            <div data-setting-label={t('settings.rtspPort')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
              <label for="setting-go2rtc-rtsp-port" class="font-medium">{t('settings.rtspPort')}</label>
              <div class="col-span-2">
                <input
                  type="number"
                  id="setting-go2rtc-rtsp-port"
                  name="go2rtcRtspPort"
                  min="1"
                  max="65535"
                  class="p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
                  value={settings.go2rtcRtspPort}
                  onChange={handleInputChange}
                  disabled={!canModifySettings}
                />
                <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.go2rtcRtspPortHelp')}</span>
              </div>
            </div>
            <div data-setting-label={t('settings.enableWebrtc')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
              <label for="setting-go2rtc-webrtc-enabled" class="font-medium">{t('settings.enableWebrtc')}</label>
              <div class="col-span-2">
                <input
                  type="checkbox"
                  id="setting-go2rtc-webrtc-enabled"
                  name="go2rtcWebrtcEnabled"
                  class="h-4 w-4 text-primary focus:ring-primary border-gray-300 rounded disabled:opacity-60 disabled:cursor-not-allowed"
                  checked={settings.go2rtcWebrtcEnabled}
                  onChange={handleInputChange}
                  disabled={!canModifySettings}
                />
                <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.enableWebrtcHelp')}</span>
              </div>
            </div>
            <div data-setting-label={t('settings.webrtcListenPort')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
              <label for="setting-go2rtc-webrtc-listen-port" class="font-medium">{t('settings.webrtcListenPort')}</label>
              <div class="col-span-2">
                <input
                  type="number"
                  id="setting-go2rtc-webrtc-listen-port"
                  name="go2rtcWebrtcListenPort"
                  min="1"
                  max="65535"
                  class="p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
                  value={settings.go2rtcWebrtcListenPort}
                  onChange={handleInputChange}
                  disabled={!canModifySettings}
                />
                <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.webrtcListenPortHelp')}</span>
              </div>
            </div>
            <div data-setting-label={t('settings.enableStun')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
              <label for="setting-go2rtc-stun-enabled" class="font-medium">{t('settings.enableStun')}</label>
              <div class="col-span-2">
                <input
                  type="checkbox"
                  id="setting-go2rtc-stun-enabled"
                  name="go2rtcStunEnabled"
                  class="h-4 w-4 text-primary focus:ring-primary border-gray-300 rounded disabled:opacity-60 disabled:cursor-not-allowed"
                  checked={settings.go2rtcStunEnabled}
                  onChange={handleInputChange}
                  disabled={!canModifySettings}
                />
                <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.enableStunHelp')}</span>
              </div>
            </div>
            <div data-setting-label={t('settings.stunServer')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
              <label for="setting-go2rtc-stun-server" class="font-medium">{t('settings.stunServer')}</label>
              <div class="col-span-2">
                <input
                  type="text"
                  id="setting-go2rtc-stun-server"
                  name="go2rtcStunServer"
                  class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
                  value={settings.go2rtcStunServer}
                  onChange={handleInputChange}
                  disabled={!canModifySettings}
                  placeholder="stun.l.google.com:19302"
                />
                <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.stunServerHelp')}</span>
              </div>
            </div>
            <div data-setting-label={t('settings.externalIp')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
              <label for="setting-go2rtc-external-ip" class="font-medium">{t('settings.externalIp')}</label>
              <div class="col-span-2">
                <input
                  type="text"
                  id="setting-go2rtc-external-ip"
                  name="go2rtcExternalIp"
                  class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
                  value={settings.go2rtcExternalIp}
                  onChange={handleInputChange}
                  disabled={!canModifySettings}
                  placeholder={t('settings.autoDetect')}
                />
                <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.externalIpHelp')}</span>
              </div>
            </div>
            <div data-setting-label={t('settings.iceServers')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
              <label for="setting-go2rtc-ice-servers" class="font-medium">{t('settings.iceServers')}</label>
              <div class="col-span-2">
                <input
                  type="text"
                  id="setting-go2rtc-ice-servers"
                  name="go2rtcIceServers"
                  class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
                  value={settings.go2rtcIceServers}
                  onChange={handleInputChange}
                  disabled={!canModifySettings}
                  placeholder={t('settings.iceServersPlaceholder')}
                />
                <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.iceServersHelp')}</span>
              </div>
            </div>

            {/* Override editor */}
            <div data-setting-label={t('settings.go2rtcConfigOverride')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-start mb-4">
              <label for="setting-go2rtc-config-override" class="font-medium pt-2">
                {t('settings.go2rtcConfigOverride')}
              </label>
              <div class="col-span-2 space-y-2">
                {/* T14/T4b — quarantine banner */}
                {settings.go2rtcOverrideDisabledReason ? (
                  <div class="p-3 border border-yellow-500 bg-yellow-50 dark:bg-yellow-900/20 rounded text-sm">
                    <div class="font-medium text-yellow-800 dark:text-yellow-200 mb-1">
                      Override quarantined
                    </div>
                    <div class="text-yellow-900 dark:text-yellow-100 whitespace-pre-wrap font-mono text-xs">
                      {settings.go2rtcOverrideDisabledReason}
                    </div>
                    {settings.go2rtcOverrideQuarantined ? (
                      <button
                        type="button"
                        class="mt-2 text-xs underline text-yellow-900 dark:text-yellow-100"
                        onClick={() => {
                          setSettings(prev => ({
                            ...prev,
                            go2rtcConfigOverride: prev.go2rtcOverrideQuarantined,
                          }));
                          setOverrideValidation(null);
                        }}
                      >
                        Restore quarantined version into the editor
                      </button>
                    ) : null}
                  </div>
                ) : null}

                {/* Editor */}
                <textarea
                  id="setting-go2rtc-config-override"
                  name="go2rtcConfigOverride"
                  class={`p-2 border rounded bg-background text-foreground w-full max-w-md font-mono text-sm disabled:opacity-60 disabled:cursor-not-allowed ${overrideValidation && overrideValidation.valid === false && overrideValidation.error ? 'border-red-500' : 'border-input'}`}
                  rows="8"
                  value={settings.go2rtcConfigOverride}
                  onChange={handleOverrideChange}
                  onBlur={handleOverrideBlur}
                  disabled={!canModifySettings}
                  maxLength={GO2RTC_OVERRIDE_MAX_BYTES}
                  placeholder={"ffmpeg:\n  h264_hw: \"-codec:v h264_v4l2m2m\"\n\nlog:\n  level: trace"}
                />

                {/* Size indicator */}
                <div class="flex items-center justify-between text-xs text-muted-foreground max-w-md">
                  <span>
                    {formatBytes(utf8ByteLength(settings.go2rtcConfigOverride))} / {formatBytes(GO2RTC_OVERRIDE_MAX_BYTES)}
                    {overrideValidating ? ' • validating…' : null}
                  </span>
                  {overrideValidation && overrideValidation.libyaml_available === false ? (
                    <span class="text-yellow-700 dark:text-yellow-300">libyaml unavailable — server-side validation skipped</span>
                  ) : null}
                </div>

                {/* Inline error */}
                {overrideValidation && overrideValidation.valid === false && overrideValidation.error ? (
                  <div class="p-2 border border-red-500 bg-red-50 dark:bg-red-900/20 rounded text-sm text-red-800 dark:text-red-200 max-w-md">
                    <div class="font-medium">
                      Line {overrideValidation.error.line || '?'}, col {overrideValidation.error.column || '?'}
                    </div>
                    <div class="text-xs mt-1 whitespace-pre-wrap">{overrideValidation.error.message}</div>
                  </div>
                ) : null}

                {/* Inline warnings */}
                {overrideValidation && Array.isArray(overrideValidation.warnings) && overrideValidation.warnings.length > 0 ? (
                  <ul class="text-xs text-yellow-800 dark:text-yellow-200 list-disc ml-5 max-w-md">
                    {overrideValidation.warnings.map((w, i) => (
                      <li key={i}>{w}</li>
                    ))}
                  </ul>
                ) : null}

                {/* Action row */}
                <div class="flex flex-wrap items-center gap-2 max-w-md">
                  <button
                    type="button"
                    class="px-2 py-1 text-xs border border-input rounded hover:bg-muted disabled:opacity-60"
                    disabled={!canModifySettings}
                    onClick={loadEffectiveConfig}
                  >
                    Show effective config
                  </button>
                  <select
                    class="px-2 py-1 text-xs border border-input rounded bg-background disabled:opacity-60"
                    value=""
                    disabled={!canModifySettings}
                    onChange={handleInsertPreset}
                  >
                    <option value="">Load example…</option>
                    {OVERRIDE_PRESETS.map((p, i) => (
                      <option key={i} value={i}>{p.label}</option>
                    ))}
                  </select>
                </div>

                {/* Supported-sections collapsible */}
                <details class="text-xs text-muted-foreground max-w-md">
                  <summary class="cursor-pointer hover:text-foreground">Supported go2rtc sections</summary>
                  <ul class="mt-2 space-y-1">
                    {KNOWN_GO2RTC_SECTIONS.map(([k, desc]) => (
                      <li key={k}><code class="font-mono">{k}</code> — {desc}</li>
                    ))}
                  </ul>
                </details>

                <span class="hint text-sm text-muted-foreground block">{t('settings.go2rtcConfigOverrideHelp')}</span>

                {/* Effective-config modal */}
                {effectiveConfigOpen ? (
                  <div class="fixed inset-0 z-50 flex items-center justify-center bg-black/50" onClick={() => setEffectiveConfigOpen(false)}>
                    <div class="bg-card text-card-foreground rounded-lg shadow-lg max-w-4xl w-full max-h-[80vh] overflow-auto p-4" onClick={(e) => e.stopPropagation()}>
                      <div class="flex items-center justify-between mb-3 pb-2 border-b">
                        <h3 class="font-semibold">go2rtc effective config (redacted)</h3>
                        <button type="button" class="text-sm underline" onClick={() => setEffectiveConfigOpen(false)}>Close</button>
                      </div>
                      {effectiveConfigLoading ? (
                        <div class="text-sm text-muted-foreground">Loading…</div>
                      ) : effectiveConfig?.error ? (
                        <div class="text-sm text-red-600">Failed to load: {effectiveConfig.error}</div>
                      ) : effectiveConfig ? (
                        <div class="space-y-3">
                          {(() => {
                            const note = describeOverrideStatus(overrideStatus);
                            if (!note) return null;
                            const cls = note.kind === 'error'
                              ? 'bg-red-50 dark:bg-red-900/20 border-red-500 text-red-800 dark:text-red-200'
                              : note.kind === 'warn'
                              ? 'bg-yellow-50 dark:bg-yellow-900/20 border-yellow-500 text-yellow-800 dark:text-yellow-200'
                              : note.kind === 'ok'
                              ? 'bg-green-50 dark:bg-green-900/20 border-green-500 text-green-800 dark:text-green-200'
                              : 'bg-blue-50 dark:bg-blue-900/20 border-blue-500 text-blue-800 dark:text-blue-200';
                            return (
                              <div class={`p-2 border rounded text-xs ${cls}`}>
                                {note.text}
                              </div>
                            );
                          })()}
                          {effectiveConfig.redaction_available === false ? (
                            <div class="p-2 bg-yellow-50 dark:bg-yellow-900/20 border border-yellow-500 rounded text-xs text-yellow-800 dark:text-yellow-200">
                              libyaml unavailable on the server — secret redaction is SKIPPED. Treat this preview as if it contained your secrets in cleartext.
                            </div>
                          ) : null}
                          {Array.isArray(effectiveConfig.warnings) && effectiveConfig.warnings.length > 0 ? (
                            <ul class="text-xs text-yellow-800 dark:text-yellow-200 list-disc ml-5">
                              {effectiveConfig.warnings.map((w, i) => <li key={i}>{w}</li>)}
                            </ul>
                          ) : null}
                          <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
                            <div>
                              <div class="text-xs font-medium mb-1">go2rtc.yaml (base)</div>
                              <pre class="text-xs font-mono p-2 border border-input bg-muted rounded overflow-auto max-h-96 whitespace-pre-wrap">{effectiveConfig.base || '(empty)'}</pre>
                            </div>
                            <div>
                              <div class="text-xs font-medium mb-1">override.yaml</div>
                              <pre class="text-xs font-mono p-2 border border-input bg-muted rounded overflow-auto max-h-96 whitespace-pre-wrap">{effectiveConfig.override || '(none)'}</pre>
                            </div>
                          </div>
                        </div>
                      ) : null}
                    </div>
                  </div>
                ) : null}
              </div>
            </div>
          </>
        )}
      </div>
    </div>
  );
}

export default Go2rtcTab;
