use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use surrealdb::sql::Thing;

use crate::DB;

pub fn routes() -> Router {
    Router::new().route("/commissions", get(wines_bottle_price).post(new_commission))
}

pub async fn wines_bottle_price() -> impl IntoResponse {
    let commissions = DB
        .query(
            "
            SELECT name.name AS name, wine.name AS wine, amount, wine.product_id AS product_id, date FROM commissions ORDER BY name ASC;
            ",
        )
        .await;

    match commissions {
        Ok(mut commissions) => {
            let commissions: Vec<CommissionSummary> =
                commissions.take(0).expect("Could not get tips data");
            Json(commissions).into_response()
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

pub async fn new_commission(Json(data): Json<CommissionForCreate>) -> impl IntoResponse {
    let staff: Vec<Commission> = DB
        .create("commissions")
        .content(Commission {
            name: Thing {
                tb: "staff".into(),
                id: data.eid.parse::<i32>().expect("Could not parse eid").into(),
            },
            wine: Thing {
                tb: "wines".into(),
                id: data
                    .product_id
                    .parse::<i32>()
                    .expect("Could not parse product_id")
                    .into(),
            },
            amount: 1234,
            date: NaiveDate::from_ymd_opt(2024, 6, 6).unwrap(),
            created: Utc::now(),
            modified: Utc::now(),
        })
        .await
        .expect("Could not create new commission");

    Json(staff).into_response()
}

#[derive(Debug, Serialize, Deserialize)]
pub struct CommissionForCreate {
    eid: String,
    product_id: String,
}

#[derive(Debug, Serialize, Deserialize)]
struct Commission {
    name: Thing,
    wine: Thing,
    amount: i32,
    date: NaiveDate,
    created: DateTime<Utc>,
    modified: DateTime<Utc>,
}

#[derive(Debug, Serialize, Deserialize)]
struct CommissionSummary {
    name: String,
    wine: String,
    amount: i32,
    date: NaiveDate,
}
