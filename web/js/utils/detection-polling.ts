import { evidenceJSON } from '../api/evidence.ts';
import type { DetectionResultsResponse } from '../types/core.ts';

export interface DetectionPollingOptions {
  streamName: string;
  /** Checked before each request. When false, the poll waits for the next tick. */
  canPoll: () => boolean;
  onResults: (data: DetectionResultsResponse) => void;
  onError: (error: unknown) => void;
  pollingInterval?: number;
  requestTimeout?: number;
}

/** Poll stored detection results without accumulating overlapping requests. */
export function startDetectionPolling({
  streamName,
  canPoll,
  onResults,
  onError,
  pollingInterval = 1000,
  requestTimeout = 5000,
}: DetectionPollingOptions): () => void {
  let stopped = false;
  let timer: ReturnType<typeof setTimeout> | null = null;
  let controller: AbortController | null = null;
  let errors = 0;
  let interval = pollingInterval;

  const poll = async () => {
    if (stopped) return;
    if (!canPoll()) {
      timer = setTimeout(poll, interval);
      return;
    }
    controller = new AbortController();
    const timeout = setTimeout(() => controller?.abort(), requestTimeout);
    try {
      const data = await evidenceJSON<DetectionResultsResponse>(`/api/detection/results/${encodeURIComponent(streamName)}`, {signal:controller.signal});
      if (!stopped) {
        errors = 0;
        interval = pollingInterval;
        onResults(data);
      }
    } catch (error) {
      if (!stopped) {
        errors++;
        if (errors > 3) interval = Math.min(5000, interval * 2);
        onError(error);
      }
    } finally {
      clearTimeout(timeout);
      controller = null;
      // Schedule after completion, so a slow request never overlaps the next.
      if (!stopped) timer = setTimeout(poll, interval);
    }
  };

  timer = setTimeout(poll, interval);
  return () => {
    stopped = true;
    clearTimeout(timer!);
    controller?.abort();
  };
}
