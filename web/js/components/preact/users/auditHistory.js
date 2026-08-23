export const EMPTY_AUDIT_FILTERS = Object.freeze({
  principalUserId: '',
  action: '',
  outcome: '',
  targetUuid: '',
  requestId: '',
  since: '',
  until: '',
});

function unixTimestamp(value) {
  if (!value) return 0;
  if (typeof value === 'number') return Number.isFinite(value) && value > 0 ? Math.floor(value) : 0;
  const milliseconds = new Date(value).getTime();
  return Number.isFinite(milliseconds) && milliseconds > 0
    ? Math.floor(milliseconds / 1000)
    : 0;
}

export function buildAuditQuery(filters = {}, page = 1, pageSize = 50) {
  const params = new URLSearchParams();
  params.set('page', String(Math.max(1, Number.parseInt(page, 10) || 1)));
  params.set('page_size', String(Math.max(1, Number.parseInt(pageSize, 10) || 50)));

  const textFilters = [
    ['principal_user_id', filters.principalUserId],
    ['action', filters.action],
    ['outcome', filters.outcome],
    ['target_uuid', filters.targetUuid],
    ['request_id', filters.requestId],
  ];
  textFilters.forEach(([name, value]) => {
    const trimmed = String(value || '').trim();
    if (trimmed) params.set(name, trimmed);
  });

  const since = unixTimestamp(filters.since);
  const until = unixTimestamp(filters.until);
  if (since > 0) params.set('since', String(since));
  if (until > 0) params.set('until', String(until));
  return params.toString();
}

export function auditDateRangeIsValid(filters = {}) {
  const since = unixTimestamp(filters.since);
  const until = unixTimestamp(filters.until);
  return since === 0 || until === 0 || since <= until;
}

export function auditPageBounds(page) {
  const total = Number(page?.total) || 0;
  const count = Number(page?.count) || 0;
  const pageNumber = Math.max(1, Number(page?.page) || 1);
  const pageSize = Math.max(1, Number(page?.page_size) || 1);
  if (total === 0 || count === 0) return { start: 0, end: 0, total };
  const start = (pageNumber - 1) * pageSize + 1;
  return { start, end: start + count - 1, total };
}

export function auditOutcomeTone(outcome) {
  if (outcome === 'success' || outcome === 'allowed') return 'success';
  if (outcome === 'error') return 'danger';
  if (outcome === 'denied' || outcome === 'failure') return 'warning';
  return 'info';
}
