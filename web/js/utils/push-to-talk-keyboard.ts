/** Track the physical activation key independently of asynchronous mic state. */
export function createPushToTalkKeyboard(start: () => void, stop: () => void) {
  let heldKey: string | null = null;
  const activationKey = (key: string) => key === ' ' || key === 'Enter';
  return {
    keyDown(event: Pick<KeyboardEvent, 'key' | 'repeat' | 'preventDefault'>) {
      if (!activationKey(event.key)) return;
      event.preventDefault();
      if (event.repeat || heldKey !== null) return;
      heldKey = event.key;
      start();
    },
    keyUp(event: Pick<KeyboardEvent, 'key' | 'preventDefault'>) {
      if (!activationKey(event.key)) return;
      event.preventDefault();
      if (heldKey !== event.key) return;
      heldKey = null;
      stop();
    },
    cancel() {
      heldKey = null;
      stop();
    },
  };
}
