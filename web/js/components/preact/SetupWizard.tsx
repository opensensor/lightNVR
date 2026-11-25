import {getSystemSettings,getSystemInformation,updateSystemSettings,updateSetupStatus} from '../../api/system.ts';
/**
 * SetupWizard.tsx
 * First-run setup wizard shown as a full-screen modal overlay.
 *
 * Steps:
 *   0 – Welcome
 *   1 – Storage & Database
 *   2 – Performance (thread pool + max streams)
 *   3 – Complete / summary
 */

import { cloneElement } from 'preact';
import type { ComponentChildren, VNode } from 'preact';
import { useState, useEffect, useCallback, useId, useRef } from 'preact/hooks';
import { showStatusMessage } from './ToastContainer.tsx';
import { useI18n } from '../../i18n.ts';

interface WizardForm {
  storagePath: string;
  maxStorageSize: string;
  dbPath: string;
  threadPoolSize: string;
  maxStreams: string;
}

type WizardFormChange = (key: keyof WizardForm, value: string) => void;

interface SetupSettingsPayload {
  storage_path?: string;
  max_storage_size?: number;
  db_path?: string;
  web_thread_pool_size?: number;
  max_streams?: number;
}

/* ─────────────────────────────────────────────
   Tiny shared sub-components
───────────────────────────────────────────── */

function StepDots({ current, total }: { current: number; total: number }) {
  return (
    <div class="flex justify-center gap-2 mt-6">
      {Array.from({ length: total }).map((_, i) => (
        <span
          key={i}
          class={`inline-block w-2.5 h-2.5 rounded-full transition-colors ${
            i === current
              ? 'bg-primary'
              : i < current
              ? 'bg-primary/40'
              : 'bg-border'
          }`}
        />
      ))}
    </div>
  );
}

function Field({ label, hint, children }: { label: ComponentChildren; hint?: ComponentChildren; children?: ComponentChildren }) {
  const inputId = useId();
  const hintId = `${inputId}-hint`;

  const childNode = children as VNode | undefined;
  const enhancedChild =
    childNode && typeof childNode === 'object' && 'type' in childNode
      ? cloneElement(childNode, {
          id: inputId,
          'aria-describedby': hint ? hintId : undefined,
        })
      : children;

  return (
    <div class="mb-5">
      <label for={inputId} class="block text-sm font-medium mb-1">{label}</label>
      {enhancedChild}
      {hint && <p id={hintId} class="text-xs text-muted-foreground mt-1">{hint}</p>}
    </div>
  );
}

/* ─────────────────────────────────────────────
   Step components
───────────────────────────────────────────── */

function WelcomeStep() {
  const { t } = useI18n();
  return (
    <div class="text-center">
      <div class="text-5xl mb-4">🎥</div>
      <h2 class="text-2xl font-bold mb-3">{t('setupWizard.welcomeTitle')}</h2>
      <p class="text-muted-foreground mb-6 max-w-sm mx-auto">
        {t('setupWizard.welcomeIntro')}
      </p>
      <ul class="text-start inline-block space-y-2 text-sm">
        {[
          `📁  ${t('setupWizard.welcomeStorage')}`,
          `⚡  ${t('setupWizard.welcomePerformance')}`,
          `🔒  ${t('setupWizard.welcomeSecurity')}`,
        ].map(item => (
          <li key={item} class="flex items-center gap-2">{item}</li>
        ))}
      </ul>
    </div>
  );
}

function StorageStep({ form, onChange }: { form: WizardForm; onChange: WizardFormChange }) {
  const { t } = useI18n();
  return (
    <div>
      <h2 class="text-xl font-bold mb-1">{t('setupWizard.storageTitle')}</h2>
      <p class="text-sm text-muted-foreground mb-5">
        {t('setupWizard.storageIntro')}
      </p>
      <Field label={t('setupWizard.storagePath')} hint={t('setupWizard.storagePathHint')}>
        <input
          type="text"
          class="w-full p-2 border border-input rounded bg-background text-foreground"
          value={form.storagePath}
          onInput={e => onChange('storagePath', (e.target as HTMLInputElement).value)}
          placeholder="/var/lib/lightnvr/data/recordings"
        />
      </Field>
      <Field label={t('setupWizard.maxStorage')} hint={t('setupWizard.maxStorageHint')}>
        <input
          type="number"
          min="0"
          step="any"
          class="w-full p-2 border border-input rounded bg-background text-foreground"
          value={form.maxStorageSize}
          onInput={e => onChange('maxStorageSize', (e.target as HTMLInputElement).value)}
        />
      </Field>
      <Field label={t('setupWizard.dbPath')} hint={t('setupWizard.dbPathHint')}>
        <input
          type="text"
          class="w-full p-2 border border-input rounded bg-background text-foreground"
          value={form.dbPath}
          onInput={e => onChange('dbPath', (e.target as HTMLInputElement).value)}
          placeholder="/var/lib/lightnvr/data/database/lightnvr.db"
        />
      </Field>
    </div>
  );
}



function PerformanceStep({ form, onChange, cpuCores, maxStreamsCeiling }: {
  form: WizardForm;
  onChange: WizardFormChange;
  cpuCores: number;
  maxStreamsCeiling: number;
}) {
  const { t } = useI18n();
  const recommended = cpuCores > 0 ? cpuCores * 2 : '?';
  return (
    <div>
      <h2 class="text-xl font-bold mb-1">{t('setupWizard.performanceTitle')}</h2>
      <p class="text-sm text-muted-foreground mb-5">
        {t('setupWizard.performanceIntro')}
      </p>
      <div class="rounded-lg border border-blue-400 bg-blue-50 dark:bg-blue-900/20 text-blue-800 dark:text-blue-200 p-3 text-sm mb-4 space-y-2">
        <p>
          <strong>{t('setupWizard.threadPool')}</strong>{' '}
          {t('setupWizard.threadPoolInfo')}
        </p>
        <p>
          <strong>{t('setupWizard.maxStreams')}</strong>{' '}
          {t('setupWizard.maxStreamsInfo')}
        </p>
      </div>
      <div class="rounded-lg border border-yellow-400 bg-yellow-50 dark:bg-yellow-900/20 text-yellow-800 dark:text-yellow-200 p-3 text-sm mb-5">
        ⚠️ <strong>{t('setupWizard.important')}</strong>{' '}
        {t('setupWizard.restartWarning')}
      </div>
      <Field
        label={t('setupWizard.threadPool')}
        hint={cpuCores > 0
          ? t('setupWizard.threadPoolHintCores', { recommended, cores: cpuCores })
          : t('setupWizard.threadPoolHint', { recommended })}
      >
        <input
          type="number"
          min="2"
          max="128"
          class="w-full p-2 border border-input rounded bg-background text-foreground"
          value={form.threadPoolSize}
          onInput={e => onChange('threadPoolSize', (e.target as HTMLInputElement).value)}
          placeholder={recommended.toString()}
        />
      </Field>
      <Field
        label={t('setupWizard.maxStreams')}
        hint={t('setupWizard.maxStreamsHint', { max: maxStreamsCeiling })}
      >
        <input
          type="number"
          min="1"
          max={maxStreamsCeiling}
          class="w-full p-2 border border-input rounded bg-background text-foreground"
          value={form.maxStreams}
          onInput={e => onChange('maxStreams', (e.target as HTMLInputElement).value)}
        />
      </Field>
    </div>
  );
}

function CompleteStep({ saved, restartRequired }: { saved: boolean; restartRequired: boolean }) {
  const { t } = useI18n();
  return (
    <div class="text-center">
      <div class="text-5xl mb-4">✅</div>
      <h2 class="text-2xl font-bold mb-3">{t('setupWizard.completeTitle')}</h2>
      <p class="text-muted-foreground mb-4">
        {saved
          ? t('setupWizard.saved')
          : t('setupWizard.notSaved')}
      </p>
      {restartRequired && (
        <div class="rounded-lg border border-yellow-400 bg-yellow-50 dark:bg-yellow-900/20 text-yellow-800 dark:text-yellow-200 p-3 text-sm mb-4">
          ⚠️ <strong>{t('setupWizard.restartRequired')}</strong>{' — '}
          {t('setupWizard.restartRequiredInfo')}
        </div>
      )}
    </div>
  );
}

/* ─────────────────────────────────────────────
   Main SetupWizard component
───────────────────────────────────────────── */

export function SetupWizard({ onClose }: { onClose: () => void }) {
  const { t } = useI18n();
  const TOTAL_STEPS = 4;
  const [step, setStep] = useState(0);
  const [saving, setSaving] = useState(false);
  const [saved, setSaved] = useState(false);
  const [restartRequired, setRestartRequired] = useState(false);
  const [cpuCores, setCpuCores] = useState(0);
  const [maxStreamsCeiling, setMaxStreamsCeiling] = useState(1024);
  const dirtyFieldsRef = useRef<Record<keyof WizardForm, boolean>>({
    storagePath: false,
    maxStorageSize: false,
    dbPath: false,
    threadPoolSize: false,
    maxStreams: false,
  });
  const BYTES_PER_GB = 1024 ** 3;

  const bytesToGbString = (bytes: number): string => {
    if (!Number.isFinite(bytes) || bytes <= 0) return '0';
    const gb = bytes / BYTES_PER_GB;
    return String(gb);
  };

  const [form, setForm] = useState<WizardForm>({
    storagePath: '',
    maxStorageSize: '0',
    dbPath: '',
    threadPoolSize: '',
    maxStreams: '32',
  });
  const formRef = useRef(form);
  formRef.current = form;

  useEffect(() => {
    getSystemSettings()
      .then((data) => {
        if (!data) return;
        setForm(f => ({
          ...f,
          storagePath: dirtyFieldsRef.current.storagePath
            ? f.storagePath
            : (data.storage_path || f.storagePath),
          maxStorageSize: dirtyFieldsRef.current.maxStorageSize
            ? f.maxStorageSize
            : (data.max_storage_size != null ? bytesToGbString(data.max_storage_size) : f.maxStorageSize),
          dbPath: dirtyFieldsRef.current.dbPath
            ? f.dbPath
            : (data.db_path || f.dbPath),
          threadPoolSize: dirtyFieldsRef.current.threadPoolSize
            ? f.threadPoolSize
            : (data.web_thread_pool_size != null ? String(data.web_thread_pool_size) : f.threadPoolSize),
          maxStreams: dirtyFieldsRef.current.maxStreams
            ? f.maxStreams
            : (data.max_streams != null ? String(data.max_streams) : f.maxStreams),
        }));
        if (data.max_streams_ceiling != null) {
          setMaxStreamsCeiling(data.max_streams_ceiling);
        }
      })
      .catch(() => {});

    getSystemInformation()
      .then((data) => {
        const cores = data.cpu.cores || 0;
        if (cores > 0) setCpuCores(cores);
      })
      .catch(() => {});
  }, []);

  const handleChange = useCallback((key: keyof WizardForm, value: string) => {
    dirtyFieldsRef.current[key] = true;
    formRef.current = { ...formRef.current, [key]: value };
    setForm(f => ({ ...f, [key]: value }));
  }, []);

  const saveSettings = async () => {
    setSaving(true);
    try {
      const submittedFormSnapshot = {
        storagePath: form.storagePath,
        maxStorageSize: form.maxStorageSize,
        dbPath: form.dbPath,
        threadPoolSize: form.threadPoolSize,
        maxStreams: form.maxStreams,
      };
      const submittedSnapshotKey = JSON.stringify(submittedFormSnapshot);

      const parsedMaxStorageGb = Number(submittedFormSnapshot.maxStorageSize);
      if (!Number.isFinite(parsedMaxStorageGb) || parsedMaxStorageGb < 0) {
        showStatusMessage(t('setupWizard.saveFailed', { error: 'Max storage size must be a finite number greater than or equal to 0.' }), 'error');
        return false;
      }
      const validatedMaxStorageBytes = Math.round(parsedMaxStorageGb * BYTES_PER_GB);
      if (!Number.isSafeInteger(validatedMaxStorageBytes)) {
        showStatusMessage(t('setupWizard.saveFailed', { error: 'Max storage size is too large.' }), 'error');
        return false;
      }

      let validatedThreadPoolSize: number | undefined = undefined;
      if (submittedFormSnapshot.threadPoolSize.trim() !== '') {
        const parsedThreadPoolSize = Number(submittedFormSnapshot.threadPoolSize);
        if (!Number.isFinite(parsedThreadPoolSize) || !Number.isInteger(parsedThreadPoolSize)) {
          showStatusMessage(t('setupWizard.saveFailed', { error: 'Thread pool size must be an integer.' }), 'error');
          return false;
        }
        if (parsedThreadPoolSize < 2 || parsedThreadPoolSize > 128) {
          showStatusMessage(t('setupWizard.saveFailed', { error: 'Thread pool size must be between 2 and 128.' }), 'error');
          return false;
        }
        validatedThreadPoolSize = parsedThreadPoolSize;
      }

      const parsedMaxStreams = Number(submittedFormSnapshot.maxStreams);
      if (!Number.isFinite(parsedMaxStreams) || !Number.isInteger(parsedMaxStreams)) {
        showStatusMessage(t('setupWizard.saveFailed', { error: 'Max streams must be an integer.' }), 'error');
        return false;
      }
      if (parsedMaxStreams < 1 || parsedMaxStreams > maxStreamsCeiling) {
        showStatusMessage(t('setupWizard.saveFailed', { error: `Max streams must be between 1 and ${maxStreamsCeiling}.` }), 'error');
        return false;
      }

      if (!settingsLoaded || settingsLoadFailed) {
        showStatusMessage(
          t('setupWizard.saveFailed', { error: 'Settings are still loading or failed to load. Please wait and try again.' }),
          'error'
        );
        return false;
      }

      const payload: SetupSettingsPayload = {
        storage_path:         submittedFormSnapshot.storagePath || undefined,
        max_storage_size:     validatedMaxStorageBytes,
        db_path:              submittedFormSnapshot.dbPath || undefined,
        web_thread_pool_size: validatedThreadPoolSize,
        max_streams:          parsedMaxStreams,
      };
      (Object.keys(payload) as (keyof SetupSettingsPayload)[]).forEach(k => payload[k] === undefined && delete payload[k]);

      const result = await updateSystemSettings(payload);

      const currentSnapshotKey = JSON.stringify(formRef.current);
      if (currentSnapshotKey !== submittedSnapshotKey) {
        showStatusMessage(
          t('setupWizard.saveFailed', { error: 'Settings changed while saving. Please save again to apply the latest values.' }),
          'error'
        );
        return false;
      }

      setRestartRequired(result.restart_required);
      setSaved(true);
      return true;
    } catch (err) {
      showStatusMessage(t('setupWizard.saveFailed', { error: (err as Error).message }), 'error');
      return false;
    } finally {
      setSaving(false);
    }
  };

  const handleNext = async () => {
    if (step === TOTAL_STEPS - 2 && !await saveSettings()) return;
    if (step < TOTAL_STEPS - 1) setStep(s => s + 1);
  };

  const handleComplete = async () => {
    if (saving) return;
    try {
      await updateSetupStatus({complete:true});
      onClose();
    } catch (err) {
      showStatusMessage(t('setupWizard.saveFailed', {error:(err as Error).message}), 'error');
    }
  };

  const isLastDataStep = step === TOTAL_STEPS - 2;
  const isConfirmStep  = step === TOTAL_STEPS - 1;
  const dialogRef = useRef<HTMLDivElement>(null);
  const previousFocusRef = useRef<HTMLElement | null>(null);
  const handleCompleteRef = useRef(handleComplete);
  const savingRef = useRef(saving);
  savingRef.current = saving;
  handleCompleteRef.current = handleComplete;
  const dialogTitleId = useId();

  useEffect(() => {
    previousFocusRef.current = document.activeElement as HTMLElement | null;
    dialogRef.current?.focus();

    const handleKeyDown = (e: KeyboardEvent) => {
      if (e.key === 'Escape') {
        if (savingRef.current) return;
        e.preventDefault();
        void handleCompleteRef.current();
        return;
      }
      if (e.key !== 'Tab' || !dialogRef.current) return;

      const focusable = dialogRef.current.querySelectorAll<HTMLElement>(
        'a[href], area[href], input:not([disabled]), select:not([disabled]), textarea:not([disabled]), button:not([disabled]), iframe, object, embed, [contenteditable], [tabindex]:not([tabindex="-1"])'
      );
      if (focusable.length === 0) {
        e.preventDefault();
        dialogRef.current.focus();
        return;
      }

      const first = focusable[0]!;
      const last = focusable[focusable.length - 1]!;
      const active = document.activeElement as HTMLElement | null;

      if (e.shiftKey) {
        if (active === first || active === dialogRef.current) {
          e.preventDefault();
          last.focus();
        }
      } else if (active === last) {
        e.preventDefault();
        first.focus();
      }
    };

    document.addEventListener('keydown', handleKeyDown);
    return () => {
      document.removeEventListener('keydown', handleKeyDown);
      previousFocusRef.current?.focus();
    };
  }, []);

  return (
    <div data-testid="setup-wizard" class="app-modal-overlay app-modal-overlay--dialog fixed inset-0 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4">
      <div
        ref={dialogRef}
        role="dialog"
        aria-modal="true"
        aria-labelledby={dialogTitleId}
        tabIndex={-1}
        class="bg-card text-card-foreground rounded-2xl shadow-2xl w-full max-w-lg p-8 max-h-[calc(100vh-2rem)] overflow-y-auto"
      >
        <h2 id={dialogTitleId} class="sr-only">{t('setupWizard.welcomeTitle')}</h2>
        {step === 0 && <WelcomeStep />}
        {step === 1 && <StorageStep     form={form} onChange={handleChange} />}
        {step === 2 && <PerformanceStep form={form} onChange={handleChange} cpuCores={cpuCores} maxStreamsCeiling={maxStreamsCeiling} />}
        {step === 3 && <CompleteStep    saved={saved} restartRequired={restartRequired} />}

        <StepDots current={step} total={TOTAL_STEPS} />

        <div class={`mt-6 flex ${step > 0 && !isConfirmStep ? 'justify-between' : 'justify-end'}`}>
          {step > 0 && !isConfirmStep && (
            <button
              class="px-4 py-2 rounded border border-input text-foreground hover:bg-accent transition-colors"
              onClick={() => setStep(s => s - 1)}
              disabled={saving}
            >
              {t('setupWizard.back')}
            </button>
          )}
          {!isConfirmStep && (
            <button
              class="px-6 py-2 rounded bg-primary text-primary-foreground hover:bg-primary/90 transition-colors disabled:opacity-50"
              onClick={handleNext}
              disabled={saving}
            >
              {saving ? t('setupWizard.saving') : isLastDataStep ? t('setupWizard.saveContinue') : t('setupWizard.next')}
            </button>
          )}
          {isConfirmStep && (
            <button
              class="px-6 py-2 rounded bg-primary text-primary-foreground hover:bg-primary/90 transition-colors"
              onClick={handleComplete}
            >
              {t('setupWizard.finish')}
            </button>
          )}
        </div>

        {!isConfirmStep && (
          <p class="text-center mt-4 text-xs text-muted-foreground">
            <button class="underline hover:text-foreground" onClick={handleComplete}>
              {t('setupWizard.skip')}
            </button>
          </p>
        )}
      </div>
    </div>
  );
}
