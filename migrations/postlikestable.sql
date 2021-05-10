CREATE TABLE `post_likes` (
    `like_id` INT NOT NULL AUTO_INCREMENT,
    `post_id` INT NOT NULL,
    `owner` TEXT NOT NULL,
    `timestamp` BIGINT NOT NULL,
    PRIMARY KEY (`like_id`)
);