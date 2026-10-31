import { isValidElement } from 'preact';
import { SaveControl } from './preact-fixture';

it('executes automatic JSX as a Preact element with typed props', () => {
  const element = <SaveControl label="Save" pending={false} onSave={async () => {}} />;
  expect(isValidElement(element)).toBe(true);
  expect(element.type).toBe(SaveControl);
  expect(element.props.label).toBe('Save');
});

it('preserves disabled state and the async handler at the DOM boundary', async () => {
  const onSave = jest.fn(async () => {});
  const element = SaveControl({ label: 'Save', pending: true, onSave });
  expect(element.type).toBe('button');
  expect(element.props.disabled).toBe(true);
  expect(element.props['aria-label']).toBe('Save');
  await element.props.onClick();
  expect(onSave).toHaveBeenCalledTimes(1);
});
