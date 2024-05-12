-- Add up migration script here
CREATE TABLE sessions (
  `user` CHAR(36) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL ,
  `token` VARCHAR(200) NOT NULL,
  `created` TIMESTAMP DEFAULT NOW(),
  PRIMARY KEY (`user`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
