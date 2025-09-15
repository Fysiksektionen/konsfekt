CREATE TABLE User (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    personal_number TEXT UNIQUE, -- Hashed
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

CREATE TABLE BankidOrder (
    id TEXT NOT NULL UNIQUE, -- Order reference
    user_id INTEGER,
    nonce TEXT NOT NULL,
    created_at INTEGER NOT NULL,
    completed_at INTEGER,
    status TEXT DEFAULT "pending",
    FOREIGN KEY(user_id) REFERENCES User(id) ON DELETE CASCADE
);
