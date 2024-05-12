-- Add up migration script here
CREATE OR REPLACE VIEW showdates AS
SELECT 
    id,
    user_id,
    STR_TO_DATE(CONCAT(`year`,'-',
        LPAD(month, 2, '00'),'-',
        LPAD(day, 2, '00')),
        '%Y-%m-%d') 
    AS showdate,
    year,
    month,
    day,
    venue,
    city,
    status,
    time,
    comments
    FROM
    user_shows
