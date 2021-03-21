CREATE TABLE ingredients (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name VARCHAR(200) NOT NULL,
    name_scientific VARCHAR(200),
    decription TEXT,
    food_group TEXT,
    food_subgroup VARCHAR(250)
);
COPY ingredients
from '/var/lib/postgres/fixtures/ingredients.csv' DELIMITER ',' CSV HEADER;