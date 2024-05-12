-- Add down migration script here
ALTER TABLE `lokoda`.`user_shows` 
DROP COLUMN `user_id`;

