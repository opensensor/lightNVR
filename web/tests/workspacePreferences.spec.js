import {
  DEFAULT_WORKSPACE_VISIBILITY,
  WORKSPACE_KEYS,
  workspaceIsVisible,
  workspaceVisibilityFromResponse,
} from '../js/utils/workspace-preferences.js';

describe('workspace preferences', () => {
  test('defaults new workspaces on so upgrades remain discoverable', () => {
    expect(workspaceVisibilityFromResponse()).toEqual(DEFAULT_WORKSPACE_VISIBILITY);
    expect(workspaceIsVisible(null, WORKSPACE_KEYS.LIVE_NAVIGATOR)).toBe(true);
    expect(workspaceIsVisible(null, WORKSPACE_KEYS.INVESTIGATION)).toBe(true);
  });

  test('applies known preferences and ignores unknown workspace keys', () => {
    const response = {
      workspaces: [
        { key: WORKSPACE_KEYS.INVESTIGATION, visible: false },
        { key: 'future.workspace', visible: false },
      ],
    };
    expect(workspaceVisibilityFromResponse(response)).toEqual({
      [WORKSPACE_KEYS.LIVE_NAVIGATOR]: true,
      [WORKSPACE_KEYS.INVESTIGATION]: false,
    });
  });
});
