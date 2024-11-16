use std::str::FromStr;
use axum::extract::Query;
use crate::enums::{AdhdSubtype, Gender, MLAlgorithms, MentalHealthCondition};
use crate::frames::demographics::adhd_subtype_info;
use crate::frames::mental_health::{patients_with_comorbid_mental_health_conditions};
use crate::requests::{DemographicCategory, DemographicParams, DisplayType, MentalHealthParams, PredictParams};
use crate::plots::demographics::{bar_plot_adhd_type_by_age_range, heat_map_adhd_type_by_age_group, plot_adhd_type_by_gender};
use crate::plots::mental_health::plot_comorbid_mental_health_conditions;
use crate::predict::{comorbidity_of_given_mental_health_condition, comorbidity_of_mental_health_condition};

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
    let split_ratio = &query.split_ratio.unwrap_or_else(|| { 0.70 });
    
    let ml_result = match condition  {
        MentalHealthCondition::All => comorbidity_of_mental_health_condition(algorithm, *split_ratio),
        _ => comorbidity_of_given_mental_health_condition(condition, gender, adhd_type, algorithm, *split_ratio),
    }.unwrap();
    
   let result = serde_json::to_string(&ml_result).unwrap();
    
    result
}

// #[cfg(test)]
// mod test {
//     use std::error::Error;
//     use axum::Router;
//     use axum::routing::get;
//     use super::*;
//     use axum_test::{TestServer};
//     
//     fn test_server() -> Result<TestServer, Box<dyn Error>> {
//         let app = Router::new()
//             .route("/mental-health", get(mental_health_handler))
//             .route("/demographics", get(demographic_handler))
//             .route("/predict", get(predict_handler));
//         
//         Ok(TestServer::new(app)?)
//     }
//     
//     #[tokio::test]
//     async fn demographic_handler_handles() {
//         let server = test_server().unwrap();
//         
//         let response = server.get("/demographics")
//             .add_raw_query_param("display=plot")
//             .add_raw_query_param("category=ADHDSubtypesByGender")
//             .add_raw_query_param("gender=''")
//             .add_raw_query_param("with_controls=false");
//         
//         
//         let result = response.await;
// 
//         result.assert_status_ok()
//     }
// 
//     #[tokio::test]
//     async fn mental_health_handler_handles() {
//         let server = test_server().unwrap();
// 
//         let response = server.get("/mental-health");
// 
//         let result = response.await;
// 
//         result.assert_status_ok()
//     }
// 
//     #[tokio::test]
//     async fn predict_handler_handles() {
//         let server = test_server().unwrap();
// 
//         let response = server.get("/predict");
// 
//         let result = response.await;
// 
//         result.assert_status_ok()
//     }
// }