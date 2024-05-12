-- Add down migration script here
ALTER TABLE `lokoda`.`user_groups`
DROP COLUMN `unread`;
