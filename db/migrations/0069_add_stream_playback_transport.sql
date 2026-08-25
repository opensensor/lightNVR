-- Per-stream live-view transport preference and fallback policy.

-- migrate:up
ALTER TABLE streams
    ADD COLUMN playback_transport TEXT NOT NULL DEFAULT 'auto'
    CHECK (playback_transport IN
        ('auto', 'webrtc_only', 'mse_only', 'hls_only',
         'webrtc_then_mse', 'mse_then_hls'));

-- migrate:down
-- SQLite cannot drop a column while retaining compatibility with older builds.
