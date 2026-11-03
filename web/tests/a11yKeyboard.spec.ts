import { clickOnActivationKey } from '../js/utils/a11y.ts';

function press(key: string, nested = false) {
  const click = jest.fn();
  const preventDefault = jest.fn();
  const currentTarget = { click };
  const event = { key, preventDefault, currentTarget, target: nested ? {} : currentTarget };
  clickOnActivationKey(event as unknown as KeyboardEvent);
  return { click, preventDefault };
}

describe('clickOnActivationKey', () => {
  test.each(['Enter', ' '])('clicks the element and stops the default action for %j', (key) => {
    const { click, preventDefault } = press(key);
    expect(click).toHaveBeenCalledTimes(1);
    expect(preventDefault).toHaveBeenCalledTimes(1);
  });

  test('ignores other keys, so Tab keeps moving focus', () => {
    const { click, preventDefault } = press('Tab');
    expect(click).not.toHaveBeenCalled();
    expect(preventDefault).not.toHaveBeenCalled();
  });

  test('leaves keys pressed on nested controls alone', () => {
    const { click, preventDefault } = press('Enter', true);
    expect(click).not.toHaveBeenCalled();
    expect(preventDefault).not.toHaveBeenCalled();
  });
});
