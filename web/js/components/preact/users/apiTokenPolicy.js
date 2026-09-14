export const TOKEN_MAX_LIFETIME_DAYS = 365;
export const TOKEN_EXPIRY_OPTIONS = [7, 30, 90, 365];

export const DEFAULT_TOKEN_SELECTOR = {
  version: 1,
  expression: { op: 'all' },
};

export function createTokenDraft() {
  return {
    description: '',
    expiryDays: 30,
    actionKeys: [],
    scopeType: 'all',
    collectionUuid: '',
    selector: DEFAULT_TOKEN_SELECTOR,
    selectorError: '',
  };
}

// The server catalog owns endpoint enforcement coverage. A client allowlist
// goes stale as handlers gain token support. Require an explicit true flag so
// older servers with missing metadata do not offer unverified permissions.
export function selectableTokenActions(actions = []) {
  return actions.filter((action) => action?.enforced === true);
}

export function validateTokenDraft(
  draft,
  actions = null,
  collectionUuids = null
) {
  if (!draft?.description?.trim()) return 'missing_description';
  if (draft.description.trim().length >= 128) return 'description_too_long';
  const expiryDays = Number(draft.expiryDays);
  if (!Number.isInteger(expiryDays) || expiryDays < 1 ||
      expiryDays > TOKEN_MAX_LIFETIME_DAYS) return 'invalid_expiry';
  if (!Array.isArray(draft.actionKeys) || draft.actionKeys.length === 0) {
    return 'missing_actions';
  }
  const actionsByKey = actions && new Map(actions.map((action) => [action.key, action]));
  if (actionsByKey && draft.actionKeys.some((key) => !actionsByKey.has(key))) {
    return 'invalid_action';
  }
  if (!['all', 'collection', 'selector'].includes(draft.scopeType)) {
    return 'invalid_scope';
  }
  // Global requests have no camera to match against a collection or selector.
  // Require explicit camera scope metadata for every selected action, including
  // mixed selections, without silently widening the user's chosen scope.
  if (draft.scopeType !== 'all' && actionsByKey &&
      draft.actionKeys.some((key) => actionsByKey.get(key).camera_scoped !== true)) {
    return 'requires_all_scope';
  }
  if (draft.scopeType === 'collection' &&
      (!draft.collectionUuid ||
       (collectionUuids && !collectionUuids.has(draft.collectionUuid)))) {
    return 'missing_collection';
  }
  if (draft.scopeType === 'selector' &&
      (!draft.selector || draft.selectorError)) {
    return 'invalid_selector';
  }
  return '';
}

export function buildTokenPayload(draft, nowSeconds = Math.floor(Date.now() / 1000)) {
  const scope = draft.scopeType === 'collection'
    ? { type: 'collection', collection_uuid: draft.collectionUuid }
    : (draft.scopeType === 'selector'
      ? { type: 'selector', selector: draft.selector }
      : { type: 'all' });
  return {
    description: draft.description.trim(),
    expires_at: nowSeconds + Number(draft.expiryDays) * 24 * 60 * 60,
    actions: [...draft.actionKeys],
    scope,
  };
}

export function getTokenStatus(token, nowSeconds = Math.floor(Date.now() / 1000)) {
  if (Number(token?.revoked_at || 0) > 0) return 'revoked';
  if (Number(token?.expires_at || 0) <= nowSeconds) return 'expired';
  return 'active';
}

export function toggleTokenAction(actionKeys, key) {
  return actionKeys.includes(key)
    ? actionKeys.filter((actionKey) => actionKey !== key)
    : [...actionKeys, key];
}
