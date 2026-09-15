import {
  AUDIT_DECISION_MODES,
  EMPTY_AUDIT_FILTERS,
  auditDateRangeIsValid,
  auditOutcomeTone,
  auditPageBounds,
  buildAuditQuery,
  changedDecisionModes,
  defaultSinceForEventType,
  formatSummaryCount,
  groupDecisionModes,
  modesByAction,
  summaryDetails,
} from '../js/components/preact/users/auditHistory.js';

describe('audit history UI helpers', () => {
  test('builds a compact paginated query and trims exact filters', () => {
    const query = new URLSearchParams(buildAuditQuery({
      ...EMPTY_AUDIT_FILTERS,
      principalUserId: ' 42 ',
      action: ' recordings.export ',
      outcome: 'denied',
      targetUuid: ' camera-1 ',
      requestId: ' request-1 ',
    }, 3, 100));
    expect(Object.fromEntries(query)).toEqual({
      page: '3',
      page_size: '100',
      principal_user_id: '42',
      action: 'recordings.export',
      outcome: 'denied',
      target_uuid: 'camera-1',
      request_id: 'request-1',
    });
  });

  test('converts local date inputs to unix timestamp bounds', () => {
    const query = new URLSearchParams(buildAuditQuery({
      since: '2026-08-22T12:30',
      until: '2026-08-23T12:30',
    }));
    expect(Number(query.get('since'))).toBe(Math.floor(new Date('2026-08-22T12:30').getTime() / 1000));
    expect(Number(query.get('until'))).toBe(Math.floor(new Date('2026-08-23T12:30').getTime() / 1000));
  });

  test('validates optional date bounds', () => {
    expect(auditDateRangeIsValid({ since: '', until: '' })).toBe(true);
    expect(auditDateRangeIsValid({ since: '2026-08-22T12:30', until: '' })).toBe(true);
    expect(auditDateRangeIsValid({ since: '2026-08-23T12:30', until: '2026-08-22T12:30' })).toBe(false);
  });

  test('reports display bounds for empty and partial pages', () => {
    expect(auditPageBounds({ page: 1, page_size: 50, count: 0, total: 0 })).toEqual({ start: 0, end: 0, total: 0 });
    expect(auditPageBounds({ page: 3, page_size: 25, count: 7, total: 57 })).toEqual({ start: 51, end: 57, total: 57 });
  });

  test('maps outcomes to stable semantic tones', () => {
    expect(auditOutcomeTone('success')).toBe('success');
    expect(auditOutcomeTone('allowed')).toBe('success');
    expect(auditOutcomeTone('denied')).toBe('warning');
    expect(auditOutcomeTone('failure')).toBe('warning');
    expect(auditOutcomeTone('error')).toBe('danger');
  });
});

describe('audit decision mode helpers', () => {
  const modes = [
    { action: 'live.view', category: 'Live video', description: 'View live', mode: 'record' },
    { action: 'audio.listen', category: 'Live video', description: 'Listen', mode: 'off' },
    { action: 'system.admin', category: 'System administration', description: 'Admin', mode: 'summarize' },
  ];

  test('exposes the three supported modes', () => {
    expect(AUDIT_DECISION_MODES).toEqual(['record', 'summarize', 'off']);
  });

  test('groups actions by category in first-seen order', () => {
    const groups = groupDecisionModes(modes);
    expect(groups.map((group) => group.category)).toEqual(['Live video', 'System administration']);
    expect(groups[0].actions.map((entry) => entry.action)).toEqual(['live.view', 'audio.listen']);
  });

  test('maps modes by action', () => {
    expect(modesByAction(modes)).toEqual({ 'live.view': 'record', 'audio.listen': 'off', 'system.admin': 'summarize' });
  });

  test('reports only valid changed modes', () => {
    const original = modesByAction(modes);
    const draft = { ...original, 'live.view': 'summarize', 'audio.listen': 'off', 'system.admin': 'loud' };
    expect(changedDecisionModes(original, draft)).toEqual({ 'live.view': 'summarize' });
    expect(changedDecisionModes(original, original)).toEqual({});
  });

  test('recognizes summary rows only', () => {
    expect(summaryDetails({ details: { event_type: 'authorization.summary', count: 3412, first_at: 10, last_at: 20 } }))
      .toEqual({ count: 3412, firstAt: 10, lastAt: 20 });
    expect(summaryDetails({ details: { event_type: 'authorization.decision' } })).toBeNull();
    expect(summaryDetails({ details: { event_type: 'authorization.summary', count: 0 } })).toBeNull();
    expect(summaryDetails({})).toBeNull();
  });

  test('formats summary counts', () => {
    expect(formatSummaryCount(3412)).toBe('×3,412');
    expect(formatSummaryCount(1)).toBe('×1');
  });

  test('adds the event type filter to the query', () => {
    const query = new URLSearchParams(buildAuditQuery({ ...EMPTY_AUDIT_FILTERS, eventType: ' authorization.summary ' }));
    expect(query.get('event_type')).toBe('authorization.summary');
    expect(new URLSearchParams(buildAuditQuery({ ...EMPTY_AUDIT_FILTERS })).has('event_type')).toBe(false);
  });

  test('prefills since with the last 24 hours only when an event type is chosen with no since bound', () => {
    const now = new Date('2026-09-14T12:00:00');
    expect(defaultSinceForEventType({ eventType: '', since: '' }, now)).toBe('');
    expect(defaultSinceForEventType({ eventType: '', since: '2026-09-01T00:00' }, now))
      .toBe('2026-09-01T00:00');
    expect(defaultSinceForEventType({ eventType: 'authorization.summary', since: '2026-09-01T00:00' }, now))
      .toBe('2026-09-01T00:00');
    expect(defaultSinceForEventType({ eventType: 'authorization.summary', since: '' }, now))
      .toBe('2026-09-13T12:00');
  });
});
