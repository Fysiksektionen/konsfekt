CREATE TABLE SwishPaymentRequest (
    id TEXT UNIQUE NOT NULL,
    user INTEGER NOT NULL,
    status TEXT NOT NULL CHECK(status IN ('pending', 'paid', 'declined', 'error', 'cancelled')),
    token TEXT NOT NULL,
    callback_identifier TEXT NOT NULL,
    location TEXT,
    FOREIGN KEY("user") REFERENCES User("id") ON DELETE CASCADE
);
