use std::error::Error;
use polars::prelude::{col, lit, when};
use crate::algo::{apply_decision_tree, apply_gaussian_naive_bayes, apply_logistic_regression, MLAlgorithmResponse};
use crate::enums::{AdhdSubtype, Gender, MLAlgorithms, MentalHealthCondition};
use crate::frames::{get_all_patient_info_raw};
use crate::traits::PatientInfoTranslation;

pub (crate) type MLResponse = Result<MLAlgorithmResponse, Box<dyn Error>>;

/// Predicts the comorbidity of a mental health condition in the population.
pub fn comorbidity_of_mental_health_condition(algo: MLAlgorithms, split_ratio: f32) -> MLResponse {
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
    

    let response = match algo {
        MLAlgorithms::LogisticRegression => apply_logistic_regression(df, vec!["ADHD", "ADD"], split_ratio)?,
        MLAlgorithms::GaussianNB => apply_gaussian_naive_bayes(df, vec!["ADHD", "ADD"], split_ratio)?,
        MLAlgorithms::DecisionTree => apply_decision_tree(df, vec!["ADHD", "ADD"], split_ratio)?,
        _ => Err("Invalid algorithm provided")?
    };

    Ok(response)
}

/// Predicts the comorbidity of a specific mental health condition in the population.
pub fn comorbidity_of_given_mental_health_condition(condition: MentalHealthCondition, gender: Gender, adhd_type: AdhdSubtype, algo: MLAlgorithms, split_ratio: f32) -> MLResponse {
    let condition_col = match condition { 
        MentalHealthCondition::BipolarDisorder => col("BIPOLAR"),
        MentalHealthCondition::UnipolarDepression => col("UNIPOLAR"),
        MentalHealthCondition::AnxietyDisorder => col("ANXIETY"),
        MentalHealthCondition::SubstanceAbuseDisorder => col("SUBSTANCE"),
        MentalHealthCondition::Other => col("OTHER"),
        MentalHealthCondition::None => lit(true),
        _ => lit(true)
    };
    
    let gender_filter = match gender {
        Gender::None => lit(true),
        Gender::Female => col("SEX").eq(0),
        Gender::Male => col("SEX").eq(1)
    };
    
    let adhd_type_filter = match adhd_type  {
        AdhdSubtype::None => col("ADHD").eq(0).and(col("ADD").eq(0)),
        AdhdSubtype::All => col("ADHD").eq(1).or(col("ADD").eq(1)),
        AdhdSubtype::PrimaryInattentive => col("ADHD").eq(1).and(col("ADD").eq(1)),
        AdhdSubtype::PrimaryHyperactive => col("ADHD").eq(1).and(col("ADD").eq(0))
    };
    
    
    let df = get_all_patient_info_raw(true)
        .filter(condition_col.to_owned().eq(1).and(gender_filter).and(adhd_type_filter))
        .select([
            col("SEX"),
            col("AGE"),
            col("ADHD"),
            col("ADD"),
            condition_col
        ])
        .collect()?;

    let response = match algo {
        MLAlgorithms::LogisticRegression => apply_logistic_regression(df, vec!["SEX", "AGE","ADHD", "ADD"], split_ratio)?,
        MLAlgorithms::GaussianNB => apply_gaussian_naive_bayes(df, vec!["SEX", "AGE","ADHD", "ADD"], split_ratio)?,
        MLAlgorithms::DecisionTree => apply_decision_tree(df, vec!["SEX", "AGE","ADHD", "ADD"], split_ratio)?,
        _ => Err("Invalid algorithm provided")?
    };

    Ok(response)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn trains_model_for_comorbidity_of_mental_health_condition() {
        let result = comorbidity_of_mental_health_condition(MLAlgorithms::LogisticRegression, 0.70).unwrap();
        println!("Comorbidity of Mental Health Conditions");
        println!("Confusion Matrix: {:?}", result.cf_matrix);
        println!("Accuracy {}", result.accuracy);
        println!("Precision {}", result.precision);
        println!("Recall {}\n", result.recall);
        
        assert_eq!(true, true)
    }

    #[test]
    fn trains_model_for_comorbidity_of_bipolar_disorder() {
        let result = comorbidity_of_given_mental_health_condition(MentalHealthCondition::BipolarDisorder, Gender::None, AdhdSubtype::All, MLAlgorithms::LogisticRegression, 0.70).unwrap();
        println!("Comorbidity of Bipolar Disorder");
        println!("Confusion Matrix: {:?}", result.cf_matrix);
        println!("Accuracy {}", result.accuracy);
        println!("Precision {}", result.precision);
        println!("Recall {}\n", result.recall);

        assert_eq!(true, true)
    }

    #[test]
    fn trains_model_for_comorbidity_of_unipolar_depression() {
        let result = comorbidity_of_given_mental_health_condition(MentalHealthCondition::UnipolarDepression, Gender::None, AdhdSubtype::All, MLAlgorithms::LogisticRegression, 0.70).unwrap();
        println!("Comorbidity of Unipolar Depression");
        println!("Confusion Matrix: {:?}", result.cf_matrix);
        println!("Accuracy {}", result.accuracy);
        println!("Precision {}", result.precision);
        println!("Recall {}\n", result.recall);
        assert_eq!(true, true)
    }

    #[test]
    fn trains_model_for_comorbidity_of_anxiety() {
        let result = comorbidity_of_given_mental_health_condition(MentalHealthCondition::AnxietyDisorder, Gender::None, AdhdSubtype::All, MLAlgorithms::LogisticRegression, 0.70).unwrap();
        println!("Comorbidity of Anxiety");
        println!("Confusion Matrix: {:?}", result.cf_matrix);
        println!("Accuracy {}", result.accuracy);
        println!("Precision {}", result.precision);
        println!("Recall {}\n", result.recall);
        assert_eq!(true, true)
    }

    #[test]
    fn trains_model_for_comorbidity_of_substance_abuse_disorder() {
        let result = comorbidity_of_given_mental_health_condition(MentalHealthCondition::SubstanceAbuseDisorder, Gender::None, AdhdSubtype::All, MLAlgorithms::LogisticRegression, 0.70).unwrap();
        println!("Comorbidity of Substance Abuse");
        println!("Confusion Matrix: {:?}", result.cf_matrix);
        println!("Accuracy {}", result.accuracy);
        println!("Precision {}", result.precision);
        println!("Recall {}\n", result.recall);
        assert_eq!(true, true)
    }
}
