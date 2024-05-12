-- Add up migration script here
ALTER TABLE `lokoda`.`groups`
ADD COLUMN `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
ADD COLUMN `chat` BOOLEAN DEFAULT false;
