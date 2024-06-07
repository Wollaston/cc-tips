use axum::{
    extract::{Multipart, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, NaiveDate, Utc};
use serde::{Deserialize, Serialize};
use serde_json::json;
use surrealdb::sql::Thing;

use crate::DB;

pub fn routes() -> Router {
    Router::new()
        .route("/tips", post(tips))
        .route("/tips/:eid", get(staff_member_tips))
        .route("/tips/csv", get(generate_csv))
}

pub async fn tips(mut data: Multipart) -> impl IntoResponse {
    #[derive(Default, Debug)]
    struct TipsQuery<String> {
        start_date: Option<String>,
        end_date: Option<String>,
    }
    dbg!(&data);

    let mut tips_query = TipsQuery::default();

    while let Some(field) = data.next_field().await.unwrap() {
        let name = field.name().unwrap();
        match name {
            "startDate" => {
                tips_query.start_date =
                    Some(field.text().await.expect("Could not get start_date text"))
            }
            "endDate" => {
                tips_query.end_date = Some(field.text().await.expect("Could not get end_date text"))
            }
            _ => continue,
        };
    }

    dbg!(&tips_query);

    let tips = DB
        .query(
            "
            SELECT * from tips WHERE date >= $start AND date <= $end ORDER BY name ASC;
            ",
        )
        .bind(("start", tips_query.start_date.unwrap_or("".to_string())))
        .bind(("end", tips_query.end_date.unwrap_or("".to_string())))
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

async fn generate_csv(body: String) -> impl IntoResponse {
    dbg!(&body);
    let mut rdr = csv::Reader::from_reader(body.as_bytes());

    let file_path = format!("public/downloads/tips-data-export_{}.csv", Utc::now());

    let mut wtr = csv::Writer::from_path(file_path.clone()).expect("Could not init writer");

    for result in rdr.deserialize() {
        let record: TippedDay = result.expect("Could not deserialize to Struct");
        wtr.serialize(record).expect("Could not serialize record");
    }

    wtr.flush().expect("Could not flush writer");

    (StatusCode::OK, file_path)
}

#[derive(Debug, Serialize, Deserialize)]
struct TippedDay {
    name: String,
    employee: Thing,
    role: String,
    net_tips: f32,
    total_pay_for_night: f32,
    hourly_pay_for_night: f32,
    tipped_hour_for_night: f32,
    duration: f32,
    eid: i32,
    date: NaiveDate,
    created: DateTime<Utc>,
    modified: DateTime<Utc>,
}
