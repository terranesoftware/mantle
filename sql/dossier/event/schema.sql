CREATE TABLE events (
    id BIGSERIAL PRIMARY KEY,
    dossier BIGINT NOT NULL,

    from TIMESTAMPTZ NOT NULL,
    hash BYTEA NOT NULL,
    position BIGINT NOT NULL,
    to TIMESTAMPTZ NOT NULL,

    FOREIGN KEY (dossier) REFERENCES dossiers (id)
);