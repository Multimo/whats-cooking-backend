use tide::prelude::*;

use crate::model;
use crate::repo::{IRepository, Repository};

pub async fn handle_get_all_recipes(req: StatefulRequest) -> tide::Result {
    let recipes = &req.state().repository.get_recipes().unwrap();
    let json = json!({
        "response": recipes,
    });
    let response = tide::Response::builder(200).body(json).build();
    return Ok(response);
}

pub async fn handle_get_recipe(req: StatefulRequest) -> tide::Result {
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

pub async fn handle_delete_recipes(req: StatefulRequest) -> tide::Result {
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

pub async fn handle_update_recipes(mut req: StatefulRequest) -> tide::Result {
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

pub async fn handle_create_recipes(mut req: StatefulRequest) -> tide::Result {
    let body: model::CreateRecipeBody = req.body_json().await?;

    &req.state().repository.create_recipe(&body).unwrap();

    let response_json = json!({ "response": "body" });

    let res = tide::Response::builder(200).body(response_json).build();
    return Ok(res);
}
