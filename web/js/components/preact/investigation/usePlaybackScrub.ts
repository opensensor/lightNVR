import { useRef } from 'preact/hooks';

/** Pause only for the pointer gesture; keyboard steps retain playback intent. */
export function usePlaybackScrub(playing: boolean, setPlaying: (playing: boolean) => void) {
  const gesture = useRef<{pointerId: number; resume: boolean} | null>(null);
  const finish = () => {
    if (!gesture.current) return;
    const {resume} = gesture.current;
    gesture.current = null;
    setPlaying(resume);
  };
  return {
    onPointerDown: (event: PointerEvent) => {
      if (gesture.current || !event.isPrimary || event.button !== 0) return;
      gesture.current = {pointerId: event.pointerId, resume: playing};
      setPlaying(false);
      try { (event.currentTarget as HTMLElement).setPointerCapture(event.pointerId); } catch { /* Native range still handles release. */ }
    },
    onPointerUp: (event: PointerEvent) => { if (gesture.current?.pointerId === event.pointerId) finish(); },
    onPointerCancel: (event: PointerEvent) => { if (gesture.current?.pointerId === event.pointerId) finish(); },
    onLostPointerCapture: (event: PointerEvent) => { if (gesture.current?.pointerId === event.pointerId) finish(); },
    onBlur: finish,
  };
}
