-- Add up migration script here
ALTER TABLE `lokoda`.`users` 
ADD COLUMN `avatar_url` VARCHAR(500) NULL AFTER `embed_url`;
