CREATE TABLE statistics (
    statistic_id INTEGER PRIMARY KEY NOT NULL,
    statistic_name TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (user_id)
      REFERENCES users (user_id)
        ON DELETE CASCADE,
)