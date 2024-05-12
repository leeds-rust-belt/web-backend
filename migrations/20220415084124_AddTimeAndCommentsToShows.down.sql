-- Add down migration script here
ALTER TABLE `lokoda`.`user_shows`
DROP COLUMN `time`,
DROP COLUMN `comments`;
