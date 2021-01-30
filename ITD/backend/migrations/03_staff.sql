DROP TABLE IF EXISTS staff;
CREATE TABLE staff (
    id SERIAL PRIMARY KEY,
    email VARCHAR UNIQUE NOT NULL,
    salt BYTEA NOT NULL,
    digest BYTEA NOT NULL
);
CREATE UNIQUE INDEX IF NOT EXISTS staff_email ON staff (email);
