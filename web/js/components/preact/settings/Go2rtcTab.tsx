import {validateGo2rtcConfiguration,getGo2rtcEffectiveConfiguration,getGo2rtcOverrideDiagnostics} from '../../../api/system.ts';
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
import type {
  Go2rtcEffectiveConfig,
  Go2rtcOverrideStatus,
  Go2rtcOverrideValidation,
  SetSettings,
  SettingsTabProps,
} from '../../../types/settings.ts';
import { useEscapeKey } from '../../../hooks/useEscapeKey.ts';

interface Go2rtcTabProps extends SettingsTabProps {
  setSettings: SetSettings;
}

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

function formatBytes(n: number) {
  if (n < 1024) return `${n} B`;
  return `${(n / 1024).toFixed(1)} KB`;
}

// Count UTF-8 bytes (NOT JS string length, which is UTF-16 code units).
// The server enforces a byte cap, so non-ASCII in the override would
// otherwise let the UI report "within limit" while the save is rejected
// with HTTP 413. TextEncoder is universally available in modern browsers.
const TEXT_ENCODER = (typeof TextEncoder !== 'undefined') ? new TextEncoder() : null;
function utf8ByteLength(s: string) {
  if (!s) return 0;
  if (TEXT_ENCODER) return TEXT_ENCODER.encode(s).length;
  return unescape(encodeURIComponent(s)).length;
}

export function Go2rtcTab({ settings, setSettings, handleInputChange, canModifySettings, t }: Go2rtcTabProps) {
  // ---------- T9 — go2rtc override validation, preview, presets ----------
  const [overrideValidation, setOverrideValidation] = useState<Go2rtcOverrideValidation | null>(null); // null | { valid, error?, warnings, libyaml_available, skipped }
  const [overrideValidating, setOverrideValidating] = useState(false);
  const [effectiveConfig, setEffectiveConfig] = useState<Go2rtcEffectiveConfig | null>(null);
  const [effectiveConfigOpen, setEffectiveConfigOpen] = useState(false);
  useEscapeKey(() => setEffectiveConfigOpen(false), effectiveConfigOpen);
  const [effectiveConfigLoading, setEffectiveConfigLoading] = useState(false);
  const [overrideStatus, setOverrideStatus] = useState<Go2rtcOverrideStatus | null>(null);
  const overrideValidateTimerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const validationGenerationRef = useRef(0);
  const effectiveConfigGenerationRef = useRef(0);
  const isMountedRef = useRef(true);
  const effectiveConfigDialogRef = useRef<HTMLDivElement>(null);
  const effectiveConfigCloseButtonRef = useRef<HTMLButtonElement>(null);

  const invalidateOverrideValidation = () => {
    validationGenerationRef.current += 1;
    if (overrideValidateTimerRef.current) {
      clearTimeout(overrideValidateTimerRef.current);
      overrideValidateTimerRef.current = null;
    }
    setOverrideValidating(false);
    setOverrideValidation(null);
  };

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

  useEffect(() => {
    if (!effectiveConfigOpen) return;
    const previousFocus = document.activeElement as HTMLElement | null;
    effectiveConfigCloseButtonRef.current?.focus();
    const handleTab = (event: KeyboardEvent) => {
      if (event.key !== 'Tab') return;
      const dialog = effectiveConfigDialogRef.current;
      if (!dialog) return;
      const focusable = [...dialog.querySelectorAll<HTMLElement>('button:not([disabled]), [href], [tabindex]:not([tabindex="-1"])')];
      const first = focusable[0];
      const last = focusable[focusable.length - 1];
      if (!first || !last) {
        event.preventDefault();
        dialog.focus();
      } else if (!dialog.contains(document.activeElement)) {
        event.preventDefault();
        first.focus();
      } else if (event.shiftKey && document.activeElement === first) {
        event.preventDefault();
        last.focus();
      } else if (!event.shiftKey && document.activeElement === last) {
        event.preventDefault();
        first.focus();
      }
    };
    document.addEventListener('keydown', handleTab);
    return () => {
      document.removeEventListener('keydown', handleTab);
      if (previousFocus?.isConnected) previousFocus.focus();
    };
  }, [effectiveConfigOpen]);

  const validateOverride = async (yaml: string, generation: number) => {
    if (isMountedRef.current && validationGenerationRef.current === generation) setOverrideValidating(true);
    try {
      const result = await validateGo2rtcConfiguration({override:yaml || ''});
      if (isMountedRef.current && validationGenerationRef.current === generation) setOverrideValidation(result);
    } catch (_err) {
      // Network/HTTP failure — don't block the user, just clear stale state.
      // The save-side validator will catch any real problem on POST /api/settings.
      if (isMountedRef.current && validationGenerationRef.current === generation) setOverrideValidation(null);
    } finally {
      if (isMountedRef.current && validationGenerationRef.current === generation) setOverrideValidating(false);
    }
  };

  const handleOverrideBlur = (e: Event) => {
    if (overrideValidateTimerRef.current) {
      clearTimeout(overrideValidateTimerRef.current);
    }
    const liveValue = (e && e.target && typeof (e.target as HTMLTextAreaElement).value === 'string')
      ? (e.target as HTMLTextAreaElement).value
      : settings.go2rtcConfigOverride;
    const generationAtSchedule = ++validationGenerationRef.current;
    overrideValidateTimerRef.current = setTimeout(
      () => validateOverride(liveValue, generationAtSchedule),
      200
    );
  };

  const handleOverrideChange = (e: Event) => {
    invalidateOverrideValidation();
    handleInputChange(e);
  };

  const loadEffectiveConfig = async () => {
    const generation = ++effectiveConfigGenerationRef.current;
    setEffectiveConfigOpen(true);
    setEffectiveConfigLoading(true);
    setOverrideStatus(null);
    try {
      /* Fetch the YAML previews and the override-pipeline status in
       * parallel.  The status surfaces DB-bytes / file-bytes /
       * quarantine state so that when override.yaml is empty, the user
       * can see WHY instead of just seeing "(none)" (see #394). */
      const [cfg, status] = await Promise.all([
        getGo2rtcEffectiveConfiguration(),
        getGo2rtcOverrideDiagnostics().catch(() => null),
      ]);
      if (isMountedRef.current && effectiveConfigGenerationRef.current === generation) {
        setEffectiveConfig(cfg);
        setOverrideStatus(status);
      }
    } catch (err) {
      if (isMountedRef.current && effectiveConfigGenerationRef.current === generation) {
        setEffectiveConfig({ error: String(err) });
      }
    } finally {
      if (isMountedRef.current && effectiveConfigGenerationRef.current === generation) {
        setEffectiveConfigLoading(false);
      }
    }
  };

  /* Summarize the override-status response into a single, actionable
   * line.  Priorities are in order of "most likely to explain why the
   * user's override isn't applying". */
  const describeOverrideStatus = (s: Go2rtcOverrideStatus | null) => {
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
    // process_state_known is false while a go2rtc start/stop/reconfigure holds
    // the lifecycle, so the PID is unavailable rather than absent. Claiming
    // "not running" there would be wrong during the very restart that applies
    // the override.
    if (s.db_bytes > 0 && s.content_matches && s.process_state_known === false) {
      return {
        kind: 'info',
        text: `Override is saved and on disk. go2rtc is busy starting or ` +
              `reconfiguring, so its runtime state is not readable yet.`,
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

  const getTopLevelYamlKeys = (yaml: string): Set<string> => {
    const keys = new Set<string>();
    for (const rawLine of yaml.split('\n')) {
      const line = rawLine.trimEnd();
      if (!line || line.trimStart().startsWith('#')) continue;
      if (/^\s/.test(rawLine)) continue;
      const m = line.match(/^([A-Za-z0-9_-]+)\s*:/);
      if (m?.[1]) keys.add(m[1]);
    }
    return keys;
  };

  const handleInsertPreset = (e: Event) => {
    const idx = parseInt((e.target as HTMLSelectElement).value, 10);
    (e.target as HTMLSelectElement).value = '';
    if (Number.isNaN(idx) || idx < 0 || idx >= OVERRIDE_PRESETS.length) return;
    const preset = OVERRIDE_PRESETS[idx];

    const cur = settings.go2rtcConfigOverride || '';
    const existingKeys = getTopLevelYamlKeys(cur);
    const presetKeys = getTopLevelYamlKeys(preset!.yaml);
    const conflicts = Array.from(presetKeys).filter(k => existingKeys.has(k));
    if (conflicts.length > 0) {
      invalidateOverrideValidation();
      setOverrideValidation({
        valid: false,
        error: {
          message: `Preset insertion would duplicate top-level section(s): ${conflicts.join(', ')}. Merge into the existing section instead of inserting the full preset.`,
        },
      });
      return;
    }

    setSettings(prev => {
      const current = prev.go2rtcConfigOverride || '';
      const sep = (current && !current.endsWith('\n')) ? '\n' : '';
      return { ...prev, go2rtcConfigOverride: current + sep + preset!.yaml };
    });
    invalidateOverrideValidation();
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
                  maxLength={255}
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
              <div class="col-span-2 min-w-0 space-y-2">
                {/* T14/T4b — quarantine banner */}
                {settings.go2rtcOverrideDisabledReason ? (
                  <div class="p-3 border border-yellow-500 bg-yellow-50 dark:bg-yellow-900/20 rounded text-sm">
                    <div class="font-medium text-yellow-800 dark:text-yellow-200 mb-1">
                      {t('settings.go2rtcOverrideQuarantined')}
                    </div>
                    <div class="text-yellow-900 dark:text-yellow-100 whitespace-pre-wrap font-mono text-xs">
                      {settings.go2rtcOverrideDisabledReason}
                    </div>
                    {settings.go2rtcOverrideQuarantined ? (
                      <button
                        type="button"
                        class="mt-2 text-xs underline text-yellow-900 dark:text-yellow-100"
                        disabled={!canModifySettings}
                        onClick={() => {
                          setSettings(prev => ({
                            ...prev,
                            go2rtcConfigOverride: prev.go2rtcOverrideQuarantined as string,
                          }));
                          invalidateOverrideValidation();
                        }}
                      >
                        {t('settings.go2rtcRestoreQuarantined')}
                      </button>
                    ) : null}
                  </div>
                ) : null}

                {/* Editor */}
                <textarea
                  id="setting-go2rtc-config-override"
                  name="go2rtcConfigOverride"
                  class={`p-2 border rounded bg-background text-foreground w-full font-mono text-sm overflow-x-auto whitespace-pre disabled:opacity-60 disabled:cursor-not-allowed ${(utf8ByteLength(settings.go2rtcConfigOverride) > GO2RTC_OVERRIDE_MAX_BYTES) || (overrideValidation && overrideValidation.valid === false && overrideValidation.error) ? 'border-red-500' : 'border-input'}`}
                  rows={8}
                  spellcheck={false}
                  autocomplete="off"
                  autocapitalize="off"
                  autocorrect="off"
                  wrap="off"
                  value={settings.go2rtcConfigOverride}
                  onInput={handleOverrideChange}
                  onBlur={handleOverrideBlur}
                  disabled={!canModifySettings}
                  aria-invalid={utf8ByteLength(settings.go2rtcConfigOverride) > GO2RTC_OVERRIDE_MAX_BYTES || !!(overrideValidation && overrideValidation.valid === false && overrideValidation.error)}
                  aria-describedby={[
                    utf8ByteLength(settings.go2rtcConfigOverride) > GO2RTC_OVERRIDE_MAX_BYTES ? 'go2rtc-config-override-size-error' : '',
                    overrideValidation && overrideValidation.valid === false && overrideValidation.error ? 'go2rtc-config-override-error' : '',
                  ].filter(Boolean).join(' ') || undefined}
                  placeholder={"ffmpeg:\n  h264_hw: \"-codec:v h264_v4l2m2m\"\n\nlog:\n  level: trace"}
                />

                {/* Size indicator */}
                {(() => {
                  const overrideBytes = utf8ByteLength(settings.go2rtcConfigOverride);
                  const overrideTooLarge = overrideBytes > GO2RTC_OVERRIDE_MAX_BYTES;
                  return (
                    <div class="flex items-center justify-between text-xs text-muted-foreground">
                      <span class={overrideTooLarge ? 'text-red-600 dark:text-red-400' : ''}>
                        {formatBytes(overrideBytes)} / {formatBytes(GO2RTC_OVERRIDE_MAX_BYTES)}
                        {overrideValidating ? t('settings.go2rtcValidating') : null}
                        {overrideTooLarge ? (
                          <span id="go2rtc-config-override-size-error" class="ml-2 text-red-600 dark:text-red-400">
                            {t('settings.go2rtcOverrideTooLarge')}
                          </span>
                        ) : null}
                      </span>
                      {overrideValidation && overrideValidation.libyaml_available === false ? (
                        <span class="text-yellow-700 dark:text-yellow-300">{t('settings.go2rtcValidationSkipped')}</span>
                      ) : null}
                    </div>
                  );
                })()}

                {/* Inline error */}
                {overrideValidation && overrideValidation.valid === false && overrideValidation.error ? (
                  <div
                    id="go2rtc-config-override-error"
                    role="alert"
                    aria-live="assertive"
                    class="p-2 border border-red-500 bg-red-50 dark:bg-red-900/20 rounded text-sm text-red-800 dark:text-red-200"
                  >
                    {overrideValidation.error.line || overrideValidation.error.column ? (
                      <div class="font-medium">
                        {t('settings.go2rtcErrorPosition', { line: overrideValidation.error.line || '?', column: overrideValidation.error.column || '?' })}
                      </div>
                    ) : null}
                    <div class="text-xs mt-1 whitespace-pre-wrap">{overrideValidation.error.message}</div>
                  </div>
                ) : null}

                {/* Inline warnings */}
                {overrideValidation && Array.isArray(overrideValidation.warnings) && overrideValidation.warnings.length > 0 ? (
                  <ul class="text-xs text-yellow-800 dark:text-yellow-200 list-disc ms-5">
                    {overrideValidation.warnings.map((w, i) => (
                      <li key={i}>{w}</li>
                    ))}
                  </ul>
                ) : null}

                {/* Action row */}
                <div class="flex flex-wrap items-center gap-2">
                  <button
                    type="button"
                    class="px-2 py-1 text-xs border border-input rounded hover:bg-muted disabled:opacity-60"
                    disabled={!canModifySettings}
                    onClick={loadEffectiveConfig}
                  >
                    {t('settings.go2rtcShowEffectiveConfig')}
                  </button>
                  <select
                    class="px-2 py-1 text-xs border border-input rounded bg-background disabled:opacity-60"
                    value=""
                    disabled={!canModifySettings}
                    aria-label={t('settings.go2rtcLoadExample')}
                    onChange={handleInsertPreset}
                  >
                    <option value="">{t('settings.go2rtcLoadExample')}</option>
                    {OVERRIDE_PRESETS.map((p, i) => (
                      <option key={i} value={i}>{p.label}</option>
                    ))}
                  </select>
                </div>

                {/* Supported-sections collapsible */}
                <details class="text-xs text-muted-foreground">
                  <summary class="cursor-pointer hover:text-foreground">{t('settings.go2rtcSupportedSections')}</summary>
                  <ul class="mt-2 space-y-1">
                    {KNOWN_GO2RTC_SECTIONS.map(([k, desc]) => (
                      <li key={k}><code class="font-mono">{k}</code> — {desc}</li>
                    ))}
                  </ul>
                </details>

                <span class="hint text-sm text-muted-foreground block">{t('settings.go2rtcConfigOverrideHelp')}</span>

                {/* Effective-config modal */}
                {effectiveConfigOpen ? (
                  <div role="presentation" class="app-modal-overlay fixed inset-0 flex items-center justify-center bg-black/50" onClick={(e) => {
                    if (e.target === e.currentTarget) setEffectiveConfigOpen(false);
                  }}>
                    <div
                      ref={effectiveConfigDialogRef}
                      role="dialog"
                      aria-modal="true"
                      aria-labelledby="go2rtc-effective-config-title"
                      tabIndex={-1}
                      class="bg-card text-card-foreground rounded-lg shadow-lg max-w-4xl w-full max-h-[80vh] overflow-auto p-4"
                    >
                      <div class="flex items-center justify-between mb-3 pb-2 border-b">
                        <h3 id="go2rtc-effective-config-title" class="font-semibold">{t('settings.go2rtcEffectiveConfig')}</h3>
                        <button ref={effectiveConfigCloseButtonRef} type="button" class="text-sm underline" onClick={() => setEffectiveConfigOpen(false)}>{t('common.close')}</button>
                      </div>
                      {effectiveConfigLoading ? (
                        <div class="text-sm text-muted-foreground">{t('settings.go2rtcLoading')}</div>
                      ) : effectiveConfig?.error ? (
                        <div class="text-sm text-red-600">{t('settings.go2rtcLoadFailed', { message: effectiveConfig.error })}</div>
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
                              {t('settings.go2rtcRedactionUnavailable')}
                            </div>
                          ) : null}
                          {Array.isArray(effectiveConfig.warnings) && effectiveConfig.warnings.length > 0 ? (
                            <ul class="text-xs text-yellow-800 dark:text-yellow-200 list-disc ms-5">
                              {effectiveConfig.warnings.map((w, i) => <li key={i}>{w}</li>)}
                            </ul>
                          ) : null}
                          <div class="grid grid-cols-1 md:grid-cols-2 gap-3">
                            <div>
                              <div class="text-xs font-medium mb-1">{t('settings.go2rtcBaseConfig')}</div>
                              <pre class="text-xs font-mono p-2 border border-input bg-muted rounded overflow-auto max-h-96 whitespace-pre-wrap">{effectiveConfig.base || t('settings.go2rtcEmpty')}</pre>
                            </div>
                            <div>
                              <div class="text-xs font-medium mb-1">override.yaml</div>
                              <pre class="text-xs font-mono p-2 border border-input bg-muted rounded overflow-auto max-h-96 whitespace-pre-wrap">{effectiveConfig.override || t('settings.go2rtcNone')}</pre>
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
