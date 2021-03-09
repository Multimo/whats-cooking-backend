table! {
    ingredients (id) {
        id -> Int4,
        name -> Varchar,
        url -> Varchar,
        createdat -> Timestamp,
    }
}

table! {
    recipes (id) {
        id -> Int4,
        name -> Varchar,
        url -> Varchar,
        description -> Text,
    }
}

allow_tables_to_appear_in_same_query!(
    ingredients,
    recipes,
);
