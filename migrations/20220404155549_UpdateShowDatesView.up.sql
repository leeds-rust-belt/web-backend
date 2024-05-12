-- Add up migration script here
CREATE OR REPLACE VIEW showdates AS
SELECT 
    user_id,
    STR_TO_DATE(CONCAT(`year`,'-',
        LPAD(month, 2, '00'),'-',
        LPAD(day, 2, '00')),
        '%Y-%m-%d') 
    AS showdate,
    venue,
    city,
    year,
    month,
    day
    FROM
    user_shows

