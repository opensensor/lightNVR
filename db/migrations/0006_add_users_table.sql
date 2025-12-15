-- Add users table for authentication

-- migrate:up

CREATE TABLE IF NOT EXISTS users (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    username TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    email TEXT,
    role TEXT DEFAULT 'viewer',
    created_at INTEGER DEFAULT (strftime('%s', 'now')),
    updated_at INTEGER DEFAULT (strftime('%s', 'now'))
);

-- Create default admin user (password: admin - should be changed on first login)
INSERT OR IGNORE INTO users (username, password_hash, role)
VALUES ('admin', '$2b$10$rQZ5hPkjXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXXX', 'admin');

-- migrate:down

DROP TABLE IF EXISTS users;

