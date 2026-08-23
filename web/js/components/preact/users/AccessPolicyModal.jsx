import { useCallback, useEffect, useMemo, useState } from 'preact/hooks';
import { fetchJSON } from '../../../query-client.js';
import { useI18n } from '../../../i18n.js';
import { CameraValuePicker, CollectionSelectorBuilder } from '../fleet/CollectionSelectorBuilder.jsx';
import { showStatusMessage } from '../ToastContainer.jsx';
import {
  ALL_CAMERAS_SELECTOR,
  actionRequiresCamera,
  buildPolicyPayload,
  createDraftGrant,
  groupActionsByCategory,
  policyResponseToDraft,
  validatePolicyDraft,
} from './authorizationPolicy.js';

const fieldClasses = 'w-full rounded-md border border-input bg-background px-3 py-2 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-[hsl(var(--primary))]';

function policyFingerprint(mode, grants) {
  return JSON.stringify(buildPolicyPayload(mode, grants, 0));
}

function GrantEditor({ grant, roles, collections, locations, tags, onChange, onSelectorChange, onRemove, t, requestHeaders }) {
  const handleSelectorChange = useCallback(
    (selector, error) => onSelectorChange(grant.draftId, selector, error),
    [grant.draftId, onSelectorChange]
  );
  const selectedRole = roles.find((role) => role.uuid === grant.roleUuid);
  return (
    <article className="rounded-lg border border-border bg-muted/10 p-4">
      <div className="flex flex-wrap items-start justify-between gap-3">
        <div className="grid min-w-0 flex-1 gap-3 sm:grid-cols-2">
          <label className="text-sm font-medium">
            {t('access.policy.role')}
            <select className={`${fieldClasses} mt-1`} value={grant.roleUuid} onChange={(event) => onChange(grant.draftId, { roleUuid: event.currentTarget.value })}>
              <option value="">{t('access.policy.chooseRole')}</option>
              {roles.map((role) => <option key={role.uuid} value={role.uuid}>{role.name}</option>)}
            </select>
          </label>
          <label className="text-sm font-medium">
            {t('access.policy.scope')}
            <select className={`${fieldClasses} mt-1`} value={grant.scopeType} onChange={(event) => onChange(grant.draftId, {
              scopeType: event.currentTarget.value,
              selector: event.currentTarget.value === 'selector' ? (grant.selector || ALL_CAMERAS_SELECTOR) : null,
              collectionUuid: event.currentTarget.value === 'collection' ? grant.collectionUuid : '',
              selectorError: '',
            })}>
              <option value="all">{t('access.policy.allCameras')}</option>
              <option value="collection">{t('access.policy.collectionScope')}</option>
              <option value="selector">{t('access.policy.dynamicScope')}</option>
            </select>
          </label>
        </div>
        <button type="button" className="rounded px-2 py-1 text-sm text-[hsl(var(--danger))] hover:bg-muted" onClick={() => onRemove(grant.draftId)}>{t('common.remove')}</button>
      </div>
      {selectedRole?.description && <p className="mt-2 text-xs text-muted-foreground">{selectedRole.description}</p>}
      {grant.scopeType === 'collection' && (
        <label className="mt-4 block text-sm font-medium">
          {t('access.policy.collection')}
          <select className={`${fieldClasses} mt-1`} value={grant.collectionUuid} onChange={(event) => onChange(grant.draftId, { collectionUuid: event.currentTarget.value })}>
            <option value="">{t('access.policy.chooseCollection')}</option>
            {collections.map((collection) => <option key={collection.uuid} value={collection.uuid}>{collection.name} ({collection.effective_count})</option>)}
          </select>
          <span className="mt-1 block text-xs font-normal text-muted-foreground">{t('access.policy.collectionHelp')}</span>
        </label>
      )}
      {grant.scopeType === 'selector' && (
        <div className="mt-4">
          <CollectionSelectorBuilder
            key={grant.draftId}
            idPrefix={`access-${grant.draftId}`}
            initialSelector={grant.selector || ALL_CAMERAS_SELECTOR}
            locations={locations}
            tags={tags}
            allowCameraSelection
            requestHeaders={requestHeaders}
            onChange={handleSelectorChange}
            t={t}
          />
          {grant.selectorError && <p className="mt-2 text-xs text-[hsl(var(--danger))]">{grant.selectorError}</p>}
        </div>
      )}
    </article>
  );
}

function SimulationPanel({ user, actions, dirty, getAuthHeaders, t }) {
  const [actionKey, setActionKey] = useState(actions[0]?.key || 'live.view');
  const [cameraUuids, setCameraUuids] = useState([]);
  const [running, setRunning] = useState(false);
  const [result, setResult] = useState(null);
  const [error, setError] = useState('');
  const groups = useMemo(() => groupActionsByCategory(actions), [actions]);
  const cameraScoped = actionRequiresCamera(actionKey, actions);

  useEffect(() => {
    if (!actions.some((action) => action.key === actionKey) && actions[0]) {
      setActionKey(actions[0].key);
    }
  }, [actions, actionKey]);

  const simulate = async () => {
    if (dirty || (cameraScoped && cameraUuids.length === 0)) return;
    setRunning(true);
    setError('');
    setResult(null);
    try {
      const body = { user_id: user.id, action: actionKey };
      if (cameraScoped) body.camera_uuid = cameraUuids[0];
      setResult(await fetchJSON('/api/authorization/simulate', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', ...getAuthHeaders() },
        body: JSON.stringify(body),
        timeout: 15000,
        retries: 0,
      }));
    } catch (requestError) {
      setError(requestError.message);
    } finally {
      setRunning(false);
    }
  };

  return (
    <div className="space-y-4">
      <div><h3 className="text-lg font-semibold">{t('access.test.title')}</h3><p className="mt-1 text-sm text-muted-foreground">{t('access.test.description', { username: user.username })}</p></div>
      {dirty && <p className="rounded-md bg-[hsl(var(--warning)/0.13)] p-3 text-sm">{t('access.test.saveFirst')}</p>}
      <label className="block text-sm font-medium">{t('access.test.action')}
        <select className={`${fieldClasses} mt-1`} value={actionKey} onChange={(event) => { setActionKey(event.currentTarget.value); setResult(null); setCameraUuids([]); }}>
          {groups.map((group) => <optgroup key={group.category} label={group.category}>{group.actions.map((action) => <option key={action.key} value={action.key}>{action.key} — {action.description}</option>)}</optgroup>)}
        </select>
      </label>
      {cameraScoped && (
        <div>
          <p className="mb-1 text-sm font-medium">{t('access.test.camera')}</p>
          <CameraValuePicker values={cameraUuids} onChange={(values) => setCameraUuids(values.slice(-1))} idPrefix="access-simulation-camera" t={t} requestHeaders={getAuthHeaders()} />
        </div>
      )}
      <div className="flex justify-end"><button type="button" className="btn-primary" onClick={simulate} disabled={running || dirty || (cameraScoped && cameraUuids.length === 0)}>{running ? t('access.test.running') : t('access.test.run')}</button></div>
      {error && <p className="rounded-md badge-danger p-3">{error}</p>}
      {result && (
        <div className={`rounded-lg border p-4 ${result.allowed ? 'border-[hsl(var(--success))] bg-[hsl(var(--success)/0.1)]' : 'border-[hsl(var(--danger))] bg-[hsl(var(--danger)/0.08)]'}`}>
          <div className="flex items-center justify-between gap-3"><h4 className="font-semibold">{t(result.allowed ? 'access.test.allowed' : 'access.test.denied')}</h4><span className="text-xs text-muted-foreground">{t('access.test.version', { version: result.policy_version })}</span></div>
          <p className="mt-2 text-sm">{result.explanation}</p>
          <p className="mt-2 text-xs text-muted-foreground">{t('access.test.source', { source: result.source, role: result.role || '—' })}</p>
        </div>
      )}
    </div>
  );
}

export function AccessPolicyModal({ user, onClose, getAuthHeaders }) {
  const { t } = useI18n();
  const [tab, setTab] = useState('grants');
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState('');
  const [roles, setRoles] = useState([]);
  const [actions, setActions] = useState([]);
  const [collections, setCollections] = useState([]);
  const [locations, setLocations] = useState([]);
  const [tags, setTags] = useState([]);
  const [mode, setMode] = useState('legacy');
  const [loadedMode, setLoadedMode] = useState('legacy');
  const [policyVersion, setPolicyVersion] = useState(0);
  const [grants, setGrants] = useState([]);
  const [savedFingerprint, setSavedFingerprint] = useState('');

  const applyPolicyResponse = useCallback((response) => {
    const draft = policyResponseToDraft(response);
    setMode(draft.mode);
    setLoadedMode(draft.mode);
    setPolicyVersion(draft.policyVersion);
    setGrants(draft.grants);
    setSavedFingerprint(policyFingerprint(draft.mode, draft.grants));
  }, []);

  const load = useCallback(async () => {
    setLoading(true);
    setError('');
    try {
      const [actionResponse, roleResponse, policyResponse, collectionResponse, locationResponse, tagResponse] = await Promise.all([
        fetchJSON('/api/authorization/actions', { headers: getAuthHeaders(), cache: 'no-store' }),
        fetchJSON('/api/authorization/roles', { headers: getAuthHeaders(), cache: 'no-store' }),
        fetchJSON(`/api/authorization/users/${encodeURIComponent(user.id)}`, { headers: getAuthHeaders(), cache: 'no-store' }),
        fetchJSON('/api/camera-collections', { headers: getAuthHeaders(), cache: 'no-store' }),
        fetchJSON('/api/locations', { headers: getAuthHeaders(), cache: 'no-store' }),
        fetchJSON('/api/camera-tags', { headers: getAuthHeaders(), cache: 'no-store' }),
      ]);
      setActions(actionResponse.actions || []);
      setRoles(roleResponse.roles || []);
      setCollections((collectionResponse.collections || []).filter((collection) => collection.shared));
      setLocations(locationResponse.locations || []);
      setTags(tagResponse.tags || []);
      applyPolicyResponse(policyResponse);
    } catch (requestError) {
      setError(requestError.message);
    } finally {
      setLoading(false);
    }
  }, [applyPolicyResponse, getAuthHeaders, user.id]);

  useEffect(() => { load(); }, [load]);
  useEffect(() => {
    const handleKey = (event) => { if (event.key === 'Escape' && !saving) onClose(); };
    window.addEventListener('keydown', handleKey);
    return () => window.removeEventListener('keydown', handleKey);
  }, [onClose, saving]);

  const fingerprint = useMemo(() => policyFingerprint(mode, grants), [mode, grants]);
  const dirty = policyVersion > 0 && fingerprint !== savedFingerprint;
  const validationCode = useMemo(
    () => validatePolicyDraft(
      mode,
      grants,
      new Set(roles.map((role) => role.uuid)),
      new Set(collections.map((collection) => collection.uuid))
    ),
    [mode, grants, roles, collections]
  );

  const updateGrant = useCallback((draftId, changes) => {
    setGrants((current) => current.map((grant) => grant.draftId === draftId ? { ...grant, ...changes } : grant));
  }, []);
  const updateSelector = useCallback((draftId, selector, selectorError) => {
    updateGrant(draftId, { selector, selectorError });
  }, [updateGrant]);
  const removeGrant = useCallback((draftId) => {
    setGrants((current) => current.filter((grant) => grant.draftId !== draftId));
  }, []);
  const addGrant = () => setGrants((current) => [
    ...current,
    createDraftGrant(roles[0]?.uuid || '', 'all'),
  ]);

  const save = async () => {
    if (validationCode) {
      showStatusMessage(t(`access.policy.validation.${validationCode}`), 'error', 7000);
      return;
    }
    if (loadedMode !== 'policy' && mode === 'policy' &&
        !window.confirm(t('access.policy.activateConfirm', { username: user.username }))) return;
    if (mode === 'policy' && grants.length === 0 &&
        !window.confirm(t('access.policy.emptyConfirm', { username: user.username }))) return;
    setSaving(true);
    try {
      const response = await fetchJSON(`/api/authorization/users/${encodeURIComponent(user.id)}`, {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json', ...getAuthHeaders() },
        body: JSON.stringify(buildPolicyPayload(mode, grants, policyVersion)),
        timeout: 30000,
        retries: 0,
      });
      applyPolicyResponse(response);
      showStatusMessage(t('access.policy.saved'), 'success');
    } catch (requestError) {
      showStatusMessage(t('access.policy.saveError', { message: requestError.message }), 'error', 8000);
      if (requestError.status === 409) await load();
    } finally {
      setSaving(false);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-3" onClick={() => !saving && onClose()}>
      <section role="dialog" aria-modal="true" aria-labelledby="access-policy-title" className="flex max-h-[95vh] w-full max-w-6xl flex-col overflow-hidden rounded-lg bg-card text-card-foreground shadow-xl" onClick={(event) => event.stopPropagation()}>
        <header className="flex items-start justify-between gap-4 border-b border-border p-5">
          <div><h2 id="access-policy-title" className="text-xl font-bold">{t('access.policy.title', { username: user.username })}</h2><p className="mt-1 text-sm text-muted-foreground">{t('access.policy.description')}</p></div>
          <button type="button" className="btn-secondary" onClick={onClose} disabled={saving}>{t('common.close')}</button>
        </header>
        <div className="border-b border-border px-5 pt-2" role="tablist">
          {['grants', 'test'].map((value) => <button key={value} type="button" role="tab" aria-selected={tab === value} className={`mr-2 rounded-t-md px-4 py-2 text-sm font-medium ${tab === value ? 'border border-b-0 border-border bg-background -mb-px' : 'text-muted-foreground'}`} onClick={() => setTab(value)}>{t(`access.policy.tab.${value}`)}</button>)}
        </div>

        <div className="min-h-0 flex-1 overflow-y-auto p-5">
          {error && <div className="rounded-md badge-danger p-3">{error} <button type="button" className="ml-2 underline" onClick={load}>{t('common.retry')}</button></div>}
          {loading && grants.length === 0 ? <div className="p-10 text-center">{t('common.loading')}</div> : tab === 'grants' ? (
            <div className="space-y-4">
              <div className="grid gap-3 rounded-lg border border-border p-4 sm:grid-cols-[minmax(0,1fr)_18rem] sm:items-start">
                <div><h3 className="font-semibold">{t('access.policy.mode')}</h3><p className="mt-1 text-sm text-muted-foreground">{t('access.policy.modeDescription')}</p></div>
                <select className={fieldClasses} value={mode} onChange={(event) => setMode(event.currentTarget.value)}>
                  <option value="legacy">{t('access.policy.legacy')}</option>
                  <option value="policy">{t('access.policy.scoped')}</option>
                </select>
              </div>
              {mode === 'legacy' && <p className="rounded-md bg-[hsl(var(--info)/0.12)] p-3 text-sm">{t('access.policy.legacyHelp')}</p>}
              {mode === 'policy' && <p className="rounded-md bg-[hsl(var(--warning)/0.13)] p-3 text-sm">{t('access.policy.scopedWarning')}</p>}

              <div className="flex items-center justify-between gap-3"><div><h3 className="text-lg font-semibold">{t('access.policy.grants')}</h3><p className="text-sm text-muted-foreground">{t('access.policy.grantsDescription')}</p></div><button type="button" className="btn-secondary whitespace-nowrap" onClick={addGrant} disabled={roles.length === 0}>{t('access.policy.addGrant')}</button></div>
              <div className="space-y-3">
                {grants.map((grant) => <GrantEditor key={grant.draftId} grant={grant} roles={roles} collections={collections} locations={locations} tags={tags} onChange={updateGrant} onSelectorChange={updateSelector} onRemove={removeGrant} t={t} requestHeaders={getAuthHeaders()} />)}
                {grants.length === 0 && <div className="rounded-lg border border-dashed border-border p-8 text-center"><h4 className="font-semibold">{t('access.policy.noGrants')}</h4><p className="mt-1 text-sm text-muted-foreground">{t('access.policy.noGrantsHelp')}</p></div>}
              </div>
              {validationCode && <p className="text-sm text-[hsl(var(--danger))]">{t(`access.policy.validation.${validationCode}`)}</p>}
            </div>
          ) : <SimulationPanel user={user} actions={actions} dirty={dirty} getAuthHeaders={getAuthHeaders} t={t} />}
        </div>

        <footer className="flex flex-wrap items-center justify-between gap-3 border-t border-border p-4">
          <span className="text-xs text-muted-foreground">{t('access.policy.version', { version: policyVersion })}{dirty ? ` · ${t('access.policy.unsaved')}` : ''}</span>
          <div className="flex gap-2"><button type="button" className="btn-secondary" onClick={onClose} disabled={saving}>{t('common.cancel')}</button><button type="button" className="btn-primary" onClick={save} disabled={saving || loading || policyVersion < 1 || !dirty || Boolean(validationCode)}>{saving ? t('common.saving') : t('common.saveChanges')}</button></div>
        </footer>
      </section>
    </div>
  );
}
