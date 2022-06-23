CREATE TABLE IF NOT EXISTS uuid_cache (
    uuid varchar(36) NOT NULL,
    signature TEXT NOT NULL,
    value TEXT NOT NULL,
    exp bigint(64) NOT NULL
);

CREATE TABLE IF NOT EXISTS player_cache (
    uuid varchar(36) PRIMARY KEY NOT NULL,
    nickname varchar(16) NOT NULL,
    exp bigint(64) NOT NULL
);