-- Add up migration script here
ALTER TABLE `lokoda`.`user_messages`
ADD COLUMN `group_id` char(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
ADD COLUMN `read` BOOLEAN DEFAULT false;
