-- Add up migration script here
ALTER TABLE `lokoda`.`users` 
ADD COLUMN `embed_url` VARCHAR(500) NULL AFTER `remember_token`,
ADD COLUMN `image_url` VARCHAR(500) NULL AFTER `embed_url`;
