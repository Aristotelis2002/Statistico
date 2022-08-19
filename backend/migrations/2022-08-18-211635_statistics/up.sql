CREATE TABLE statistics (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    user_id INTEGER NOT NULL,
    FOREIGN KEY (user_id)
      REFERENCES users (id)
        ON DELETE CASCADE
)