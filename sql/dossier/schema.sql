CREATE TABLE dossiers (
    id BIGSERIAL PRIMARY KEY,
    varve BIGINT NOT NULL,

    from TIMESTAMPTZ NOT NULL,
    hash BYTEA NOT NULL,
    position BIGINT NOT NULL,
    to TIMESTAMPTZ NOT NULL,

    FOREIGN KEY (varve) REFERENCES varves (id)
);