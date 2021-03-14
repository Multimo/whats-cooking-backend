// use crate::schema::ingredients;
use crate::schema::recipes;
use serde::{Deserialize, Serialize};

#[derive(Queryable, Serialize, Identifiable)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub url: String,
    pub description: String,
    // pub link: String,
    // pub ingredients: Vec<(i32, i32)>,
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
    // pub link: String,
    pub url: String,
    // pub ingredients: Vec<i32>,
}
