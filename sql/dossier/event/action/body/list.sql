CREATE TABLE lists (
    id BIGSERIAL PRIMARY KEY,
    body BIGINT NOT NULL,

    contains BIGINT[] NOT NULL,

    FOREIGN KEY (body) REFERENCES bodies (id)
);