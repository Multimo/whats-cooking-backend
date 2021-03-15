CREATE TABLE shopping_list_recipes (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    shopping_list_id INTEGER REFERENCES shopping_list(id) NOT NULL,
    recipe_id INTEGER REFERENCES recipes(id) NOT NULL,
    planned_day VARCHAR(100)
);