CREATE TABLE ingredients (
    id INTEGER PRIMARY KEY GENERATED ALWAYS AS IDENTITY,
    name VARCHAR(200) NOT NULL,
    name_scientific VARCHAR(200),
    decription TEXT,
    food_group TEXT,
    food_subgroup VARCHAR(250)
);
COPY ingredients (
    name,
    name_scientific,
    decription,
    food_group,
    food_subgroup
)
from '/var/lib/postgres/fixtures/ingredients.csv' DELIMITER ',' CSV HEADER;