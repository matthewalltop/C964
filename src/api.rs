use axum::extract::Query;
use crate::enums::{AdhdSubtype, Age, Gender};
use crate::frames::demographics::adhd_subtype_info;
use crate::frames::mental_health::{patient_info_has_anxiety, patient_info_has_bipolar_disorder, patient_info_has_other_mental_health_condition, patient_info_has_substance_abuse_disorder, patient_info_has_unipolar_depression, patients_with_comorbid_mental_health_conditions};
use crate::http::requests::{DemographicCategory, DemographicParams, DisplayType, MentalHealthCategory, MentalHealthParams, PredictParams};
use crate::plots::demographics::{plot_adhd_type_by_age_group, plot_adhd_type_by_gender};

pub async fn demographic_plot_handler(params: Query<DemographicParams>) -> String {
    let query = params.0;
    let display = query.display.unwrap_or(DisplayType::Plot);
    let category = query.category.unwrap_or(DemographicCategory::None);
    let gender = query.gender.unwrap_or(Gender::None);
    let with_controls = query.with_controls.unwrap_or(false);
    
    let result = match display { 
        DisplayType::Plot => match category {
            DemographicCategory::ADHDSubtypesByGender => plot_adhd_type_by_gender(Some(gender), with_controls),
            DemographicCategory::ADHDSubtypesByAgeGroup => plot_adhd_type_by_age_group(with_controls),
            _ => plot_adhd_type_by_age_group(with_controls)
        },
        DisplayType::Table => adhd_subtype_info(with_controls)
    }; 
 
    result.unwrap()
}

pub async fn mental_health_handler(params: Query<MentalHealthParams>) -> String {
    let query = params.0;
    let display = query.display.unwrap_or(DisplayType::Plot);
    let category = query.category.unwrap_or(MentalHealthCategory::None);
    // let gender = query.gender.unwrap_or(Gender::None);
    let with_controls = query.with_controls.unwrap_or(false);

    let result = match display {
        DisplayType::Plot => match category {
            // TODO
            _ => patients_with_comorbid_mental_health_conditions(with_controls)
        },
        DisplayType::Table => match category {
            MentalHealthCategory::HasCoMorbidMentalHealthCondition => patients_with_comorbid_mental_health_conditions(with_controls),
            MentalHealthCategory::HasBipolarDisorder => patient_info_has_bipolar_disorder(with_controls),
            MentalHealthCategory::HasUnipolarDepression => patient_info_has_unipolar_depression(with_controls),
            MentalHealthCategory::HasAnxiety => patient_info_has_anxiety(with_controls),
            MentalHealthCategory::HasSubstanceAbuseDisorder => patient_info_has_substance_abuse_disorder(with_controls),
            MentalHealthCategory::HasOther => patient_info_has_other_mental_health_condition(with_controls),
            _ => patients_with_comorbid_mental_health_conditions(with_controls)
        } 
    };

    result.unwrap()
}


pub async fn predict_handler(predict_query: Query<PredictParams>) -> String {
    unimplemented!()
}