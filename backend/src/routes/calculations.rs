use axum::{extract::Multipart, response::IntoResponse, routing::post, Json, Router};
use chrono::NaiveDate;
use serde::Serialize;

use crate::calculations::{self, LaborReportUpload, Summary, TippedDayCalculation};

pub fn routes() -> Router {
    Router::new().route("/calculations", post(calculate))
}

pub async fn calculate(mut data: Multipart) -> impl IntoResponse {
    let mut labor_report_data = LaborReportUpload::default();
    let mut summary = Summary::default();
    let mut tips = Vec::new();

    while let Some(field) = data.next_field().await.unwrap() {
        let name = field.name().unwrap();
        match name {
            "date" => {
                labor_report_data.date =
                    NaiveDate::parse_from_str(&field.text().await.unwrap(), "%Y-%m-%d").unwrap()
            }
            "totalSales" => {
                labor_report_data.total_sales = field.text().await.unwrap().parse::<f32>().unwrap();
            }
            "gotabTips" => {
                labor_report_data.go_tab_tips = field.text().await.unwrap().parse::<f32>().unwrap();
            }
            "cashTips" => {
                labor_report_data.cash_tips = field.text().await.unwrap().parse::<f32>().unwrap();
            }
            "laborReport" => {
                (
                    labor_report_data.data_csv_link,
                    labor_report_data.template_csv_link,
                    summary,
                    tips,
                ) = calculations::read_csv(
                    labor_report_data.clone(),
                    &field.bytes().await.unwrap(),
                )
                .await
                .expect("Could not calculate");
            }
            _ => continue,
        };
    }

    let response = CalculationsResponse {
        calculations_link: labor_report_data.data_csv_link,
        template_link: labor_report_data.template_csv_link,
        summary,
        tips,
    };

    dbg!(&response);

    Json(response).into_response()
}

#[derive(Debug, Serialize)]
pub struct CalculationsResponse {
    calculations_link: String,
    template_link: String,
    summary: Summary,
    tips: Vec<TippedDayCalculation>,
}
