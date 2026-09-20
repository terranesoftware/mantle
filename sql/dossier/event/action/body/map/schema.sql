CREATE TABLE maps (
    id BIGSERIAL PRIMARY KEY,
    body BIGINT NOT NULL,

    FOREIGN KEY (body) REFERENCES bodies (id)
);