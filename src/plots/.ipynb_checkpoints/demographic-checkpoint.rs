use error::Error;
use std::error;
use plotlars::{Axis, BarPlot, Plot, Text};
use polars::prelude::{col, DataType};
use crate::frames::enums::Age;
use crate::frames::subtypes::adhd_subtypes_with_gender_and_age;

/// Produces a plot visualizing the distribution of ADHD Types by Gender
pub fn plot_by_adhd_type_with_gender_and_age() -> Result<(), Box<dyn Error>> {
    let df = adhd_subtypes_with_gender_and_age()
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
    
    // See if I'm even getting output here.
    println!("{}", df);
    
    BarPlot::builder()
        .data(&df)
        .group("ADHD Type")
        .labels("Age Range") // x-axis label
        .values("ADHD Subtypes")// y-axis label
        .plot_title(
            Text::from("ADHD by Gender and Age")
                // .font()
                // .size()
        )
        .legend_title(
            Text::from("ADHD Type")
        )
        .x_axis(&Axis::new().show_line(true).tick_labels(vec![Age::SeventeenToTwentyNine.to_string(), Age::ThirtyToThirtyNine.to_string(), Age::FortyToFortyNine.to_string(), Age::FiftyToSixtySeven.to_string()]))
        .y_axis(&Axis::new().show_line(true).value_range(vec![1.0, 25.0]).value_thousands(false))
        .build()
        .;
    
    Ok(())
}