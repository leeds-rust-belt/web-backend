-- Add down migration script here
ALTER TABLE `lokoda`.`users` 
DROP COLUMN `embed_url`,
DROP COLUMN `image_url`;
