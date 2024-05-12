-- Add down migration script here
ALTER TABLE `lokoda`.`user_groups`
DROP COLUMN `joined_on`,
DROP COLUMN `left`,
DROP COLUMN `left_on`;
