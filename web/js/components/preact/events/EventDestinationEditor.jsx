import { useEffect, useMemo, useState } from 'preact/hooks';
import { showStatusMessage } from '../ToastContainer.jsx';
import {
  buildDestinationPayload,
  createDestinationDraft,
  destinationToDraft,
  validateDestinationDraft,
} from './eventRouting.js';

const fieldClasses = 'mt-1 w-full rounded-md border border-input bg-background px-3 py-2 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-[hsl(var(--primary))]';

function Field({ label, help, children }) {
  return (
    <label className="block text-sm font-medium">
      {label}
      {children}
      {help && <span className="mt-1 block text-xs font-normal text-muted-foreground">{help}</span>}
    </label>
  );
}

export function EventDestinationEditor({ destination, onSave, onClose, t }) {
  const creating = !destination;
  const [draft, setDraft] = useState(() => destination ? destinationToDraft(destination) : createDestinationDraft());
  const [saving, setSaving] = useState(false);
  const validationCode = useMemo(() => validateDestinationDraft(draft), [draft]);

  useEffect(() => {
    const handleKey = (event) => {
      if (event.key === 'Escape' && !saving) onClose();
    };
    window.addEventListener('keydown', handleKey);
    return () => window.removeEventListener('keydown', handleKey);
  }, [onClose, saving]);

  const update = (changes) => setDraft((current) => ({ ...current, ...changes }));
  const submit = async (event) => {
    event.preventDefault();
    if (validationCode) {
      showStatusMessage(t(`events.destination.validation.${validationCode}`), 'error', 7000);
      return;
    }
    setSaving(true);
    try {
      await onSave(buildDestinationPayload(draft, creating));
    } catch (_error) {
      // The workspace surfaces request errors and keeps this editor open.
    } finally {
      setSaving(false);
    }
  };

  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/55 p-4" role="dialog" aria-modal="true" aria-labelledby="event-destination-editor-title">
      <form className="flex max-h-[92vh] w-full max-w-4xl flex-col overflow-hidden rounded-xl border border-border bg-card text-card-foreground shadow-2xl" onSubmit={submit}>
        <div className="flex items-start justify-between gap-4 border-b border-border p-5">
          <div>
            <h2 id="event-destination-editor-title" className="text-xl font-bold">{t(creating ? 'events.destination.addTitle' : 'events.destination.editTitle')}</h2>
            <p className="mt-1 text-sm text-muted-foreground">{t('events.destination.editorDescription')}</p>
          </div>
          <button type="button" className="rounded px-2 text-2xl text-muted-foreground hover:bg-muted hover:text-foreground" onClick={onClose} aria-label={t('common.close')}>×</button>
        </div>

        <div className="space-y-6 overflow-y-auto p-5">
          <section>
            <h3 className="mb-3 font-semibold">{t('events.destination.identity')}</h3>
            <div className="grid gap-4 md:grid-cols-2">
              <Field label={t('common.name')}>
                <input className={fieldClasses} value={draft.name} maxLength="127" autoFocus onInput={(event) => update({ name: event.currentTarget.value })} />
              </Field>
              <label className="flex items-center gap-3 self-end rounded-md border border-border bg-muted/20 px-3 py-2.5 text-sm">
                <input type="checkbox" checked={draft.enabled} onChange={(event) => update({ enabled: event.currentTarget.checked })} />
                <span><span className="block font-medium">{t('events.destination.enabled')}</span><span className="block text-xs text-muted-foreground">{t('events.destination.enabledHelp')}</span></span>
              </label>
              <div className="md:col-span-2">
                <Field label={t('common.description')}>
                  <textarea className={`${fieldClasses} min-h-20`} value={draft.description} maxLength="511" onInput={(event) => update({ description: event.currentTarget.value })}></textarea>
                </Field>
              </div>
            </div>
          </section>

          <section className="border-t border-border pt-5">
            <h3 className="mb-3 font-semibold">{t('events.destination.broker')}</h3>
            <div className="grid gap-4 md:grid-cols-3">
              <div className="md:col-span-2">
                <Field label={t('events.destination.host')} help={t('events.destination.hostHelp')}>
                  <input className={fieldClasses} value={draft.host} maxLength="255" placeholder="mqtt.example.net" onInput={(event) => update({ host: event.currentTarget.value })} />
                </Field>
              </div>
              <Field label={t('events.destination.port')}>
                <input className={fieldClasses} type="number" min="1" max="65535" value={draft.port} onInput={(event) => update({ port: event.currentTarget.value })} />
              </Field>
              <div className="md:col-span-2">
                <Field label={t('events.destination.clientId')} help={t('events.destination.clientIdHelp')}>
                  <input className={fieldClasses} value={draft.clientId} maxLength="127" onInput={(event) => update({ clientId: event.currentTarget.value })} />
                </Field>
              </div>
              <Field label={t('events.destination.qos')}>
                <select className={fieldClasses} value={draft.qos} onChange={(event) => update({ qos: Number(event.currentTarget.value) })}>
                  <option value="0">0 — {t('events.destination.qos0')}</option>
                  <option value="1">1 — {t('events.destination.qos1')}</option>
                  <option value="2">2 — {t('events.destination.qos2')}</option>
                </select>
              </Field>
              <div className="md:col-span-2">
                <Field label={t('events.destination.topicTemplate')} help={t('events.destination.topicHelp')}>
                  <input className={`${fieldClasses} font-mono`} value={draft.topicTemplate} maxLength="511" onInput={(event) => update({ topicTemplate: event.currentTarget.value })} />
                </Field>
              </div>
              <Field label={t('events.destination.keepalive')}>
                <input className={fieldClasses} type="number" min="5" max="3600" value={draft.keepaliveSeconds} onInput={(event) => update({ keepaliveSeconds: event.currentTarget.value })} />
              </Field>
            </div>
          </section>

          <section className="border-t border-border pt-5">
            <h3 className="mb-3 font-semibold">{t('events.destination.authentication')}</h3>
            <div className="grid gap-4 md:grid-cols-2">
              <Field label={t('auth.username')}>
                <input className={fieldClasses} autoComplete="off" value={draft.username} maxLength="127" onInput={(event) => update({ username: event.currentTarget.value })} />
              </Field>
              <Field label={t('auth.password')} help={draft.passwordConfigured ? t('events.destination.passwordPreserved') : t('events.destination.passwordOptional')}>
                <input className={fieldClasses} type="password" autoComplete="new-password" value={draft.password} maxLength="255" disabled={draft.clearPassword} placeholder={draft.passwordConfigured ? '••••••••' : ''} onInput={(event) => update({ password: event.currentTarget.value, clearPassword: false })} />
              </Field>
            </div>
            {!creating && draft.passwordConfigured && (
              <label className="mt-3 inline-flex items-center gap-2 text-sm text-muted-foreground">
                <input type="checkbox" checked={draft.clearPassword} onChange={(event) => update({ clearPassword: event.currentTarget.checked, password: '' })} />
                {t('events.destination.clearPassword')}
              </label>
            )}
          </section>

          <section className="border-t border-border pt-5">
            <h3 className="mb-3 font-semibold">{t('events.destination.transportSecurity')}</h3>
            <div className="grid gap-4 md:grid-cols-2">
              <Field label={t('events.destination.tlsMode')}>
                <select className={fieldClasses} value={draft.tlsMode} onChange={(event) => update({ tlsMode: event.currentTarget.value })}>
                  <option value="system">{t('events.destination.tlsSystem')}</option>
                  <option value="custom_ca">{t('events.destination.tlsCustomCa')}</option>
                  <option value="mutual">{t('events.destination.tlsMutual')}</option>
                  <option value="disabled">{t('events.destination.tlsDisabled')}</option>
                </select>
              </Field>
              <div className="self-end rounded-md border border-border bg-muted/20 p-3 text-xs text-muted-foreground">
                {t(`events.destination.tlsHelp.${draft.tlsMode}`)}
              </div>
              {(draft.tlsMode === 'custom_ca' || draft.tlsMode === 'mutual') && (
                <div className="md:col-span-2">
                  <Field label={t('events.destination.caFile')} help={t('events.destination.absolutePathHelp')}>
                    <input className={`${fieldClasses} font-mono`} value={draft.caFile} maxLength="511" placeholder="/etc/lightnvr/certs/broker-ca.pem" onInput={(event) => update({ caFile: event.currentTarget.value })} />
                  </Field>
                </div>
              )}
              {draft.tlsMode === 'mutual' && (
                <>
                  <Field label={t('events.destination.certFile')}>
                    <input className={`${fieldClasses} font-mono`} value={draft.certFile} maxLength="511" placeholder="/etc/lightnvr/certs/client.pem" onInput={(event) => update({ certFile: event.currentTarget.value })} />
                  </Field>
                  <Field label={t('events.destination.keyFile')}>
                    <input className={`${fieldClasses} font-mono`} value={draft.keyFile} maxLength="511" placeholder="/etc/lightnvr/certs/client.key" onInput={(event) => update({ keyFile: event.currentTarget.value })} />
                  </Field>
                </>
              )}
            </div>
            {draft.tlsMode === 'disabled' && <p className="mt-3 rounded-md bg-[hsl(var(--warning)/0.15)] p-3 text-sm">{t('events.destination.plaintextWarning')}</p>}
          </section>
        </div>

        <div className="flex flex-wrap items-center justify-end gap-3 border-t border-border bg-muted/20 p-4">
          {validationCode && <span className="mr-auto text-xs text-[hsl(var(--danger))]">{t(`events.destination.validation.${validationCode}`)}</span>}
          <button type="button" className="btn-secondary" onClick={onClose} disabled={saving}>{t('common.cancel')}</button>
          <button type="submit" className="btn-primary" disabled={saving || Boolean(validationCode)}>{saving ? t('common.saving') : t(creating ? 'events.destination.create' : 'common.saveChanges')}</button>
        </div>
      </form>
    </div>
  );
}
