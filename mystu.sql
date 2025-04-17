CREATE TABLE users (
    email varchar(24) PRIMARY KEY,          -- 汕大邮箱
    username varchar(18) UNIQUE NOT NULL,   -- 用户名
    password varchar(255) NOT NULL,         -- 哈希密码
    created_at date DEFAULT CURRENT_DATE,   -- 创建日期
    active boolean DEFAULT TRUE,            -- 1表示正常，0表示被封禁
    role smallint DEFAULT 0                 -- 0表示普通用户，1表示管理员
);

CREATE TABLE posts (
    id SERIAL PRIMARY KEY,
    title varchar(64) UNIQUE NOT NULL,
    author varchar(18) NOT NULL,
    time timestamp DEFAULT date_trunc('minute', LOCALTIMESTAMP),
    content text NOT NULL
);

-- 找到某帖子的所有一级评论
-- SELECT xxx FROM comments WHERE post_id = $1 AND parent_id = NULL

-- 找到某条一级评论之下的所有二级评论
-- SELECT xxx FROM comments WHERE post_id = $1 AND parent_id = $2
CREATE TABLE comments (
    post_id integer NOT NULL,           -- 不唯一，与posts关联
    comment_id SERIAL PRIMARY KEY,      -- 评论id
    parent_id integer,                  -- 父评论id, 只有二级评论拥有，所以也可以用来区分一级和二级
    reply_to varchar(18),               -- @对象，只有二级评论可拥有
    username varchar(18) NOT NULL,      -- 评论者
    content text NOT NULL,              -- 评论内容 
    time timestamp DEFAULT date_trunc('minute', LOCALTIMESTAMP)
    -- FOREIGN KEY (parent_id) REFERENCES comments(comment_id)         -- 自引用外键
);