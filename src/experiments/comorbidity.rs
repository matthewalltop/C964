use polars::prelude::{col};
use crate::algo::{apply_logistic_regression, MLAlgorithmResponse};
use crate::frames::{get_all_patient_info_raw};


pub fn comorbidity_of_mental_health_condition() {
    // TODO: This one will need to be trained differently as it needs to evaluate ALL mental health.
    unimplemented!()
}

/// Experiment to determine comorbidity of bipolar disorder among patient population.
pub fn comorbidity_of_bipolar_disorder() -> Result<MLAlgorithmResponse, Box<dyn std::error::Error>> {
    // Start by grabbing all the patients
    let df = get_all_patient_info_raw(true)
        .select([
            col("ADHD"),
            col("ADD"),
            col("BIPOLAR")
        ])
        .collect()?;

    let response = apply_logistic_regression(df, vec!["ADHD", "ADD"], 0.70)?;    
    
    Ok(response)
}

pub fn comorbidity_of_unipolar_depression() -> Result<MLAlgorithmResponse, Box<dyn std::error::Error>> {
    // Start by grabbing all the patients
    let df = get_all_patient_info_raw(true)
        .select([
            col("ADHD"),
            col("ADD"),
            col("UNIPOLAR")
        ])
        .collect()?;

    let response = apply_logistic_regression(df, vec!["ADHD", "ADD"], 0.70)?;

    Ok(response)
}

pub fn comborbidity_of_anxiety_disorder() -> Result<MLAlgorithmResponse, Box<dyn std::error::Error>> {
    // Start by grabbing all the patients
    let df = get_all_patient_info_raw(true)
        .select([
            col("ADHD"),
            col("ADD"),
            col("ANXIETY")
        ])
        .collect()?;

    let response = apply_logistic_regression(df, vec!["ADHD", "ADD"], 0.70)?;

    Ok(response)
}

pub fn comorbidity_of_substance_abuse_disorder() -> Result<MLAlgorithmResponse, Box<dyn std::error::Error>> {
    // Start by grabbing all the patients
    let df = get_all_patient_info_raw(true)
        .select([
            col("ADHD"),
            col("ADD"),
            col("SUBSTANCE")
        ])
        .collect()?;

    let response = apply_logistic_regression(df, vec!["ADHD", "ADD"], 0.70)?;

    Ok(response)
}

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn trains_model_for_comorbidity_of_bipolar_disorder() {
        let result = comorbidity_of_bipolar_disorder().unwrap();
        println!("Comorbidity of Bipolar Disorder");
        println!("Confusion Matrix: {:?}", result.raw_cf_matrix);
        println!("Accuracy {}", result.accuracy);
        println!("Precision {}", result.precision);
        println!("Recall {}\n", result.recall);
 
        assert_eq!(true, true)
    }

    #[test]
    fn trains_model_for_comorbidity_of_unipolar_depression() {
        let result = comorbidity_of_unipolar_depression().unwrap();
        println!("Comorbidity of Unipolar Depression");
        println!("Confusion Matrix: {:?}", result.raw_cf_matrix);
        println!("Accuracy {}", result.accuracy);
        println!("Precision {}", result.precision);
        println!("Recall {}\n", result.recall);
        assert_eq!(true, true)
    }

    #[test]
    fn trains_model_for_comorbidity_of_anxiety() {
        let result = comborbidity_of_anxiety_disorder().unwrap();
        println!("Comorbidity of Anxiety");
        println!("Confusion Matrix: {:?}", result.raw_cf_matrix);
        println!("Accuracy {}", result.accuracy);
        println!("Precision {}", result.precision);
        println!("Recall {}\n", result.recall);
        assert_eq!(true, true)
    }

    #[test]
    fn trains_model_for_comorbidity_of_substance_abuse_disorder() {
        let result = comorbidity_of_substance_abuse_disorder().unwrap();
        println!("Comorbidity of Substance Abuse");
        println!("Confusion Matrix: {:?}", result.raw_cf_matrix);
        println!("Accuracy {}", result.accuracy);
        println!("Precision {}", result.precision);
        println!("Recall {}\n", result.recall);
        assert_eq!(true, true)
    }
}
