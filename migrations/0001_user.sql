CREATE TABLE User (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    google_id TEXT UNIQUE,
    balance REAL NOT NULL DEFAULT 0
);

-- Session schema based on lucia-auth
CREATE TABLE Session (
    id TEXT NOT NULL UNIQUE,
    secret_hash TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    user INTEGER NOT NULL,
    FOREIGN KEY(user) REFERENCES User(id) ON DELETE CASCADE
);
