export const ALL_CAMERAS_SELECTOR = {
  version: 1,
  expression: { op: 'all' },
};

let nextDraftGrantId = 1;

export function createDraftGrant(roleUuid = '', scopeType = 'all') {
  return {
    draftId: `grant-${nextDraftGrantId++}`,
    roleUuid,
    scopeType,
    selector: scopeType === 'selector' ? ALL_CAMERAS_SELECTOR : null,
    collectionUuid: '',
    selectorError: '',
  };
}

export function policyResponseToDraft(response) {
  return {
    // Legacy principals are migrated server-side at startup. Treat an old or
    // incomplete response as policy so the editor cannot recreate legacy mode.
    mode: 'policy',
    policyVersion: Number(response?.policy_version || 0),
    grants: (response?.grants || []).map((grant) => ({
      ...createDraftGrant(grant.role_uuid, grant.scope?.type || 'all'),
      persistedUuid: grant.uuid,
      selector: grant.scope?.type === 'selector'
        ? grant.scope.selector
        : null,
      collectionUuid: grant.scope?.type === 'collection'
        ? (grant.scope.collection_uuid || '')
        : '',
    })),
  };
}

export function validatePolicyDraft(mode, grants, roleUuids = null, collectionUuids = null) {
  if (mode !== 'policy') return 'invalid_mode';
  const seen = new Set();
  for (const grant of grants || []) {
    if (!grant.roleUuid || (roleUuids && !roleUuids.has(grant.roleUuid))) {
      return 'missing_role';
    }
    if (grant.scopeType !== 'all' && grant.scopeType !== 'selector' && grant.scopeType !== 'collection') {
      return 'invalid_scope';
    }
    if (grant.scopeType === 'selector' && (!grant.selector || grant.selectorError)) {
      return 'invalid_selector';
    }
    if (grant.scopeType === 'collection' &&
        (!grant.collectionUuid || (collectionUuids && !collectionUuids.has(grant.collectionUuid)))) {
      return 'missing_collection';
    }
    const scopeKey = grant.scopeType === 'selector'
      ? JSON.stringify(grant.selector)
      : (grant.scopeType === 'collection' ? grant.collectionUuid : '');
    const key = `${grant.roleUuid}\n${grant.scopeType}\n${scopeKey}`;
    if (seen.has(key)) return 'duplicate_grant';
    seen.add(key);
  }
  return '';
}

export function buildPolicyPayload(mode, grants, policyVersion) {
  return {
    expected_policy_version: policyVersion,
    mode,
    grants: (grants || []).map((grant) => ({
      role_uuid: grant.roleUuid,
      scope: grant.scopeType === 'selector'
        ? { type: 'selector', selector: grant.selector }
        : (grant.scopeType === 'collection'
          ? { type: 'collection', collection_uuid: grant.collectionUuid }
          : { type: 'all' }),
    })),
  };
}

export function groupActionsByCategory(actions = []) {
  const grouped = new Map();
  for (const action of actions) {
    const category = action.category || 'Other';
    if (!grouped.has(category)) grouped.set(category, []);
    grouped.get(category).push(action);
  }
  return [...grouped.entries()].map(([category, items]) => ({ category, actions: items }));
}

export function roleHasDestructiveActions(roleActions = [], actionCatalog = []) {
  const destructive = new Set(
    actionCatalog.filter((action) => action.destructive).map((action) => action.key)
  );
  return roleActions.some((key) => destructive.has(key));
}

export function actionRequiresCamera(actionKey, actionCatalog = []) {
  return actionCatalog.find((action) => action.key === actionKey)?.camera_scoped === true;
}

// The action catalog is authored ahead of endpoint coverage: a role may include
// an action that no handler consults yet. The server reports that per action so
// the UI can say so instead of implying a boundary that is not applied. Treat a
// missing flag as enforced so an older server does not paint every action as a
// gap.
export function actionIsEnforced(action) {
  return action?.enforced !== false;
}

export function unenforcedActionKeys(roleActions = [], actionCatalog = []) {
  const unenforced = new Set(
    actionCatalog.filter((action) => !actionIsEnforced(action)).map((action) => action.key)
  );
  return (roleActions || []).filter((key) => unenforced.has(key));
}
