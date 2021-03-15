CREATE TABLE recipe_ingredients (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    recipe_id INTEGER REFERENCES recipes(id) NOT NULL,
    ingredient_id INTEGER REFERENCES ingredients(id) NOT NULL,
    quantity INTEGER NOT NULL,
    metric VARCHAR(50),
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP
)