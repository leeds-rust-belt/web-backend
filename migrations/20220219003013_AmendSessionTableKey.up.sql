-- Add up migration script here
ALTER TABLE `lokoda`.`sessions`
DROP PRIMARY KEY,
ADD PRIMARY KEY (`user`, `token`);
