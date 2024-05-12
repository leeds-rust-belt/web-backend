-- Add down migration script here
ALTER TABLE `lokoda`.`user_messages`
DROP COLUMN `group_id`,
DROP COLUMN `read`;
