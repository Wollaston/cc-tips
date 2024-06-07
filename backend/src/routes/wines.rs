use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::DB;

pub fn routes() -> Router {
    Router::new().route("/wines/name-bottle-price", get(wines_bottle_price))
}

pub async fn wines_bottle_price() -> impl IntoResponse {
    let wines = DB
        .query(
            "
            SELECT base_price, display_price, name, product_id from wines ORDER BY name ASC;
            ",
        )
        .await;

    match wines {
        Ok(mut wines) => {
            let wines: Vec<WinesBottlePrice> = wines.take(0).expect("Could not get tips data");
            Json(wines).into_response()
        }
        Err(err) => (
            StatusCode::OK,
            Json(json!({
            "error": err
            })),
        )
            .into_response(),
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct WinesBottlePrice {
    base_price: i32,
    display_price: Option<String>,
    name: String,
    product_id: i32,
}
