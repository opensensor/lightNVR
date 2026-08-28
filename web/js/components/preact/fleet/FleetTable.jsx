const HEALTH_STYLES = {
  up: 'bg-[hsl(var(--success)/0.14)] text-[hsl(var(--success))]',
  degraded: 'bg-[hsl(var(--warning)/0.18)] text-[hsl(var(--warning-foreground))]',
  down: 'bg-[hsl(var(--danger)/0.14)] text-[hsl(var(--danger))]',
  disabled: 'bg-muted text-muted-foreground',
  unknown: 'bg-muted text-muted-foreground',
};

function SortButton({ field, label, state, onSort }) {
  const active = state.sortBy === field;
  return (
    <button
      type="button"
      className="inline-flex items-center gap-1 rounded px-1 py-0.5 text-left hover:text-foreground focus:outline-none focus:ring-2 focus:ring-[hsl(var(--primary))]"
      onClick={() => onSort(field)}
      aria-label={`${label}${active ? `, ${state.sortOrder}` : ''}`}
    >
      {label}
      <span aria-hidden="true" className={active ? 'text-foreground' : 'text-muted-foreground/50'}>
        {active ? (state.sortOrder === 'asc' ? '↑' : '↓') : '↕'}
      </span>
    </button>
  );
}

function HealthBadge({ health, t }) {
  const value = health || 'unknown';
  return (
    <span className={`inline-flex rounded-full px-2 py-1 text-xs font-semibold ${HEALTH_STYLES[value] || HEALTH_STYLES.unknown}`}>
      {t(`fleet.health.${value}`)}
    </span>
  );
}

function RecordingState({ camera, t }) {
  const isActive = camera.recording_active === true;
  return (
    <div>
      <div className="font-medium">{t(`fleet.recording.${camera.recording_mode || 'off'}`)}</div>
      <div className={`text-xs ${isActive ? 'text-[hsl(var(--success))]' : 'text-muted-foreground'}`}>
        {isActive ? t('fleet.recordingActive') : t('fleet.recordingInactive')}
      </div>
    </div>
  );
}

function CameraTags({ tags = [] }) {
  if (tags.length === 0) return <span className="text-muted-foreground">—</span>;
  return (
    <div className="flex max-w-xs flex-wrap gap-1">
      {tags.slice(0, 3).map((tag) => (
        <span key={tag.uuid} className="rounded-full border border-border bg-muted px-2 py-0.5 text-xs">
          {tag.label || tag.uuid}
        </span>
      ))}
      {tags.length > 3 && <span className="px-1 py-0.5 text-xs text-muted-foreground">+{tags.length - 3}</span>}
    </div>
  );
}

function CameraActions({ camera, onEditCamera, t }) {
  const encodedName = encodeURIComponent(camera.name);
  const classes = 'inline-flex min-h-8 items-center justify-center rounded-md border border-border px-2.5 py-1.5 text-xs font-medium no-underline hover:bg-muted focus:outline-none focus:ring-2 focus:ring-[hsl(var(--primary))]';
  const editLabel = `${t('common.edit')}: ${camera.name}`;
  return (
    <div className="flex flex-wrap justify-end gap-2">
      {camera.can_configure === true && typeof onEditCamera === 'function' && (
        <button
          type="button"
          className={classes}
          onClick={() => onEditCamera(camera.name)}
          title={editLabel}
          aria-label={editLabel}
        >
          <svg className="h-4 w-4" fill="currentColor" viewBox="0 0 20 20" aria-hidden="true">
            <path d="M13.586 3.586a2 2 0 1 1 2.828 2.828l-.793.793-2.828-2.828.793-.793ZM11.379 5.793 3 14.172V17h2.828l8.38-8.379-2.829-2.828Z" />
          </svg>
          <span className="sr-only">{t('common.edit')}</span>
        </button>
      )}
      <a className={classes} href={`index.html?stream=${encodedName}`}>{t('fleet.viewLive')}</a>
      <a className={classes} href={`recordings.html?stream=${encodedName}`}>{t('nav.recordings')}</a>
    </div>
  );
}

function formatLastFrame(timestamp, locale, t) {
  if (!timestamp) return t('common.never');
  const milliseconds = timestamp < 1e12 ? timestamp * 1000 : timestamp;
  const date = new Date(milliseconds);
  return Number.isNaN(date.getTime()) ? t('common.unknown') : date.toLocaleString(locale);
}

function CameraContext({ camera, locale, t }) {
  const capabilityLabels = (camera.capabilities || []).map((value) => value.toUpperCase()).join(' · ');
  return (
    <div className="min-w-0">
      <div className="truncate font-semibold text-foreground">{camera.name}</div>
      <div className="truncate text-xs text-muted-foreground">{camera.address || t('fleet.noAddress')}</div>
      {(camera.manufacturer || camera.model || capabilityLabels) && (
        <div className="mt-1 truncate text-xs text-muted-foreground">
          {[camera.manufacturer, camera.model, capabilityLabels].filter(Boolean).join(' · ')}
        </div>
      )}
      <div className="mt-1 text-xs text-muted-foreground" title={formatLastFrame(camera.last_frame_ts, locale, t)}>
        {t('fleet.lastFrame')}: {formatLastFrame(camera.last_frame_ts, locale, t)}
      </div>
    </div>
  );
}

export function FleetTable({ cameras, state, onSort, locale, t, selectable = false, selectedIds, onToggleCamera, onTogglePage, onEditCamera }) {
  const selectedOnPage = selectable ? cameras.filter((camera) => selectedIds.has(camera.camera_uuid)).length : 0;
  const allOnPageSelected = cameras.length > 0 && selectedOnPage === cameras.length;
  return (
    <>
      <div className="hidden overflow-x-auto md:block">
        <table className="min-w-full divide-y divide-border">
          <thead className="bg-muted/70 text-xs uppercase tracking-wide text-muted-foreground">
            <tr>
              {selectable && <th className="w-10 px-3 py-3 text-left">
                <label className="inline-flex cursor-pointer items-center justify-center">
                  <input
                    type="checkbox"
                    className="h-4 w-4 rounded"
                    checked={allOnPageSelected}
                    ref={(element) => { if (element) element.indeterminate = selectedOnPage > 0 && !allOnPageSelected; }}
                    onChange={(event) => onTogglePage(cameras, event.currentTarget.checked)}
                    aria-label={t('fleet.selectPage')}
                  />
                </label>
              </th>}
              <th className="px-4 py-3 text-left"><SortButton field="name" label={t('fleet.column.camera')} state={state} onSort={onSort} /></th>
              <th className="px-4 py-3 text-left"><SortButton field="health" label={t('fleet.column.health')} state={state} onSort={onSort} /></th>
              <th className="px-4 py-3 text-left"><SortButton field="location" label={t('fleet.column.location')} state={state} onSort={onSort} /></th>
              <th className="px-4 py-3 text-left">{t('fleet.column.tags')}</th>
              <th className="px-4 py-3 text-left"><SortButton field="recording_mode" label={t('fleet.column.recording')} state={state} onSort={onSort} /></th>
              <th className="px-4 py-3 text-right">{t('common.actions')}</th>
            </tr>
          </thead>
          <tbody className="divide-y divide-border bg-card">
            {cameras.map((camera) => (
              <tr key={camera.camera_uuid} className="align-top hover:bg-muted/40">
                {selectable && <td className="w-10 px-3 py-3">
                  <label className="inline-flex cursor-pointer items-center justify-center">
                    <input type="checkbox" className="h-4 w-4 rounded" checked={selectedIds.has(camera.camera_uuid)} onChange={(event) => onToggleCamera(camera, event.currentTarget.checked)} aria-label={t('fleet.selectCamera', { name: camera.name })} />
                  </label>
                </td>}
                <td className="px-4 py-3"><CameraContext camera={camera} locale={locale} t={t} /></td>
                <td className="whitespace-nowrap px-4 py-3">
                  <HealthBadge health={camera.health} t={t} />
                  {camera.current_fps > 0 && <div className="mt-1 text-xs tabular-nums text-muted-foreground">{camera.current_fps.toFixed(1)} FPS</div>}
                </td>
                <td className="max-w-xs px-4 py-3">
                  <div className="truncate" title={camera.location?.path || ''}>{camera.location?.path || t('fleet.unassigned')}</div>
                </td>
                <td className="px-4 py-3"><CameraTags tags={camera.tags} /></td>
                <td className="whitespace-nowrap px-4 py-3"><RecordingState camera={camera} t={t} /></td>
                <td className="px-4 py-3"><CameraActions camera={camera} onEditCamera={onEditCamera} t={t} /></td>
              </tr>
            ))}
          </tbody>
        </table>
      </div>

      <div className="divide-y divide-border md:hidden">
        {cameras.map((camera) => (
          <article key={camera.camera_uuid} className="space-y-3 p-4">
            <div className="flex items-start justify-between gap-3">
              {selectable && <label className="inline-flex cursor-pointer items-center justify-center">
                <input type="checkbox" className="h-4 w-4 flex-none rounded" checked={selectedIds.has(camera.camera_uuid)} onChange={(event) => onToggleCamera(camera, event.currentTarget.checked)} aria-label={t('fleet.selectCamera', { name: camera.name })} />
              </label>}
              <div className="min-w-0 flex-1"><CameraContext camera={camera} locale={locale} t={t} /></div>
              <HealthBadge health={camera.health} t={t} />
            </div>
            <div className="grid grid-cols-2 gap-3 text-sm">
              <div>
                <div className="text-xs uppercase tracking-wide text-muted-foreground">{t('fleet.column.location')}</div>
                <div className="mt-1 break-words">{camera.location?.path || t('fleet.unassigned')}</div>
              </div>
              <div>
                <div className="text-xs uppercase tracking-wide text-muted-foreground">{t('fleet.column.recording')}</div>
                <div className="mt-1"><RecordingState camera={camera} t={t} /></div>
              </div>
            </div>
            <CameraTags tags={camera.tags} />
            <CameraActions camera={camera} onEditCamera={onEditCamera} t={t} />
          </article>
        ))}
      </div>
    </>
  );
}
