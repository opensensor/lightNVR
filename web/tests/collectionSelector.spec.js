import {
  buildCollectionSelector,
  collectionRuleToExpression,
  createCollectionRule,
  parseCollectionSelector,
  updateCollectionRuleType,
  validateCollectionSelectorJson,
} from '../js/components/preact/fleet/collectionSelector.js';

describe('camera collection selector builder', () => {
  test('creates an all selector when no complete rules exist', () => {
    expect(buildCollectionSelector([createCollectionRule('location_subtree')])).toEqual({
      version: 1,
      expression: { op: 'all' },
    });
  });

  test('maps typed rules to selector v1 nodes', () => {
    expect(collectionRuleToExpression({ type: 'tag_none', values: ['a', 'a', 'b'] })).toEqual({
      op: 'tag_none', uuids: ['a', 'b'],
    });
    expect(collectionRuleToExpression({ type: 'enabled', value: false })).toEqual({
      op: 'enabled', value: false,
    });
    expect(collectionRuleToExpression({ type: 'camera_uuid', values: ['a', 'a', 'b'] })).toEqual({
      op: 'camera_uuid', values: ['a', 'b'],
    });
  });

  test('builds any and all groups without unnecessary nesting', () => {
    const rules = [
      { type: 'health', values: ['down'] },
      { type: 'recording_mode', values: ['off'] },
    ];
    expect(buildCollectionSelector(rules, 'any').expression).toEqual({
      op: 'or',
      children: [
        { op: 'health', values: ['down'] },
        { op: 'recording_mode', values: ['off'] },
      ],
    });
    expect(buildCollectionSelector([rules[0]], 'any').expression).toEqual({ op: 'health', values: ['down'] });
  });

  test('round trips selectors supported by the visual builder', () => {
    const selector = buildCollectionSelector([
      { type: 'location_subtree', uuid: 'location-a' },
      { type: 'capability_all', values: ['onvif', 'ptz'] },
      { type: 'enabled', value: true },
      { type: 'camera_uuid', values: ['camera-a'] },
    ]);
    const parsed = parseCollectionSelector(selector);
    expect(parsed.supported).toBe(true);
    expect(buildCollectionSelector(parsed.rules, parsed.match)).toEqual(selector);
  });

  test('routes nested and negated expressions to advanced JSON mode', () => {
    expect(parseCollectionSelector({
      version: 1,
      expression: { op: 'not', child: { op: 'health', values: ['down'] } },
    }).supported).toBe(false);
  });

  test('resets values when changing a rule type and validates advanced JSON', () => {
    expect(updateCollectionRuleType({ type: 'health', values: ['up'] }, 'enabled')).toEqual({ type: 'enabled', value: true });
    expect(validateCollectionSelectorJson('{"version":1,"expression":{"op":"all"}}').error).toBe('');
    expect(validateCollectionSelectorJson('{broken').selector).toBeNull();
  });
});
