-- Keep stopped recordings visible while reporting the result of media checks.
-- migrate:up
ALTER TABLE recordings ADD COLUMN playback_issue TEXT NOT NULL DEFAULT '';

-- migrate:down
ALTER TABLE recordings DROP COLUMN playback_issue;
