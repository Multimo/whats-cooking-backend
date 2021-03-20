use crate::handlers::StatefulRequest;
// use crate::model;
use crate::repo::IRepository;
use tide::prelude::*;

pub async fn handle_get_all_ingredients(req: StatefulRequest) -> tide::Result {
    let recipes = &req.state().repository.get_all_ingredients().unwrap();
    let json = json!({
        "response": recipes,
    });
    let response = tide::Response::builder(200).body(json).build();
    return Ok(response);
}
