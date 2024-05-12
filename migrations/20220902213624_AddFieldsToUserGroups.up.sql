-- Add up migration script here
ALTER TABLE `lokoda`.`user_groups`
ADD COLUMN `joined_on` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
ADD COLUMN `left` BOOLEAN DEFAULT false,
ADD COLUMN `left_on` timestamp NULL;
