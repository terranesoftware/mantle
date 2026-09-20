CREATE TABLE ibans (
    id BIGSERIAL PRIMARY KEY,
    address BIGINT NOT NULL,

    iban TEXT NOT NULL,

    FOREIGN KEY (address) REFERENCES addresses (id)
);