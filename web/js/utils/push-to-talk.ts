/** Own microphone requests and serialize sender changes across press/release races. */
export function createPushToTalk({
  onStarted,
  onStopped,
  onDetachError,
}: {
  onStarted: (stream: MediaStream) => void;
  onStopped: () => void;
  onDetachError: (error: unknown) => void;
}) {
  type Request = { sender: RTCRtpSender; stream: MediaStream | null };
  let active: Request | null = null;
  const senderQueues = new WeakMap<RTCRtpSender, Promise<void>>();

  function replace(sender: RTCRtpSender, operation: () => Promise<void>) {
    const pending = (senderQueues.get(sender) || Promise.resolve()).then(operation);
    senderQueues.set(sender, pending.catch(() => {}));
    return pending;
  }

  function stop() {
    const request = active;
    active = null;
    if (request?.stream) {
      request.stream.getTracks().forEach((track) => track.stop());
      // Queue after any pending attach, and before a subsequent press's attach.
      void replace(request.sender, () => request.sender.replaceTrack(null)).catch(onDetachError);
    }
    onStopped();
  }

  async function start(sender: RTCRtpSender, acquire: () => Promise<MediaStream>) {
    stop();
    const request: Request = { sender, stream: null };
    active = request;
    try {
      const stream = await acquire();
      if (active !== request) {
        stream.getTracks().forEach((track) => track.stop());
        return false;
      }
      request.stream = stream;
      const track = stream.getAudioTracks()[0];
      if (!track) throw new Error('Microphone returned no audio track');
      await replace(sender, async () => {
        if (active === request) await sender.replaceTrack(track);
      });
      if (active !== request) return false;
      onStarted(stream);
      return true;
    } catch (error) {
      // A rejected permission prompt from a released/old request must not
      // overwrite the state or error of a subsequent press.
      if (active !== request) return false;
      stop();
      throw error;
    }
  }

  return { start, stop };
}
