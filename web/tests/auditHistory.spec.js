import {
  EMPTY_AUDIT_FILTERS,
  auditDateRangeIsValid,
  auditOutcomeTone,
  auditPageBounds,
  buildAuditQuery,
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
