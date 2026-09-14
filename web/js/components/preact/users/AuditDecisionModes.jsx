import { useEffect, useMemo, useState } from 'preact/hooks';
import { fetchJSON } from '../../../query-client.js';
import { useI18n } from '../../../i18n.js';
import { showStatusMessage } from '../ToastContainer.jsx';
import {
  AUDIT_DECISION_MODES,
  changedDecisionModes,
  groupDecisionModes,
  modesByAction,
} from './auditHistory.js';

export function AuditDecisionModes({ settings, getAuthHeaders, onSaved }) {
  const { t } = useI18n();
  const entries = useMemo(() => settings?.allowed_decision_modes || [], [settings]);
  const original = useMemo(() => modesByAction(entries), [entries]);
  const groups = useMemo(() => groupDecisionModes(entries), [entries]);
  const [draft, setDraft] = useState(original);
  const [saving, setSaving] = useState(false);

  useEffect(() => { setDraft(original); }, [original]);

  if (entries.length === 0) return null;

  const changes = changedDecisionModes(original, draft);
  const hasChanges = Object.keys(changes).length > 0;
  const minutes = Math.round((Number(settings?.summary_window_seconds) || 900) / 60);

  const save = async () => {
    setSaving(true);
    try {
      const response = await fetchJSON('/api/audit/settings', {
        method: 'PUT',
        headers: { 'Content-Type': 'application/json', ...getAuthHeaders() },
        body: JSON.stringify({ allowed_decision_modes: changes }),
        timeout: 20000,
        retries: 0,
      });
      onSaved(response);
      showStatusMessage(t('audit.modesSaved'), 'success');
    } catch (requestError) {
      showStatusMessage(t('audit.modesError', { message: requestError.message }), 'error', 8000);
    } finally {
      setSaving(false);
    }
  };

  return (
    <section className="mb-4 rounded-lg border border-border bg-muted/40 p-4" aria-labelledby="audit-modes-title">
      <div className="flex flex-wrap items-start justify-between gap-3">
        <div>
          <h3 id="audit-modes-title" className="font-semibold">{t('audit.modesTitle')}</h3>
          <p className="mt-1 text-xs text-muted-foreground">{t('audit.modesDescription', { minutes })}</p>
        </div>
        <button type="button" className="btn-secondary" onClick={save} disabled={saving || !hasChanges}>
          {saving ? t('common.saving') : t('common.saveChanges')}
        </button>
      </div>
      <div className="mt-3 grid gap-4 lg:grid-cols-2">
        {groups.map((group) => (
          <fieldset key={group.category} className="rounded-md border border-border p-3">
            <legend className="px-1 text-xs font-medium uppercase tracking-wide text-muted-foreground">{group.category}</legend>
            {group.actions.map((entry) => (
              <div key={entry.action} className="mt-2">
                <label className="flex flex-wrap items-center justify-between gap-2 text-sm">
                  <span>
                    <span className="font-mono text-xs">{entry.action}</span>
                    <span className="block text-xs text-muted-foreground">{entry.description}</span>
                  </span>
                  <select
                    className="rounded-md border border-input bg-background px-2 py-1 text-sm"
                    value={draft[entry.action] || 'record'}
                    disabled={saving}
                    onChange={(event) => {
                      const mode = event.currentTarget.value;
                      setDraft((current) => ({ ...current, [entry.action]: mode }));
                    }}
                  >
                    {AUDIT_DECISION_MODES.map((mode) => (
                      <option key={mode} value={mode}>{t(`audit.mode.${mode}`)}</option>
                    ))}
                  </select>
                </label>
                {draft[entry.action] === 'off' && (
                  <p className="mt-1 rounded px-2 py-1 text-xs badge-warning">{t('audit.modeOffWarning')}</p>
                )}
              </div>
            ))}
          </fieldset>
        ))}
      </div>
    </section>
  );
}
