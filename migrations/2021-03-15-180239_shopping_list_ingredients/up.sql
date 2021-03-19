CREATE TABLE shopping_list_ingredients (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    shopping_list_id INTEGER REFERENCES shopping_lists(id) NOT NULL,
    ingredient_id INTEGER REFERENCES ingredients(id) NOT NULL,
    amount INTEGER NOT NULL,
    metric VARCHAR(100),
    complete BOOLEAN NOT NULL,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);