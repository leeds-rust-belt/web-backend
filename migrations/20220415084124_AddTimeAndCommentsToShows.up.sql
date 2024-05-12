-- Add up migration script here
ALTER TABLE `lokoda`.`user_shows`
ADD COLUMN `time` VARCHAR(15) NULL AFTER `status`,
ADD COLUMN `comments` TEXT NULL;
