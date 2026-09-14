import {
  buildTokenPayload,
  createTokenDraft,
  getTokenStatus,
  selectableTokenActions,
  toggleTokenAction,
  validateTokenDraft,
} from '../js/components/preact/users/apiTokenPolicy.js';

describe('scoped API token UI helpers', () => {
  const actions = [
    { key: 'live.view', enforced: true },
    { key: 'audio.listen', enforced: false },
    { key: 'audio.talk', enforced: false },
    { key: 'recordings.replay', enforced: true },
    { key: 'recordings.export', enforced: true, camera_scoped: true },
    { key: 'snapshot.create', enforced: true },
    { key: 'ptz.control', enforced: true },
    { key: 'evidence.protect', enforced: true },
    { key: 'recording.delete', enforced: true },
    { key: 'camera.configure', enforced: true },
    { key: 'fleet.execute_job', enforced: false },
    { key: 'storage.configure', enforced: true, camera_scoped: false },
    { key: 'events.configure', enforced: true, camera_scoped: false },
    { key: 'users.manage', enforced: true, camera_scoped: false },
    { key: 'system.admin', enforced: true, camera_scoped: false },
    { key: 'lpr.read', enforced: true },
    { key: 'lpr.search', enforced: true },
    { key: 'lpr.export', enforced: true },
    { key: 'lpr.delete', enforced: true },
  ];

  test('offers the full enforced server catalog, including read and administration actions', () => {
    expect(selectableTokenActions(actions).map((action) => action.key)).toEqual([
      'live.view',
      'recordings.replay',
      'recordings.export',
      'snapshot.create',
      'ptz.control',
      'evidence.protect',
      'recording.delete',
      'camera.configure',
      'storage.configure',
      'events.configure',
      'users.manage',
      'system.admin',
      'lpr.read',
      'lpr.search',
      'lpr.export',
      'lpr.delete',
    ]);
  });

  test('requires explicit server enforcement instead of assuming old permissions are safe', () => {
    const reported = [
      { key: 'recordings.export', enforced: true },
      { key: 'ptz.control', enforced: false },
      { key: 'recording.delete' },
      { key: 'evidence.protect', enforced: null },
      { key: 'live.view', enforced: 'true' },
    ];
    expect(selectableTokenActions(reported).map((action) => action.key)).toEqual([
      'recordings.export',
    ]);
  });

  test('exposes newly enforced actions without a client release and preserves metadata', () => {
    const futureAction = {
      key: 'future.action',
      category: 'New feature',
      description: 'A newly enforced server action',
      camera_scoped: true,
      destructive: false,
      enforced: true,
    };
    expect(selectableTokenActions([futureAction])).toEqual([futureAction]);
    const draft = { ...createTokenDraft(), description: 'Integration',
      actionKeys: [futureAction.key] };
    expect(validateTokenDraft(draft, [futureAction])).toBe('');
    expect(buildTokenPayload(draft, 1000).actions).toEqual([futureAction.key]);
  });

  test('handles an empty catalog', () => {
    expect(selectableTokenActions()).toEqual([]);
    expect(selectableTokenActions([])).toEqual([]);
  });

  test('validates required fields and scoped resources', () => {
    const draft = createTokenDraft();
    expect(validateTokenDraft(draft)).toBe('missing_description');
    draft.description = 'Evidence exporter';
    expect(validateTokenDraft(draft)).toBe('missing_actions');
    draft.actionKeys = ['recordings.export'];
    draft.scopeType = 'collection';
    expect(validateTokenDraft(draft, actions, new Set(['north']))).toBe('missing_collection');
    draft.collectionUuid = 'north';
    expect(validateTokenDraft(draft, actions, new Set(['north']))).toBe('');
    draft.expiryDays = 367;
    expect(validateTokenDraft(draft)).toBe('invalid_expiry');
  });

  test('serializes a deterministic expiring collection token payload', () => {
    const draft = {
      ...createTokenDraft(),
      description: '  North garage bridge  ',
      expiryDays: 30,
      actionKeys: ['recordings.export', 'ptz.control'],
      scopeType: 'collection',
      collectionUuid: 'north',
    };
    expect(buildTokenPayload(draft, 1_000)).toEqual({
      description: 'North garage bridge',
      expires_at: 2_593_000,
      actions: ['recordings.export', 'ptz.control'],
      scope: { type: 'collection', collection_uuid: 'north' },
    });
  });

  test.each(['storage.configure', 'events.configure', 'users.manage', 'system.admin'])(
    '%s requires all scope alone and in mixed selections', (key) => {
      for (const actionKeys of [[key], ['recordings.export', key]]) {
        const draft = { ...createTokenDraft(), description: 'Integration', actionKeys,
          collectionUuid: 'north' };
        for (const scopeType of ['collection', 'selector']) {
          draft.scopeType = scopeType;
          expect(validateTokenDraft(draft, actions, new Set(['north'])))
            .toBe('requires_all_scope');
          expect(draft.scopeType).toBe(scopeType);
        }
        draft.scopeType = 'all';
        expect(validateTokenDraft(draft, actions)).toBe('');
        expect(buildTokenPayload(draft, 1000).scope).toEqual({ type: 'all' });
      }
    }
  );

  test.each(['all', 'collection', 'selector'])('camera actions allow %s scope', (scopeType) => {
    const draft = { ...createTokenDraft(), description: 'Camera integration',
      actionKeys: ['recordings.export'], scopeType, collectionUuid: 'north' };
    expect(validateTokenDraft(draft, actions, new Set(['north']))).toBe('');
    expect(buildTokenPayload(draft, 1000).scope.type).toBe(scopeType);
  });

  test('uses scope metadata for future permissions and rejects unknown actions', () => {
    const draft = { ...createTokenDraft(), description: 'Integration',
      actionKeys: ['future.action'], scopeType: 'selector' };
    for (const camera_scoped of [false, undefined, null, 'true']) {
      expect(validateTokenDraft(draft, [{ key: 'future.action', camera_scoped }]))
        .toBe('requires_all_scope');
    }
    expect(validateTokenDraft(draft, [{ key: 'future.action', camera_scoped: true }])).toBe('');
    expect(validateTokenDraft(draft, actions)).toBe('invalid_action');
  });

  test('classifies token lifecycle and toggles action selection', () => {
    expect(getTokenStatus({ expires_at: 2_000 }, 1_000)).toBe('active');
    expect(getTokenStatus({ expires_at: 999 }, 1_000)).toBe('expired');
    expect(getTokenStatus({ expires_at: 2_000, revoked_at: 900 }, 1_000)).toBe('revoked');
    expect(toggleTokenAction(['ptz.control'], 'recordings.export')).toEqual([
      'ptz.control',
      'recordings.export',
    ]);
    expect(toggleTokenAction(['ptz.control'], 'ptz.control')).toEqual([]);
  });
});
