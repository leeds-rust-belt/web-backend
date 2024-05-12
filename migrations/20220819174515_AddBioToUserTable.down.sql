-- Add down migration script here
ALTER TABLE `lokoda`.`users`
DROP COLUMN `bio`;
