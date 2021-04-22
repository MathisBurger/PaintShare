CREATE TABLE `refresh_token` (
    `id` INT NOT NULL AUTO_INCREMENT,
    `username` TEXT NOT NULL , `token` TEXT NOT NULL,
    `deadline` DATETIME NOT NULL,
    PRIMARY KEY (`ID`)
);