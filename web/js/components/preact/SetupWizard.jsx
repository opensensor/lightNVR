/**
 * SetupWizard.jsx
 * First-run setup wizard shown as a full-screen modal overlay.
 *
 * Steps:
 *   0 – Welcome
 *   1 – Storage & Database
 *   2 – Performance (thread pool + max streams)
 *   3 – Complete / summary
 */

import { useState, useEffect, useCallback } from 'preact/hooks';
import { showStatusMessage } from './Toast.jsx';

/* ─────────────────────────────────────────────
   Tiny shared sub-components
───────────────────────────────────────────── */

function StepDots({ current, total }) {
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

function Field({ label, hint, children }) {
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
  return (
    <div class="text-center">
      <div class="text-5xl mb-4">🎥</div>
      <h2 class="text-2xl font-bold mb-3">Welcome to LightNVR</h2>
      <p class="text-muted-foreground mb-6 max-w-sm mx-auto">
        Let's get your NVR configured in a few quick steps. You can always
        revisit these settings later from the <strong>Settings</strong> page.
      </p>
      <ul class="text-left inline-block space-y-2 text-sm">
        {[
          '📁  Storage path & size limits',
          '⚡  Thread pool & max stream count',
          '🔒  Admin password reminder',
        ].map(item => (
          <li key={item} class="flex items-center gap-2">{item}</li>
        ))}
      </ul>
    </div>
  );
}

function StorageStep({ form, onChange }) {
  return (
    <div>
      <h2 class="text-xl font-bold mb-1">Storage &amp; Database</h2>
      <p class="text-sm text-muted-foreground mb-5">
        Where should recordings and the database be stored?
      </p>
      <Field label="Recordings storage path" hint="Absolute path where video files are written.">
        <input
          type="text"
          class="w-full p-2 border border-input rounded bg-background text-foreground"
          value={form.storagePath}
          onInput={e => onChange('storagePath', e.target.value)}
          placeholder="/var/lib/lightnvr/recordings"
        />
      </Field>
      <Field label="Max storage size (GB)" hint="0 = unlimited.">
        <input
          type="number"
          min="0"
          class="w-full p-2 border border-input rounded bg-background text-foreground"
          value={form.maxStorageSize}
          onInput={e => onChange('maxStorageSize', e.target.value)}
        />
      </Field>
      <Field label="Database path" hint="SQLite database file location.">
        <input
          type="text"
          class="w-full p-2 border border-input rounded bg-background text-foreground"
          value={form.dbPath}
          onInput={e => onChange('dbPath', e.target.value)}
          placeholder="/var/lib/lightnvr/lightnvr.db"
        />
      </Field>
    </div>
  );
}



function PerformanceStep({ form, onChange, cpuCores }) {
  const recommended = cpuCores > 0 ? cpuCores * 2 : '?';
  return (
    <div>
      <h2 class="text-xl font-bold mb-1">Performance</h2>
      <p class="text-sm text-muted-foreground mb-5">
        Tune LightNVR for your hardware. Both settings require a restart.
      </p>
      <Field
        label="Worker thread pool size"
        hint={`Recommended: ${recommended} (2× your ${cpuCores > 0 ? cpuCores + '-core' : ''} CPU). Controls libuv UV_THREADPOOL_SIZE. Range: 2–128.`}
      >
        <input
          type="number"
          min="2"
          max="128"
          class="w-full p-2 border border-input rounded bg-background text-foreground"
          value={form.threadPoolSize}
          onInput={e => onChange('threadPoolSize', e.target.value)}
          placeholder={recommended.toString()}
        />
      </Field>
      <Field
        label="Maximum concurrent streams"
        hint="How many camera streams can be configured. Default 32, ceiling 256. Requires restart."
      >
        <input
          type="number"
          min="1"
          max="256"
          class="w-full p-2 border border-input rounded bg-background text-foreground"
          value={form.maxStreams}
          onInput={e => onChange('maxStreams', e.target.value)}
        />
      </Field>
    </div>
  );
}

function CompleteStep({ saved, restartRequired }) {
  return (
    <div class="text-center">
      <div class="text-5xl mb-4">✅</div>
      <h2 class="text-2xl font-bold mb-3">Setup Complete!</h2>
      <p class="text-muted-foreground mb-4">
        {saved
          ? 'Your settings have been saved successfully.'
          : 'No settings were changed — defaults will be used.'}
      </p>
      {restartRequired && (
        <div class="rounded-lg border border-yellow-400 bg-yellow-50 dark:bg-yellow-900/20 text-yellow-800 dark:text-yellow-200 p-3 text-sm mb-4">
          ⚠️ <strong>Restart required</strong> — thread pool size and/or max streams
          changes take effect after the service restarts.
        </div>
      )}
      <div class="rounded-lg border border-blue-400 bg-blue-50 dark:bg-blue-900/20 text-blue-800 dark:text-blue-200 p-3 text-sm">
        🔒 <strong>Security reminder:</strong> Make sure to change the default admin
        password in the <strong>Users</strong> section if you haven't already.
      </div>
    </div>
  );
}

/* ─────────────────────────────────────────────
   Main SetupWizard component
───────────────────────────────────────────── */

export function SetupWizard({ onClose }) {
  const TOTAL_STEPS = 4;
  const [step, setStep] = useState(0);
  const [saving, setSaving] = useState(false);
  const [saved, setSaved] = useState(false);
  const [restartRequired, setRestartRequired] = useState(false);
  const [cpuCores, setCpuCores] = useState(0);

  const [form, setForm] = useState({
    storagePath: '',
    maxStorageSize: '0',
    dbPath: '',
    threadPoolSize: '',
    maxStreams: '32',
  });

  useEffect(() => {
    fetch('/api/settings')
      .then(r => r.ok ? r.json() : null)
      .then(data => {
        if (!data) return;
        setForm(f => ({
          ...f,
          storagePath:    data.storage_path   || f.storagePath,
          maxStorageSize: data.max_storage_size != null ? String(data.max_storage_size) : f.maxStorageSize,
          dbPath:         data.db_path         || f.dbPath,
          threadPoolSize: data.web_thread_pool_size != null ? String(data.web_thread_pool_size) : f.threadPoolSize,
          maxStreams:     data.max_streams     != null ? String(data.max_streams)      : f.maxStreams,
        }));
      })
      .catch(() => {});

    fetch('/api/system/info')
      .then(r => r.ok ? r.json() : null)
      .then(data => {
        const cores = data?.system?.cpu_cores || data?.cpu_cores || 0;
        if (cores > 0) setCpuCores(cores);
      })
      .catch(() => {});
  }, []);

  const handleChange = useCallback((key, value) => {
    setForm(f => ({ ...f, [key]: value }));
  }, []);

  const saveSettings = async () => {
    setSaving(true);
    try {
      const payload = {
        storage_path:         form.storagePath    || undefined,
        max_storage_size:     parseInt(form.maxStorageSize, 10) || 0,
        db_path:              form.dbPath         || undefined,
        web_thread_pool_size: form.threadPoolSize ? parseInt(form.threadPoolSize, 10) : undefined,
        max_streams:          parseInt(form.maxStreams, 10) || 32,
      };
      Object.keys(payload).forEach(k => payload[k] === undefined && delete payload[k]);

      const res = await fetch('/api/settings', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
      });
      if (!res.ok) throw new Error(`HTTP ${res.status}`);

      setRestartRequired(!!(payload.web_thread_pool_size || payload.max_streams));
      setSaved(true);
    } catch (err) {
      showStatusMessage('Failed to save settings: ' + err.message, 'error');
    } finally {
      setSaving(false);
    }
  };

  const handleNext = async () => {
    if (step === TOTAL_STEPS - 2) await saveSettings();
    if (step < TOTAL_STEPS - 1) setStep(s => s + 1);
  };

  const handleComplete = async () => {
    await fetch('/api/setup/status', { method: 'POST' }).catch(() => {});
    onClose();
  };

  const isLastDataStep = step === TOTAL_STEPS - 2;
  const isConfirmStep  = step === TOTAL_STEPS - 1;

  return (
    <div class="fixed inset-0 z-[100] flex items-center justify-center bg-black/60 backdrop-blur-sm p-4">
      <div class="bg-card text-card-foreground rounded-2xl shadow-2xl w-full max-w-lg p-8">
        {step === 0 && <WelcomeStep />}
        {step === 1 && <StorageStep     form={form} onChange={handleChange} />}
        {step === 2 && <PerformanceStep form={form} onChange={handleChange} cpuCores={cpuCores} />}
        {step === 3 && <CompleteStep    saved={saved} restartRequired={restartRequired} />}

        <StepDots current={step} total={TOTAL_STEPS} />

        <div class={`mt-6 flex ${step > 0 && !isConfirmStep ? 'justify-between' : 'justify-end'}`}>
          {step > 0 && !isConfirmStep && (
            <button
              class="px-4 py-2 rounded border border-input text-foreground hover:bg-accent transition-colors"
              onClick={() => setStep(s => s - 1)}
              disabled={saving}
            >
              Back
            </button>
          )}
          {!isConfirmStep && (
            <button
              class="px-6 py-2 rounded bg-primary text-primary-foreground hover:bg-primary/90 transition-colors disabled:opacity-50"
              onClick={handleNext}
              disabled={saving}
            >
              {saving ? 'Saving…' : isLastDataStep ? 'Save & Continue' : 'Next →'}
            </button>
          )}
          {isConfirmStep && (
            <button
              class="px-6 py-2 rounded bg-primary text-primary-foreground hover:bg-primary/90 transition-colors"
              onClick={handleComplete}
            >
              Go to Dashboard
            </button>
          )}
        </div>

        {!isConfirmStep && (
          <p class="text-center mt-4 text-xs text-muted-foreground">
            <button class="underline hover:text-foreground" onClick={async () => {
              await fetch('/api/setup/status', { method: 'POST' }).catch(() => {});
              onClose();
            }}>
              Skip setup wizard
            </button>
          </p>
        )}
      </div>
    </div>
  );
}
