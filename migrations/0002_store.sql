CREATE TABLE Product (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    price REAL NOT NULL,
    description TEXT NOT NULL DEFAULT "",
    stock INTEGER,
    flags TEXT NOT NULL
);

CREATE TABLE StoreTransaction ( -- Transaction is a SQLite reserved keyword
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    user INTEGER NOT NULL,
    amount REAL NOT NULL,
    datetime INTEGER NOT NULL,
    FOREIGN KEY("user") REFERENCES User("id") ON DELETE SET NULL
);

CREATE TABLE TransactionItem (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    transaction_id INTEGER NOT NULL,
    product INTEGER,
    quantity INTEGER NOT NULL,
    name TEXT NOT NULL,
    price REAL NOT NULL,
    FOREIGN KEY("transaction_id") REFERENCES StoreTransaction("id") ON DELETE CASCADE,
    FOREIGN KEY("product") REFERENCES Product("id") ON DELETE SET NULL
);
