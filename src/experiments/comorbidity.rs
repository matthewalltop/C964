use polars::prelude::{col};
use crate::algo::apply_binomial_logistic_regression;
use crate::frames::{get_all_patient_info_raw};


pub fn comorbidity_of_mental_health_condition() {
    unimplemented!()
}

/// Experiment to determine comorbidity of bipolar disorder among patient population.
pub fn comorbidity_of_bipolar_disorder() -> Result<(), Box<dyn std::error::Error>> {
    // Start by grabbing all the patients
    let df = get_all_patient_info_raw(false)
        .select([
            col("ADHD"),
            col("ADD"),
            col("BIPOLAR")
        ])
        .collect()?;

    let _accuracy = apply_binomial_logistic_regression(df, vec!["ADHD", "ADD"], 0.20);    
    
    Ok(())
}

pub fn comorbidity_of_unipolar_depression() {
    unimplemented!()
}

pub fn comborbidity_of_anxiety_disorder() {
    unimplemented!()
}

pub fn comorbidity_of_substance_abuse_disorder() {
    unimplemented!()
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn trains_model_for_comorbidity_of_bipolar_disorder() {
        let result = comorbidity_of_bipolar_disorder();
        match result {
            Ok(_) => assert!(true),
            _ => assert!(false)
        }
    }
    
}
