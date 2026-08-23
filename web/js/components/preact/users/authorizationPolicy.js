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
    selectorError: '',
  };
}

export function policyResponseToDraft(response) {
  return {
    mode: response?.mode === 'policy' ? 'policy' : 'legacy',
    policyVersion: Number(response?.policy_version || 0),
    grants: (response?.grants || []).map((grant) => ({
      ...createDraftGrant(grant.role_uuid, grant.scope?.type || 'all'),
      persistedUuid: grant.uuid,
      selector: grant.scope?.type === 'selector'
        ? grant.scope.selector
        : null,
    })),
  };
}

export function validatePolicyDraft(mode, grants, roleUuids = new Set()) {
  if (mode !== 'legacy' && mode !== 'policy') return 'invalid_mode';
  const seen = new Set();
  for (const grant of grants || []) {
    if (!grant.roleUuid || (roleUuids.size > 0 && !roleUuids.has(grant.roleUuid))) {
      return 'missing_role';
    }
    if (grant.scopeType !== 'all' && grant.scopeType !== 'selector') {
      return 'invalid_scope';
    }
    if (grant.scopeType === 'selector' && (!grant.selector || grant.selectorError)) {
      return 'invalid_selector';
    }
    const key = `${grant.roleUuid}\n${grant.scopeType}\n${grant.scopeType === 'selector' ? JSON.stringify(grant.selector) : ''}`;
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
        : { type: 'all' },
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
