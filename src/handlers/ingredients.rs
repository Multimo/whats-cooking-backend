use crate::handlers::StatefulRequest;
use crate::model::NewIngredient;
use crate::repo::IRepository;
use tide::prelude::*;

pub async fn handle_get_all_ingredients(req: StatefulRequest) -> tide::Result {
    let ingredients = &req.state().repository.get_all_ingredients().unwrap();
    let json = json!({
        "response": ingredients,
    });
    let response = tide::Response::builder(200).body(json).build();
    return Ok(response);
}

pub async fn handle_create_ingredients(mut req: StatefulRequest) -> tide::Result {
    let new_ingredient = req.body_json::<NewIngredient>().await?;
    &req.state().repository.create_ingredient(&new_ingredient);

    let response_json = json!({ "response": "success" });
    let res = tide::Response::builder(200).body(response_json).build();
    return Ok(res);
}

pub async fn handle_delete_ingredient(mut req: StatefulRequest) -> tide::Result {
    let param_id = req.param("id")?;
    let ingredient_id = param_id.parse::<i32>()?;
    &req.state().repository.delete_ingredient(ingredient_id);

    let response_json = json!({ "response": "success" });
    let res = tide::Response::builder(200).body(response_json).build();
    return Ok(res);
}
