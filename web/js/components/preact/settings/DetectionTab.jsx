/**
 * DetectionTab — Detection models path, API URL / backend, default detection
 * threshold (used when a stream enables detection-based recording).
 *
 * Part of PRD UXD_01 §5.2 / T2 settings restructure (#399).
 */

export function DetectionTab({ settings, handleInputChange, handleThresholdChange, canModifySettings, t }) {
  return (
    <div class="space-y-6">
      <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4">
        <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('settings.detectionBasedRecording')}</h3>
        <div class="setting mb-4">
          <p class="setting-description mb-2 text-gray-700 dark:text-gray-300">
            {t('settings.detectionBasedRecordingDescription')}
          </p>
          <p class="setting-description mb-2 text-gray-700 dark:text-gray-300">
            <strong>{t('settings.motionDetectionLabel')}</strong> {t('settings.motionDetectionDescription')}
          </p>
          <p class="setting-description mb-2 text-gray-700 dark:text-gray-300">
            <strong>{t('settings.optimizedMotionDetectionLabel')}</strong> {t('settings.optimizedMotionDetectionDescription')}
          </p>
        </div>
        <div data-setting-label={t('settings.detectionModelsPath')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-detection-models-path" class="font-medium">{t('settings.detectionModelsPath')}</label>
          <div class="col-span-2">
            <input
              type="text"
              id="setting-detection-models-path"
              name="detectionModelsPath"
              placeholder="/var/lib/lightnvr/models"
              class="w-full p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.detectionModelsPath}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.detectionModelsPathHelp')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.apiDetectionUrl')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-api-detection-url" class="font-medium">{t('settings.apiDetectionUrl')}</label>
          <div class="col-span-2">
            <input
              type="text"
              id="setting-api-detection-url"
              name="apiDetectionUrl"
              class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.apiDetectionUrl}
              onChange={handleInputChange}
              disabled={!canModifySettings}
              placeholder="http://localhost:8000/detect"
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.apiDetectionUrlHelp')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.apiDetectionBackend')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-api-detection-backend" class="font-medium">{t('settings.apiDetectionBackend')}</label>
          <div class="col-span-2">
            <select
              id="setting-api-detection-backend"
              name="apiDetectionBackend"
              class="p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.apiDetectionBackend}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            >
              <option value="onnx">{t('settings.apiDetectionBackendOnnx')}</option>
              <option value="tflite">{t('settings.apiDetectionBackendTflite')}</option>
              <option value="opencv">{t('settings.apiDetectionBackendOpencv')}</option>
            </select>
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.apiDetectionBackendHelp')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.defaultDetectionThreshold')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-default-detection-threshold" class="font-medium">{t('settings.defaultDetectionThreshold')}</label>
          <div class="col-span-2">
            <div class="flex items-center">
              <input
                type="range"
                id="setting-default-detection-threshold"
                name="defaultDetectionThreshold"
                min="0"
                max="100"
                step="1"
                class="w-full h-2 bg-secondary rounded-lg appearance-none cursor-pointer accent-primary disabled:opacity-60 disabled:cursor-not-allowed"
                value={settings.defaultDetectionThreshold}
                onChange={handleThresholdChange}
                disabled={!canModifySettings}
              />
              <span id="threshold-value" class="ml-2 min-w-[3rem] text-center">{settings.defaultDetectionThreshold}%</span>
            </div>
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.defaultDetectionThresholdHelp')}</span>
          </div>
        </div>
      </div>
    </div>
  );
}

export default DetectionTab;
