// use crate::schema::ingredients;
use crate::schema::recipes;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Identifiable)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub description: String,
    // pub ingredients: Vec<i32>,
    pub url: String,
}

// #[derive(Queryable, Serialize, Identifiable)]
// pub struct Ingredient {
//     pub id: i32,
//     pub name: String,
//     pub description: String,
//     pub url: String,
// }

#[derive(Insertable, Deserialize, AsChangeset)]
#[table_name = "recipes"]
pub struct CreateRecipeBody {
    pub name: String,
    pub description: String,
    // pub ingredients: Vec<i32>,
    pub url: String,
}
