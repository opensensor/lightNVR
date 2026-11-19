import { useCallback, useEffect, useRef, useState } from 'preact/hooks';
import type { RefObject } from 'preact';
import { createLastFrameController } from './lastFrame.ts';
import type { LastFrameController, LastFrameControllerDeps } from './lastFrame.ts';
import type { LastFrameImage } from '../../../types/live.ts';

export interface UseLastFrameOptions {
  streamName: string;
  videoRef: RefObject<HTMLVideoElement | null>;
  hasError: boolean;
  enabled: boolean;
  /** Test seam; production uses the real fetch, canvas and URL helpers. */
  deps?: LastFrameControllerDeps;
}

export interface LastFrameState {
  /** Image to show behind the error overlay, or null for the plain overlay. */
  image: LastFrameImage | null;
  /**
   * Capture the current frame. Call it only on a failure path, right before
   * the cell resets the video's source (never on healthy teardown such as a
   * settings change). It is a no-op unless the video has played since the
   * last capture, and the image is only encoded if the tile enters error.
   */
  capture: () => boolean;
}

/**
 * Last known image of a live tile's camera while the tile is in its error
 * state (#974). Shared by the MSE, HLS and WebRTC cells; see lastFrame.ts.
 *
 * Call it before the cell's own effects so its unmount cleanup runs first and
 * teardown on unmount does not capture a frame nobody will see.
 */
export function useLastFrame({ streamName, videoRef, hasError, enabled, deps }: UseLastFrameOptions): LastFrameState {
  const [image, setImage] = useState<LastFrameImage | null>(null);
  const controllerRef = useRef<LastFrameController | null>(null);
  if (!controllerRef.current) {
    controllerRef.current = createLastFrameController(setImage, deps, enabled);
  }
  const controller = controllerRef.current;

  useEffect(() => () => controller.dispose(), []);

  useEffect(() => {
    controller.setStream(streamName || '');
  }, [streamName]);

  useEffect(() => {
    controller.setEnabled(!!enabled);
  }, [enabled]);

  useEffect(() => {
    controller.setError(!!hasError, videoRef.current);
  }, [hasError]);

  // "playing" arms capture and, after an outage, removes the image at once.
  const video = videoRef.current;
  useEffect(() => {
    const element = videoRef.current;
    if (!element || typeof element.addEventListener !== 'function') return undefined;
    const onPlaying = () => controller.recovered();
    element.addEventListener('playing', onPlaying);
    return () => element.removeEventListener('playing', onPlaying);
  }, [video]);

  const capture = useCallback(() => controller.capture(videoRef.current), []);

  return { image, capture };
}

export default useLastFrame;
