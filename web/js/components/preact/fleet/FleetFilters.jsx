import { HEALTH_VALUES, RECORDING_VALUES, facetCount, toggleFleetValue } from './fleetQuery.js';

const inputClasses = 'h-4 w-4 rounded border-input text-[hsl(var(--primary))] focus:ring-[hsl(var(--primary))]';

function FilterCheckbox({ id, checked, label, count, onChange }) {
  return (
    <label htmlFor={id} className="flex items-center justify-between gap-3 py-1.5 text-sm cursor-pointer">
      <span className="flex min-w-0 items-center gap-2">
        <input id={id} type="checkbox" className={inputClasses} checked={checked} onChange={onChange} />
        <span className="truncate">{label}</span>
      </span>
      <span className="text-xs tabular-nums text-muted-foreground">{count}</span>
    </label>
  );
}

function FilterGroup({ title, children }) {
  return (
    <fieldset className="border-0 border-b border-border pb-4 last:border-b-0 last:pb-0">
      <legend className="mb-2 text-xs font-semibold uppercase tracking-wide text-muted-foreground">{title}</legend>
      {children}
    </fieldset>
  );
}

export function FleetFilters({ state, facets = {}, locations: locationCatalog = [], collections = [], onChange, t, idPrefix = 'fleet' }) {
  const tags = facets.tags || [];
  const locations = facets.locations || [];
  const locationNames = new Map(locationCatalog.map((location) => [location.uuid, location.path || location.name]));
  const labelHealth = (value) => t(`fleet.health.${value}`);
  const labelRecording = (value) => t(`fleet.recording.${value}`);

  return (
    <div className="space-y-4">
      {collections.length > 0 && (
        <FilterGroup title={t('fleet.filter.collection')}>
          <select
            id={`${idPrefix}-collection`}
            className="w-full rounded-md border border-input bg-background px-3 py-2 text-sm text-foreground"
            value={state.collectionUuid}
            onChange={(event) => onChange({ collectionUuid: event.currentTarget.value })}
            aria-label={t('fleet.filter.collection')}
          >
            <option value="">{t('collections.all')}</option>
            {collections.map((collection) => (
              <option key={collection.uuid} value={collection.uuid}>
                {collection.name} ({collection.effective_count})
              </option>
            ))}
          </select>
        </FilterGroup>
      )}

      <FilterGroup title={t('fleet.filter.health')}>
        {HEALTH_VALUES.map((value) => (
          <FilterCheckbox
            key={value}
            id={`${idPrefix}-health-${value}`}
            checked={state.health.includes(value)}
            label={labelHealth(value)}
            count={facetCount(facets, 'health', value)}
            onChange={() => onChange({ health: toggleFleetValue(state.health, value) })}
          />
        ))}
      </FilterGroup>

      <FilterGroup title={t('fleet.filter.enabled')}>
        <select
          id={`${idPrefix}-enabled`}
          className="w-full rounded-md border border-input bg-background px-3 py-2 text-sm text-foreground"
          value={state.enabled}
          onChange={(event) => onChange({ enabled: event.currentTarget.value })}
          aria-label={t('fleet.filter.enabled')}
        >
          <option value="all">{t('fleet.filter.anyState')}</option>
          <option value="true">{t('common.enabled')} ({facetCount(facets, 'enabled', true)})</option>
          <option value="false">{t('common.disabled')} ({facetCount(facets, 'enabled', false)})</option>
        </select>
      </FilterGroup>

      <FilterGroup title={t('fleet.filter.recording')}>
        {RECORDING_VALUES.map((value) => (
          <FilterCheckbox
            key={value}
            id={`${idPrefix}-recording-${value}`}
            checked={state.recordingModes.includes(value)}
            label={labelRecording(value)}
            count={facetCount(facets, 'recording_mode', value)}
            onChange={() => onChange({ recordingModes: toggleFleetValue(state.recordingModes, value) })}
          />
        ))}
      </FilterGroup>

      <FilterGroup title={t('fleet.filter.location')}>
        <select
          id={`${idPrefix}-location`}
          className="w-full rounded-md border border-input bg-background px-3 py-2 text-sm text-foreground"
          value={state.locationUuid}
          onChange={(event) => onChange({ locationUuid: event.currentTarget.value })}
          aria-label={t('fleet.filter.location')}
        >
          <option value="">{t('fleet.filter.allLocations')}</option>
          {locations.map((location) => (
            <option key={location.uuid} value={location.uuid}>
              {location.label || locationNames.get(location.uuid) || location.uuid} ({location.count})
            </option>
          ))}
        </select>
      </FilterGroup>

      {tags.length > 0 && (
        <FilterGroup title={t('fleet.filter.tags')}>
          <div className="max-h-56 overflow-y-auto pr-1">
            {tags.map((tag) => (
              <FilterCheckbox
                key={tag.uuid}
                id={`${idPrefix}-tag-${tag.uuid}`}
                checked={state.tagUuids.includes(tag.uuid)}
                label={tag.label || tag.uuid}
                count={tag.count}
                onChange={() => onChange({ tagUuids: toggleFleetValue(state.tagUuids, tag.uuid) })}
              />
            ))}
          </div>
        </FilterGroup>
      )}
    </div>
  );
}
