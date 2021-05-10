CREATE TABLE `post_comments` (
    `comment_id` INT NOT NULL AUTO_INCREMENT,
    `post_id` INT NOT NULL,
    `owner` TEXT NOT NULL,
    `message` TEXT NOT NULL,
    PRIMARY KEY (`comment_id`)
);