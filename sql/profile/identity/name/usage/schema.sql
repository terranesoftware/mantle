CREATE TABLE usages (
    id BIGSERIAL PRIMARY KEY,
    name BIGINT NOT NULL,

    kind USAGE_KIND NOT NULL,

    FOREIGN KEY (name) REFERENCES names (id)
);