export const EMPTY_AUDIT_FILTERS = Object.freeze({
  principalUserId: '',
  action: '',
  outcome: '',
  eventType: '',
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
    ['event_type', filters.eventType],
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

export const AUDIT_DECISION_MODES = Object.freeze(['record', 'summarize', 'off']);
export const AUDIT_SUMMARY_EVENT_TYPE = 'authorization.summary';

export function groupDecisionModes(modes = []) {
  const groups = [];
  const byCategory = new Map();
  modes.forEach((entry) => {
    if (!entry || !entry.action) return;
    const category = entry.category || '';
    if (!byCategory.has(category)) {
      const group = { category, actions: [] };
      byCategory.set(category, group);
      groups.push(group);
    }
    byCategory.get(category).actions.push(entry);
  });
  return groups;
}

export function modesByAction(modes = []) {
  return Object.fromEntries(
    modes.filter((entry) => entry && entry.action).map((entry) => [entry.action, entry.mode])
  );
}

export function changedDecisionModes(original = {}, draft = {}) {
  const changes = {};
  Object.entries(draft).forEach(([action, mode]) => {
    if (AUDIT_DECISION_MODES.includes(mode) && original[action] !== mode) {
      changes[action] = mode;
    }
  });
  return changes;
}

export function summaryDetails(event) {
  const details = event?.details;
  if (!details || details.event_type !== AUDIT_SUMMARY_EVENT_TYPE) return null;
  const count = Number(details.count);
  if (!Number.isFinite(count) || count < 1) return null;
  return {
    count,
    firstAt: Number(details.first_at) || 0,
    lastAt: Number(details.last_at) || 0,
  };
}

export function formatSummaryCount(count, locale = 'en-US') {
  return `×${new Intl.NumberFormat(locale).format(Number(count) || 0)}`;
}

function formatDateTimeLocal(date) {
  const pad = (value) => String(value).padStart(2, '0');
  return `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())}` +
    `T${pad(date.getHours())}:${pad(date.getMinutes())}`;
}

/**
 * `event_type` filtering scans each row's stored details rather than using an
 * index (~1.5s per page on 790k rows vs 7ms with a 24h range, under the
 * global DB lock), so once the user picks a non-empty event type with no
 * `since` bound yet, suggest one: now minus 24 hours, in the same
 * datetime-local format the since input already uses. Leaves an existing
 * since value alone, and does nothing while no event type is selected.
 */
export function defaultSinceForEventType(draftFilters = {}, now = new Date()) {
  if (!draftFilters.eventType || draftFilters.since) return draftFilters.since || '';
  return formatDateTimeLocal(new Date(now.getTime() - 24 * 60 * 60 * 1000));
}
