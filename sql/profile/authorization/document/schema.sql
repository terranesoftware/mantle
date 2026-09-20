CREATE TABLE documents (
    id BIGSERIAL PRIMARY KEY,
    authorization BIGINT NOT NULL,

    name TEXT NOT NULL,
    number TEXT,
    validity DATERANGE,

    FOREIGN KEY (authorization) REFERENCES authorizations (id)
);