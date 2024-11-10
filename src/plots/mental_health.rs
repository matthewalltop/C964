use std::ops::Div;
use polars::prelude::{as_struct, col, lit, when, Expr};
use crate::enums::MentalHealthCondition;
use crate::frames::{get_all_patient_info_raw};
use crate::JsonResponse;
use crate::traits::{PatientInfoFilter, PatientInfoTranslation};

pub fn plot_comorbid_mental_health_conditions(with_controls: bool) -> JsonResponse {
    let df = get_all_patient_info_raw(with_controls)
        .with_age_range_translation()
        .with_adhd_type_translation()
        .with_gender_translation()
        .with_column(
            when(as_struct(vec![col("BIPOLAR"), col("UNIPOLAR"), col("ANXIETY"), col("SUBSTANCE"), col("OTHER")]).sum().gt(1)).then(6)
                .when(col("BIPOLAR").eq(1)).then(lit(1))
                .when(col("UNIPOLAR").eq(1)).then(lit(2))
                .when(col("ANXIETY").eq(1)).then(lit(3))
                .when(col("SUBSTANCE").eq(1)).then(lit(4))
                .when(col("OTHER").eq(1)).then(lit(5))
                .otherwise(0)
                .alias("Mental Illness Type")
        )
        .select([
            col("Gender"),
            col("ADHD Type"),
            col("Mental Illness Type"),
            col("Age Range"),
        ])
        .collect()?;

    // DEBUG
    println!("{}", df);

    // let response = Histogram::builder()
    //     .data(&df)
    //     .group("ADHD Type")
    //     .x("")
    //     .values("Total Patients")
    //     .build();
    // 
    // 


    Ok("".into())
}

pub fn plot_occurrence_of_mental_illness(condition: MentalHealthCondition, with_controls: bool) -> JsonResponse {
    let df = get_all_patient_info_raw(with_controls)
        .with_presence_of_given_mental_health_condition(condition)
        .with_age_range_translation()
        .with_adhd_type_translation()
        .with_gender_translation()
        // .group_by([])
        .collect()?;

    Ok("".into())
}

fn calculate_total_patients() -> Expr {
    col("ID")
        .count()
        .alias("Total Patients")
}

fn calculate_representation_vs_total(condition: &str) -> Expr {
    calculate_occurrence(condition)
        .div(calculate_total_patients())
        .alias("Occurrence as %")
}


fn calculate_occurrence(condition: &str) -> Expr {
    col(condition).eq(1)
        .sum()
        .alias(format!("{}_SUM", condition))
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