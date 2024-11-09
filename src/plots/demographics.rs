﻿use error::Error;
use std::error;
use plotlars::{Axis, BarPlot, HeatMap, Plot, ScatterPlot, Text};
use polars::prelude::{col, lit, DataType, SortMultipleOptions};
use crate::enums::{AdhdSubtype, Age, Gender};
use crate::frames::{get_all_patient_info_raw};
use crate::http::responses::{PlotlyPlot};
use crate::plots;
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
        .sort_by_exprs(vec![col("Age Range")], SortMultipleOptions::default().with_order_descending(false))
        .collect()?;
    
    let plots = HeatMap::builder()
        .data(&df)
        .x("Age Range")
        .y("ADHD Type")
        .z("ADHD Subtypes")
        .plot_title(
            Text::from("Distribution of ADHD Types by Age Group")
        )
        .x_title("Age Range")
        .y_title("ADHD Subtype")
        .x_axis(&Axis::new().show_line(true).tick_labels(vec![Age::SeventeenToTwentyNine.to_string(), Age::ThirtyToThirtyNine.to_string(), Age::FortyToFortyNine.to_string(), Age::FiftyToSixtySeven.to_string()]))
        .y_axis(&Axis::new().show_line(true).tick_labels(vec![AdhdSubtype::PrimaryInattentive.to_string(), AdhdSubtype::PrimaryHyperactive.to_string()]))
        .build()
        .to_json()?;
    
    Ok(serde_json::to_string(&plots)?)
}

/// Produces a plot visualizing the distribution of ADHD Types by Gender
pub fn plot_adhd_type_by_gender(gender: Option<Gender>, with_controls: bool) -> Result<String, Box<dyn Error>> {
    let df = {
        let filter = match  gender {
            Some(g) => col("Gender").eq(lit(g.to_string())),
            None => lit(true)
        };
        
        get_all_patient_info_raw(with_controls)
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
            .collect()
    }?;

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
        .build()
        .to_json()?;
 
    Ok(serde_json::to_string(&plots)?)
}


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn plots_adhd_type_by_age_group() {
        let result = plot_adhd_type_by_age_group(false);
        assert!(result.is_ok());
    }
}