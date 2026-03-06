-- Clean up any streams with blank or whitespace-only names

-- migrate:up

DELETE FROM streams WHERE TRIM(name) = '' OR name IS NULL;

-- migrate:down

SELECT 1;

