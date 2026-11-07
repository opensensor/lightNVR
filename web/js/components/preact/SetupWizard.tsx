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

import type { ComponentChildren } from 'preact';
import { useState, useEffect, useCallback } from 'preact/hooks';
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
  return (
    <div class="mb-5">
      <label class="block text-sm font-medium mb-1">{label}</label>
      {children}
      {hint && <p class="text-xs text-muted-foreground mt-1">{hint}</p>}
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
          placeholder="/var/lib/lightnvr/recordings"
        />
      </Field>
      <Field label={t('setupWizard.maxStorage')} hint={t('setupWizard.maxStorageHint')}>
        <input
          type="number"
          min="0"
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
          placeholder="/var/lib/lightnvr/lightnvr.db"
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

  const [form, setForm] = useState<WizardForm>({
    storagePath: '',
    maxStorageSize: '0',
    dbPath: '',
    threadPoolSize: '',
    maxStreams: '32',
  });

  useEffect(() => {
    getSystemSettings()
      .then((data) => {
        if (!data) return;
        setForm(f => ({
          ...f,
          storagePath:    data.storage_path   || f.storagePath,
          maxStorageSize: data.max_storage_size != null ? String(data.max_storage_size) : f.maxStorageSize,
          dbPath:         data.db_path         || f.dbPath,
          threadPoolSize: data.web_thread_pool_size != null ? String(data.web_thread_pool_size) : f.threadPoolSize,
          maxStreams:     data.max_streams     != null ? String(data.max_streams)      : f.maxStreams,
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
    setForm(f => ({ ...f, [key]: value }));
  }, []);

  const saveSettings = async () => {
    setSaving(true);
    try {
      const payload: SetupSettingsPayload = {
        storage_path:         form.storagePath    || undefined,
        max_storage_size:     parseInt(form.maxStorageSize, 10) || 0,
        db_path:              form.dbPath         || undefined,
        web_thread_pool_size: form.threadPoolSize ? parseInt(form.threadPoolSize, 10) : undefined,
        max_streams:          parseInt(form.maxStreams, 10) || 32,
      };
      (Object.keys(payload) as (keyof SetupSettingsPayload)[]).forEach(k => payload[k] === undefined && delete payload[k]);

      const result = await updateSystemSettings(payload);

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
    try {
      await updateSetupStatus({complete:true});
      onClose();
    } catch (err) {
      showStatusMessage(t('setupWizard.saveFailed', {error:(err as Error).message}), 'error');
    }
  };

  const isLastDataStep = step === TOTAL_STEPS - 2;
  const isConfirmStep  = step === TOTAL_STEPS - 1;

  return (
    <div data-testid="setup-wizard" class="app-modal-overlay app-modal-overlay--dialog fixed inset-0 flex items-center justify-center bg-black/60 backdrop-blur-sm p-4">
      <div class="bg-card text-card-foreground rounded-2xl shadow-2xl w-full max-w-lg p-8">
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
