-- Versioned client-side fisheye/ePTZ calibration. The browser owns the
-- projection and view state; the NVR continues to record the untouched source.

-- migrate:up
ALTER TABLE streams ADD COLUMN eptz_config TEXT NOT NULL DEFAULT ''
    CHECK(length(eptz_config) <= 1023);

-- migrate:down
-- SQLite cannot drop a column while retaining compatibility with older builds.
