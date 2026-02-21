CREATE TABLE SwishPayment (
    id TEXT UNIQUE NOT NULL,
    user INTEGER NOT NULL,
    status TEXT NOT NULL CHECK(status IN ('pending', 'paid')),
    token TEXT NOT NULL,
    location TEXT,
    FOREIGN KEY("user") REFERENCES User("id") ON DELETE CASCADE
);
