-- Add up migration script here
CREATE TABLE `genres` (
  `id` int NOT NULL AUTO_INCREMENT,
  `genre` varchar(191) CHARACTER SET utf8mb4 COLLATE utf8mb4_unicode_ci NOT NULL,
  `created_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
  `updated_at` timestamp NULL DEFAULT CURRENT_TIMESTAMP,
  PRIMARY KEY (`id`)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;
INSERT INTO `genres` (`genre`) VALUES ("Alternative");
INSERT INTO `genres` (`genre`) VALUES ("Blues");
INSERT INTO `genres` (`genre`) VALUES ("Classic Rock");
INSERT INTO `genres` (`genre`) VALUES ("Country");
INSERT INTO `genres` (`genre`) VALUES ("Emo");
INSERT INTO `genres` (`genre`) VALUES ("Folk");
INSERT INTO `genres` (`genre`) VALUES ("Grime");
INSERT INTO `genres` (`genre`) VALUES ("Grunge");
INSERT INTO `genres` (`genre`) VALUES ("Hardcore");
INSERT INTO `genres` (`genre`) VALUES ("Hip Hop");
INSERT INTO `genres` (`genre`) VALUES ("Metal");
INSERT INTO `genres` (`genre`) VALUES ("Pop");
