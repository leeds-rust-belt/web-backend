-- Add up migration script here
ALTER TABLE `lokoda`.`users`
ADD COLUMN `bio` VARCHAR(450) NULL AFTER `image_url`;
