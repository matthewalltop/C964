use plotlars::{BarPlot, Plot};
use polars::prelude::{col, Expr};
use crate::enums::MentalHealthCondition;
use crate::frames::{get_all_patient_info_raw};
use crate::JsonResponse;
use crate::traits::{PatientInfoFilter, PatientInfoTranslation};

pub fn plot_comorbid_mental_health_conditions(with_controls: bool) -> JsonResponse {
    let df = get_all_patient_info_raw(with_controls)
        .with_age_range_translation()
        .with_adhd_type_translation()
        .with_gender_translation()
        .group_by(["ID", "ADHD Type", "Gender"])
        .agg([
            calculate_occurrence("BIPOLAR"),
            calculate_occurrence("UNIPOLAR"),
            calculate_occurrence("ANXIETY"),
            calculate_occurrence("SUBSTANCE"),
            calculate_occurrence("OTHER")
        ])
        .collect()?;
    
    // DEBUG
    println!("{}", df);
    
    // let response = BarPlot::builder()
    //     .data(&df)
    //     .group("ADHD Type")
    //     .labels("")
    //     .values("Total Patients")
    //     .build();
    
    
    
            
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