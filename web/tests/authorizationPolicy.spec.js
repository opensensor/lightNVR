import {
  actionIsEnforced,
  unenforcedActionKeys,
  actionRequiresCamera,
  buildPolicyPayload,
  createDraftGrant,
  groupActionsByCategory,
  policyResponseToDraft,
  roleHasDestructiveActions,
  validatePolicyDraft,
} from '../js/components/preact/users/authorizationPolicy.js';

describe('authorization policy UI helpers', () => {
  const roles = new Set(['viewer', 'operator']);
  const collections = new Set(['collection-1']);
  const selector = {
    version: 1,
    expression: { op: 'tag_any', uuids: ['tag-1'] },
  };

  test('normalizes API policies into stable editable grants', () => {
    const draft = policyResponseToDraft({
      mode: 'policy',
      policy_version: 17,
      grants: [{
        uuid: 'grant-uuid',
        role_uuid: 'viewer',
        scope: { type: 'selector', selector },
      }],
    });
    expect(draft.mode).toBe('policy');
    expect(draft.policyVersion).toBe(17);
    expect(draft.grants[0]).toMatchObject({
      persistedUuid: 'grant-uuid',
      roleUuid: 'viewer',
      scopeType: 'selector',
      selector,
    });
  });

  test('validates incomplete, invalid, and duplicate grants', () => {
    expect(validatePolicyDraft('policy', [createDraftGrant()], roles)).toBe('missing_role');
    const invalidSelector = createDraftGrant('viewer', 'selector');
    invalidSelector.selectorError = 'bad';
    expect(validatePolicyDraft('policy', [invalidSelector], roles)).toBe('invalid_selector');
    const first = createDraftGrant('viewer', 'selector');
    first.selector = selector;
    const second = createDraftGrant('viewer', 'selector');
    second.selector = selector;
    expect(validatePolicyDraft('legacy', [first, second], roles)).toBe('duplicate_grant');
    expect(validatePolicyDraft('policy', [first], roles)).toBe('');
  });

  test('serializes complete replacement payloads', () => {
    const all = createDraftGrant('operator', 'all');
    const scoped = createDraftGrant('viewer', 'selector');
    scoped.selector = selector;
    expect(buildPolicyPayload('policy', [all, scoped], 22)).toEqual({
      expected_policy_version: 22,
      mode: 'policy',
      grants: [
        { role_uuid: 'operator', scope: { type: 'all' } },
        { role_uuid: 'viewer', scope: { type: 'selector', selector } },
      ],
    });
  });

  test('round-trips and validates collection-backed grants', () => {
    const draft = policyResponseToDraft({
      mode: 'policy',
      policy_version: 31,
      grants: [{
        uuid: 'collection-grant',
        role_uuid: 'viewer',
        scope: { type: 'collection', collection_uuid: 'collection-1' },
      }],
    });
    expect(draft.grants[0]).toMatchObject({
      roleUuid: 'viewer',
      scopeType: 'collection',
      collectionUuid: 'collection-1',
    });
    expect(validatePolicyDraft('policy', draft.grants, roles, collections)).toBe('');
    expect(buildPolicyPayload('policy', draft.grants, 31).grants[0]).toEqual({
      role_uuid: 'viewer',
      scope: { type: 'collection', collection_uuid: 'collection-1' },
    });
    draft.grants[0].collectionUuid = '';
    expect(validatePolicyDraft('policy', draft.grants, roles, collections)).toBe('missing_collection');
  });

  test('groups action metadata and identifies sensitive behavior', () => {
    const catalog = [
      { key: 'live.view', category: 'Live', camera_scoped: true, destructive: false },
      { key: 'recording.delete', category: 'Evidence', camera_scoped: true, destructive: true },
      { key: 'users.manage', category: 'System', camera_scoped: false, destructive: true },
    ];
    expect(groupActionsByCategory(catalog).map((group) => group.category)).toEqual(['Live', 'Evidence', 'System']);
    expect(roleHasDestructiveActions(['recording.delete'], catalog)).toBe(true);
    expect(roleHasDestructiveActions(['live.view'], catalog)).toBe(false);
    expect(actionRequiresCamera('live.view', catalog)).toBe(true);
    expect(actionRequiresCamera('users.manage', catalog)).toBe(false);
  });
});

describe('action enforcement reporting', () => {
  test('treats a missing enforced flag as enforced so old servers look normal', () => {
    expect(actionIsEnforced({ key: 'live.view' })).toBe(true);
    expect(actionIsEnforced({ key: 'live.view', enforced: true })).toBe(true);
    expect(actionIsEnforced({ key: 'audio.talk', enforced: false })).toBe(false);
  });

  test('names the role actions no endpoint applies yet', () => {
    const catalog = [
      { key: 'live.view', enforced: true },
      { key: 'audio.talk', enforced: false },
      { key: 'storage.configure', enforced: false },
    ];
    expect(unenforcedActionKeys(['live.view', 'audio.talk'], catalog))
      .toEqual(['audio.talk']);
    expect(unenforcedActionKeys(['live.view'], catalog)).toEqual([]);
    expect(unenforcedActionKeys([], catalog)).toEqual([]);
  });
});
