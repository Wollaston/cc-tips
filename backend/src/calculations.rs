use chrono::NaiveDate;
use serde::{Deserialize, Serialize};
use std::error::Error;

mod compute;
mod generate;
mod transform;

pub async fn read_csv(
    labor_report_data: LaborReportUpload,
    bytes: &[u8],
) -> Result<(String, String, Summary, Vec<TippedDayCalculation>), Box<dyn Error>> {
    let date = labor_report_data.date.clone().to_string();

    let df = transform::transform(bytes)?;
    let df = compute::compute(labor_report_data, df)?;
    generate::generate(df, date).await
}

#[derive(Debug, Default, Deserialize, Serialize)]
pub struct Summary {
    pub total_tips: f32,
    pub average_net_hourly_pay: f32,
}

#[derive(Serialize, Deserialize, Debug, Clone, Default)]
pub struct LaborReportUpload {
    pub date: NaiveDate,
    pub total_sales: f32,
    pub go_tab_tips: f32,
    pub cash_tips: f32,
    pub data_csv_link: String,
    pub template_csv_link: String,
}

#[derive(Debug, Serialize, Clone)]
pub struct TippedDayCalculation {
    pub employee: String,
    pub role: String,
    pub net_tips: f32,
    pub total_pay_for_night: f32,
    pub hourly_pay_for_night: f32,
    pub tipped_hour_for_night: f32,
    pub duration: f32,
    pub eid: i32,
    pub date: String,
}
