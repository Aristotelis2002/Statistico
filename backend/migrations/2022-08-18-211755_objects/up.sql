CREATE TABLE objects (
    id INTEGER PRIMARY KEY NOT NULL,
    name TEXT NOT NULL,
    counter INTEGER NOT NULL,
    statistic_id INTEGER NOT NULL,
    FOREIGN KEY (statistic_id)
        REFERENCES statistics (id)
        ON DELETE CASCADE
)