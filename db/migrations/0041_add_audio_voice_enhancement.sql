-- Add audio_voice_enhancement flag to streams table
--
-- Opt-in per-stream toggle for the voice-enhancement filter chain
-- (e.g. afftdn / highpass / lowpass) applied during the existing
-- G.711 → AAC transcode for recordings.  See discussion #395.
--
-- Defaults off so upgrades preserve bit-exact recordings.

-- migrate:up

ALTER TABLE streams ADD COLUMN audio_voice_enhancement INTEGER DEFAULT 0;

-- migrate:down

SELECT 1;
