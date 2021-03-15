table! {
    ingredients (id) {
        id -> Int4,
        name -> Varchar,
        scientific_name -> Varchar,
        group -> Varchar,
        created_at -> Timestamp,
        url -> Nullable<Varchar>,
    }
}

table! {
    recipe_ingredients (id) {
        id -> Int4,
        recipe_id -> Int4,
        ingredient_id -> Int4,
        quantity -> Int4,
        metric -> Nullable<Varchar>,
        created_at -> Timestamp,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    recipes (id) {
        id -> Int4,
        name -> Varchar,
        url -> Varchar,
        description -> Nullable<Text>,
        link -> Nullable<Varchar>,
        created_at -> Timestamp,
    }
}

table! {
    shopping_lists (id) {
        id -> Int4,
        createdat -> Timestamp,
        updatedat -> Nullable<Timestamp>,
        active -> Bool,
    }
}

table! {
    shopping_list_ingredients (id) {
        id -> Int4,
        shopping_list_id -> Int4,
        ingredient_id -> Int4,
        amount -> Int4,
        metric -> Nullable<Varchar>,
        complete -> Bool,
        updated_at -> Nullable<Timestamp>,
    }
}

table! {
    shopping_list_recipes (id) {
        id -> Int4,
        shopping_list_id -> Int4,
        recipe_id -> Int4,
        planned_day -> Nullable<Varchar>,
    }
}

joinable!(recipe_ingredients -> ingredients (ingredient_id));
joinable!(recipe_ingredients -> recipes (recipe_id));
joinable!(shopping_list_ingredients -> ingredients (ingredient_id));
joinable!(shopping_list_ingredients -> shopping_lists (shopping_list_id));
joinable!(shopping_list_recipes -> recipes (recipe_id));
joinable!(shopping_list_recipes -> shopping_lists (shopping_list_id));

allow_tables_to_appear_in_same_query!(
    ingredients,
    recipe_ingredients,
    recipes,
    shopping_lists,
    shopping_list_ingredients,
    shopping_list_recipes,
);
