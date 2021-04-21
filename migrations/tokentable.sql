CREATE TABLE `refresh_token` (
    `ID` INT NOT NULL AUTO_INCREMENT,
    `username` TEXT NOT NULL , `token` TEXT NOT NULL,
    `Deadline` DATETIME NOT NULL,
    PRIMARY KEY (`ID`)
);