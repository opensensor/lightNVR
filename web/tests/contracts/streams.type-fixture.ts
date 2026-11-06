import type { StreamFullResponse, StreamSummaryPage, StreamUpdateRequest } from '../../js/api/streams.ts';
import { listStreamSummaries, updateStream } from '../../js/api/streams.ts';
import { validateStreamFullResponse } from '../../generated/api/validators.js';

/** Compiled by tsconfig.tests.json; deliberately not executed as a Jest test. */
export async function proveContractTypes(untrusted: unknown): Promise<void> {
  // @ts-expect-error Unknown wire data must pass a guard before typed access.
  const unchecked: StreamFullResponse = untrusted;
  void unchecked;
  if (validateStreamFullResponse(untrusted)) {
    const name: string = untrusted.stream.name;
    void name;
  }
  // @ts-expect-error enabled is boolean, not editable form text.
  const invalid: StreamUpdateRequest = { enabled: 'false' };
  void invalid;
  // @ts-expect-error Pagination fields are required even for empty responses.
  const missing: StreamSummaryPage = { streams: [] };
  void missing;
  const lpr = await listStreamSummaries({ surface: 'lpr' });
  // @ts-expect-error Minimal identity rows do not promise a boolean enabled field.
  const enabled: boolean = lpr.streams[0]?.enabled;
  void enabled;
  const result = await updateStream('A', { enabled: true });
  if (result.status === 202) {
    const message: string = result.data.message;
    void message;
  } else {
    const privacy: boolean = result.data.privacy_mode;
    void privacy;
  }
}
