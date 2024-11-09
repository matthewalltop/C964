use std::error::Error;
use plotlars::{Plot, HeatMap };
use polars::prelude::{col, Expr};
use crate::frames::{get_all_patient_info_raw};
use crate::traits::{PatientInfoFilter, PatientInfoTranslation};

pub fn plot_comorbid_mental_health_conditions() -> Result<String, Box<dyn Error>> {
    let df = get_all_patient_info_raw(false)
        .with_presence_of_mental_health_condition()
        .with_age_range_translation()
        .with_adhd_type_translation()
        .with_gender_translation()
        .group_by(["ADHD Type", "Gender"])
        .agg([
            calculate_occurence("BIPOLAR"),
            calculate_occurence("UNIPOLAR"),
            calculate_occurence("ANXIETY"),
            calculate_occurence("SUBSTANCE"),
            calculate_occurence("OTHER")
        ])
        .collect()?;
    
    // DEBUG
    // println!("{}", df);
            
    HeatMap::builder()
        .data(&df)
        .x("Gender")
        .y("ADHD Type")
        .z("BIPOLAR_SUM")
        .build()
        .plot();
    
    Ok("".into())
}

fn calculate_occurence(condition: &str) -> Expr {
    col(condition).eq(1)
        .sum()
        .alias(format!("{}_SUM", condition))
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn plots_comorbid_mental_health_conditions() { 
        let result = plot_comorbid_mental_health_conditions();
        
        assert!(result.is_ok())
    }
}