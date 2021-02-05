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
    shop_id INTEGER NOT NULL REFERENCES shop(id) ON DELETE CASCADE,
    description VARCHAR NOT NULL,
    capacity INTEGER NOT NULL,
    ma_visit REAL NOT NULL DEFAULT 15,
    ma_est_visit REAL NOT NULL DEFAULT 15,
    CHECK (capacity >= 0)
);

DROP TABLE IF EXISTS schedule;
CREATE TABLE schedule (
    shop_id INTEGER NOT NULL REFERENCES shop(id) ON DELETE CASCADE,
    dow SMALLINT NOT NULL,
    open TIME NOT NULL,
    close TIME NOT NULL,
    PRIMARY KEY (shop_id, dow, open),
    CHECK (dow > 0 AND dow <= 7)
);

DROP TABLE IF EXISTS ticket;
CREATE TABLE ticket (
    id SERIAL PRIMARY KEY,
    customer_id INT NOT NULL REFERENCES customer(id) ON DELETE CASCADE,
    shop_id INTEGER NOT NULL REFERENCES shop(id) ON DELETE CASCADE,
    creation TIMESTAMP NOT NULL,
    expiration TIMESTAMP NOT NULL,
    entry TIMESTAMP,
    exit TIMESTAMP,
    est_minutes INTEGER NOT NULL,
    valid BOOLEAN NOT NULL,
    active BOOLEAN NOT NULL,
    CHECK(est_minutes > 0 AND est_minutes < 1440)
);
CREATE INDEX IF NOT EXISTS ticket_creation ON ticket(creation);

DROP TABLE IF EXISTS ticket_department;
CREATE TABLE ticket_department(
    ticket_id INT NOT NULL REFERENCES ticket(id) ON DELETE CASCADE,
    department_id INT NOT NULL REFERENCES department(id) ON DELETE CASCADE,
    PRIMARY KEY (ticket_id, department_id)
);

CREATE OR REPLACE FUNCTION check_ticket_departments_same_shop() RETURNS TRIGGER
    LANGUAGE PLPGSQL
    AS
    $$
    BEGIN
        IF (SELECT shop_id FROM ticket WHERE id = NEW.ticket_id) <> (SELECT shop_id FROM department WHERE id = NEW.department_id) THEN
            RAISE EXCEPTION 'All of the departments of a ticket must be from the same shop';
        END IF;
        RETURN NEW;
    END;
    $$;
CREATE TRIGGER ticket_departments_same_shop
    BEFORE INSERT ON ticket_department
    FOR EACH ROW
    EXECUTE FUNCTION check_ticket_departments_same_shop();
