use std::error::Error;
use polars::prelude::col;
use crate::enums::AdhdSubtype;
use crate::frames::get_all_patient_info_raw;
use crate::JsonResponse;
use crate::traits::{PatientInfoFilter, PatientInfoTranslation};

/// Returns filtered data frame containing filtered ADHD Subtype info.
pub fn adhd_subtype_info(with_controls: bool) -> JsonResponse {
    let df = get_all_patient_info_raw(with_controls)
        .with_adhd(Some(AdhdSubtype::All))
        .with_gender_translation()
        .with_adhd_type_translation()
        .with_age_range_translation()
        .select([
            col("ID"),
            col("Gender"),
            col("ADHD Type"),
            col("Age Range")
        ]).collect()?;
    
    let result = serde_json::to_string(&df).unwrap();
    
    Ok(result)
}