CREATE OR REPLACE VIEW ticket_with_departments AS
    SELECT
        ticket.id AS id,
        customer_id,
        ticket.shop_id AS shop_id,
        array_agg(department.id) AS department_ids,
        creation,
        expiration,
        entry,
        exit,
        est_minutes,
        valid,
        active
    FROM
        ticket, ticket_department, department
    WHERE
        ticket_department.ticket_id = ticket.id AND
        ticket_department.department_id = department.id
    GROUP BY
        ticket.id,
        customer_id,
        ticket.shop_id,
        creation,
        expiration,
        entry,
        exit,
        est_minutes,
        valid,
        active;