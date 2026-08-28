import { fetchJSON } from '../fetch-utils.js';

const STREAM_SUMMARY_PAGE_SIZE = 100;

/**
 * Load the credential-free stream list without ever asking the server for an
 * unbounded response. Selector and live surfaces still need the complete set,
 * so they walk fixed-size pages; Stream Configuration renders a single page.
 */
export async function fetchAllStreamSummaries({ surface = 'admin', availability = 'all', signal } = {}) {
  const streams = [];
  let page = 1;
  let totalPages = 1;
  do {
    const params = new URLSearchParams({
      summary: 'true',
      surface,
      page: String(page),
      page_size: String(STREAM_SUMMARY_PAGE_SIZE),
      sort_by: 'name',
      sort_order: 'asc',
      availability,
    });
    const response = await fetchJSON(`/api/streams?${params.toString()}`, {
      signal,
      timeout: 15000,
      retries: 1,
    });
    streams.push(...(response?.streams || []));
    totalPages = Math.max(0, Number(response?.total_pages) || 0);
    page += 1;
  } while (page <= totalPages);
  return streams;
}

/**
 * Resolve the current stream configuration for recorded media. Camera UUID is
 * authoritative because stream names can be changed after a recording was
 * captured; the name remains a compatibility fallback for legacy recordings.
 */
export function resolveRecordedStreamSummary(
  streams,
  { cameraUuid = '', streamName = '' } = {},
) {
  if (!Array.isArray(streams)) return null;

  if (cameraUuid) {
    const byCameraUuid = streams.find((stream) => stream?.camera_uuid === cameraUuid);
    if (byCameraUuid) return byCameraUuid;
  }

  if (streamName) {
    return streams.find((stream) => stream?.name === streamName) || null;
  }

  return null;
}
