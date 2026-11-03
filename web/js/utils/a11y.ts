/**
 * Lets a non-button element with role="button" respond to the keyboard the way
 * a button does, by re-dispatching Enter and Space as a click on that element.
 * Keys pressed on controls nested inside it are left alone.
 */
export function clickOnActivationKey(event: KeyboardEvent): void {
  if (event.target !== event.currentTarget) return;
  if (event.key !== 'Enter' && event.key !== ' ') return;
  event.preventDefault();
  (event.currentTarget as HTMLElement).click();
}

/**
 * Maps a left or right arrow key to the inline side it points at, so arrow
 * navigation follows the visual layout in both LTR and RTL.
 */
export function inlineSideOfArrow(
  key: string,
  dir: string = document.documentElement.dir,
): 'start' | 'end' | null {
  if (key !== 'ArrowLeft' && key !== 'ArrowRight') return null;
  return (key === 'ArrowRight') === (dir !== 'rtl') ? 'end' : 'start';
}
