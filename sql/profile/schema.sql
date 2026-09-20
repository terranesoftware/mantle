CREATE TABLE profiles (
    id BIGSERIAL PRIMARY KEY,
    varve BIGINT NOT NULL,

    FOREIGN KEY (varve) REFERENCES varves (id)
);