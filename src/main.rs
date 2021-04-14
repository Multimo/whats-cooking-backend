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

use tide::http::headers;
use tide::security::{CorsMiddleware, Origin};

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

    let cors = CorsMiddleware::new()
        .allow_methods(
            "GET, POST, OPTIONS"
                .parse::<headers::HeaderValue>()
                .unwrap(),
        )
        .allow_origin(Origin::from("*"))
        .allow_credentials(false);
    api.with(cors);

    api.at("/recipes")
        .get(handlers::recipes::handle_get_all_recipes)
        .post(handlers::recipes::handle_create_recipes);
    api.at("/recipe/:id")
        .get(handlers::recipes::handle_get_recipe)
        .patch(handlers::recipes::handle_update_recipes)
        .delete(handlers::recipes::handle_delete_recipes);
    api.at("/ingredients")
        .get(handlers::ingredients::handle_get_all_ingredients)
        .post(handlers::ingredients::handle_create_ingredients)
        .delete(handlers::ingredients::handle_delete_ingredient);
    api.listen("127.0.0.1:8082").await?;
    Ok(())
}
