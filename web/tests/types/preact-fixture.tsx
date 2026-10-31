import type { ComponentChildren, JSX } from 'preact';

export interface SaveControlProps {
  label: string;
  pending: boolean;
  onSave: () => Promise<void>;
  children?: ComponentChildren;
}

/** A real typed Preact boundary used to verify compiler and Jest transforms. */
export function SaveControl({ label, pending, onSave, children }: SaveControlProps): JSX.Element {
  return (
    <button type="button" disabled={pending} aria-label={label} onClick={onSave}>
      {children ?? label}
    </button>
  );
}
