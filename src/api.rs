use std::str::FromStr;
use axum::extract::Query;
use crate::enums::{Gender};
use crate::frames::demographics::adhd_subtype_info;
use crate::frames::mental_health::{patient_info_has_anxiety, patient_info_has_bipolar_disorder, patient_info_has_other_mental_health_condition, patient_info_has_substance_abuse_disorder, patient_info_has_unipolar_depression, patients_with_comorbid_mental_health_conditions};
use crate::requests::{DemographicCategory, DemographicParams, DisplayType, MentalHealthCategory, MentalHealthParams, PredictParams};
use crate::plots::demographics::{plot_adhd_type_by_age_group, plot_adhd_type_by_gender};

pub async fn demographic_handler(params: Query<DemographicParams>) -> String {
    let query = params.0;
    let display = DisplayType::from_str(&query.display.unwrap_or("Plot".into())).unwrap();
    let category = DemographicCategory::from_str(&query.sub_category.unwrap_or("".into())).unwrap();
    let gender = Gender::from_str(&query.gender.unwrap_or("".into())).unwrap();
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
    let display = DisplayType::from_str(&query.display.unwrap_or("Plot".into())).unwrap();
    let category = MentalHealthCategory::from_str(&query.category.unwrap_or("".into())).unwrap();
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



// #[cfg(test)]
// mod test {
//     use super::*;
//     use axum::{body::Body, http::Request, Router};
//     use axum::routing::{get, post};
//     use serde::de::Error;
//     use tower::ServiceExt;
// 
//     async fn app() -> Router {
//         Router::new().route("/demographics", get(demographic_handler))
//             .route("/mental-health", get(mental_health_handler))
//             .route("/predict", post(predict_handler))
//     }
//     
//     #[tokio::test]
//     async fn test_demographics() {
//         assert!(send_request_get_body("/demographics", "display=plot"))
//         
//     }
// 
// 
//     async fn send_request_get_body(url: &str, query: &str) -> Result<String, Box<dyn Error>> {
//         app()
//             .oneshot(
//                 Request::builder()
//                     .uri(format!("{url}/?{query}"))
//                     .body(Body::empty())
//                     .unwrap(),
//             )
//             .await
//             .unwrap()
//     }
//     
//     
// }