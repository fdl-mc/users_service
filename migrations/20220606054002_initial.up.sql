create table if not exists users (
    id INT GENERATED ALWAYS AS IDENTITY,
    nickname TEXT,
    admin BOOL
);

CREATE TABLE IF NOT EXISTS credentials (
    id INT GENERATED ALWAYS AS IDENTITY,
    user_id INT UNIQUE,
    password TEXT,
    salt TEXT
);