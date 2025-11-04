CREATE TABLE User (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT,
    email TEXT NOT NULL,
    google_id TEXT UNIQUE,
    role TEXT CHECK(role IN ('admin', 'maintainer', 'bot')), -- can be null
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
