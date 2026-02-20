-- Full-text search table
CREATE VIRTUAL TABLE "TransactionFts" USING fts5 (
    transaction_id UNINDEXED,
    user_id UNINDEXED,
    product_name, 
    username,
    user_email,
    tokenize = "unicode61 remove_diacritics 0"
);

-- Automatic updates of fts table
CREATE TRIGGER "InsertTransactionItemTrigger" 
    AFTER INSERT ON "TransactionItem"
BEGIN
    INSERT INTO "TransactionFts" (transaction_id, user_id, product_name, username, user_email)
    SELECT NEW.transaction_id, u.id, NEW.name, u.name, u.email
    FROM StoreTransaction st
    JOIN User u ON u.id = st.user
    WHERE st.id = NEW.transaction_id;
END;

-- User can change username, link new username to all previous transactions made by user
CREATE TRIGGER "UpdateUserNameTrigger"
    AFTER UPDATE OF name ON "User"
    WHEN OLD.name != NEW.name
BEGIN
    INSERT INTO "TransactionFts" (transaction_id, user_id, username)
    SELECT st.id, NEW.id, NEW.name
    FROM StoreTransaction st
    WHERE st.user = NEW.id;
END; 

-- User can change email, link new email to all previous transactions made by user
CREATE TRIGGER "UpdateUserEmailTrigger"
    AFTER UPDATE OF email ON "User"
    WHEN OLD.email != NEW.email
BEGIN
    INSERT INTO "TransactionFts" (transaction_id, user_id, user_email)
    SELECT st.id, NEW.id, NEW.email
    FROM StoreTransaction st
    WHERE st.user = NEW.id;
END; 

CREATE TRIGGER "DeleteTransactionTrigger" 
    AFTER DELETE ON "StoreTransaction"
BEGIN
    DELETE FROM "TransactionFts"
    WHERE transaction_id = OLD.id;
END;

-- Direct access to fts table using aux/vocab table
CREATE VIRTUAL TABLE "TransactionFtsVocab" USING fts5vocab("TransactionFts", "row");
