CREATE TABLE credentials (
    id BIGSERIAL PRIMARY KEY,
    profile BIGINT NOT NULL,

    kind CREDENTIAL_KIND NOT NULL,

    FOREIGN KEY (profile) REFERENCES profiles (id)
);