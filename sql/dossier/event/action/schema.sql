CREATE TABLE actions (
    id BIGSERIAL PRIMARY KEY,
    event BIGINT NOT NULL,

    hash BYTEA NOT NULL,
    name TEXT NOT NULL,
    position BIGINT NOT NULL,
    source TEXT NOT NULL,
    time TIMESTAMPTZ NOT NULL,

    FOREIGN KEY (event) REFERENCES events (id)
);