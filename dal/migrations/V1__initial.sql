CREATE TABLE IF NOT EXISTS premium_skin_cache (
    uuid VARCHAR(36) NOT NULL PRIMARY KEY,
    signature TEXT NOT NULL,
    value TEXT NOT NULL,
    expires_at BIGINT(64) NOT NULL
);

CREATE TABLE IF NOT EXISTS premium_playername_cache (
    nickname VARCHAR(16) PRIMARY KEY NOT NULL,
    uuid VARCHAR(36) NOT NULL,
    expires_at bigint(64) NOT NULL
);