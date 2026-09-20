CREATE TABLE upis (
    id BIGSERIAL PRIMARY KEY,
    address BIGINT NOT NULL,

    vpa TEXT NOT NULL,

    FOREIGN KEY (address) REFERENCES addresses (id)
);