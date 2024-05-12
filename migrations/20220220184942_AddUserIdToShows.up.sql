-- Add up migration script here
ALTER TABLE `lokoda`.`user_shows` 
ADD COLUMN `user_id` VARCHAR(199) NULL AFTER `venue`;

