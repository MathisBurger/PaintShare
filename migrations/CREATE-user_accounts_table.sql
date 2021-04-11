CREATE TABLE `user_accounts` ( 
    `user_id` INT NOT NULL AUTO_INCREMENT ,
    `displayname` TEXT NOT NULL ,
    `password` TEXT NOT NULL ,
    `created_at` BIGINT NOT NULL ,
    PRIMARY KEY(`user_id`)
)