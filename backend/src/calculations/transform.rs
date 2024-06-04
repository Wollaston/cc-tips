use csv::Reader;
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::error::Error;

#[derive(Debug, Deserialize, Serialize)]
struct LaborReportRecord {
    #[serde(rename = "Employee")]
    employee: String,
    #[serde(rename = "Payroll Id")]
    payroll_id: String,
    #[serde(rename = "Role")]
    role: String,
    #[serde(rename = "Type")]
    shift_type: String,
    #[serde(rename = "Time In")]
    time_in: String,
    #[serde(rename = "Time Out")]
    time_out: String,
    #[serde(rename = "Duration (hrs)")]
    duration: String,
    #[serde(rename = "Total Pay ($)")]
    total_pay: String,
    #[serde(rename = "Declared Tips ($)")]
    tips: String,
    #[serde(rename = "External Source")]
    source: String,
}

pub fn transform(bytes: &[u8]) -> Result<DataFrame, Box<dyn Error>> {
    let file = std::str::from_utf8(bytes)?;
    let file = file.replace("\"Shifts\"\n", "");
    let (file, _) = file.split_once("\"Breaks").expect("Could not split file");
    let reader = csv::Reader::from_reader(file.as_bytes());

    let df = convert_reader_to_df(reader)?;
    transform_types(df).map_err(|err| err.into())
}

macro_rules! struct_to_dataframe {
    ($input:expr, [$($field:ident),+]) => {
        {
            let len = $input.len().to_owned();

            $(let mut $field = Vec::with_capacity(len);)*

            for e in $input.into_iter() {
                $($field.push(e.$field);)*
            }
            df! {
                $(stringify!($field) => $field,)*
            }
        }
    }
}

fn convert_reader_to_df(mut reader: Reader<&[u8]>) -> Result<DataFrame, Box<dyn Error>> {
    let mut records: Vec<LaborReportRecord> = Vec::new();

    for result in reader.deserialize() {
        let record: LaborReportRecord = result?;
        if let "Steward" | "Server" | "Bartender" = record.role.as_str() {
            records.push(record)
        }
    }

    struct_to_dataframe!(
        records,
        [employee, payroll_id, role, shift_type, time_in, time_out, duration, total_pay, tips]
    )
    .map_err(|err| err.into())
}

fn transform_types(df: DataFrame) -> Result<DataFrame, PolarsError> {
    df.lazy()
        .select([
            col("employee"),
            col("payroll_id"),
            col("role"),
            cols(["total_pay", "duration"]).cast(DataType::Float32),
        ])
        .collect()
}
