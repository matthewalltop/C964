use std::str::FromStr;
use axum::extract::Query;
use crate::frames::enums::Gender;
use crate::frames::subtypes::adhd_subtypes_with_gender_and_age;
use crate::http::requests::queries::{ApiParameters, DemographicsParams, MedicationParams, MentalHealthParams, SubtypeParams};
use crate::plots::adhd_types::{plot_by_adhd_type_by_age_group, plot_by_adhd_type_by_gender};

pub async fn demographic_handler(demographics_query: Query<ApiParameters<DemographicsParams>>) -> String {
    let qry_params = demographics_query.0;
    let sub_query = qry_params.sub_query.unwrap_or(DemographicsParams::default());
    
    let gender = sub_query.gender.unwrap_or_else(|| "".into());
    let age_group = sub_query.age_range.unwrap_or_else(|| "".into());
    
    // Types
    // By Gender
    //  - All (No param, default)
    //  - Male
    //  - Female
    
    // By Age Group
    
    if !gender.is_empty() {
        let parsed = Gender::from_str(gender.as_str()).unwrap();
        plot_by_adhd_type_by_gender(Some(parsed)).unwrap()
    } else {
        plot_by_adhd_type_by_age_group().unwrap()
    }
}


pub async fn subtype_handler(subtype_query: Query<ApiParameters<SubtypeParams>>) -> String {
    let qry_params = subtype_query.0;
    let sub_query = qry_params.sub_query.unwrap_or(SubtypeParams::default());

    let gender = Gender::from_str(sub_query.gender.unwrap_or_else(|| "".into()).as_str());
    let sub_type = sub_query.adhd_subtype.unwrap_or_else(|| "".into()).as_str();
    
    
    
    
    
    
    
    
    
    
    
    
    
    serde_json::to_string(&adhd_subtypes_with_gender_and_age().collect().unwrap()).unwrap()
}

pub async fn mental_health_handler(mental_health_query: Query<ApiParameters<MentalHealthParams>>) -> String {
    let qry_params = mental_health_query.0;
    let sub_query = qry_params.sub_query.unwrap_or(MentalHealthParams::default());

    let conditions = sub_query.conditions.unwrap_or_else(|| vec![].into());
    
    
    
    unimplemented!()
}

pub async fn medication_handler(medication_query: Query<ApiParameters<MedicationParams>>) -> String {
    let qry_params = medication_query.0;
    let sub_query = qry_params.sub_query.unwrap_or(MedicationParams::default());

    let conditions = sub_query.medications.unwrap_or_else(|| vec![].into());
    
    
    
    unimplemented!()
}


#[cfg(test)]
mod test {

}