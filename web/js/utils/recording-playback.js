/** Wait for the server's bounded background conversion before loading media. */
export async function prepareRecordingPlayback(videoUrl, { signal, onWaiting = () => {} } = {}) {
  const url = new URL(videoUrl, globalThis.location?.href || 'http://localhost');
  if (!/^\/api\/recordings\/play\/\d+$/.test(url.pathname)) return videoUrl;
  url.searchParams.set('prepare', '1');

  while (true) {
    signal?.throwIfAborted();
    const response = await fetch(url.href, { signal, cache: 'no-store' });
    signal?.throwIfAborted();
    if (response.status === 200) return videoUrl;
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
