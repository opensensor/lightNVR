import { useEffect, useState } from 'preact/hooks';
import { useI18n } from '../../i18n.js';
import { showStatusMessage } from './ToastContainer.jsx';
import { pictureInPictureAdapter } from './pictureInPicture.js';

export function PictureInPictureButton({ videoRef, disabled = false }) {
  const { t } = useI18n();
  const [supported, setSupported] = useState(false);
  const [active, setActive] = useState(false);

  useEffect(() => {
    const video = videoRef?.current;
    const adapter = pictureInPictureAdapter(video);
    if (!video || !adapter) return undefined;
    const sync = () => setActive(adapter.active());
    setSupported(true);
    sync();
    video.addEventListener('enterpictureinpicture', sync);
    video.addEventListener('leavepictureinpicture', sync);
    video.addEventListener('webkitpresentationmodechanged', sync);
    return () => {
      video.removeEventListener('enterpictureinpicture', sync);
      video.removeEventListener('leavepictureinpicture', sync);
      video.removeEventListener('webkitpresentationmodechanged', sync);
    };
  }, [videoRef]);

  if (!supported) return null;

  const toggle = async (event) => {
    event.stopPropagation();
    const adapter = pictureInPictureAdapter(videoRef?.current);
    if (!adapter) return;
    try {
      await (adapter.active() ? adapter.exit() : adapter.enter());
      setActive(adapter.active());
    } catch (error) {
      showStatusMessage(t('live.pictureInPictureError', { message: error.message }), 'error');
    }
  };

  const label = active ? t('live.exitPictureInPicture') : t('live.enterPictureInPicture');
  return (
    <button
      type="button"
      className={`pip-btn inline-flex items-center justify-center rounded text-white transition-colors ${active ? 'bg-blue-600' : 'bg-black/70 hover:bg-black/85'}`}
      onClick={toggle}
      disabled={disabled}
      title={label}
      aria-label={label}
      aria-pressed={active}
      style={{ padding: '5px', border: 0, cursor: disabled ? 'not-allowed' : 'pointer' }}
    >
      <svg viewBox="0 0 24 24" width="24" height="24" fill="none" stroke="currentColor" strokeWidth="2" strokeLinecap="round" strokeLinejoin="round" aria-hidden="true">
        <rect x="3" y="5" width="18" height="14" rx="2" />
        <rect x="12" y="11" width="7" height="6" rx="1" fill="currentColor" stroke="none" />
      </svg>
    </button>
  );
}

export default PictureInPictureButton;
