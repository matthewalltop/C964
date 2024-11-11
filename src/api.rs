use std::str::FromStr;
use axum::extract::Query;
use crate::enums::{Gender};
use crate::frames::demographics::adhd_subtype_info;
use crate::frames::mental_health::{patients_with_comorbid_mental_health_conditions};
use crate::requests::{DemographicCategory, DemographicParams, DisplayType, MentalHealthCategory, MentalHealthParams, PredictParams};
use crate::plots::demographics::{bar_plot_adhd_type_by_age_range, heat_map_adhd_type_by_age_group, plot_adhd_type_by_gender};
use crate::plots::mental_health::plot_comorbid_mental_health_conditions;

pub async fn demographic_handler(params: Query<DemographicParams>) -> String {
    let query = params.0;
    let display = DisplayType::from_str(&query.display.unwrap_or("Plot".into())).unwrap();
    let category = DemographicCategory::from_str(&query.sub_category.unwrap_or("".into())).unwrap();
    let gender = Gender::from_str(&query.gender.unwrap_or("".into())).unwrap();
    let with_controls = query.with_controls.unwrap_or(false);
    
    let result = match display { 
        DisplayType::Plot => match category {
            DemographicCategory::ADHDSubtypesByGender => plot_adhd_type_by_gender(Some(gender), with_controls),
            DemographicCategory::ADHDSubtypesByAgeGroup =>  heat_map_adhd_type_by_age_group(with_controls),
            _ => bar_plot_adhd_type_by_age_range(with_controls)
        },
        DisplayType::Table => adhd_subtype_info(with_controls)
    }; 
 
    result.unwrap()
}

pub async fn mental_health_handler(params: Query<MentalHealthParams>) -> String {
    let query = params.0;
    let display = DisplayType::from_str(&query.display.unwrap_or("Plot".into())).unwrap();
    let category = MentalHealthCategory::from_str(&query.category.unwrap_or("".into())).unwrap();
    let with_controls = query.with_controls.unwrap_or(false);

    let result = match display {
        DisplayType::Plot => plot_comorbid_mental_health_conditions(with_controls),
        DisplayType::Table => match category {
            MentalHealthCategory::HasCoMorbidMentalHealthCondition => patients_with_comorbid_mental_health_conditions(with_controls),
            _ => patients_with_comorbid_mental_health_conditions(with_controls)
        } 
    };

    result.unwrap()
}


pub async fn predict_handler(params: Query<PredictParams>) -> String {
    //let query = params.0;
    //let condition = MentalHealthCategory::from_str(&query.condition.unwrap_or("".into())).unwrap();
    // let gender = Gender::from_str(&query.gender.unwrap_or("".into())).unwrap();
    // let age_range: Vec<Age> = query.age_ranges.unwrap_or(vec!["All".into()]).iter().map(|x| Age::from_str(x).unwrap()).collect();
    // let adhd_type = AdhdSubtype::from_str(&query.adhd_type.unwrap_or("".into())).unwrap();
    // let with_controls = query.with_controls.unwrap_or(false);
    
    // let result: MLAlgorithmResponse = match condition {
    //     MentalHealthCategory::All => comorbidity_of_mental_health_condition(),
    //     _ => comorbidity_of_mental_health_condition()
    // }.unwrap();
    
   "".into()
}