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
    { key: 'live.view' },
    { key: 'recordings.export' },
    { key: 'ptz.control' },
    { key: 'recording.delete' },
  ];

  test('offers only actions whose endpoints centrally enforce token scope', () => {
    expect(selectableTokenActions(actions).map((action) => action.key)).toEqual([
      'recordings.export',
      'ptz.control',
      'recording.delete',
    ]);
  });

  test('drops actions the server reports as unenforced', () => {
    const reported = [
      { key: 'recordings.export', enforced: true },
      { key: 'ptz.control', enforced: false },
      { key: 'recording.delete' },
    ];
    expect(selectableTokenActions(reported).map((action) => action.key)).toEqual([
      'recordings.export',
      'recording.delete',
    ]);
  });

  test('validates required fields and scoped resources', () => {
    const draft = createTokenDraft();
    expect(validateTokenDraft(draft)).toBe('missing_description');
    draft.description = 'Evidence exporter';
    expect(validateTokenDraft(draft)).toBe('missing_actions');
    draft.actionKeys = ['recordings.export'];
    draft.scopeType = 'collection';
    expect(validateTokenDraft(draft, new Set(['recordings.export']), new Set(['north']))).toBe('missing_collection');
    draft.collectionUuid = 'north';
    expect(validateTokenDraft(draft, new Set(['recordings.export']), new Set(['north']))).toBe('');
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
