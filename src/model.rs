use crate::schema::{
    ingredients, recipe_ingredients, recipes, shopping_list_recipes, shopping_lists,
};
use chrono::NaiveDateTime;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Identifiable)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub description: Option<String>,
    pub link: Option<String>,
    pub created_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name = "recipes"]
pub struct CreateRecipeBody {
    pub name: String,
    pub description: String,
    pub link: String,
    pub url: String,
}

#[derive(Queryable, Serialize, Identifiable)]
pub struct Ingredient {
    pub id: i32,
    pub created_at: NaiveDateTime,
    pub name: String,
    pub scientific_name: String,
    pub group: String,
    pub url: String,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name = "ingredients"]
pub struct NewIngredient {
    pub name: String,
    pub scientific_name: String,
    pub group: String,
    pub url: String,
}

#[derive(Queryable, Serialize, Identifiable)]
pub struct RecipeIngredient {
    pub id: i32,
    pub recipe_id: i32,
    pub ingredient_id: i32,
    pub quantity: i32,
    pub metric: String,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name = "recipe_ingredients"]
pub struct NewRecipeIngredient {
    pub recipe_id: i32,
    pub ingredient_id: i32,
    pub quantity: i32,
    pub metric: String,
}

#[derive(Queryable, Serialize, Identifiable)]
pub struct ShoppingList {
    pub id: i32,
    pub active: bool,
    pub created_at: NaiveDateTime,
    pub updated_at: NaiveDateTime,
}

#[derive(Queryable, Serialize, Identifiable)]
pub struct ShoppingListRecipe {
    pub id: i32,
    pub shopping_list_id: i32,
    pub recipe_id: i32,
    pub planned_day: String,
}
