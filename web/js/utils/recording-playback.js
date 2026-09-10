/** Prepare a compatibility copy only after the browser rejects the original. */
export async function prepareRecordingPlayback(videoUrl, { signal, onWaiting = () => {} } = {}) {
  const url = new URL(videoUrl, globalThis.location?.href || 'http://localhost');
  if (!/^\/api\/recordings\/play\/\d+$/.test(url.pathname)) return videoUrl;
  url.searchParams.set('transcode', '1');
  const playbackUrl = url.href;
  url.searchParams.set('prepare', '1');

  while (true) {
    signal?.throwIfAborted();
    const response = await fetch(url.href, { signal, cache: 'no-store' });
    signal?.throwIfAborted();
    if (response.status === 200) return playbackUrl;
    if (response.status !== 202) {
      const error = await response.json().catch(() => ({}));
      throw new Error(error.error || 'Unable to prepare recording for playback. Please try again later.');
    }
    onWaiting();
    const retryAfter = Number(response.headers.get('Retry-After')) || 2;
    await new Promise((resolve, reject) => {
      const abort = () => {
        clearTimeout(timer);
        reject(signal.reason);
      };
      const timer = setTimeout(() => {
        signal?.removeEventListener('abort', abort);
        resolve();
      }, Math.min(10, Math.max(1, retryAfter)) * 1000);
      signal?.addEventListener('abort', abort, { once: true });
      if (signal?.aborted) abort();
    });
  }
}

/**
 * Load the original immediately. Fall back once on a decode/unsupported-format
 * error; network errors and aborted loads must not start an expensive encode.
 * The returned cleanup owns both media loading and any fallback polling.
 */
export function loadRecordingPlayback(video, videoUrl, {
  onPreparing = () => {}, onReady = () => {}, onError = () => {},
} = {}) {
  const controller = new AbortController();
  let fallbackStarted = false;
  const handleError = async () => {
    const code = video.error?.code;
    if (controller.signal.aborted || !code || code === 1) return;
    if (fallbackStarted || (code !== 3 && code !== 4)) {
      onError(new Error('Unable to play this recording. Download the original or try again later.'));
      return;
    }
    fallbackStarted = true;
    onPreparing();
    // Clear the failed source while polling; no further media requests are
    // needed until the compatible copy is ready.
    video.removeAttribute('src');
    video.load();
    try {
      const url = await prepareRecordingPlayback(videoUrl, { signal: controller.signal });
      if (controller.signal.aborted) return;
      video.src = url;
      video.load();
    } catch (error) {
      if (!controller.signal.aborted) onError(error);
    }
  };
  video.addEventListener('error', handleError);
  video.addEventListener('loadedmetadata', onReady);
  video.src = videoUrl;
  video.load();
  return () => {
    controller.abort();
    video.removeEventListener('error', handleError);
    video.removeEventListener('loadedmetadata', onReady);
    video.pause();
    video.removeAttribute('src');
    video.load();
  };
}
