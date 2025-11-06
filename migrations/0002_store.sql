CREATE TABLE Product (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    price REAL NOT NULL
    description TEXT NOT NULL DEFAULT "",
    stock INTEGER
)

CREATE TABLE Transaction (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    product INTEGER NOT NULL,
    user INTEGER NOT NULL,
    amount REAL NOT NULL,
    FOREIGN KEY("product") REFERENCES Product("id"),
    FOREIGN KEY("user") REFERENCES User("id"),
)