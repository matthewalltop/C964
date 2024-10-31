use axum::extract::Query;
use crate::frames::subtypes::adhd_subtypes_with_gender_and_age;
use crate::http::requests::queries::DemographicsParams;
use crate::plots::adhd_types::{plot_by_adhd_type_by_age_group, plot_by_adhd_type_by_gender};

async fn demographic_handler(demographics_query: Query<DemographicsParams>) -> String {
    let qry = demographics_query.0;
    let gender = qry.gender.unwrap_or_else(|| "".into());

    if !gender.is_empty() {
        plot_by_adhd_type_by_gender().unwrap()
    } else {
        plot_by_adhd_type_by_age_group().unwrap()
    }
}



async fn subtype_handler() -> String {
    serde_json::to_string(&adhd_subtypes_with_gender_and_age().collect().unwrap()).unwrap()
}