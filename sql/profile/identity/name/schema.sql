CREATE TABLE names (
    id BIGSERIAL PRIMARY KEY,
    identity BIGINT NOT NULL,

    latin IDENTIFIERS,
    native IDENTIFIERS,

    FOREIGN KEY (identity) REFERENCES identities (id)
);