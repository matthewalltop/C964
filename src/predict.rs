use polars::prelude::{col, lit, when};
use crate::algo::{apply_logistic_regression, MLAlgorithmResponse};
use crate::frames::{get_all_patient_info_raw};
use crate::traits::PatientInfoTranslation;

pub fn comorbidity_of_mental_health_condition() -> Result<MLAlgorithmResponse, Box<dyn std::error::Error>> {
    let df = get_all_patient_info_raw(true)
        .with_mental_health_translation()
        .with_column(
            when(
                col("Mental Health Condition").neq(lit("N/A"))
            ).then(
                lit(1)
            ).otherwise(lit(0))
                .alias("Presence")
        )
        .select([
            col("ADHD"),
            col("ADD"),
            col("Presence"),
        ])
        .collect()?;

    let response = apply_logistic_regression(df, vec!["ADHD", "ADD"], 0.50)?;

    Ok(response)
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
    fn trains_model_for_comorbidity_of_mental_health_condition() {
        let result = comorbidity_of_mental_health_condition().unwrap();
        println!("Comorbidity of Mental Health Conditions");
        println!("Confusion Matrix: {:?}", result.cf_matrix);
        println!("Accuracy {}", result.accuracy);
        println!("Precision {}", result.precision);
        println!("Recall {}\n", result.recall);

        assert_eq!(true, true)
    }

    #[test]
    fn trains_model_for_comorbidity_of_bipolar_disorder() {
        let result = comorbidity_of_bipolar_disorder().unwrap();
        println!("Comorbidity of Bipolar Disorder");
        println!("Confusion Matrix: {:?}", result.cf_matrix);
        println!("Accuracy {}", result.accuracy);
        println!("Precision {}", result.precision);
        println!("Recall {}\n", result.recall);

        assert_eq!(true, true)
    }

    #[test]
    fn trains_model_for_comorbidity_of_unipolar_depression() {
        let result = comorbidity_of_unipolar_depression().unwrap();
        println!("Comorbidity of Unipolar Depression");
        println!("Confusion Matrix: {:?}", result.cf_matrix);
        println!("Accuracy {}", result.accuracy);
        println!("Precision {}", result.precision);
        println!("Recall {}\n", result.recall);
        assert_eq!(true, true)
    }

    #[test]
    fn trains_model_for_comorbidity_of_anxiety() {
        let result = comborbidity_of_anxiety_disorder().unwrap();
        println!("Comorbidity of Anxiety");
        println!("Confusion Matrix: {:?}", result.cf_matrix);
        println!("Accuracy {}", result.accuracy);
        println!("Precision {}", result.precision);
        println!("Recall {}\n", result.recall);
        assert_eq!(true, true)
    }

    #[test]
    fn trains_model_for_comorbidity_of_substance_abuse_disorder() {
        let result = comorbidity_of_substance_abuse_disorder().unwrap();
        println!("Comorbidity of Substance Abuse");
        println!("Confusion Matrix: {:?}", result.cf_matrix);
        println!("Accuracy {}", result.accuracy);
        println!("Precision {}", result.precision);
        println!("Recall {}\n", result.recall);
        assert_eq!(true, true)
    }
}
