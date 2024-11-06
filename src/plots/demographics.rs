use error::Error;
use std::error;
use plotlars::{Axis, BarPlot, Plot, Text};
use polars::prelude::{col, lit, DataType, SortMultipleOptions};
use crate::enums::{Age, Gender};
use crate::frames::{get_all_patient_info_raw};
use crate::http::responses::PlotlyGraph;
use crate::traits::PatientInfoTranslation;

/// Produces a plot visualizing the distribution of ADHD Types by Gender
pub fn plot_adhd_type_by_age_group(with_controls: bool) -> Result<String, Box<dyn Error>> {
    let df = get_all_patient_info_raw(with_controls)
        .with_adhd_type_translation()
        .with_gender_translation()
        .with_age_range_translation()
        .filter(col("ADHD Type").neq(lit("N/A")))
        .group_by([col("Gender"), col("Age Range"), col("ADHD Type")])
        .agg([
            col("ADHD Type").count().cast(DataType::Float64).alias("ADHD Subtypes")
        ])
        .select([
            col("Gender"),
            col("Age Range"),
            col("ADHD Type"),
            col("ADHD Subtypes")
        ])
        .sort_by_exprs(vec![col("Age Range")], SortMultipleOptions::default().with_order_descending(true))
        .collect()?;
    
    // TODO: This seems like a good place for a different kind of visualization.
    let plots = BarPlot::builder()
        .data(&df)
        .group("ADHD Type")
        .labels("Age Range") // x-axis label
        .values("ADHD Subtypes")// y-axis label
        .plot_title(
            Text::from("ADHD Type by Age Group")
        )
        .legend_title(
            Text::from("ADHD Type")
        )
        .x_axis(&Axis::new().show_line(true).tick_labels(vec![Age::SeventeenToTwentyNine.to_string(), Age::ThirtyToThirtyNine.to_string(), Age::FortyToFortyNine.to_string(), Age::FiftyToSixtySeven.to_string()]))
        .y_axis(&Axis::new().show_line(true).value_range(vec![1.0, 25.0]).value_thousands(false))
        .build();
        
    let layout = plots.get_layout().to_json();
    let mut traces: Vec<String> = vec![];
    for x in plots.get_traces() {
        let trace = x.as_ref();
        traces.push(trace.to_json());
    }
    
    let response = PlotlyGraph::new(layout, traces);

    Ok(serde_json::to_string(&response).unwrap())
}

/// Produces a plot visualizing the distribution of ADHD Types by Gender
pub fn plot_adhd_type_by_gender(gender: Option<Gender>, with_controls: bool) -> Result<String, Box<dyn Error>> {
    let filter = match  gender {
        Some(g) => col("Gender").eq(lit(g.to_string())),
        None => lit(true)
    };
    
    let df = get_all_patient_info_raw(with_controls)
        .with_adhd_type_translation()
        .with_gender_translation()
        .with_age_range_translation()
        .filter(col("ADHD Type").neq(lit("N/A")).and(filter))
        .group_by([col("Gender"), col("Age Range"), col("ADHD Type")])
        .agg([
            col("ADHD Type").count().cast(DataType::Float64).alias("ADHD Subtypes")
        ])
        .select([
            col("Gender"),
            col("Age Range"),
            col("ADHD Type"),
            col("ADHD Subtypes")
        ])
        .collect()?;

    let plots = BarPlot::builder()
        .data(&df)
        .group("ADHD Type")
        .labels("Gender") // x-axis label
        .values("ADHD Subtypes")// y-axis label
        .plot_title(
            Text::from("ADHD Type by Gender")
        )
        .legend_title(
            Text::from("ADHD Type")
        )
        .x_axis(&Axis::new().show_line(true).tick_labels(vec![Gender::Male.to_string(), Gender::Female.to_string()]))
        .y_axis(&Axis::new().show_line(true).value_range(vec![1.0, 25.0]).value_thousands(false))
        .build();

    let layout = plots.get_layout().to_json();
    let mut traces: Vec<String> = vec![];
    for x in plots.get_traces() {
        let trace = x.as_ref();
        traces.push(trace.to_json());
    }

    let response = PlotlyGraph::new(layout, traces);

    Ok(serde_json::to_string(&response).unwrap())
}