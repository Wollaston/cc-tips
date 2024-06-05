use axum::{extract::Path, http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use surrealdb::sql::Thing;

use crate::DB;

pub fn routes() -> Router {
    Router::new()
        .route("/tips", get(tips))
        .route("/tips/:eid", get(staff_member_tips))
}

pub async fn tips() -> impl IntoResponse {
    let tips = DB
        .query(
            "
            SELECT * from tips ORDER BY name ASC;
            ",
        )
        .await;

    match tips {
        Ok(mut tips) => {
            let tips: Vec<TippedDay> = tips.take(0).expect("Could not get tips data");
            Json(tips).into_response()
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

pub async fn staff_member_tips(Path(eid): Path<i32>) -> impl IntoResponse {
    let tips = DB
        .query(
            "
            SELECT * from tips WHERE eid=$eid ORDER BY name ASC;
            ",
        )
        .bind(("eid", eid))
        .await;

    match tips {
        Ok(mut tips) => {
            let tips: Vec<TippedDay> = tips.take(0).expect("Could not get tips data");
            Json(tips).into_response()
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
struct TippedDay {
    name: String,
    employee: Thing,
    role: String,
    net_tips: f32,
    total_pay_for_night: f32,
    hourly_pay_for_night: f32,
    duration: f32,
    eid: i32,
    date: NaiveDate,
    created: DateTime<Utc>,
    modified: DateTime<Utc>,
}
