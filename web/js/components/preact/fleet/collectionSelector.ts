import type {
  CollectionRule,
  CollectionSelectorParseResult,
  CollectionSelectorValidation,
  FleetSelector,
  FleetSelectorExpression,
  FleetSelectorInput,
} from '../../../types/fleet.ts';

export const COLLECTION_RULE_TYPES = [
  'location_subtree',
  'tag_any',
  'tag_all',
  'tag_none',
  'health',
  'enabled',
  'recording_mode',
  'capability_any',
  'capability_all',
  'vendor',
  'model',
];

export const CAMERA_SELECTOR_RULE_TYPE = 'camera_uuid';

const VISUAL_RULE_TYPES = new Set([
  ...COLLECTION_RULE_TYPES,
  CAMERA_SELECTOR_RULE_TYPE,
]);

const MULTI_VALUE_RULES = new Set([
  'tag_any', 'tag_all', 'tag_none', 'health', 'recording_mode',
  'capability_any', 'capability_all', 'vendor', 'model',
]);

export function createCollectionRule(type = 'health'): CollectionRule {
  if (type === 'location_subtree') return { type, uuid: '' };
  if (type === 'enabled') return { type, value: true };
  return { type, values: [] };
}

function normalizeValues(values: unknown[] | null | undefined): string[] {
  return [...new Set((values || []).map((value) => String(value).trim()).filter(Boolean))];
}

export function collectionRuleToExpression(rule: CollectionRule): FleetSelectorExpression | null {
  if (!VISUAL_RULE_TYPES.has(rule.type)) return null;
  if (rule.type === 'location_subtree') {
    return rule.uuid ? { op: rule.type, uuid: rule.uuid } : null;
  }
  if (rule.type === 'enabled') {
    return typeof rule.value === 'boolean' ? { op: rule.type, value: rule.value } : null;
  }
  const values = normalizeValues(rule.values);
  if (values.length === 0) return null;
  const key = rule.type.startsWith('tag_') ? 'uuids' : 'values';
  return { op: rule.type, [key]: values };
}

export function buildCollectionSelector(rules: CollectionRule[], match = 'all'): FleetSelector {
  const children = rules.map(collectionRuleToExpression).filter(Boolean) as FleetSelectorExpression[];
  let expression: FleetSelectorExpression = { op: 'all' };
  if (children.length === 1) expression = children[0]!;
  else if (children.length > 1) expression = { op: match === 'any' ? 'or' : 'and', children };
  return { version: 1, expression };
}

function expressionToRule(expression: FleetSelectorExpression | null | undefined): CollectionRule | null {
  if (!expression || !VISUAL_RULE_TYPES.has(expression.op)) return null;
  if (expression.op === 'location_subtree') return { type: expression.op, uuid: expression.uuid || '' };
  if (expression.op === 'enabled') return { type: expression.op, value: expression.value === true };
  if (MULTI_VALUE_RULES.has(expression.op) || expression.op === CAMERA_SELECTOR_RULE_TYPE) {
    const values = expression.op.startsWith('tag_') ? expression.uuids : expression.values;
    return { type: expression.op, values: normalizeValues(values) };
  }
  return null;
}

export function parseCollectionSelector(selector: FleetSelectorInput | null | undefined): CollectionSelectorParseResult {
  if (!selector || selector.version !== 1 || !selector.expression) {
    return { supported: false, match: 'all', rules: [] };
  }
  const expression = selector.expression;
  if (expression.op === 'all') return { supported: true, match: 'all', rules: [] };
  if (expression.op === 'and' || expression.op === 'or') {
    const rules = (expression.children || []).map(expressionToRule);
    if (rules.some((rule) => !rule)) return { supported: false, match: 'all', rules: [] };
    return { supported: true, match: expression.op === 'or' ? 'any' : 'all', rules: rules as CollectionRule[] };
  }
  const rule = expressionToRule(expression);
  return rule
    ? { supported: true, match: 'all', rules: [rule] }
    : { supported: false, match: 'all', rules: [] };
}

export function updateCollectionRuleType(rule: CollectionRule, type: string): CollectionRule {
  if (rule.type === type) return rule;
  return createCollectionRule(type);
}

export function validateCollectionSelectorJson(value: string): CollectionSelectorValidation {
  try {
    const selector: FleetSelector | null = JSON.parse(value);
    if (selector?.version !== 1 || !selector.expression?.op) {
      return { selector: null, error: 'Selector must contain version 1 and an expression' };
    }
    return { selector, error: '' };
  } catch (error) {
    return { selector: null, error: (error as Error).message };
  }
}
