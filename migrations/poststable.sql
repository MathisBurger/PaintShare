CREATE TABLE `user_posts` (
    `id` INT NOT NULL AUTO_INCREMENT,
    `user_id` INT NOT NULL,
    `storage_destination` TEXT NOT NULL,
    `comment` TEXT NOT NULL,
    `tags` TEXT NOT NULL,
    `likes` INT NOT NULL,
    `comments` INT NOT NULL,
    `created_at` BIGINT NOT NULL,
    PRIMARY KEY (`id`)
);