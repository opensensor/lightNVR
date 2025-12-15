-- Add buffer strategy for pre-detection recording

-- migrate:up

-- Buffer strategy: 'auto', 'none', 'go2rtc', 'hls_segment', 'memory_packet', 'mmap_hybrid'
ALTER TABLE streams ADD COLUMN buffer_strategy TEXT DEFAULT 'auto';

-- migrate:down

SELECT 1;

