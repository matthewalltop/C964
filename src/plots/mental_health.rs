use plotlars::{Axis, Histogram, Plot, Rgb, Text};
use polars::prelude::{col, lit, when, DataType};
use crate::enums::{AdhdSubtype, MentalHealthCondition};
use crate::frames::{get_all_patient_info_raw};
use crate::JsonResponse;
use crate::traits::{PatientInfoFilter, PatientInfoTranslation};

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
    fn plots_comorbid_mental_health_conditions() {
        let result = plot_comorbid_mental_health_conditions(true);
        assert!(result.is_ok())
    }
}