INSERT INTO customer (id, email, salt, digest) VALUES
    (1111222, 'ciccio@mail.com', 'aabbccddeeff', 'aabbccddeeff'),
    (1111333, 'pepega@mail.com', 'aabbccddeeff', 'aabbccddeeff');


INSERT INTO shop (id, name, description, location) VALUES
    (1234111, 'Unes Milano', 'Unes via unes numero unes','49.1234N,12.3456E'),
    (1234222, 'Lidl Torino', 'Lidl via lidl numero lidl','123.1234N,45.3456E'),
    (1234333, 'Fruttivendolo da Attilio', 'Frutta e verdura','2.1234S,23.3456W');

INSERT INTO department (id, shop_id, description, capacity) VALUES
    (4444111, 1234111, 'Frutta', 20),
    (4444222, 1234111, 'Pane', 15),

    (4444333, 1234222, 'Surgelati', 12),
    (4444444, 1234222, 'Carne', 20),
    (4444555, 1234222, 'Pane', 8),

    (4444666, 1234333, 'all', 4);

INSERT INTO ticket (id, customer_id, shop_id, creation, expiration, valid, active) VALUES
    (2222111, 1111222, 1234111, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, TRUE, TRUE),
    (2222222, 1111333, 1234111, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, TRUE, TRUE),
    (2222333, 1111222, 1234222, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP, TRUE, TRUE);

INSERT INTO ticket_department (ticket_id, department_id) VALUES
    (2222111, 4444111),
    (2222111, 4444222),
    (2222222, 4444222),
    (2222333, 4444444),
    (2222333, 4444555);
