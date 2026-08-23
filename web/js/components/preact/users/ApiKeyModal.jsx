import { useCallback, useEffect, useMemo, useState } from 'preact/hooks';
import { fetchJSON } from '../../../query-client.js';
import { formatLocalDateTime } from '../../../utils/date-utils.js';
import { useI18n } from '../../../i18n.js';
import { CollectionSelectorBuilder } from '../fleet/CollectionSelectorBuilder.jsx';
import { showStatusMessage } from '../ToastContainer.jsx';
import { groupActionsByCategory } from './authorizationPolicy.js';
import {
  DEFAULT_TOKEN_SELECTOR,
  TOKEN_EXPIRY_OPTIONS,
  buildTokenPayload,
  createTokenDraft,
  getTokenStatus,
  selectableTokenActions,
  toggleTokenAction,
  validateTokenDraft,
} from './apiTokenPolicy.js';

const fieldClasses = 'w-full rounded-md border border-input bg-background px-3 py-2 text-sm text-foreground focus:outline-none focus:ring-2 focus:ring-[hsl(var(--primary))]';

function SecretReveal({ revealed, copied, confirmed, onCopy, onConfirmChange, onDone, t }) {
  const scoped = revealed.kind === 'scoped';
  return (
    <div className="flex min-h-0 flex-1 flex-col overflow-y-auto p-6">
      <div className="mx-auto w-full max-w-2xl">
        <div className="rounded-lg border border-[hsl(var(--warning))] bg-[hsl(var(--warning)/0.1)] p-5">
          <p className="text-xs font-semibold uppercase tracking-wide text-muted-foreground">{t('tokens.secret.oneTime')}</p>
          <h3 className="mt-1 text-xl font-bold">{t(scoped ? 'tokens.secret.scopedTitle' : 'tokens.secret.legacyTitle')}</h3>
          <p className="mt-2 text-sm text-muted-foreground">{t(scoped ? 'tokens.secret.scopedDescription' : 'tokens.secret.legacyDescription')}</p>
          {scoped && revealed.token && (
            <div className="mt-4 flex flex-wrap gap-2 text-xs">
              <span className="rounded-full border border-border bg-background px-2.5 py-1">{revealed.token.description}</span>
              <span className="rounded-full border border-border bg-background px-2.5 py-1">{t('tokens.expiresValue', { date: formatLocalDateTime(revealed.token.expires_at) })}</span>
            </div>
          )}
          <div className="mt-5 flex">
            <input
              aria-label={t('tokens.secret.value')}
              className="min-w-0 flex-1 rounded-l-md border border-input bg-background px-3 py-2 font-mono text-sm text-foreground"
              type="text"
              value={revealed.secret}
              readOnly
              onFocus={(event) => event.currentTarget.select()}
            />
            <button type="button" className="btn-primary rounded-l-none" onClick={onCopy}>
              {t(copied ? 'tokens.secret.copied' : 'common.copy')}
            </button>
          </div>
        </div>

        <label className="mt-5 flex cursor-pointer items-start gap-3 rounded-lg border border-border p-4 text-sm">
          <input
            type="checkbox"
            className="mt-0.5 h-4 w-4"
            checked={confirmed}
            onChange={(event) => onConfirmChange(event.currentTarget.checked)}
          />
          <span><span className="font-semibold">{t('tokens.secret.savedConfirm')}</span><span className="mt-1 block text-muted-foreground">{t('tokens.secret.savedHelp')}</span></span>
        </label>
        <div className="mt-5 flex justify-end">
          <button type="button" className="btn-primary" disabled={!confirmed} onClick={onDone}>{t('common.done')}</button>
        </div>
      </div>
    </div>
  );
}

function ActionPicker({ actions, selected, onToggle, t }) {
  const groups = useMemo(() => groupActionsByCategory(actions), [actions]);
  return (
    <div className="space-y-3">
      {groups.map((group) => (
        <fieldset key={group.category} className="rounded-md border border-border p-3">
          <legend className="px-1 text-sm font-semibold">{group.category}</legend>
          <div className="grid gap-2 sm:grid-cols-2">
            {group.actions.map((action) => (
              <label key={action.key} className="flex cursor-pointer items-start gap-2 rounded p-2 hover:bg-muted/50">
                <input type="checkbox" className="mt-0.5 h-4 w-4" checked={selected.includes(action.key)} onChange={() => onToggle(action.key)} />
                <span className="min-w-0 text-sm"><span className="font-medium">{action.key}</span>{action.destructive && <span className="ml-2 rounded-full bg-[hsl(var(--danger)/0.12)] px-1.5 py-0.5 text-[0.65rem] text-[hsl(var(--danger))]">{t('tokens.destructive')}</span>}<span className="mt-0.5 block text-xs text-muted-foreground">{action.description}</span></span>
              </label>
            ))}
          </div>
        </fieldset>
      ))}
      {actions.length === 0 && <p className="rounded-md badge-warning p-3 text-sm">{t('tokens.noEnforcedActions')}</p>}
    </div>
  );
}

function TokenCard({ token, collections, revoking, onRevoke, t }) {
  const status = getTokenStatus(token);
  const collection = collections.find((item) => item.uuid === token.scope?.collection_uuid);
  const scopeLabel = token.scope?.type === 'collection'
    ? t('tokens.scope.collectionValue', { name: collection?.name || token.scope.collection_uuid })
    : t(token.scope?.type === 'selector' ? 'tokens.scope.selectorValue' : 'tokens.scope.allValue');
  const statusClasses = status === 'active'
    ? 'badge-success'
    : (status === 'expired' ? 'badge-warning' : 'badge-danger');
  return (
    <article className="rounded-lg border border-border bg-background p-4">
      <div className="flex flex-wrap items-start justify-between gap-3">
        <div className="min-w-0">
          <div className="flex flex-wrap items-center gap-2"><h4 className="font-semibold">{token.description}</h4><span className={`rounded-full px-2 py-0.5 text-xs font-semibold ${statusClasses}`}>{t(`tokens.status.${status}`)}</span></div>
          <p className="mt-1 font-mono text-xs text-muted-foreground">{token.prefix}…</p>
        </div>
        {status === 'active' && <button type="button" className="btn-danger" disabled={revoking} onClick={() => onRevoke(token)}>{revoking ? t('tokens.revoking') : t('tokens.revoke')}</button>}
      </div>
      <div className="mt-3 flex flex-wrap gap-1.5">
        {(token.actions || []).map((action) => <span key={action} className="rounded-full border border-border bg-muted/40 px-2 py-1 text-xs">{action}</span>)}
      </div>
      <dl className="mt-3 grid gap-x-4 gap-y-1 text-xs text-muted-foreground sm:grid-cols-2">
        <div><dt className="inline font-medium text-foreground">{t('tokens.scope.label')}: </dt><dd className="inline">{scopeLabel}</dd></div>
        <div><dt className="inline font-medium text-foreground">{t('tokens.expires')}: </dt><dd className="inline">{formatLocalDateTime(token.expires_at)}</dd></div>
        <div><dt className="inline font-medium text-foreground">{t('tokens.created')}: </dt><dd className="inline">{formatLocalDateTime(token.created_at)}</dd></div>
        <div><dt className="inline font-medium text-foreground">{t('tokens.lastUsed')}: </dt><dd className="inline">{token.last_used_at ? formatLocalDateTime(token.last_used_at) : t('common.never')}</dd></div>
      </dl>
    </article>
  );
}

export function ApiKeyModal({ currentUser, getAuthHeaders, onClose }) {
  const { t } = useI18n();
  const [loading, setLoading] = useState(true);
  const [loadError, setLoadError] = useState('');
  const [tokens, setTokens] = useState([]);
  const [actions, setActions] = useState([]);
  const [collections, setCollections] = useState([]);
  const [locations, setLocations] = useState([]);
  const [tags, setTags] = useState([]);
  const [draft, setDraft] = useState(createTokenDraft);
  const [creating, setCreating] = useState(false);
  const [revokingUuid, setRevokingUuid] = useState('');
  const [showInactive, setShowInactive] = useState(false);
  const [legacyGenerating, setLegacyGenerating] = useState(false);
  const [revealed, setRevealed] = useState(null);
  const [secretCopied, setSecretCopied] = useState(false);
  const [secretConfirmed, setSecretConfirmed] = useState(false);

  const load = useCallback(async () => {
    setLoading(true);
    setLoadError('');
    try {
      const [actionResponse, tokenResponse, collectionResponse, locationResponse, tagResponse] = await Promise.all([
        fetchJSON('/api/authorization/actions', { headers: getAuthHeaders(), cache: 'no-store' }),
        fetchJSON(`/api/authorization/users/${encodeURIComponent(currentUser.id)}/tokens`, { headers: getAuthHeaders(), cache: 'no-store' }),
        fetchJSON('/api/camera-collections', { headers: getAuthHeaders(), cache: 'no-store' }),
        fetchJSON('/api/locations', { headers: getAuthHeaders(), cache: 'no-store' }),
        fetchJSON('/api/camera-tags', { headers: getAuthHeaders(), cache: 'no-store' }),
      ]);
      setActions(selectableTokenActions(actionResponse.actions || []));
      setTokens(tokenResponse.tokens || []);
      setCollections((collectionResponse.collections || []).filter((collection) => collection.shared));
      setLocations(locationResponse.locations || []);
      setTags(tagResponse.tags || []);
    } catch (error) {
      setLoadError(error.message);
    } finally {
      setLoading(false);
    }
  }, [currentUser.id, getAuthHeaders]);

  useEffect(() => { load(); }, [load]);
  useEffect(() => {
    const handleKey = (event) => {
      if (event.key === 'Escape' && !revealed && !creating && !legacyGenerating) onClose();
    };
    window.addEventListener('keydown', handleKey);
    return () => window.removeEventListener('keydown', handleKey);
  }, [creating, legacyGenerating, onClose, revealed]);

  const actionKeys = useMemo(() => new Set(actions.map((action) => action.key)), [actions]);
  const collectionUuids = useMemo(() => new Set(collections.map((collection) => collection.uuid)), [collections]);
  const validationCode = useMemo(
    () => validateTokenDraft(draft, actionKeys, collectionUuids),
    [actionKeys, collectionUuids, draft]
  );
  const visibleTokens = useMemo(
    () => showInactive ? tokens : tokens.filter((token) => getTokenStatus(token) === 'active'),
    [showInactive, tokens]
  );
  const inactiveCount = tokens.length - tokens.filter((token) => getTokenStatus(token) === 'active').length;

  const updateDraft = (changes) => setDraft((current) => ({ ...current, ...changes }));
  const updateSelector = useCallback((selector, selectorError) => {
    setDraft((current) => ({ ...current, selector, selectorError }));
  }, []);

  const createToken = async () => {
    if (validationCode) {
      showStatusMessage(t(`tokens.validation.${validationCode}`), 'error', 7000);
      return;
    }
    setCreating(true);
    try {
      const response = await fetchJSON(`/api/authorization/users/${encodeURIComponent(currentUser.id)}/tokens`, {
        method: 'POST',
        headers: { 'Content-Type': 'application/json', ...getAuthHeaders() },
        body: JSON.stringify(buildTokenPayload(draft)),
        timeout: 20000,
        retries: 0,
      });
      if (!response?.token?.uuid || typeof response?.secret !== 'string' ||
          !response.secret) {
        throw new Error(t('tokens.invalidResponse'));
      }
      setTokens((current) => [response.token, ...current]);
      setSecretCopied(false);
      setSecretConfirmed(false);
      setRevealed({ kind: 'scoped', secret: response.secret, token: response.token });
      setDraft(createTokenDraft());
      showStatusMessage(t('tokens.createdSuccess'), 'success');
    } catch (error) {
      showStatusMessage(t('tokens.createError', { message: error.message }), 'error', 8000);
    } finally {
      setCreating(false);
    }
  };

  const revokeToken = async (token) => {
    if (!window.confirm(t('tokens.revokeConfirm', { description: token.description }))) return;
    setRevokingUuid(token.uuid);
    try {
      await fetchJSON(`/api/authorization/users/${encodeURIComponent(currentUser.id)}/tokens/${encodeURIComponent(token.uuid)}`, {
        method: 'DELETE',
        headers: getAuthHeaders(),
        timeout: 15000,
        retries: 0,
      });
      const revokedAt = Math.floor(Date.now() / 1000);
      setTokens((current) => current.map((item) => item.uuid === token.uuid ? { ...item, revoked_at: revokedAt } : item));
      showStatusMessage(t('tokens.revokedSuccess'), 'success');
    } catch (error) {
      showStatusMessage(t('tokens.revokeError', { message: error.message }), 'error', 8000);
    } finally {
      setRevokingUuid('');
    }
  };

  const generateLegacyKey = async () => {
    if (!window.confirm(t('tokens.legacy.confirm'))) return;
    setLegacyGenerating(true);
    try {
      const response = await fetchJSON(`/api/auth/users/${encodeURIComponent(currentUser.id)}/api-key`, {
        method: 'POST',
        headers: getAuthHeaders(),
        timeout: 20000,
        retries: 0,
      });
      if (typeof response?.api_key !== 'string' || !response.api_key) {
        throw new Error(t('tokens.invalidResponse'));
      }
      setSecretCopied(false);
      setSecretConfirmed(false);
      setRevealed({ kind: 'legacy', secret: response.api_key });
      showStatusMessage(t('tokens.legacy.createdSuccess'), 'success');
    } catch (error) {
      showStatusMessage(t('tokens.legacy.createError', { message: error.message }), 'error', 8000);
    } finally {
      setLegacyGenerating(false);
    }
  };

  const copySecret = async () => {
    try {
      await navigator.clipboard.writeText(revealed.secret);
      setSecretCopied(true);
      showStatusMessage(t('tokens.secret.copySuccess'), 'success');
    } catch (error) {
      showStatusMessage(t('tokens.secret.copyError'), 'error');
    }
  };

  const closeAllowed = !revealed && !creating && !legacyGenerating;
  return (
    <div className="fixed inset-0 z-50 flex items-center justify-center bg-black/50 p-3" onClick={() => closeAllowed && onClose()}>
      <section role="dialog" aria-modal="true" aria-labelledby="api-access-title" className="flex max-h-[95vh] w-full max-w-6xl flex-col overflow-hidden rounded-lg bg-card text-card-foreground shadow-xl" onClick={(event) => event.stopPropagation()}>
        <header className="flex items-start justify-between gap-4 border-b border-border p-5">
          <div><h2 id="api-access-title" className="text-xl font-bold">{t('tokens.title', { username: currentUser.username })}</h2><p className="mt-1 text-sm text-muted-foreground">{t('tokens.description')}</p></div>
          {!revealed && <button type="button" className="btn-secondary" onClick={onClose} disabled={!closeAllowed}>{t('common.close')}</button>}
        </header>

        {revealed ? (
          <SecretReveal
            revealed={revealed}
            copied={secretCopied}
            confirmed={secretConfirmed}
            onCopy={copySecret}
            onConfirmChange={setSecretConfirmed}
            onDone={onClose}
            t={t}
          />
        ) : (
          <div className="min-h-0 flex-1 overflow-y-auto p-5">
            {loadError && <div className="mb-4 rounded-md badge-danger p-3">{loadError} <button type="button" className="ml-2 underline" onClick={load}>{t('common.retry')}</button></div>}
            {loading ? <div className="p-10 text-center">{t('common.loading')}</div> : (
              <div className="space-y-5">
                <div className="rounded-lg border border-[hsl(var(--info))] bg-[hsl(var(--info)/0.1)] p-4"><h3 className="font-semibold">{t('tokens.recommendedTitle')}</h3><p className="mt-1 text-sm text-muted-foreground">{t('tokens.recommendedDescription')}</p><p className="mt-2 text-xs text-muted-foreground">{t('tokens.coverageNotice')}</p></div>

                <div className="grid gap-5 xl:grid-cols-[minmax(0,1.15fr)_minmax(22rem,0.85fr)]">
                  <section className="space-y-4 rounded-lg border border-border p-4">
                    <div><h3 className="text-lg font-semibold">{t('tokens.createTitle')}</h3><p className="mt-1 text-sm text-muted-foreground">{t('tokens.createDescription')}</p></div>
                    <div className="grid gap-3 sm:grid-cols-[minmax(0,1fr)_12rem]">
                      <label className="text-sm font-medium">{t('tokens.name')}<input className={`${fieldClasses} mt-1`} maxLength="127" value={draft.description} placeholder={t('tokens.namePlaceholder')} onInput={(event) => updateDraft({ description: event.currentTarget.value })} /></label>
                      <label className="text-sm font-medium">{t('tokens.expiry')}<select className={`${fieldClasses} mt-1`} value={draft.expiryDays} onChange={(event) => updateDraft({ expiryDays: Number(event.currentTarget.value) })}>{TOKEN_EXPIRY_OPTIONS.map((days) => <option key={days} value={days}>{t('tokens.expiryDays', { days })}</option>)}</select></label>
                    </div>

                    <div><h4 className="text-sm font-semibold">{t('tokens.actions')}</h4><p className="mb-2 mt-1 text-xs text-muted-foreground">{t('tokens.actionsHelp')}</p><ActionPicker actions={actions} selected={draft.actionKeys} onToggle={(key) => updateDraft({ actionKeys: toggleTokenAction(draft.actionKeys, key) })} t={t} /></div>

                    <div>
                      <label className="text-sm font-medium">{t('tokens.scope.label')}<select className={`${fieldClasses} mt-1`} value={draft.scopeType} onChange={(event) => updateDraft({ scopeType: event.currentTarget.value, collectionUuid: '', selector: DEFAULT_TOKEN_SELECTOR, selectorError: '' })}><option value="all">{t('tokens.scope.all')}</option><option value="collection">{t('tokens.scope.collection')}</option><option value="selector">{t('tokens.scope.selector')}</option></select></label>
                      {draft.scopeType === 'collection' && <label className="mt-3 block text-sm font-medium">{t('tokens.scope.chooseCollection')}<select className={`${fieldClasses} mt-1`} value={draft.collectionUuid} onChange={(event) => updateDraft({ collectionUuid: event.currentTarget.value })}><option value="">{t('access.policy.chooseCollection')}</option>{collections.map((collection) => <option key={collection.uuid} value={collection.uuid}>{collection.name} ({collection.effective_count})</option>)}</select></label>}
                      {draft.scopeType === 'selector' && <div className="mt-3"><CollectionSelectorBuilder key="api-token-selector" idPrefix="api-token-selector" initialSelector={draft.selector} locations={locations} tags={tags} allowCameraSelection requestHeaders={getAuthHeaders()} onChange={updateSelector} t={t} /></div>}
                    </div>
                    {validationCode && <p className="text-sm text-[hsl(var(--danger))]">{t(`tokens.validation.${validationCode}`)}</p>}
                    <div className="flex justify-end"><button type="button" className="btn-primary" disabled={creating || Boolean(validationCode)} onClick={createToken}>{creating ? t('tokens.creating') : t('tokens.create')}</button></div>
                  </section>

                  <section className="space-y-3 rounded-lg border border-border p-4">
                    <div className="flex flex-wrap items-start justify-between gap-3"><div><h3 className="text-lg font-semibold">{t('tokens.existingTitle')}</h3><p className="mt-1 text-sm text-muted-foreground">{t('tokens.existingDescription')}</p></div>{inactiveCount > 0 && <label className="flex items-center gap-2 text-xs"><input type="checkbox" checked={showInactive} onChange={(event) => setShowInactive(event.currentTarget.checked)} />{t('tokens.showInactive', { count: inactiveCount })}</label>}</div>
                    <div className="max-h-[42rem] space-y-3 overflow-y-auto pr-1">
                      {visibleTokens.map((token) => <TokenCard key={token.uuid} token={token} collections={collections} revoking={revokingUuid === token.uuid} onRevoke={revokeToken} t={t} />)}
                      {visibleTokens.length === 0 && <div className="rounded-lg border border-dashed border-border p-8 text-center"><h4 className="font-semibold">{t('tokens.emptyTitle')}</h4><p className="mt-1 text-sm text-muted-foreground">{t('tokens.emptyDescription')}</p></div>}
                    </div>
                  </section>
                </div>

                <details className="rounded-lg border border-[hsl(var(--warning))] bg-[hsl(var(--warning)/0.07)] p-4">
                  <summary className="cursor-pointer font-semibold">{t('tokens.legacy.title')}</summary>
                  <div className="mt-3 border-t border-border pt-3"><p className="text-sm text-muted-foreground">{t('tokens.legacy.description')}</p><p className="mt-2 text-sm font-medium text-[hsl(var(--warning-foreground))]">{t('tokens.legacy.warning')}</p><button type="button" className="btn-secondary mt-3" disabled={legacyGenerating} onClick={generateLegacyKey}>{legacyGenerating ? t('users.generatingApiKey') : t('tokens.legacy.generate')}</button></div>
                </details>
              </div>
            )}
          </div>
        )}
      </section>
    </div>
  );
}
