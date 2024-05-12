-- Add up migration script here
ALTER TABLE `lokoda`.`user_shows`
ADD COLUMN `status` VARCHAR(45) NULL DEFAULT 'SCHEDULED' AFTER `user_id`;

