use polars::prelude::*;

use super::LaborReportUpload;

pub fn compute(
    labor_report_upload: LaborReportUpload,
    df: DataFrame,
) -> Result<DataFrame, PolarsError> {
    let base_hours = compute_base_hours(df.clone())?;
    let total_tips = labor_report_upload.cash_tips + labor_report_upload.go_tab_tips;
    let df = proportion_of_total_tipped_hours(df, base_hours.clone())?;
    let df = proportion_of_total_tips(total_tips, df)?;
    let df = proportion_of_total_sales(labor_report_upload.total_sales, df)?;
    let df = steward_tip_out(df)?;
    let df = proportion_of_total_steward_hours(df, base_hours)?;
    let df = proportion_of_total_steward_tips(df)?;
    let df = net_tips(df)?;
    let df = total_pay_for_night(df)?;
    hourly_pay_for_night(df)
}

fn compute_base_hours(df: DataFrame) -> Result<DataFrame, PolarsError> {
    df.lazy()
        .group_by([col("role")])
        .agg([col("duration").sum().alias("role_hours")])
        .collect()
}

fn proportion_of_total_tipped_hours(
    df: DataFrame,
    base_hours: DataFrame,
) -> Result<DataFrame, PolarsError> {
    let tipped_hours: f32 = base_hours
        .lazy()
        .filter(col("role").neq(lit("Steward")))
        .collect()?
        .column("role_hours")?
        .sum()?;

    df.lazy()
        .with_column(
            when(col("role").neq(lit("Steward")))
                .then(col("duration") / lit(tipped_hours))
                .otherwise(0)
                .alias("proportion_of_total_tipped_hours"),
        )
        .collect()
}

fn proportion_of_total_steward_hours(
    df: DataFrame,
    base_hours: DataFrame,
) -> Result<DataFrame, PolarsError> {
    let steward_hours: f32 = base_hours
        .lazy()
        .filter(col("role").eq(lit("Steward")))
        .collect()?
        .column("role_hours")?
        .sum()?;

    df.lazy()
        .with_column(
            when(col("role").eq(lit("Steward")))
                .then(col("duration") / lit(steward_hours))
                .otherwise(0)
                .alias("proportion_of_total_steward_hours"),
        )
        .collect()
}

fn proportion_of_total_tips(total_tips: f32, df: DataFrame) -> Result<DataFrame, PolarsError> {
    df.lazy()
        .with_column(
            (col("proportion_of_total_tipped_hours") * lit(total_tips))
                .alias("proportion_of_total_tips"),
        )
        .collect()
}

fn proportion_of_total_sales(total_sales: f32, df: DataFrame) -> Result<DataFrame, PolarsError> {
    df.lazy()
        .with_column(
            (col("proportion_of_total_tipped_hours") * lit(total_sales))
                .alias("proportion_of_total_sales"),
        )
        .collect()
}

fn steward_tip_out(df: DataFrame) -> Result<DataFrame, PolarsError> {
    let steward_tip_out = 0.025;
    df.lazy()
        .with_column(
            (col("proportion_of_total_sales") * lit(steward_tip_out)).alias("steward_tip_out"),
        )
        .collect()
}

fn net_tips(df: DataFrame) -> Result<DataFrame, PolarsError> {
    df.lazy()
        .with_column(
            when(col("role").neq(lit("Steward")))
                .then(col("proportion_of_total_tips") - col("steward_tip_out"))
                .otherwise(col("proportion_of_total_steward_tips"))
                .alias("net_tips"),
        )
        .collect()
}

fn total_steward_tip_out(df: DataFrame) -> Result<f32, PolarsError> {
    df.column("steward_tip_out")?.sum::<f32>()
}

fn proportion_of_total_steward_tips(df: DataFrame) -> Result<DataFrame, PolarsError> {
    let total_steward_tip_out = total_steward_tip_out(df.clone())?;

    df.lazy()
        .with_column(
            (col("proportion_of_total_steward_hours") * lit(total_steward_tip_out))
                .alias("proportion_of_total_steward_tips"),
        )
        .collect()
}

fn total_pay_for_night(df: DataFrame) -> Result<DataFrame, PolarsError> {
    df.lazy()
        .with_column((col("total_pay") + col("net_tips")).alias("total_pay_for_night"))
        .collect()
}

fn hourly_pay_for_night(df: DataFrame) -> Result<DataFrame, PolarsError> {
    df.lazy()
        .with_column((col("total_pay_for_night") / col("duration")).alias("hourly_pay_for_night"))
        .sort(["role"], Default::default())
        .collect()
}
