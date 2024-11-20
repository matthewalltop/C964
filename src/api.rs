use std::str::FromStr;
use axum::extract::Query;
use crate::enums::{AdhdSubtype, Gender, MLAlgorithms, MentalHealthCondition};
use crate::frames::{adhd_subtype_info, patients_with_comorbid_mental_health_conditions};
use crate::requests::{DemographicCategory, DemographicParams, DisplayType, MentalHealthParams, PredictParams};
use crate::plots::{ heat_map_adhd_type_by_age_group, plot_adhd_type_by_gender, plot_comorbid_mental_health_conditions};
use crate::predict::{comorbidity_of_given_mental_health_condition, comorbidity_of_mental_health_condition};

pub async fn demographic_handler(params: Query<DemographicParams>) -> String {
    let query = params.0;
    let display = DisplayType::from_str(&query.display.unwrap_or("Plot".into())).unwrap();
    let category = DemographicCategory::from_str(&query.sub_category.unwrap_or("".into())).unwrap();
    let with_controls = query.with_controls.unwrap_or(false);
    
    let result = match display { 
        DisplayType::Plot => match category {
            DemographicCategory::ADHDSubtypesByGender => plot_adhd_type_by_gender(with_controls),
            DemographicCategory::ADHDSubtypesByAgeGroup =>  heat_map_adhd_type_by_age_group(with_controls),
            _ => plot_adhd_type_by_gender(with_controls)
        },
        DisplayType::Table => adhd_subtype_info(with_controls)
    }; 
 
    result.unwrap()
}

pub async fn mental_health_handler(params: Query<MentalHealthParams>) -> String {
    let query = params.0;
    let display = DisplayType::from_str(&query.display.unwrap_or("Plot".into())).unwrap();
    let with_controls = query.with_controls.unwrap_or(false);

    let result = match display {
        DisplayType::Plot => plot_comorbid_mental_health_conditions(with_controls),
        DisplayType::Table => patients_with_comorbid_mental_health_conditions(with_controls)
    };

    result.unwrap()
}


pub async fn predict_handler(params: Query<PredictParams>) -> String {
    let query = params.0;
    let condition = MentalHealthCondition::from_str(&query.condition.unwrap_or("".into())).unwrap();
    let gender = Gender::from_str(&query.gender.unwrap_or("".into())).unwrap();
    let adhd_type = AdhdSubtype::from_str(&query.adhd_type.unwrap_or("".into())).unwrap();
    let algorithm = MLAlgorithms::from_str(&query.algorithm.unwrap_or("LogisticRegression".into())).unwrap();
    let split_ratio = &query.split_ratio.unwrap_or(0.70 );
    
    let ml_result = match condition  {
        MentalHealthCondition::All => comorbidity_of_mental_health_condition(algorithm, *split_ratio),
        _ => comorbidity_of_given_mental_health_condition(condition, gender, adhd_type, algorithm, *split_ratio),
    }.unwrap();
    
   
    
    serde_json::to_string(&ml_result).unwrap()
}