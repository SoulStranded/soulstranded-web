-- 将 SQLite 数据迁移到 MySQL 的 SQL 脚本
-- 字符集：utf8mb4，排序规则：utf8mb4_0900_ai_ci

CREATE DATABASE IF NOT EXISTS soulstranded
    CHARACTER SET utf8mb4
    COLLATE utf8mb4_0900_ai_ci;

USE soulstranded;

CREATE TABLE users (
    id INT AUTO_INCREMENT PRIMARY KEY,
    username VARCHAR(255) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    password VARCHAR(255) NOT NULL,
    avatar VARCHAR(255)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

CREATE TABLE contents (
    id INT AUTO_INCREMENT PRIMARY KEY,
    content TEXT NOT NULL,
    pub_time DATETIME NOT NULL,
    user_id INT NOT NULL,
    update_time DATETIME NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

CREATE TABLE comments (
    id INT AUTO_INCREMENT PRIMARY KEY,
    content TEXT NOT NULL,
    post_id INT NOT NULL,
    user_id INT NOT NULL,
    created_at DATETIME NOT NULL,
    updated_at DATETIME NOT NULL,
    FOREIGN KEY (post_id) REFERENCES contents(id) ON DELETE CASCADE,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

CREATE TABLE likes (
    id INT AUTO_INCREMENT PRIMARY KEY,
    user_id INT NOT NULL,
    post_id INT NOT NULL,
    created_at DATETIME NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE,
    FOREIGN KEY (post_id) REFERENCES contents(id) ON DELETE CASCADE
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_0900_ai_ci;

INSERT INTO users (id, username, email, password, avatar) VALUES
(1, 'SoulStranded', 'fahui.zhu@qq.com', '$2b$12$R37RTBoOYicmeaaHjt.iw.DcqjdgID0gMfKTnMu/wZRpbSzhzP0ia', '/uploads/avatars/fahui.zhu@qq.com.png');

INSERT INTO contents (id, content, pub_time, user_id, update_time) VALUES
(1, '我是网站的作者，欢迎大家在这里畅所欲言，后续会不断完善新功能，敬请期待！', '2026-01-18 16:57:08', 1, '2026-01-18 16:57:08'),
(2, '最新版本支持自定义上传头像，欢迎大家使用。', '2026-02-14 16:29:44', 1, '2026-02-14 16:29:44'),
(3, '最新版本已支持帖子点赞功能，欢迎大家使用。', '2026-03-29 21:02:41', 1, '2026-03-29 21:02:41');

INSERT INTO comments (id, content, post_id, user_id, created_at, updated_at) VALUES
(1, '可以评论了哦', 3, 1, '2026-04-04 15:02:07', '2026-04-04 15:02:07'),
(2, '这是一条测试评论', 3, 1, '2026-04-12 12:21:36', '2026-04-12 12:21:36');

INSERT INTO likes (id, user_id, post_id, created_at) VALUES
(3, 1, 2, '2026-03-12 20:52:55'),
(4, 1, 3, '2026-03-29 21:02:52'),
(5, 1, 1, '2026-04-12 12:21:27');

ALTER TABLE users AUTO_INCREMENT = 2;
ALTER TABLE contents AUTO_INCREMENT = 4;
ALTER TABLE comments AUTO_INCREMENT = 3;
ALTER TABLE likes AUTO_INCREMENT = 6;
