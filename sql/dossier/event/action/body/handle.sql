CREATE TABLE handles (
    id BIGSERIAL PRIMARY KEY,
    body BIGINT NOT NULL,

    handle TEXT NOT NULL,

    FOREIGN KEY (body) REFERENCES bodies (id)
);