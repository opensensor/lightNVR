import { useState } from 'preact/hooks';

import { fetchJSON } from '../../../query-client.js';
import {
  buildInvestigationBookmarkPayload,
  investigationBookmarkUrl,
} from './investigationUtils.js';

function formatBookmarkRange(bookmark) {
  const start = new Date(bookmark.start_time * 1000).toLocaleString();
  const end = new Date(bookmark.end_time * 1000).toLocaleString();
  return `${start} – ${end}`;
}

export function InvestigationBookmarks({
  timeline,
  cursor,
  primaryCameraUuid,
  searchFilters,
  selectedResult,
  t,
}) {
  const [mode, setMode] = useState(null);
  const [title, setTitle] = useState('');
  const [note, setNote] = useState('');
  const [bookmarks, setBookmarks] = useState([]);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState('');
  const [confirmDelete, setConfirmDelete] = useState(null);

  const close = () => {
    if (loading) return;
    setMode(null);
    setError('');
    setConfirmDelete(null);
  };

  const showSave = () => {
    setTitle('');
    setNote('');
    setError('');
    setMode('save');
  };

  const showList = async () => {
    setMode('list');
    setLoading(true);
    setError('');
    try {
      const data = await fetchJSON('/api/investigation-bookmarks', {
        timeout: 15000,
        retries: 1,
      });
      setBookmarks(data.bookmarks || []);
    } catch (requestError) {
      setError(requestError.message);
    } finally {
      setLoading(false);
    }
  };

  const save = async (event) => {
    event.preventDefault();
    const payload = buildInvestigationBookmarkPayload({
      title,
      note,
      timeline,
      cursor,
      primaryCameraUuid,
      searchFilters,
      selectedResult,
    });
    if (!payload?.title) {
      setError(t('investigation.bookmarks.titleRequired'));
      return;
    }
    setLoading(true);
    setError('');
    try {
      await fetchJSON('/api/investigation-bookmarks', {
        method: 'POST',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify(payload),
        timeout: 15000,
        retries: 0,
      });
      setMode(null);
    } catch (requestError) {
      setError(requestError.message);
    } finally {
      setLoading(false);
    }
  };

  const remove = async (bookmark) => {
    if (confirmDelete !== bookmark.uuid) {
      setConfirmDelete(bookmark.uuid);
      return;
    }
    setLoading(true);
    setError('');
    try {
      await fetchJSON(`/api/investigation-bookmarks/${bookmark.uuid}`, {
        method: 'DELETE',
        headers: { 'Content-Type': 'application/json' },
        body: JSON.stringify({ revision: bookmark.revision }),
        timeout: 15000,
        retries: 0,
      });
      setBookmarks((current) => current.filter((item) => item.uuid !== bookmark.uuid));
      setConfirmDelete(null);
    } catch (requestError) {
      setError(requestError.message);
    } finally {
      setLoading(false);
    }
  };

  return (
    <div className="investigation-bookmark-actions">
      <button
        type="button"
        className="btn-secondary"
        disabled={!timeline || !primaryCameraUuid}
        onClick={showSave}
      >
        {t('investigation.bookmarks.save')}
      </button>
      <button type="button" className="btn-secondary" onClick={showList}>
        {t('investigation.bookmarks.open')}
      </button>

      {mode && (
        <div className="investigation-bookmark-backdrop" role="presentation">
          <section
            className="investigation-bookmark-dialog"
            role="dialog"
            aria-modal="true"
            aria-labelledby="investigation-bookmark-dialog-title"
          >
            <header>
              <div>
                <h2 id="investigation-bookmark-dialog-title">
                  {mode === 'save'
                    ? t('investigation.bookmarks.saveTitle')
                    : t('investigation.bookmarks.listTitle')}
                </h2>
                <p>{t('investigation.bookmarks.notHold')}</p>
              </div>
              <button
                type="button"
                className="btn-secondary"
                disabled={loading}
                aria-label={t('common.close')}
                onClick={close}
              >
                ×
              </button>
            </header>
            {error && <div className="investigation-error" role="alert">{error}</div>}

            {mode === 'save' ? (
              <form className="investigation-bookmark-form" onSubmit={save}>
                <label>
                  <span>{t('investigation.bookmarks.title')}</span>
                  <input
                    autoFocus
                    type="text"
                    maxLength="127"
                    required
                    value={title}
                    onInput={(event) => setTitle(event.target.value)}
                  />
                </label>
                <label>
                  <span>{t('investigation.bookmarks.note')}</span>
                  <textarea
                    rows="4"
                    maxLength="2047"
                    value={note}
                    onInput={(event) => setNote(event.target.value)}
                  />
                </label>
                <div className="investigation-bookmark-dialog-actions">
                  <button type="button" className="btn-secondary" onClick={close}>
                    {t('common.cancel')}
                  </button>
                  <button type="submit" className="btn-primary" disabled={loading}>
                    {loading
                      ? t('common.saving')
                      : t('investigation.bookmarks.save')}
                  </button>
                </div>
              </form>
            ) : (
              <div className="investigation-bookmark-list">
                {loading && bookmarks.length === 0 && <p>{t('common.loading')}</p>}
                {!loading && bookmarks.length === 0 && !error && (
                  <p className="investigation-bookmark-empty">
                    {t('investigation.bookmarks.empty')}
                  </p>
                )}
                {bookmarks.map((bookmark) => (
                  <article key={bookmark.uuid} className="investigation-bookmark-card">
                    <div>
                      <h3>{bookmark.title}</h3>
                      <time>{formatBookmarkRange(bookmark)}</time>
                      <small>
                        {t('investigation.bookmarks.cameraCount', {
                          count: bookmark.camera_uuids.length,
                        })}
                      </small>
                      {bookmark.note && <p>{bookmark.note}</p>}
                    </div>
                    <div className="investigation-bookmark-card-actions">
                      <button
                        type="button"
                        className="btn-primary"
                        onClick={() => window.location.assign(
                          investigationBookmarkUrl(bookmark),
                        )}
                      >
                        {t('investigation.bookmarks.restore')}
                      </button>
                      <button
                        type="button"
                        className="btn-secondary"
                        disabled={loading}
                        onClick={() => remove(bookmark)}
                      >
                        {confirmDelete === bookmark.uuid
                          ? t('investigation.bookmarks.confirmDelete')
                          : t('common.delete')}
                      </button>
                    </div>
                  </article>
                ))}
              </div>
            )}
          </section>
        </div>
      )}
    </div>
  );
}
