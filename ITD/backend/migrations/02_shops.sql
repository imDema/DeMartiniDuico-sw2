DROP TABLE IF EXISTS shop;
CREATE TABLE shop (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    description VARCHAR NOT NULL,
    image VARCHAR,
    location VARCHAR NOT NULL
);

DROP TABLE IF EXISTS department;
CREATE TABLE department (
    id SERIAL PRIMARY KEY,
    shop_id INTEGER NOT NULL REFERENCES shop(id),
    description VARCHAR NOT NULL,
    capacity INTEGER NOT NULL,
    CHECK (capacity >= 0)
);

DROP TABLE IF EXISTS schedule;
CREATE TABLE schedule (
    shop_id INTEGER NOT NULL REFERENCES shop(id),
    dow SMALLINT NOT NULL,
    open TIME WITH TIME ZONE NOT NULL,
    close TIME WITH TIME ZONE NOT NULL,
    PRIMARY KEY (shop_id, dow, open),
    CHECK (dow > 0 AND dow <= 7)
);

DROP TABLE IF EXISTS ticket;
CREATE TABLE ticket (
    id SERIAL PRIMARY KEY,
    shop_id INTEGER NOT NULL REFERENCES shop(id),
    creation TIMESTAMP WITH TIME ZONE NOT NULL,
    expiration TIMESTAMP WITH TIME ZONE NOT NULL,
    valid BOOLEAN NOT NULL
) -- TODO: Departments