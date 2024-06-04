use axum::{http::StatusCode, response::IntoResponse, routing::get, Json, Router};
use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::DB;

pub fn routes() -> Router {
    Router::new().route("/staff", get(staff).post(new_staff_member))
}

pub async fn staff() -> impl IntoResponse {
    let staff = DB
        .query(
            "
            SELECT * from staff;
            ",
        )
        .await;

    match staff {
        Ok(mut staff) => {
            let staff: Vec<StaffMember> = staff.take(0).expect("Could not get staff data");
            (StatusCode::OK, Json(staff)).into_response()
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

pub async fn new_staff_member(Json(data): Json<StaffMemberForCreate>) -> impl IntoResponse {
    let staff: Vec<StaffMember> = DB
        .create("staff")
        .content(StaffMember {
            name: data.name,
            card_id: data.card_id,
            created: Utc::now(),
        })
        .await
        .expect("Could not create new Staff Member");

    Json(staff).into_response()
}

#[derive(Deserialize, Serialize, Debug)]
struct StaffMember {
    name: String,
    card_id: String,
    created: DateTime<Utc>,
}

#[derive(Deserialize, Debug)]
pub struct StaffMemberForCreate {
    name: String,
    card_id: String,
}
