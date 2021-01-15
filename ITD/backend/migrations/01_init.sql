-- SET ROLE clup;

DROP TABLE IF EXISTS customer;
CREATE TABLE customer (
    id SERIAL PRIMARY KEY,
    email VARCHAR NOT NULL,
    salt BYTEA NOT NULL,
    digest BYTEA NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS customer_email ON customer (email);

DROP TABLE IF EXISTS temp_customer;
CREATE TABLE temp_customer (
    code BYTEA PRIMARY KEY,
    email VARCHAR NOT NULL,
    salt BYTEA NOT NULL,
    digest BYTEA NOT NULL
);
