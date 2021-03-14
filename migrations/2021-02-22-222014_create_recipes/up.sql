CREATE TABLE recipes (
    id SERIAL PRIMARY KEY,
    name VARCHAR NOT NULL,
    url VARCHAR NOT NULL,
    description TEXT,
    link VARCHAR,
    createdAt TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    ingredients: INTEGER [],
)