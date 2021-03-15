CREATE TABLE shopping_lists (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    createdAt TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updatedAt TIMESTAMP,
    active BOOLEAN NOT NULL
);