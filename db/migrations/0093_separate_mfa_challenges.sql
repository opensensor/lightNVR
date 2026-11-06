-- Keep password-verified MFA challenges distinct from authenticated sessions.
-- Old challenges are indistinguishable from full sessions, so require MFA
-- accounts to sign in again once on upgrade. Other accounts retain sessions.
-- migrate:up
DELETE FROM sessions WHERE user_id IN (SELECT id FROM users WHERE totp_enabled = 1);
ALTER TABLE sessions ADD COLUMN mfa_pending INTEGER NOT NULL DEFAULT 0 CHECK (mfa_pending IN (0, 1));
ALTER TABLE sessions ADD COLUMN mfa_attempts INTEGER NOT NULL DEFAULT 0;

-- migrate:down
DELETE FROM sessions WHERE mfa_pending = 1;
ALTER TABLE sessions DROP COLUMN mfa_attempts;
ALTER TABLE sessions DROP COLUMN mfa_pending;
