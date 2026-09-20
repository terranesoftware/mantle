CREATE TABLE pixs (
    id BIGSERIAL PRIMARY KEY,
    address BIGINT NOT NULL,

    identifier TEXT NOT NULL,

    FOREIGN KEY (address) REFERENCES addresses (id)
);