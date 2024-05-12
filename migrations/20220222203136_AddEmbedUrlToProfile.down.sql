-- Add down migration script here
ALTER TABLE `lokoda`.`user_profiles` 
DROP COLUMN `embed_url`;
