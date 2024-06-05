use chrono::{DateTime, NaiveDate, Utc};
use futures::future::join_all;
use polars::prelude::*;
use serde::{Deserialize, Serialize};
use std::collections::HashSet;
use std::error::Error;
use std::hash::Hash;
use surrealdb::sql::Thing;

use crate::DB;

use super::{Summary, TippedDayCalculation};

pub async fn generate(
    df: DataFrame,
    date: String,
) -> Result<(String, String, Summary, Vec<TippedDayCalculation>), Box<dyn Error>> {
    let df = verify_cardholders(df)?;
    let df = verify_unique_timekeepers(df)?;
    let df = join_card_ids(df, date.clone())?;

    let tips: Vec<TippedDayCalculation> = post_to_db(df.clone()).await?;

    let mut df = df.sort(["role"], Default::default())?;

    let data_filename = format!("{}_tip_pool_calculations.csv", &date);

    let mut template_df = generate_upload_template(df.clone(), date.clone())?;

    let template_file_name = format!("{}_rapidpay_upload_template.csv", date);

    let mut file = std::fs::File::create(format!("public/downloads/{}", data_filename))
        .expect("Could not create .csv file in public/downlads (1)");
    CsvWriter::new(&mut file)
        .finish(&mut df)
        .expect("Could not write to .csv file");
    let mut file = std::fs::File::create(format!("public/downloads/{}", template_file_name))
        .expect("Could not create .csv file in public/downloads (2)");
    CsvWriter::new(&mut file)
        .finish(&mut template_df)
        .expect("Could not write to .csv file");

    let summary: Summary = summarize(df)?;

    Ok((data_filename, template_file_name, summary, tips))
}

fn generate_upload_template(df: DataFrame, date: String) -> Result<DataFrame, PolarsError> {
    let df = df
        .lazy()
        .select(&[
            col("card_id").alias("Cardholder Account"),
            col("net_tips").round(2).alias("Amount"),
        ])
        .with_columns(vec![
            lit("4845607938").alias("Funding Card ID"),
            lit("4047").alias("Funding Card Passcode"),
            lit("").alias("Reserved1"),
            lit("").alias("Reserved2"),
            lit("").alias("Reserved3"),
            lit("").alias("Reserved4"),
            lit(date).alias("Reference"),
        ])
        .collect()?;

    df.lazy()
        .select(&[
            col("Funding Card ID"),
            col("Funding Card Passcode"),
            col("Reserved1"),
            col("Cardholder Account"),
            col("Amount"),
            col("Reserved2"),
            col("Reserved3"),
            col("Reserved4"),
            col("Reference"),
        ])
        .collect()
}

fn verify_cardholders(df: DataFrame) -> Result<DataFrame, Box<dyn Error>> {
    let mut cardholders: Vec<String> = Vec::new();

    let mut reader = csv::Reader::from_path("public/cards.csv")?;
    for result in reader.deserialize() {
        let cardholder: String = result?;
        cardholders.push(cardholder);
    }

    let workers = df
        .column("employee")?
        .str()?
        .into_iter()
        .map(|str| str.unwrap_or("NoWorker").to_string())
        .collect::<Vec<String>>();

    let eval = workers.iter().all(|item| cardholders.contains(item));

    match eval {
        true => Ok(df),
        false => Err("Staff member not a cardholder".into()),
    }
}

fn verify_unique_timekeepers(df: DataFrame) -> Result<DataFrame, Box<dyn Error>> {
    let workers = df
        .column("employee")?
        .str()?
        .into_iter()
        .map(|str| str.unwrap_or("NoWorker").to_string())
        .collect::<Vec<String>>();

    let eval = has_unique_elements(workers);

    match eval {
        true => Ok(df),
        false => Err("Timekeepers not unique".into()),
    }
}

fn has_unique_elements<T>(iter: T) -> bool
where
    T: IntoIterator,
    T::Item: Eq + Hash,
{
    let mut uniq = HashSet::new();
    iter.into_iter().all(move |x| uniq.insert(x))
}

fn join_card_ids(df: DataFrame, date: String) -> Result<DataFrame, PolarsError> {
    let df_card_ids = CsvReadOptions::default()
        .with_has_header(true)
        .try_into_reader_with_file_path(Some("public/cards.csv".into()))?
        .finish()?;

    df.lazy()
        .join(
            df_card_ids.lazy(),
            [col("employee")],
            [col("name")],
            JoinArgs::new(JoinType::Inner),
        )
        .with_columns(vec![lit(date).alias("date")])
        .collect()
}

fn summarize(df: DataFrame) -> Result<Summary, Box<dyn Error>> {
    let total_tips: f32 = df.column("net_tips")?.sum()?;

    let total_hourly_pay: f32 = df.column("hourly_pay_for_night")?.sum()?;

    let num_employees = df.column("employee")?.len();

    let average_net_hourly_pay: f32 = total_hourly_pay / num_employees as f32;

    Ok(Summary {
        total_tips,
        average_net_hourly_pay,
    })
}

async fn post_to_db(df: DataFrame) -> Result<Vec<TippedDayCalculation>, PolarsError> {
    let tips: Vec<TippedDayCalculation> = df
        .into_struct("StructChunked")
        .iter()
        .map(|row| TippedDayCalculation {
            employee: row[0].get_str().unwrap().to_string(),
            role: row[2].get_str().unwrap().to_string(),
            net_tips: row[11].try_extract().unwrap(),
            total_pay_for_night: row[12].try_extract().unwrap(),
            hourly_pay_for_night: row[13].try_extract().unwrap(),
            duration: row[4].try_extract().unwrap(),
            eid: row[15].try_extract().unwrap(),
            date: row[16].get_str().unwrap().to_string(),
        })
        .collect();

    join_all(
        tips.clone()
            .into_iter()
            .map(|tip| async move {
                DB.update::<Option<TippedDayForCreate>>((
                    "tips",
                    format!("{}_{}", tip.eid.clone(), tip.date.clone().as_str()),
                ))
                .content(TippedDayForCreate {
                    name: tip.employee,
                    employee: Thing {
                        tb: "staff".to_string(),
                        id: tip.eid.into(),
                    },
                    role: tip.role,
                    net_tips: tip.net_tips,
                    total_pay_for_night: tip.total_pay_for_night,
                    hourly_pay_for_night: tip.hourly_pay_for_night,
                    duration: tip.duration,
                    eid: tip.eid,
                    date: NaiveDate::parse_from_str(tip.date.as_str(), "%Y-%m-%d")
                        .expect("Could not create NaiveDate"),
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

    Ok(tips)
}

#[derive(Debug, Serialize, Deserialize)]
struct TippedDayForCreate {
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
