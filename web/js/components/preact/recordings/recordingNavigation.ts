export type RecordingView = 'table' | 'grid' | 'timeline' | 'investigation' | 'plates';
export interface RecordingNavigationContext {
  recordingsUrl?: string | null;
  streams?: string[];
  cameras?: string[];
  start?: number | null;
  end?: number | null;
  cursor?: number | null;
}

/** Return destinations stay in this workspace; never accept an arbitrary URL. */
export function recordingsReturnUrl(value?: string | null): string | undefined {
  if (!value || !/^recordings\.html(?:\?|$)/.test(value)) return undefined;
  const url = new URL(value, 'https://workspace.invalid/');
  url.searchParams.delete('return_to');
  return `recordings.html${url.search}`;
}

function localParts(seconds?: number | null) {
  if (seconds == null || !Number.isFinite(seconds)) return null;
  const date = new Date(seconds * 1000);
  if (!Number.isFinite(date.getTime())) return null;
  const pad = (n: number) => String(n).padStart(2, '0');
  return {date: `${date.getFullYear()}-${pad(date.getMonth() + 1)}-${pad(date.getDate())}`,
    time: `${pad(date.getHours())}:${pad(date.getMinutes())}:${pad(date.getSeconds())}`};
}

export function recordingViewLinks(context: RecordingNavigationContext = {}): Record<RecordingView, string> {
  const { streams = [], cameras = [], start, end, cursor } = context;
  const inherited = recordingsReturnUrl(context.recordingsUrl);
  const list = new URL(inherited || 'recordings.html', 'https://workspace.invalid/');
  const from = localParts(start), to = localParts(end), at = localParts(cursor ?? start);
  if (!inherited) {
    if (streams.length) list.searchParams.set('stream', streams.join(','));
    if (from && to) {
      list.searchParams.set('dateRange', 'custom');
      list.searchParams.set('startDate', from.date); list.searchParams.set('startTime', from.time);
      list.searchParams.set('endDate', to.date); list.searchParams.set('endTime', to.time);
    }
  }
  const listHref = `recordings.html${list.search}`;
  const params = new URLSearchParams({return_to: listHref});
  if (streams.length) params.set('stream', streams.join(','));
  if (cameras.length) params.set('cameras', cameras.join(','));
  if (from) params.set('start', String(start));
  if (to) params.set('end', String(end));
  if (localParts(cursor)) params.set('cursor', String(cursor));
  const investigation = `investigation.html?${params}`;
  // Timeline and Plates operate on a single camera; keep the full list return scope.
  if (streams.length) params.set('stream', streams[0]!);
  if (cameras.length) params.set('camera_uuid', cameras[0]!);
  if (at) { params.set('date', at.date); params.set('time', at.time); }
  const timeline = `timeline.html?${params}`;
  const plates = `plates.html?${params}`;
  list.searchParams.set('view', 'table');
  const table = `recordings.html${list.search}`;
  list.searchParams.set('view', 'grid');
  return {table, grid: `recordings.html${list.search}`, timeline, investigation, plates};
}
