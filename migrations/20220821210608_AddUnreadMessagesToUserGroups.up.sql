-- Add up migration script here
ALTER TABLE `lokoda`.`user_groups`
ADD COLUMN `unread` INT DEFAULT 0;
