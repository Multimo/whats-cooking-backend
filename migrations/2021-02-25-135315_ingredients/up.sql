CREATE TABLE ingredients (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name VARCHAR(200) NOT NULL,
    name_scientific VARCHAR(200),
    food_group TEXT,
    food_subgroup VARCHAR(250),
    decription TEXT
);
COPY ingredients
from '/var/lib/postgres/fixtures/ingredients.csv' DELIMITER ',' CSV HEADER;