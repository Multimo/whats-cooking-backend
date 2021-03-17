#[macro_use]
extern crate diesel;
pub mod model;
pub mod schema;

use dotenv::dotenv;
use std::env;

mod database;
use crate::handlers::{IRepository, Repository};

use crate::repo::*;
mod repo;

#[derive(Clone)]
pub struct State<R: 'static + IRepository + Sync + Send> {
    pub repository: R,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    log::info!("Initializing");
    dotenv().ok();
    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let connection = Repository(database::Repo::new(&database_url));
    let context = State {
        repository: connection,
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
