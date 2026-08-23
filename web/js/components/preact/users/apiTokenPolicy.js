export const TOKEN_MAX_LIFETIME_DAYS = 365;
export const TOKEN_EXPIRY_OPTIONS = [7, 30, 90, 365];

// Scoped tokens are deliberately offered only for endpoints that immediately
// evaluate their action and camera scope. Expand this list as endpoint coverage
// moves from legacy checks to the centralized evaluator.
export const ENFORCED_SCOPED_TOKEN_ACTIONS = new Set([
  'recordings.export',
  'ptz.control',
  'evidence.protect',
  'recording.delete',
]);

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

// Offer an action only when this client believes a scoped token can use it AND
// the server agrees the action is enforced. Intersecting the two means the list
// can only shrink toward reality: a client that is ahead of the daemon never
// offers a token permission that would be silently ignored.
export function selectableTokenActions(actions = []) {
  return actions.filter((action) =>
    ENFORCED_SCOPED_TOKEN_ACTIONS.has(action.key) && action?.enforced !== false);
}

export function validateTokenDraft(
  draft,
  actionKeys = null,
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
  if (actionKeys && draft.actionKeys.some((key) => !actionKeys.has(key))) {
    return 'invalid_action';
  }
  if (!['all', 'collection', 'selector'].includes(draft.scopeType)) {
    return 'invalid_scope';
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
