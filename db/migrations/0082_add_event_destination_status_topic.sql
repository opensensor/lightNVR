-- Optional explicit MQTT presence topic for managed event destinations.

-- migrate:up

ALTER TABLE event_destinations
ADD COLUMN status_topic_template TEXT NOT NULL DEFAULT '';

-- migrate:down

ALTER TABLE event_destinations DROP COLUMN status_topic_template;
