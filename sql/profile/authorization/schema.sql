CREATE TABLE authorizations (
    id BIGSERIAL PRIMARY KEY,
    profile BIGINT NOT NULL,

    FOREIGN KEY (profile) REFERENCES profiles (id)
);