/** Poll stored detection results without accumulating overlapping requests. */
export function startDetectionPolling({
  streamName,
  canPoll,
  onResults,
  onError,
  pollingInterval = 1000,
  requestTimeout = 5000,
}) {
  let stopped = false;
  let timer = null;
  let controller = null;
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
      const response = await fetch(`/api/detection/results/${encodeURIComponent(streamName)}`, {
        signal: controller.signal,
      });
      if (!response.ok) {
        throw new Error(`Failed to fetch detection results: ${response.status}`);
      }
      const data = await response.json();
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
    clearTimeout(timer);
    controller?.abort();
  };
}
