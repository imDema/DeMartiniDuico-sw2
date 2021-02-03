CREATE OR REPLACE FUNCTION check_matching_pairs() RETURNS TRIGGER
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
    EXECUTE FUNCTION check_matching_pairs();