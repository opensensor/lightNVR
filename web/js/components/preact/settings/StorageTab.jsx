/**
 * StorageTab — Storage path, HLS path, retention, auto-delete, thumbnails,
 * DB path + backup schedule + post-backup script.
 *
 * Part of PRD UXD_01 §5.2 / T2 settings restructure (#399).
 */

export function StorageTab({ settings, handleInputChange, canModifySettings, t }) {
  return (
    <div class="space-y-6">
      <div class="settings-group bg-card text-card-foreground rounded-lg shadow p-4">
        <h3 class="text-lg font-semibold mb-4 pb-2 border-b border-border">{t('settings.storage')}</h3>
        <div data-setting-label={t('settings.storagePath')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-storage-path" class="font-medium">{t('settings.storagePath')}</label>
          <input
            type="text"
            id="setting-storage-path"
            name="storagePath"
            class="col-span-2 p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
            value={settings.storagePath}
            onChange={handleInputChange}
            disabled={!canModifySettings}
          />
        </div>
        <div data-setting-label={t('settings.hlsStoragePath')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-storage-path-hls" class="font-medium">{t('settings.hlsStoragePath')}</label>
          <div class="col-span-2">
            <input
              type="text"
              id="setting-storage-path-hls"
              name="storagePathHls"
              class="w-full p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.storagePathHls}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.hlsStoragePathHelp')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.maxStorageGb')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-max-storage" class="font-medium">{t('settings.maxStorageGb')}</label>
          <div class="col-span-2">
            <input
              type="number"
              id="setting-max-storage"
              name="maxStorage"
              min="0"
              class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.maxStorage}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.zeroUnlimited')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.retentionDays')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-retention" class="font-medium">{t('settings.retentionDays')}</label>
          <input
            type="number"
            id="setting-retention"
            name="retention"
            min="1"
            class="col-span-2 p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
            value={settings.retention}
            onChange={handleInputChange}
            disabled={!canModifySettings}
          />
        </div>
        <div data-setting-label={t('settings.autoDeleteOldest')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-auto-delete" class="font-medium">{t('settings.autoDeleteOldest')}</label>
          <div class="col-span-2">
            <input
              type="checkbox"
              id="setting-auto-delete"
              name="autoDelete"
              class="w-4 h-4 rounded focus:ring-2 disabled:opacity-60 disabled:cursor-not-allowed"
              style={{ accentColor: 'hsl(var(--primary))' }}
              checked={settings.autoDelete}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
          </div>
        </div>
        <div data-setting-label={t('settings.enableGridViewThumbnails')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-generate-thumbnails" class="font-medium">{t('settings.enableGridViewThumbnails')}</label>
          <div class="col-span-2">
            <input
              type="checkbox"
              id="setting-generate-thumbnails"
              name="generateThumbnails"
              class="w-4 h-4 rounded focus:ring-2 disabled:opacity-60 disabled:cursor-not-allowed"
              style={{ accentColor: 'hsl(var(--primary))' }}
              checked={settings.generateThumbnails}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.enableGridViewThumbnailsHelp')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.thumbnailsPerRecording')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-thumbnails-per-recording" class="font-medium">{t('settings.thumbnailsPerRecording')}</label>
          <div class="col-span-2">
            <select
              id="setting-thumbnails-per-recording"
              name="thumbnailsPerRecording"
              class="p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.thumbnailsPerRecording}
              onChange={handleInputChange}
              disabled={!canModifySettings || !settings.generateThumbnails}
            >
              <option value={1}>{t('settings.thumbnailsPerRecordingOne')}</option>
              <option value={3}>{t('settings.thumbnailsPerRecordingThree')}</option>
            </select>
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.thumbnailsPerRecordingHelp')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.databasePath')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-db-path" class="font-medium">{t('settings.databasePath')}</label>
          <input
            type="text"
            id="setting-db-path"
            name="dbPath"
            class="col-span-2 p-2 border border-input rounded bg-background text-foreground disabled:opacity-60 disabled:cursor-not-allowed"
            value={settings.dbPath}
            onChange={handleInputChange}
            disabled={!canModifySettings}
          />
        </div>
        <div data-setting-label={t('settings.databaseBackupIntervalMinutes')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-db-backup-interval" class="font-medium">{t('settings.databaseBackupIntervalMinutes')}</label>
          <div class="col-span-2">
            <input
              type="number"
              id="setting-db-backup-interval"
              name="dbBackupIntervalMinutes"
              min="0"
              class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.dbBackupIntervalMinutes}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.databaseBackupIntervalHelp')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.databaseBackupRetentionCopies')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-center mb-4">
          <label for="setting-db-backup-retention" class="font-medium">{t('settings.databaseBackupRetentionCopies')}</label>
          <div class="col-span-2">
            <input
              type="number"
              id="setting-db-backup-retention"
              name="dbBackupRetentionCount"
              min="0"
              class="p-2 border border-input rounded bg-background text-foreground w-full max-w-md disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.dbBackupRetentionCount}
              onChange={handleInputChange}
              disabled={!canModifySettings}
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.databaseBackupRetentionHelpBefore')} <code>.bak</code> {t('settings.databaseBackupRetentionHelpAfter')}</span>
          </div>
        </div>
        <div data-setting-label={t('settings.postBackupScript')} class="setting grid grid-cols-1 md:grid-cols-3 gap-4 items-start mb-4">
          <label for="setting-db-post-backup-script" class="font-medium">{t('settings.postBackupScript')}</label>
          <div class="col-span-2">
            <input
              type="text"
              id="setting-db-post-backup-script"
              name="dbPostBackupScript"
              class="p-2 border border-input rounded bg-background text-foreground w-full max-w-2xl disabled:opacity-60 disabled:cursor-not-allowed"
              value={settings.dbPostBackupScript}
              onChange={handleInputChange}
              disabled={!canModifySettings}
              placeholder="/usr/local/bin/lightnvr-post-backup"
            />
            <span class="hint text-sm text-muted-foreground block mt-1">{t('settings.postBackupScriptHelp')}</span>
          </div>
        </div>
      </div>
    </div>
  );
}

export default StorageTab;
