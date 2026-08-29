import { createPortal } from 'preact/compat';
import { useEffect, useRef, useState } from 'preact/hooks';

const FOCUSABLE_SELECTOR = [
  'button:not([disabled])',
  '[href]',
  'input:not([disabled])',
  'select:not([disabled])',
  'textarea:not([disabled])',
  '[tabindex]:not([tabindex="-1"])',
].join(',');

let nextDialogId = 0;

function ModalShell({
  isOpen,
  onClose,
  title,
  description,
  children,
  closeDisabled = false,
  initialFocusRef,
}) {
  const dialogRef = useRef(null);
  const idRef = useRef(null);
  if (!idRef.current) {
    nextDialogId += 1;
    idRef.current = `lightnvr-dialog-${nextDialogId}`;
  }

  useEffect(() => {
    if (!isOpen) return undefined;

    const previouslyFocused = document.activeElement;
    const focusTimer = window.setTimeout(() => {
      const preferred = initialFocusRef?.current;
      const firstFocusable = dialogRef.current?.querySelector(FOCUSABLE_SELECTOR);
      (preferred || firstFocusable || dialogRef.current)?.focus();
    }, 0);

    const handleKeyDown = (event) => {
      if (event.key === 'Escape') {
        event.preventDefault();
        event.stopPropagation();
        if (!closeDisabled) onClose();
        return;
      }
      if (event.key !== 'Tab' || !dialogRef.current) return;

      const focusable = [...dialogRef.current.querySelectorAll(FOCUSABLE_SELECTOR)];
      if (focusable.length === 0) {
        event.preventDefault();
        dialogRef.current.focus();
        return;
      }
      const first = focusable[0];
      const last = focusable[focusable.length - 1];
      if (event.shiftKey && document.activeElement === first) {
        event.preventDefault();
        last.focus();
      } else if (!event.shiftKey && document.activeElement === last) {
        event.preventDefault();
        first.focus();
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    return () => {
      window.clearTimeout(focusTimer);
      document.removeEventListener('keydown', handleKeyDown);
      if (previouslyFocused && typeof previouslyFocused.focus === 'function') {
        previouslyFocused.focus();
      }
    };
  }, [closeDisabled, initialFocusRef, isOpen, onClose]);

  if (!isOpen) return null;

  const titleId = `${idRef.current}-title`;
  const descriptionId = description ? `${idRef.current}-description` : undefined;
  const handleBackdropMouseDown = (event) => {
    event.stopPropagation();
    if (event.target === event.currentTarget && !closeDisabled) onClose();
  };

  return createPortal(
    <div
      className="fixed inset-0 z-[80] flex items-center justify-center bg-black/55 p-4"
      data-keyboard-nav-preserve
      onMouseDown={handleBackdropMouseDown}
      onClick={(event) => event.stopPropagation()}
    >
      <section
        ref={dialogRef}
        role="dialog"
        aria-modal="true"
        aria-labelledby={titleId}
        aria-describedby={descriptionId}
        tabIndex="-1"
        className="w-full max-w-md rounded-lg border border-border bg-card text-card-foreground shadow-2xl"
      >
        <header className="border-b border-border px-5 py-4">
          <h2 id={titleId} className="text-lg font-semibold">{title}</h2>
          {description && (
            <p id={descriptionId} className="mt-1 text-sm text-muted-foreground">
              {description}
            </p>
          )}
        </header>
        {children}
      </section>
    </div>,
    document.body
  );
}

/** A reusable confirmation modal for warning and destructive actions. */
export function ConfirmDialog({
  isOpen,
  onClose,
  onConfirm,
  title,
  message,
  confirmLabel = 'Continue',
  cancelLabel = 'Cancel',
  variant = 'warning',
  confirmDisabled = false,
}) {
  const cancelRef = useRef(null);
  const danger = variant === 'danger';

  const handleConfirm = () => {
    onConfirm();
    onClose();
  };

  return (
    <ModalShell
      isOpen={isOpen}
      onClose={onClose}
      title={title}
      initialFocusRef={cancelRef}
    >
      <div className="px-5 py-4">
        <div className="flex items-start gap-3">
          <span
            aria-hidden="true"
            className={`flex h-9 w-9 flex-none items-center justify-center rounded-full text-lg ${
              danger
                ? 'bg-red-100 text-red-700 dark:bg-red-900/30 dark:text-red-300'
                : 'bg-amber-100 text-amber-700 dark:bg-amber-900/30 dark:text-amber-300'
            }`}
          >
            !
          </span>
          <p className="pt-1.5 text-sm text-muted-foreground">{message}</p>
        </div>
      </div>
      <footer className="flex justify-end gap-3 border-t border-border px-5 py-4">
        <button ref={cancelRef} type="button" className="btn-secondary" onClick={onClose}>
          {cancelLabel}
        </button>
        <button
          type="button"
          className={danger ? 'btn-danger' : 'btn-primary'}
          disabled={confirmDisabled}
          onClick={handleConfirm}
        >
          {confirmLabel}
        </button>
      </footer>
    </ModalShell>
  );
}

/** A modal form backed by a real text input, optionally with one checkbox. */
export function TextInputDialog({
  isOpen,
  onClose,
  onSubmit,
  title,
  description,
  inputLabel = 'Name',
  initialValue = '',
  placeholder = '',
  confirmLabel = 'Save',
  cancelLabel = 'Cancel',
  checkboxLabel,
  checkboxInitialValue = false,
  maxLength = 127,
}) {
  const inputRef = useRef(null);
  const [value, setValue] = useState(initialValue);
  const [checked, setChecked] = useState(checkboxInitialValue);
  const [submitting, setSubmitting] = useState(false);

  useEffect(() => {
    if (!isOpen) return;
    setValue(initialValue);
    setChecked(checkboxInitialValue);
    setSubmitting(false);
  }, [checkboxInitialValue, initialValue, isOpen]);

  const submit = async (event) => {
    event.preventDefault();
    const trimmed = value.trim();
    if (!trimmed || submitting) return;
    setSubmitting(true);
    try {
      const result = await onSubmit(trimmed, checked);
      if (result !== false) onClose();
    } finally {
      setSubmitting(false);
    }
  };

  return (
    <ModalShell
      isOpen={isOpen}
      onClose={onClose}
      title={title}
      description={description}
      closeDisabled={submitting}
      initialFocusRef={inputRef}
    >
      <form onSubmit={submit}>
        <div className="space-y-4 px-5 py-4">
          <label className="block text-sm font-medium">
            <span className="mb-1.5 block">{inputLabel}</span>
            <input
              ref={inputRef}
              type="text"
              className="w-full rounded-md border border-input bg-background px-3 py-2 text-foreground focus:outline-none focus:ring-2 focus:ring-[hsl(var(--primary))]"
              value={value}
              placeholder={placeholder}
              maxLength={maxLength}
              required
              disabled={submitting}
              onInput={(event) => setValue(event.currentTarget.value)}
            />
          </label>
          {checkboxLabel && (
            <label className="touch-target flex cursor-pointer items-start gap-3 text-sm">
              <input
                type="checkbox"
                className="mt-0.5 h-4 w-4"
                checked={checked}
                disabled={submitting}
                onChange={(event) => setChecked(event.currentTarget.checked)}
              />
              <span>{checkboxLabel}</span>
            </label>
          )}
        </div>
        <footer className="flex justify-end gap-3 border-t border-border px-5 py-4">
          <button type="button" className="btn-secondary" disabled={submitting} onClick={onClose}>
            {cancelLabel}
          </button>
          <button type="submit" className="btn-primary" disabled={submitting || !value.trim()}>
            {confirmLabel}
          </button>
        </footer>
      </form>
    </ModalShell>
  );
}

/** A one-action informational modal, used in place of browser alert(). */
export function AlertDialog({ isOpen, onClose, title, message, closeLabel = 'Close' }) {
  const closeRef = useRef(null);
  return (
    <ModalShell isOpen={isOpen} onClose={onClose} title={title} initialFocusRef={closeRef}>
      <div className="px-5 py-4">
        <p className="text-sm text-muted-foreground">{message}</p>
      </div>
      <footer className="flex justify-end border-t border-border px-5 py-4">
        <button ref={closeRef} type="button" className="btn-primary" onClick={onClose}>
          {closeLabel}
        </button>
      </footer>
    </ModalShell>
  );
}
