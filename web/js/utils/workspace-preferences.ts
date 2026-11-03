import type { WorkspacesResponse } from '../types/core.ts';

export const WORKSPACE_KEYS = Object.freeze({
  LIVE_NAVIGATOR: 'live.navigator',
  INVESTIGATION: 'investigation',
} as const);

export type WorkspaceKey = (typeof WORKSPACE_KEYS)[keyof typeof WORKSPACE_KEYS];

export type WorkspaceVisibility = Record<WorkspaceKey, boolean>;

export const DEFAULT_WORKSPACE_VISIBILITY: Readonly<WorkspaceVisibility> = Object.freeze({
  [WORKSPACE_KEYS.LIVE_NAVIGATOR]: true,
  [WORKSPACE_KEYS.INVESTIGATION]: true,
});

export function workspaceVisibilityFromResponse(response?: WorkspacesResponse | null): WorkspaceVisibility {
  const visibility: WorkspaceVisibility = { ...DEFAULT_WORKSPACE_VISIBILITY };
  for (const workspace of response?.workspaces || []) {
    if (Object.hasOwn(visibility, workspace?.key as string) &&
        typeof workspace.visible === 'boolean') {
      visibility[workspace.key as WorkspaceKey] = workspace.visible;
    }
  }
  return visibility;
}

export function workspaceIsVisible(response: WorkspacesResponse | null | undefined, key: WorkspaceKey): boolean {
  return workspaceVisibilityFromResponse(response)[key] !== false;
}
