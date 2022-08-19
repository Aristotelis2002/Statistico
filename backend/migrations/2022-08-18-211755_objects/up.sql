CREATE TABLE objects (
    object_id INTEGER PRIMARY KEY NOT NULL,
    object_name TEXT NOT NULL,
    object_counter INTEGER,
    statistic_id INTEGER NOT NULL,
    FOREIGN KEY (statistic_id)
        REFERENCES statistics (statistic_id)
        ON DELETE CASCADE
)