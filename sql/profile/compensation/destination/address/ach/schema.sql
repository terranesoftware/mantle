CREATE TABLE achs (
    id BIGSERIAL PRIMARY KEY,
    address BIGINT NOT NULL,

    account TEXT NOT NULL,
    kind ACCOUNT_KIND NOT NULL,
    routing TEXT NOT NULL,

    FOREIGN KEY (address) REFERENCES addresses (id)
);