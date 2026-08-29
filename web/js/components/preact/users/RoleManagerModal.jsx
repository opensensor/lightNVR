import { useCallback, useEffect, useMemo, useState } from 'preact/hooks';
import { fetchJSON } from '../../../query-client.js';
import { useI18n } from '../../../i18n.js';
import { showStatusMessage } from '../ToastContainer.jsx';
import { ConfirmDialog } from '../common/ModalDialog.jsx';
import {
  actionIsEnforced,
  groupActionsByCategory,
  roleHasDestructiveActions,
} from './authorizationPolicy.js';

const fieldClasses = 'w-full rounded-md border border-input bg-background px-3 py-2 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-[hsl(var(--primary))]';

function roleToDraft(role) {
  return {
    uuid: role?.uuid || '',
    name: role?.name || '',
    description: role?.description || '',
    actions: [...(role?.actions || [])],
    builtin: role?.builtin === true,
  };
}

export function RoleManagerModal({ onClose, getAuthHeaders }) {
  const { t } = useI18n();
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState('');
  const [roles, setRoles] = useState([]);
  const [actions, setActions] = useState([]);
  const [policyVersion, setPolicyVersion] = useState(0);
  const [selectedUuid, setSelectedUuid] = useState('');
  const [draft, setDraft] = useState(roleToDraft(null));
  const [confirmDelete, setConfirmDelete] = useState(false);

  const load = useCallback(async (preferredUuid = '') => {
    setLoading(true);
    setError('');
    try {
      const [actionResponse, roleResponse] = await Promise.all([
        fetchJSON('/api/authorization/actions', { headers: getAuthHeaders(), cache: 'no-store' }),
        fetchJSON('/api/authorization/roles', { headers: getAuthHeaders(), cache: 'no-store' }),
      ]);
      const loadedRoles = roleResponse.roles || [];
      const nextUuid = preferredUuid && loadedRoles.some((role) => role.uuid === preferredUuid)
        ? preferredUuid
        : loadedRoles[0]?.uuid || '';
      setActions(actionResponse.actions || []);
      setRoles(loadedRoles);
      setPolicyVersion(roleResponse.policy_version || 0);
      setSelectedUuid(nextUuid);
      setDraft(roleToDraft(loadedRoles.find((role) => role.uuid === nextUuid)));
    } catch (requestError) {
      setError(requestError.message);
    } finally {
      setLoading(false);
    }
  }, [getAuthHeaders]);

  useEffect(() => { load(); }, [load]);
  useEffect(() => {
    const handleKey = (event) => { if (event.key === 'Escape' && !saving) onClose(); };
    window.addEventListener('keydown', handleKey);
    return () => window.removeEventListener('keydown', handleKey);
  }, [onClose, saving]);

  const groupedActions = useMemo(() => groupActionsByCategory(actions), [actions]);
  const destructive = useMemo(
    () => roleHasDestructiveActions(draft.actions, actions),
    [draft.actions, actions]
  );

  const selectRole = (role) => {
    setSelectedUuid(role.uuid);
    setDraft(roleToDraft(role));
  };
  const startNew = (source = null) => {
    setSelectedUuid('');
    setDraft({
      ...roleToDraft(source),
      uuid: '',
      name: source ? t('access.roles.copyName', { name: source.name }) : '',
      builtin: false,
    });
  };
  const toggleAction = (key) => {
    setDraft((current) => ({
      ...current,
      actions: current.actions.includes(key)
        ? current.actions.filter((action) => action !== key)
        : [...current.actions, key],
    }));
  };

  const saveRole = async (event) => {
    event.preventDefault();
    if (draft.builtin || !draft.name.trim() || draft.actions.length === 0) return;
    setSaving(true);
    try {
      const response = await fetchJSON(
        draft.uuid ? `/api/authorization/roles/${encodeURIComponent(draft.uuid)}` : '/api/authorization/roles',
        {
          method: draft.uuid ? 'PUT' : 'POST',
          headers: { 'Content-Type': 'application/json', ...getAuthHeaders() },
          body: JSON.stringify({
            expected_policy_version: policyVersion,
            name: draft.name.trim(),
            description: draft.description.trim(),
            actions: draft.actions,
          }),
          timeout: 20000,
          retries: 0,
        }
      );
      showStatusMessage(t(draft.uuid ? 'access.roles.updated' : 'access.roles.created'), 'success');
      await load(response.role?.uuid || draft.uuid);
    } catch (requestError) {
      showStatusMessage(t('access.roles.saveError', { message: requestError.message }), 'error', 8000);
      if (requestError.status === 409) await load(draft.uuid);
    } finally {
      setSaving(false);
    }
  };

  const deleteRole = async () => {
    if (!draft.uuid || draft.builtin) return;
    setSaving(true);
    try {
      await fetchJSON(`/api/authorization/roles/${encodeURIComponent(draft.uuid)}`, {
        method: 'DELETE',
        headers: { 'Content-Type': 'application/json', ...getAuthHeaders() },
        body: JSON.stringify({ expected_policy_version: policyVersion }),
        timeout: 20000,
        retries: 0,
      });
      showStatusMessage(t('access.roles.deleted'), 'success');
      await load();
    } catch (requestError) {
      showStatusMessage(t('access.roles.deleteError', { message: requestError.message }), 'error', 8000);
      if (requestError.status === 409) await load(draft.uuid);
    } finally {
      setSaving(false);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-3" onClick={() => !saving && onClose()}>
      <section role="dialog" aria-modal="true" aria-labelledby="role-manager-title" className="flex max-h-[94vh] w-full max-w-6xl flex-col overflow-hidden rounded-lg bg-card text-card-foreground shadow-xl" onClick={(event) => event.stopPropagation()}>
        <header className="flex items-start justify-between gap-4 border-b border-border p-5">
          <div><h2 id="role-manager-title" className="text-xl font-bold">{t('access.roles.title')}</h2><p className="mt-1 text-sm text-muted-foreground">{t('access.roles.description')}</p></div>
          <button type="button" className="btn-secondary" onClick={onClose} disabled={saving}>{t('common.close')}</button>
        </header>

        {error && <div className="m-5 rounded-md badge-danger p-3">{error} <button type="button" className="ml-2 underline" onClick={() => load()}>{t('common.retry')}</button></div>}
        {loading && roles.length === 0 ? <div className="p-10 text-center">{t('common.loading')}</div> : (
          <div className="grid min-h-0 flex-1 md:grid-cols-[18rem_1fr]">
            <aside className="overflow-y-auto border-b border-border p-4 md:border-b-0 md:border-r">
              <button type="button" className="btn-primary mb-3 w-full" onClick={() => startNew()}>{t('access.roles.create')}</button>
              <div className="space-y-2">
                {roles.map((role) => (
                  <button key={role.uuid} type="button" className={`w-full rounded-md border p-3 text-left ${selectedUuid === role.uuid ? 'border-[hsl(var(--primary))] bg-[hsl(var(--primary)/0.1)]' : 'border-border hover:bg-muted'}`} onClick={() => selectRole(role)}>
                    <span className="block font-medium">{role.name}</span>
                    <span className="mt-1 block text-xs text-muted-foreground">{t('access.roles.actionCount', { count: role.actions?.length || 0 })}{role.builtin ? ` · ${t('access.roles.builtin')}` : ''}</span>
                  </button>
                ))}
              </div>
            </aside>

            <form className="min-h-0 overflow-y-auto p-5" onSubmit={saveRole}>
              <div className="mb-4 flex flex-wrap items-start justify-between gap-3">
                <div><h3 className="text-lg font-semibold">{draft.uuid ? draft.name : t('access.roles.newRole')}</h3>{draft.builtin && <p className="text-xs text-muted-foreground">{t('access.roles.builtinHelp')}</p>}</div>
                {draft.uuid && <button type="button" className="btn-secondary" onClick={() => startNew(draft)}>{t('access.roles.clone')}</button>}
              </div>
              <div className="grid gap-3 sm:grid-cols-2">
                <label className="text-sm font-medium">{t('common.name')}<input className={`${fieldClasses} mt-1`} value={draft.name} maxLength="127" disabled={draft.builtin} onInput={(event) => setDraft({ ...draft, name: event.currentTarget.value })} required /></label>
                <label className="text-sm font-medium sm:col-span-2">{t('common.description')}<textarea className={`${fieldClasses} mt-1`} rows="2" maxLength="255" disabled={draft.builtin} value={draft.description} onInput={(event) => setDraft({ ...draft, description: event.currentTarget.value })}></textarea></label>
              </div>

              <div className="mt-5 space-y-5">
                {groupedActions.map((group) => (
                  <fieldset key={group.category}>
                    <legend className="mb-2 font-semibold">{group.category}</legend>
                    <div className="grid gap-2 lg:grid-cols-2">
                      {group.actions.map((action) => {
                        const checked = draft.actions.includes(action.key);
                        return (
                          <label key={action.key} className={`touch-target flex gap-3 rounded-md border p-3 ${checked ? 'border-[hsl(var(--primary))] bg-[hsl(var(--primary)/0.06)]' : 'border-border'} ${draft.builtin ? 'cursor-default' : 'cursor-pointer'}`}>
                            <input type="checkbox" className="mt-1 h-4 w-4" checked={checked} disabled={draft.builtin} onChange={() => toggleAction(action.key)} />
                            <span><span className="block text-sm font-medium">{action.key}</span><span className="mt-0.5 block text-xs text-muted-foreground">{action.description}</span>{action.destructive && <span className="mt-1 block text-xs font-medium text-[hsl(var(--danger))]">{t('access.roles.destructive')}</span>}{!actionIsEnforced(action) && <span className="mt-1 block text-xs font-medium text-[hsl(var(--warning))]">{t('access.roles.notEnforced')}</span>}</span>
                          </label>
                        );
                      })}
                    </div>
                  </fieldset>
                ))}
              </div>
              {destructive && !draft.builtin && <p className="mt-4 rounded-md bg-[hsl(var(--warning)/0.13)] p-3 text-sm">{t('access.roles.destructiveWarning')}</p>}

              <footer className="mt-5 flex flex-wrap justify-end gap-2 border-t border-border pt-4">
                {draft.uuid && !draft.builtin && <button type="button" className="btn-danger mr-auto" onClick={() => setConfirmDelete(true)} disabled={saving}>{t('common.delete')}</button>}
                {!draft.builtin && <button type="submit" className="btn-primary" disabled={saving || !draft.name.trim() || draft.actions.length === 0}>{saving ? t('common.saving') : t('common.saveChanges')}</button>}
              </footer>
            </form>
          </div>
        )}
      </section>
      <ConfirmDialog
        isOpen={confirmDelete}
        onClose={() => setConfirmDelete(false)}
        onConfirm={deleteRole}
        title={t('access.roles.title')}
        message={t('access.roles.deleteConfirm', { name: draft.name })}
        confirmLabel={t('common.delete')}
        cancelLabel={t('common.cancel')}
        variant="danger"
      />
    </div>
  );
}
