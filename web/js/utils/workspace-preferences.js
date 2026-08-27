export const WORKSPACE_KEYS = Object.freeze({
  LIVE_NAVIGATOR: 'live.navigator',
  INVESTIGATION: 'investigation',
});

export const DEFAULT_WORKSPACE_VISIBILITY = Object.freeze({
  [WORKSPACE_KEYS.LIVE_NAVIGATOR]: true,
  [WORKSPACE_KEYS.INVESTIGATION]: true,
});

export function workspaceVisibilityFromResponse(response) {
  const visibility = { ...DEFAULT_WORKSPACE_VISIBILITY };
  for (const workspace of response?.workspaces || []) {
    if (Object.hasOwn(visibility, workspace?.key) &&
        typeof workspace.visible === 'boolean') {
      visibility[workspace.key] = workspace.visible;
    }
  }
  return visibility;
}

export function workspaceIsVisible(response, key) {
  return workspaceVisibilityFromResponse(response)[key] !== false;
}
