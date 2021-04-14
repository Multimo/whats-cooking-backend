use crate::database::Repo;
use crate::model::{
    CreateRecipeBody, Ingredient, NewIngredient, NewRecipeIngredient, Recipe, RecipeIngredient,
};
use crate::schema;

use diesel::prelude::*;

pub trait IRepository {
    // Recipes
    fn get_recipes(&self) -> Result<Vec<Recipe>, String>;
    fn get_recipe(&self, id: i32) -> Result<Recipe, String>;
    fn create_recipe(&self, recipe: &CreateRecipeBody) -> Result<(), String>;
    fn delete_recipe(&self, id: i32) -> Result<(), String>;
    fn update_recipe(&self, id: i32, recipe: &CreateRecipeBody) -> Result<(), String>;

    // Ingredients
    fn get_all_ingredients(&self) -> Result<Vec<Ingredient>, String>;
    fn create_ingredient(&self, new_ingredient: &NewIngredient) -> Result<(), String>;
    fn delete_ingredient(&self, id: i32) -> Result<(), String>;

    // Recipe Ingredient
    fn create_recipe_ingredients(
        &self,
        recipe_ingredient: &NewRecipeIngredient,
    ) -> Result<(), String>;
    fn get_all_recipe_ingredients(&self, recipe_id: i32) -> Result<Vec<RecipeIngredient>, String>;
}

#[derive(Clone)]
pub struct Repository(pub Repo);

impl IRepository for Repository {
    fn get_recipes(&self) -> Result<Vec<Recipe>, String> {
        use schema::recipes::dsl::*;

        let connection = self.0.get_connection();

        let recipe = recipes
            .order(id)
            .limit(10)
            .load::<Recipe>(&connection)
            .expect("failed to get nachoes");

        Ok(recipe)
    }

    fn get_recipe(&self, req_id: i32) -> Result<Recipe, String> {
        use schema::recipes::dsl::*;

        let connection = self.0.get_connection();

        let mut recipe = recipes
            .filter(id.eq(req_id))
            .load::<Recipe>(&connection)
            .expect("failed to get by id");

        let res = recipe.pop().expect("No item found");

        Ok(res)
    }

    fn delete_recipe(&self, req_id: i32) -> Result<(), String> {
        use schema::recipes::dsl::*;

        let connection = self.0.get_connection();

        let recipe_to_delete = recipes.filter(id.eq(req_id));

        diesel::delete(recipe_to_delete)
            .execute(&connection)
            .expect("Error deleting recipe");
        Ok(())
    }

    fn update_recipe(&self, req_id: i32, body: &CreateRecipeBody) -> Result<(), String> {
        use schema::recipes::dsl::*;

        let connection = self.0.get_connection();

        let recipe_to_update = recipes.filter(id.eq(req_id));

        diesel::update(recipe_to_update)
            .set(body)
            .execute(&connection)
            .expect("Error updating recipe");

        Ok(())
    }

    fn create_recipe(&self, recipe: &CreateRecipeBody) -> Result<(), String> {
        use schema::recipes;

        let connection = self.0.get_connection();

        diesel::insert_into(recipes::table)
            .values(recipe)
            .get_result::<Recipe>(&connection)
            .expect("Error saving new recipe");

        Ok(())
    }

    fn get_all_ingredients(&self) -> Result<Vec<Ingredient>, String> {
        use schema::ingredients::dsl::*;

        let connection = self.0.get_connection();

        let ingredients_res = ingredients
            .load::<Ingredient>(&connection)
            .expect("Error getting all ingredients");
        return Ok(ingredients_res);
    }

    fn create_ingredient(&self, new_ingredient: &NewIngredient) -> Result<(), String> {
        use schema::ingredients::dsl::*;

        let connection = self.0.get_connection();

        log::info!("new_ingredient {:?}", new_ingredient);

        diesel::insert_into(ingredients)
            .values(new_ingredient)
            .execute(&connection)
            .expect("Error create new ingredient");
        return Ok(());
    }

    fn delete_ingredient(&self, ingredient_id: i32) -> Result<(), String> {
        use schema::ingredients::dsl::*;

        let connection = self.0.get_connection();

        log::info!("deleting ingredient {:?}", ingredient_id);
        let ingredient_to_delete = ingredients.filter(id.eq(ingredient_id));

        diesel::delete(ingredient_to_delete)
            .execute(&connection)
            .expect("Error deleting ingredient");
        return Ok(());
    }

    fn create_recipe_ingredients(
        &self,
        recipe_ingredient: &NewRecipeIngredient,
    ) -> Result<(), String> {
        use schema::recipe_ingredients::dsl::*;

        let connection = self.0.get_connection();

        diesel::insert_into(recipe_ingredients)
            .values(recipe_ingredient)
            .execute(&connection)
            .expect("Error saving new recipe");

        Ok(())
    }

    fn get_all_recipe_ingredients(&self, recipe: i32) -> Result<Vec<RecipeIngredient>, String> {
        use schema::recipe_ingredients::dsl::*;
        let connection = self.0.get_connection();

        let result = recipe_ingredients
            .filter(recipe_id.eq(recipe))
            .get_results::<RecipeIngredient>(&connection)
            .expect("Error saving new recipe");

        Ok(result)
    }
}
