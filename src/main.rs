use serde::{Deserialize, Serialize};
use tide::prelude::*;

#[macro_use]
extern crate diesel;
use diesel::pg::PgConnection;
use diesel::prelude::*;

use dotenv::dotenv;
use std::env;

mod database;
pub mod model;
pub mod schema;

pub fn establish_connection() -> PgConnection {
    log::info!("Starting connection to database");
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    PgConnection::establish(&database_url).expect(&format!("Error connecting to {}", database_url))
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    log::info!("Initializing");
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let _db = establish_connection();
    // let results = db.execute("select * from recipes;")

    let connection = Repository(database::Repo::new(&database_url));
    let context = State {
        repository: connection,
        name: "Jojo".to_string(),
    };

    log::info!("Starting server");

    let mut api = tide::Server::with_state(context);
    tide::log::start();

    api.at("/recipes")
        .get(handle_get_all_recipes)
        .post(handle_create_recipes);
    api.at("/recipe/:id")
        .get(handle_get_recipe)
        .patch(handle_update_recipes)
        .delete(handle_delete_recipes);

    api.listen("127.0.0.1:8082").await?;
    Ok(())
}

async fn handle_get_all_recipes(req: StatefulRequest) -> tide::Result {
    let recipes = &req.state().repository.get_recipes().unwrap();
    let json = json!({
        "response": recipes,
    });
    let response = tide::Response::builder(200).body(json).build();
    return Ok(response);
}

async fn handle_get_recipe(req: StatefulRequest) -> tide::Result {
    let param_id = req.param("id").expect("No id in request params");
    let recipe_id = param_id
        .parse::<i32>()
        .expect("failed to parse param into i32");

    let recipes = &req.state().repository.get_recipe(recipe_id).unwrap();
    let json = json!({
        "response": recipes,
    });
    let response = tide::Response::builder(200).body(json).build();
    return Ok(response);
}

async fn handle_delete_recipes(req: StatefulRequest) -> tide::Result {
    let param_id = req.param("id").expect("No id in request params");
    let recipe_id = param_id
        .parse::<i32>()
        .expect("failed to parse param into i32");

    &req.state()
        .repository
        .delete_recipe(recipe_id)
        .expect("Failed to delete recipe");
    let json = json!({
        "response": "success",
    });

    let response = tide::Response::builder(200).body(json).build();
    return Ok(response);
}

async fn handle_update_recipes(mut req: StatefulRequest) -> tide::Result {
    let param_id = req.param("id").expect("No id in request params");
    let recipe_id = param_id
        .parse::<i32>()
        .expect("failed to parse param into i32");

    let body = req.body_json::<model::CreateRecipeBody>().await;
    let body_json = body.expect("failed to parse body json");

    req.state()
        .repository
        .update_recipe(recipe_id, &body_json)
        .unwrap();

    let json = json!({
        "response": "success",
    });
    let response = tide::Response::builder(200).body(json).build();
    return Ok(response);
}

async fn handle_create_recipes(mut req: StatefulRequest) -> tide::Result {
    let body: model::CreateRecipeBody = req.body_json().await?;

    &req.state().repository.create_recipe(&body).unwrap();

    let response_json = json!({ "response": "body" });

    let res = tide::Response::builder(200).body(response_json).build();
    return Ok(res);
}

pub type StatefulRequest = tide::Request<State<Repository>>;

#[derive(Clone)]
pub struct State<R: 'static + IRepository + Sync + Send> {
    pub repository: R,
    pub name: String,
}

#[derive(Clone, Debug, PartialEq, Deserialize, Serialize)]
pub struct Recipe {
    pub id: i32,
    pub name: String,
    pub description: String,
    pub ingredients: Vec<i32>,
    pub url: String,
}

pub trait IRepository {
    fn get_recipes(&self) -> Result<Vec<model::Recipe>, String>;
    fn get_recipe(&self, id: i32) -> Result<model::Recipe, String>;
    fn create_recipe(&self, recipe: &model::CreateRecipeBody) -> Result<(), String>;
    fn delete_recipe(&self, id: i32) -> Result<(), String>;
    fn update_recipe(&self, id: i32, recipe: &model::CreateRecipeBody) -> Result<(), String>;
}

#[derive(Clone)]
pub struct Repository(pub database::Repo);

impl IRepository for Repository {
    fn get_recipes(&self) -> Result<Vec<model::Recipe>, String> {
        use schema::recipes::dsl::*;

        let pool = self.0.connection_pool.clone();
        let connection = pool.get().unwrap();

        let recipe = recipes
            .order(id)
            .limit(10)
            .load::<model::Recipe>(&connection)
            .expect("failed to get nachoes");

        Ok(recipe)
    }

    fn get_recipe(&self, req_id: i32) -> Result<model::Recipe, String> {
        use schema::recipes::dsl::*;

        let pool = self.0.connection_pool.clone();
        let connection = pool.get().unwrap();

        let mut recipe = recipes
            .filter(id.eq(req_id))
            .load::<model::Recipe>(&connection)
            .expect("failed to get by id");

        let res = recipe.pop().expect("No item found");

        Ok(res)
    }

    fn delete_recipe(&self, req_id: i32) -> Result<(), String> {
        use schema::recipes::dsl::*;

        let pool = self.0.connection_pool.clone();
        let connection = pool.get().unwrap();

        let recipe_to_delete = recipes.filter(id.eq(req_id));

        diesel::delete(recipe_to_delete)
            .execute(&connection)
            .expect("Error deleting recipe");
        Ok(())
    }

    fn update_recipe(&self, req_id: i32, body: &model::CreateRecipeBody) -> Result<(), String> {
        use schema::recipes::dsl::*;

        let pool = self.0.connection_pool.clone();
        let connection = pool.get().unwrap();

        let recipe_to_update = recipes.filter(id.eq(req_id));

        diesel::update(recipe_to_update)
            .set(body)
            .execute(&connection)
            .expect("Error updating recipe");
        Ok(())
    }

    fn create_recipe(&self, recipe: &model::CreateRecipeBody) -> Result<(), String> {
        use schema::recipes;

        let pool = self.0.connection_pool.clone();
        let connection = pool.get().unwrap();

        diesel::insert_into(recipes::table)
            .values(recipe)
            .get_result::<model::Recipe>(&connection)
            .expect("Error saving new recipe");

        Ok(())
    }
}
