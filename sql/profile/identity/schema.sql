CREATE SCHEMA identity;

CREATE TABLE identities (
    id BIGSERIAL PRIMARY KEY,
    profile BIGINT NOT NULL,

    birth DATE NOT NULL,
    emails TEXT[] NOT NULL,
    phones TEXT[] NOT NULL,

    FOREIGN KEY (profile) REFERENCES profiles (id)
);