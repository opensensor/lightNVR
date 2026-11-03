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
