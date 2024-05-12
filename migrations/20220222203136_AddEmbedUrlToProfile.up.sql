-- Add up migration script here
ALTER TABLE `lokoda`.`user_profiles` 
ADD COLUMN `embed_url` VARCHAR(500) NULL AFTER `image_url`;

