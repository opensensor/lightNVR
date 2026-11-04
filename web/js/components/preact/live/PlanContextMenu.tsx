import { useEffect, useLayoutEffect, useRef, useState } from 'preact/hooks';
import type { PlanMenuState } from '../../../types/live.ts';

export interface PlanContextMenuProps {
  menu: PlanMenuState | null;
  label: string;
  onClose: () => void;
}

/**
 * Right-click menu for the building-plan editor. Items are
 * `{ id, label, shortcut?, disabled?, danger?, onSelect }`, `{ separator: true }`
 * or `{ id, swatches: [{ id, className, label, active, onSelect }] }`.
 * It closes on outside press, Escape, wheel, resize or blur, and supports
 * arrow-key navigation like a native menu.
 */
export function PlanContextMenu({ menu, label, onClose }: PlanContextMenuProps) {
  const ref = useRef<HTMLDivElement | null>(null);
  const [position, setPosition] = useState<{ left: number; top: number } | null>(null);

  useLayoutEffect(() => {
    if (!menu || !ref.current) return;
    const rect = ref.current.getBoundingClientRect();
    const margin = 8;
    setPosition({
      left: Math.max(margin, Math.min(menu.clientX, window.innerWidth - rect.width - margin)),
      top: Math.max(margin, Math.min(menu.clientY, window.innerHeight - rect.height - margin)),
    });
    const first = ref.current.querySelector<HTMLElement>('[role="menuitem"]:not([disabled])');
    try {
      first?.focus({ preventScroll: true });
    } catch {
      // Focus is a keyboard convenience only.
    }
  }, [menu]);

  useEffect(() => {
    if (!menu) return undefined;
    const closeOnOutside = (event: Event) => {
      if (ref.current && !ref.current.contains(event.target as Node | null)) onClose();
    };
    const close = () => onClose();
    document.addEventListener('pointerdown', closeOnOutside, true);
    window.addEventListener('wheel', close, { passive: true });
    window.addEventListener('resize', close);
    window.addEventListener('blur', close);
    return () => {
      document.removeEventListener('pointerdown', closeOnOutside, true);
      window.removeEventListener('wheel', close);
      window.removeEventListener('resize', close);
      window.removeEventListener('blur', close);
    };
  }, [menu, onClose]);

  if (!menu) return null;

  const onKeyDown = (event: KeyboardEvent) => {
    event.stopPropagation();
    if (event.key === 'Escape' || event.key === 'Tab') {
      event.preventDefault();
      onClose();
      return;
    }
    if (event.key !== 'ArrowDown' && event.key !== 'ArrowUp' &&
      event.key !== 'Home' && event.key !== 'End') return;
    event.preventDefault();
    const items = Array.from(ref.current?.querySelectorAll<HTMLElement>('[role="menuitem"]:not([disabled])') || []);
    if (items.length === 0) return;
    const index = items.indexOf(document.activeElement as HTMLElement);
    let next = 0;
    if (event.key === 'ArrowDown') next = index < 0 ? 0 : (index + 1) % items.length;
    if (event.key === 'ArrowUp') next = index <= 0 ? items.length - 1 : index - 1;
    if (event.key === 'End') next = items.length - 1;
    items[next]!.focus();
  };

  const run = (callback: () => unknown) => {
    onClose();
    callback();
  };

  return (
    <div ref={ref} className="live-plan-context-menu" role="menu" aria-label={label} tabIndex={-1}
      style={position
        ? { left: `${position.left}px`, top: `${position.top}px` }
        : { left: `${menu.clientX}px`, top: `${menu.clientY}px`, visibility: 'hidden' }}
      onKeyDown={onKeyDown}
      onContextMenu={(event) => event.preventDefault()}>
      {menu.items.map((item, index) => {
        if (item.separator) {
          return <div key={`sep-${index}`} role="separator" className="live-plan-context-separator" />;
        }
        if (item.swatches) {
          return (
            <div key={item.id} className="live-plan-context-swatches" role="group" aria-label={item.label}>
              {item.swatches.map((swatch) => (
                <button key={swatch.id} type="button" role="menuitem"
                  className={`${swatch.className} ${swatch.active ? 'is-active' : ''}`}
                  aria-label={swatch.label} title={swatch.label}
                  onClick={() => run(swatch.onSelect)}>
                  <i />
                </button>
              ))}
            </div>
          );
        }
        return (
          <button key={item.id} type="button" role="menuitem" disabled={item.disabled}
            className={item.danger ? 'is-danger' : ''}
            onClick={() => run(item.onSelect)}>
            <span>{item.label}</span>
            {item.shortcut && <kbd>{item.shortcut}</kbd>}
          </button>
        );
      })}
    </div>
  );
}
