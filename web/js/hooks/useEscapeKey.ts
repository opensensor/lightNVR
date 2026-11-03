import { useEffect, useRef } from 'preact/hooks';

const layers: Array<{ current: () => void }> = [];

function handleKeyDown(event: KeyboardEvent): void {
  if (event.key !== 'Escape' || event.defaultPrevented) return;
  const top = layers[layers.length - 1];
  if (!top) return;
  event.preventDefault();
  top.current();
}

/**
 * Calls onEscape when Escape is pressed while active. Only the most recently
 * activated layer responds, so a dialog opened over another closes first. Layers
 * activated in the same render have no defined order. The listener is on window, so a nested ModalDialog, which stops Escape at
 * document, handles the key first.
 *
 * To ignore Escape during work in progress, guard inside onEscape instead of
 * clearing active. Clearing it would hand the key to the layer underneath.
 */
export function useEscapeKey(onEscape: () => void, active = true): void {
  const latest = useRef(onEscape);
  latest.current = onEscape;

  useEffect(() => {
    if (!active) return undefined;
    if (layers.length === 0) window.addEventListener('keydown', handleKeyDown);
    layers.push(latest);
    return () => {
      layers.splice(layers.indexOf(latest), 1);
      if (layers.length === 0) window.removeEventListener('keydown', handleKeyDown);
    };
  }, [active]);
}
