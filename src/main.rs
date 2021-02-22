use tide::prelude::*;
use tide::Request;

#[derive(Debug, Deserialize)]
struct Animal {
    name: String,
    legs: u8,
}

#[async_std::main]
async fn main() -> tide::Result<()> {
    log::info!("Starting server");
    let mut app = tide::new();
    tide::log::start();
    app.at("/orders/shoes").post(order_shoes);
    app.at("/recipes").get(|_| async {
        Ok(json!({
            "meta": { "count": 2 },
            "animals": [
                { "type": "cat", "name": "chashu" },
                { "type": "cat", "name": "nori" }
            ]
        }))
    });
    app.listen("127.0.0.1:8081").await?;
    Ok(())
}

async fn order_shoes(mut req: Request<()>) -> tide::Result {
    let Animal { name, legs } = req.body_json().await?;
    Ok(format!("Hello, {}! I've put in an order for {} shoes", name, legs).into())
}

// async fn get_recipes() {
//     let response = json!({
//         "meta": { "count": 2 },
//         "animals": [
//             { "type": "cat", "name": "chashu" },
//             { "type": "cat", "name": "nori" }
//         ]
//     });
//     Ok(response);
// }
