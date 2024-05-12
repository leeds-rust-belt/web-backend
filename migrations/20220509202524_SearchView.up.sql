-- Add up migration script here
CREATE OR REPLACE VIEW vw_discover AS
SELECT 
        `lokoda`.`users`.`id` AS `id`,
        `lokoda`.`users`.`name` AS `name`,
        `lokoda`.`users`.`account_type` AS `account_type`,
        `lokoda`.`users`.`location` AS `location`,
        `lokoda`.`users`.`avatar_url` AS `avatar_url`,
        `lokoda`.`users`.`image_url` AS `image_url`,
        concat('[',`x`.`genres` , ']') AS `genres`
    FROM
        (`lokoda`.`users`
        LEFT JOIN (SELECT 
            `lokoda`.`user_genres`.`user_id` AS `user_id`,
                GROUP_CONCAT('{"id":"', `lokoda`.`user_genres`.`genre_id`, '","', 'genre":"', `lokoda`.`genres`.`genre`, '"}'
                    ORDER BY `lokoda`.`genres`.`genre` ASC
                    SEPARATOR ', ') AS `genres`
        FROM
            (`lokoda`.`user_genres`
        JOIN `lokoda`.`genres` ON ((`lokoda`.`genres`.`id` = `lokoda`.`user_genres`.`genre_id`)))
        GROUP BY `lokoda`.`user_genres`.`user_id`) `x` ON ((`x`.`user_id` = `lokoda`.`users`.`id`)));
