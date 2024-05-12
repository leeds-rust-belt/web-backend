-- Add down migration script here
ALTER TABLE `lokoda`.`groups`
DROP COLUMN `created_at`,
DROP COLUMN `chat`;
