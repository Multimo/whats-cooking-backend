#[macro_use]
extern crate diesel;
use dotenv::dotenv;
use std::env;

mod database;
mod handlers;
pub mod model;
pub mod schema;
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
        .get(handlers::recipes::handle_get_all_recipes)
        .post(handlers::recipes::handle_create_recipes);
    api.at("/recipe/:id")
        .get(handlers::recipes::handle_get_recipe)
        .patch(handlers::recipes::handle_update_recipes)
        .delete(handlers::recipes::handle_delete_recipes);

    api.listen("127.0.0.1:8082").await?;
    Ok(())
}
