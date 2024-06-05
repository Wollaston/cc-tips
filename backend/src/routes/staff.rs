use std::error::Error;

use axum::{
    extract::{Multipart, Path},
    http::StatusCode,
    response::IntoResponse,
    routing::{get, post},
    Json, Router,
};
use chrono::{DateTime, NaiveDate, Utc};
use futures::future::join_all;
use serde::{Deserialize, Serialize};
use serde_json::json;

use crate::DB;

pub fn routes() -> Router {
    Router::new()
        .route("/staff", get(staff).post(new_staff_member))
        .route(
            "/staff/:eid",
            get(staff_detail).post(staff_detail_tip_summary),
        )
        .route("/staff/:eid/summary", get(staff_summary_stats))
        .route("/import-staff", post(import_staff))
        .route("/staff/csv", post(generate_csv))
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

pub async fn staff_detail(Path(eid): Path<i32>) -> impl IntoResponse {
    let member = DB
        .query(
            "
            SELECT * FROM staff WHERE eid=$eid;
            ",
        )
        .bind(("eid", eid))
        .await;

    match member {
        Ok(mut member) => {
            let member: Option<StaffMember> = member.take(0).expect("Could not get staff data");
            Json(member).into_response()
        }
        Err(err) => {
            dbg!(&err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                "error": err
                })),
            )
                .into_response()
        }
    }
}

pub async fn staff_summary_stats(Path(eid): Path<i32>) -> impl IntoResponse {
    #[derive(Serialize)]
    struct StaffSummaryStats {
        net_tips_sum: f32,
        total_hours: f32,
        total_pay: f32,
        average_hourly: f32,
    }

    let net_tips_sum: Option<f32> = DB
        .query(
            "
            let $value = SELECT array::flatten(net_tips) FROM tips WHERE eid=$eid GROUP ALL;
            RETURN math::sum(object::values($value[0])[0]);
            ",
        )
        .bind(("eid", eid))
        .await
        .expect("Could not sum net tips")
        .take(1)
        .expect("Could not get net tips");

    let net_tips_sum = net_tips_sum.expect("Could not get sum of net tips");

    let total_hours: Option<f32> = DB
        .query(
            "
            let $value = SELECT array::flatten(duration) FROM tips WHERE eid=$eid GROUP ALL;
            RETURN math::sum(object::values($value[0])[0]);
            ",
        )
        .bind(("eid", eid))
        .await
        .expect("Could not sum net tips")
        .take(1)
        .expect("Could not get net tips");

    let total_hours = total_hours.expect("Could not get total hours");

    let total_pay: Option<f32> = DB
        .query(
            "
            let $value = SELECT array::flatten(total_pay_for_night) FROM tips WHERE eid=$eid GROUP ALL;
            RETURN math::sum(object::values($value[0])[0]);
            ",
        )
        .bind(("eid", eid))
        .await
        .expect("Could not sum net tips")
        .take(1)
        .expect("Could not get net tips");

    let total_pay = total_pay.expect("Could not get total pay");

    let average_hourly: f32 = net_tips_sum / total_hours;

    Json(StaffSummaryStats {
        net_tips_sum,
        total_pay,
        total_hours,
        average_hourly,
    })
}

pub async fn staff_detail_tip_summary(Path(eid): Path<i32>) -> impl IntoResponse {
    let member = DB
        .query(
            "
            SELECT * FROM staff WHERE eid=$eid;
            ",
        )
        .bind(("eid", eid))
        .await;

    let tips = DB
        .query(
            "
            SELECT date, net_tips FROM tips WHERE employee=type::thing('staff', $eid) ORDER BY date DESC LIMIT 10;
            ",
        )
        .bind(("eid", eid))
        .await;

    let tips: Vec<TipSummary> = tips
        .expect("Could not unwrap tips response")
        .take(0)
        .expect("Could not take tips response");

    match member {
        Ok(mut member) => {
            let member: Option<StaffMember> = member.take(0).expect("Could not get staff data");
            Json(MemberSummary {
                staff_member: member.expect("Could not unwrap staff member"),
                tips,
            })
            .into_response()
        }
        Err(err) => {
            dbg!(&err);
            (
                StatusCode::INTERNAL_SERVER_ERROR,
                Json(json!({
                "error": err
                })),
            )
                .into_response()
        }
    }
}

pub async fn new_staff_member(Json(data): Json<StaffMemberForCreate>) -> impl IntoResponse {
    let staff: Vec<StaffMember> = DB
        .create("staff")
        .content(StaffMember {
            name: data.name,
            card_id: data.card_id,
            eid: data.eid,
            created: Utc::now(),
            modified: Utc::now(),
        })
        .await
        .expect("Could not create new Staff Member");

    Json(staff).into_response()
}

pub async fn import_staff(mut data: Multipart) -> impl IntoResponse {
    let mut imported_data = Vec::new();

    while let Some(field) = data.next_field().await.unwrap() {
        let name = field.name().unwrap();
        match name {
            "importFile" => {
                (imported_data) = read_import_csv(&field.bytes().await.unwrap())
                    .await
                    .expect("Could not import staff data");
            }
            _ => continue,
        };
    }

    Json(imported_data).into_response()
}

pub async fn read_import_csv(bytes: &[u8]) -> Result<Vec<StaffMemberForCreate>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_reader(bytes);

    let mut staff: Vec<StaffMemberForCreate> = Vec::new();

    for result in rdr.deserialize() {
        let record: StaffMemberForCreate = result?;
        staff.push(record);
    }

    join_all(
        staff
            .clone()
            .into_iter()
            .map(|member| async move {
                DB.update::<Option<StaffMember>>(("staff", member.eid))
                    .content(StaffMember {
                        name: member.name,
                        card_id: member.card_id,
                        eid: member.eid,
                        created: Utc::now(),
                        modified: Utc::now(),
                    })
                    .await
                    .expect("Could not post tab to DB")
                    .expect("Could not post tab to DB")
            })
            .collect::<Vec<_>>(),
    )
    .await;

    Ok(staff)
}

async fn generate_csv(body: String) -> impl IntoResponse {
    #[derive(Serialize, Deserialize)]
    struct StaffCsvData {
        name: String,
        role: String,
        date: String,
        net_tips: f32,
        total_pay_for_night: f32,
        hourly_pay_for_night: f32,
    }

    let mut rdr = csv::Reader::from_reader(body.as_bytes());

    let file_path = format!("public/downloads/staff-data-export_{}.csv", Utc::now());

    let mut wtr = csv::Writer::from_path(file_path.clone()).expect("Could not init writer");

    for result in rdr.deserialize() {
        let record: StaffCsvData = result.expect("Could not deserialize to Struct");
        wtr.serialize(record).expect("Could not serialize record");
    }

    wtr.flush().expect("Could not flush writer");

    (StatusCode::OK, file_path)
}

#[derive(Deserialize, Serialize, Debug)]
pub struct StaffMember {
    name: String,
    card_id: String,
    eid: i32,
    created: DateTime<Utc>,
    modified: DateTime<Utc>,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct StaffMemberForCreate {
    name: String,
    card_id: String,
    eid: i32,
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct TipSummary {
    date: NaiveDate,
    net_tips: f32,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct MemberSummary {
    staff_member: StaffMember,
    tips: Vec<TipSummary>,
}
