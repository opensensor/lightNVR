import { useEffect, useState } from 'preact/hooks';
import { fetchJSON, useMutation, useQuery, useQueryClient } from '../../../query-client.js';
import { showStatusMessage } from '../ToastContainer.jsx';
import { workspaceVisibilityFromResponse } from '../../../utils/workspace-preferences.js';

export function WorkspacesTab({ t }) {
  const queryClient = useQueryClient();
  const { data, isLoading, error } = useQuery(
    ['ui-workspaces'],
    '/api/ui/workspaces',
    { timeout: 10000, retries: 1 }
  );
  const [visibility, setVisibility] = useState(() => workspaceVisibilityFromResponse());

  useEffect(() => {
    if (data) setVisibility(workspaceVisibilityFromResponse(data));
  }, [data]);

  const mutation = useMutation({
    mutationKey: ['save-ui-workspaces'],
    mutationFn: (workspaces) => fetchJSON('/api/ui/workspaces', {
      method: 'PUT',
      headers: { 'Content-Type': 'application/json' },
      body: JSON.stringify({ workspaces }),
      timeout: 10000,
    }),
    onSuccess: (response) => {
      setVisibility(workspaceVisibilityFromResponse(response));
      queryClient.setQueryData(['ui-workspaces'], response);
      if (typeof window !== 'undefined') {
        window.dispatchEvent(new CustomEvent('lightnvr:workspaces-changed', {
          detail: response,
        }));
      }
      showStatusMessage(t('workspaces.saved'), 'success', 3000);
    },
    onError: (saveError) => {
      showStatusMessage(t('workspaces.saveError', { message: saveError.message }), 'error', 6000);
    },
  });

  const toggle = (key, visible) => {
    const next = { ...visibility, [key]: visible };
    setVisibility(next);
    mutation.mutate({ [key]: visible });
  };

  return (
    <div className="settings-group bg-card text-card-foreground rounded-lg shadow p-4 space-y-4">
      <div>
        <h3 className="text-lg font-semibold">{t('workspaces.title')}</h3>
        <p className="mt-1 text-sm text-muted-foreground">{t('workspaces.description')}</p>
      </div>
      {isLoading ? (
        <p className="text-sm text-muted-foreground">{t('common.loading')}</p>
      ) : error ? (
        <p className="text-sm text-[hsl(var(--danger))]">{t('workspaces.loadError')}</p>
      ) : (
        <div className="divide-y divide-border rounded-md border border-border">
          {(data?.workspaces || []).map((workspace) => (
            <label
              key={workspace.key}
              data-setting-label={`${workspace.label} ${workspace.description}`}
              className="flex min-h-16 items-center justify-between gap-4 p-4"
            >
              <span className="min-w-0">
                <span className="block font-medium">{workspace.label}</span>
                <span className="mt-1 block text-sm text-muted-foreground">{workspace.description}</span>
              </span>
              <input
                type="checkbox"
                className="h-5 w-5 shrink-0 rounded border-input"
                checked={visibility[workspace.key] !== false}
                disabled={!workspace.configurable || mutation.isPending}
                onChange={(event) => toggle(workspace.key, event.currentTarget.checked)}
                aria-label={t('workspaces.showWorkspace', { name: workspace.label })}
              />
            </label>
          ))}
        </div>
      )}
      <p className="text-xs text-muted-foreground">{t('workspaces.authorizationNote')}</p>
    </div>
  );
}
