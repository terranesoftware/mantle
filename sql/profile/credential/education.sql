CREATE TABLE educations (
    id BIGSERIAL PRIMARY KEY,
    credential BIGINT NOT NULL,

    degree TEXT NOT NULL,
    discipline TEXT NOT NULL,
    location LOCATION NOT NULL,
    school TEXT NOT NULL,
    status STATUS NOT NULL,

    FOREIGN KEY (credential) REFERENCES credentials (id)
);