use plotlars::{Axis, AxisType, BarPlot, HeatMap, Histogram, Plot, Rgb, Text};
use polars::prelude::{col, lit, when, DataType, SortMultipleOptions};
use crate::enums::{AdhdSubtype, Age, MentalHealthCondition};
use crate::frames::{get_all_patient_info_raw};
use crate::JsonResponse;
use crate::traits::{PatientInfoFilter, PatientInfoTranslation};

/// Produces a plot visualizing the distribution of ADHD Types by Gender
pub fn heat_map_adhd_type_by_age_group(with_controls: bool) -> JsonResponse {
    let df =
        get_all_patient_info_raw(with_controls)
            .with_adhd_type_translation()
            .with_gender_translation()
            .with_age_range_translation()
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
        .x_axis(&Axis::new()
            .show_line(true)
            .tick_labels(vec![Age::SeventeenToTwentyNine.to_string(), Age::ThirtyToThirtyNine.to_string(), Age::FortyToFortyNine.to_string(), Age::FiftyToSixtySeven.to_string()]))
        .y_axis(&Axis::new()
            .show_line(true)
            .tick_labels(vec![AdhdSubtype::None.to_string(), AdhdSubtype::PrimaryInattentive.to_string(), AdhdSubtype::PrimaryHyperactive.to_string()]))
        .build()
        .to_json()?;

    Ok(plots)
}

/// Produces a plot visualizing the distribution of ADHD Types by Gender
pub fn bar_plot_adhd_type_by_age_range(with_controls: bool) -> JsonResponse {
    let df = {
        let control_filter = if  with_controls { lit(true) } else { col("ADHD Type").neq(lit("N/A"))  };

        get_all_patient_info_raw(with_controls)
            .with_adhd_type_translation()
            .with_gender_translation()
            .with_age_range_translation()
            .filter(control_filter)
            .group_by([col("Age Range"), col("ADHD Type")])
            .agg([
                col("ADHD Type").count().cast(DataType::Float64).alias("ADHD Subtypes")
            ])
            .select([
                col("Age Range"),
                col("ADHD Type"),
                col("ADHD Subtypes")
            ])
            .collect()
    }?;

    println!("{}", &df);

    let plots = BarPlot::builder()
        .data(&df)
        .group("Gender")
        .labels("ADHD Type") // x-axis label
        .values("ADHD Subtypes")// y-axis label
        .plot_title(
            Text::from("ADHD Type by Gender")
        )
        .legend_title(
            Text::from("ADHD Type")
        )
        .x_axis(&Axis::new()
            .show_line(true)
            .show_axis(true)
            .show_grid(true)
            .tick_labels(vec![Age::SeventeenToTwentyNine.to_string(), Age::ThirtyToThirtyNine.to_string(), Age::FortyToFortyNine.to_string(), Age::FiftyToSixtySeven.to_string()]))
        .y_axis(&Axis::new()
            .show_line(true)
            .show_axis(true)
            .show_grid(true)
            .value_range(vec![1.0, 25.0])
            .value_thousands(false))
        .build()
        .to_json()?;

    Ok(plots)
}


/// Produces a plot visualizing the distribution of ADHD Types by Gender
pub fn plot_adhd_type_by_gender(with_controls: bool) -> JsonResponse {
    let with_control_filter = if with_controls { lit(true) } else { col("ADHD Type").neq(lit("N/A"))  } ;

    let df =
        get_all_patient_info_raw(with_controls)
            .with_adhd_type_translation()
            .with_gender_translation()
            .with_age_range_translation()
            .filter(with_control_filter)
            .group_by([col("Gender"), col("ADHD Type")])
            .agg([
                col("ADHD Type").count().cast(DataType::Float64).alias("ADHD Subtypes")
            ])
            .select([
                col("Gender"),
                col("ADHD Type"),
                col("ADHD Subtypes")
            ])
            .collect()?;

    let plots = BarPlot::builder()
        .data(&df)
        .group("Gender")
        .labels("ADHD Type") // x-axis label
        .values("ADHD Subtypes")// y-axis label
        .plot_title(
            Text::from("ADHD Type by Gender")
        )
        .legend_title(
            Text::from("ADHD Type")
        )
        .x_axis(&Axis::new()
            .show_line(true)
            .show_grid(true)
            .show_axis(true)
            .axis_type(AxisType::Category)
            .tick_labels(vec![Age::SeventeenToTwentyNine.to_string(), Age::ThirtyToThirtyNine.to_string(), Age::FortyToFortyNine.to_string(), Age::FiftyToSixtySeven.to_string()]))
        .y_axis(&Axis::new()
            .show_line(true)
            .show_grid(true)
            .show_line(true)
            .value_range(vec![1.0, 25.0])
            .value_thousands(false))
        .build()
        .to_json()?;

    Ok(plots)
}

pub fn plot_comorbid_mental_health_conditions(with_controls: bool) -> JsonResponse {
    let df = get_all_patient_info_raw(with_controls)
        .with_adhd(Some(AdhdSubtype::All))
        .with_adhd_type_translation()
        .with_gender_translation()
        .with_column(
            when(
                col("BIPOLAR").eq(1)).then(lit(1))
                .when(col("UNIPOLAR").eq(1)).then(lit(2))
                .when(col("ANXIETY").eq(1)).then(lit(3))
                .when(col("SUBSTANCE").eq(1)).then(lit(4))
                .when(col("OTHER").eq(1)).then(lit(5))
                .otherwise(lit(0))
                .alias("Mental Illness Type")
        )
        .select([
            col("ADHD Type"),
            col("Mental Illness Type").cast(DataType::Float32),
        ])
        .collect()?;

    let plot = Histogram::builder()
        .data(&df)
        .x("Mental Illness Type")
        .group("ADHD Type")
        .legend_title(
            Text::from("ADHD Type")
        )
        .y_axis(&Axis::new()
            .show_axis(true)
            .show_grid(true)
            .show_line(true)
            .value_thousands(true)
            .tick_values(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0])
        )
        .x_axis(&Axis::new()
            .show_grid(true)
            .show_axis(true)
            .show_line(true)
            .value_thousands(true)
            .tick_values(vec![0.0, 1.0, 2.0, 3.0, 4.0, 5.0])
            .tick_labels(vec![MentalHealthCondition::None.to_string(), MentalHealthCondition::BipolarDisorder.to_string(), MentalHealthCondition::UnipolarDepression.to_string(), MentalHealthCondition::AnxietyDisorder.to_string(), MentalHealthCondition::SubstanceAbuseDisorder.to_string(), MentalHealthCondition::Other.to_string()]))
        .y_title("Total Patients")
        .x_title("Mental Health Condition")
        .colors(
            vec![
                Rgb(9, 74, 77),
                Rgb(112, 228, 76),
                Rgb(55, 57, 58),
            ]
        )
        .plot_title("Distribution of Mental Health Conditions")
        .build()
        .to_json();

    Ok(plot.unwrap())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn plots_adhd_type_by_age_group() {
        let result = bar_plot_adhd_type_by_age_range(false);
        assert!(result.is_ok());
    }

    #[test]
    fn plots_comorbid_mental_health_conditions() {
        let result = plot_comorbid_mental_health_conditions(true);
        assert!(result.is_ok())
    }
    
    #[test]
    fn plots_adhd_type_by_gender() {
        let result = plot_adhd_type_by_gender( false);
        assert!(result.is_ok());
    }
}