/**
 * StreamsDefaultsTab — Default pre/post buffer and buffer strategy defaults
 * applied to newly-created streams.
 *
 * Part of PRD UXD_01 §5.2 / T2 settings restructure (#399).
 */

export function StreamsDefaultsTab({ settings, handleInputChange, canModifySettings, t }) {
  return (
    <div class="space-y-6">
      <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4">
        <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">
          {t('settings.audioCompliance')}
        </h3>
        <div data-setting-label={t('settings.disableAudioGlobally')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-start">
          <label for="setting-audio-disabled" class="font-medium">
            {t('settings.disableAudioGlobally')}
          </label>
          <div class="col-span-2">
            <label class="inline-flex items-center gap-2 cursor-pointer">
              <input
                type="checkbox"
                id="setting-audio-disabled"
                name="audioDisabled"
                class="h-4 w-4 rounded border-input"
                checked={settings.audioDisabled}
                onChange={handleInputChange}
                disabled={!canModifySettings}
              />
              <span>{t('settings.audioDisabled')}</span>
            </label>
            <p class="hint text-sm text-muted-foreground mt-2">
              {t('settings.audioComplianceHelp')}
            </p>
          </div>
        </div>
      </div>

      <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4">
        <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('settings.streamsDefaults') || 'Streams Defaults'}</h3>
        <p class="text-sm text-muted-foreground mb-4">
          {t('settings.streamsDefaultsDescription') || 'Defaults applied to new streams. Per-stream overrides always win.'}
        </p>
        <div data-setting-label={t('settings.defaultPreDetectionBufferSeconds')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-default-pre-buffer" class="font-medium">{t('settings.defaultPreDetectionBufferSeconds')}</label>
          <div class="col-span-2">
            <input
              type="number"
              id="setting-default-pre-buffer"
              name="defaultPreBuffer"
              min="0"
              max="60"
              class="p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.defaultPreBuffer}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.defaultPreDetectionBufferHelp')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.defaultPostDetectionBufferSeconds')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-default-post-buffer" class="font-medium">{t('settings.defaultPostDetectionBufferSeconds')}</label>
          <div class="col-span-2">
            <input
              type="number"
              id="setting-default-post-buffer"
              name="defaultPostBuffer"
              min="0"
              max="300"
              class="p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.defaultPostBuffer}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.defaultPostDetectionBufferHelp')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.defaultBufferStrategy')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-buffer-strategy" class="font-medium">{t('settings.defaultBufferStrategy')}</label>
          <div class="col-span-2">
            <select
              id="setting-buffer-strategy"
              name="bufferStrategy"
              class="p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.bufferStrategy}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            >
              <option value="auto">{t('settings.bufferStrategyAuto')}</option>
              <option value="go2rtc">{t('settings.bufferStrategyGo2rtc')}</option>
              <option value="hls_segment">{t('settings.bufferStrategyHlsSegment')}</option>
              <option value="memory_packet">{t('settings.bufferStrategyMemoryPacket')}</option>
              <option value="mmap_hybrid">{t('settings.bufferStrategyMmapHybrid')}</option>
            </select>
            <p class="hint text-sm text-muted-foreground mt-1">
              {t('settings.defaultBufferStrategyHelp')}
            </p>
          </div>
        </div>
        <div data-setting-label={t('settings.defaultPlaybackTransport')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-default-playback-transport" class="font-medium">
            {t('settings.defaultPlaybackTransport')}
          </label>
          <div class="col-span-2">
            <select
              id="setting-default-playback-transport"
              name="defaultPlaybackTransport"
              class="p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.defaultPlaybackTransport || 'auto'}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            >
              <option value="auto">{t('playbackTransport.auto')}</option>
              <option value="webrtc_only" disabled={settings.webrtcDisabled}>{t('playbackTransport.webrtcOnly')}</option>
              <option value="mse_only" disabled={settings.mseDisabled}>{t('playbackTransport.mseOnly')}</option>
              <option value="hls_only" disabled={settings.hlsDisabled}>{t('playbackTransport.hlsOnly')}</option>
              <option value="webrtc_then_mse" disabled={settings.webrtcDisabled || settings.mseDisabled}>{t('playbackTransport.webrtcThenMse')}</option>
              <option value="mse_then_hls" disabled={settings.mseDisabled || settings.hlsDisabled}>{t('playbackTransport.mseThenHls')}</option>
            </select>
            <p class="hint text-sm text-muted-foreground mt-1">
              {t('settings.defaultPlaybackTransportHelp')}
            </p>
          </div>
        </div>
      </div>
    </div>
  );
}

export default StreamsDefaultsTab;
