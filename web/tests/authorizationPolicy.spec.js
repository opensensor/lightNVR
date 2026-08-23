import {
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
