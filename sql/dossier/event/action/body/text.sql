CREATE TABLE texts (
    id BIGSERIAL PRIMARY KEY,
    body BIGINT NOT NULL,

    text TEXT NOT NULL,

    FOREIGN KEY (body) REFERENCES bodies (id)
);