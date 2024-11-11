use polars::prelude::{col};
use crate::frames::{PatientInfoTranslation};
use crate::frames::hyperaktiv::load_patient_info;
use crate::JsonResponse;

/// Returns all patients who reported a mental health or substance abuse condition.
pub fn patients_with_comorbid_mental_health_conditions(with_controls: bool) -> JsonResponse {
    let df =  load_patient_info(with_controls)
        .with_age_range_translation()
        .with_adhd_type_translation()
        .with_gender_translation()
        .with_mental_health_translation()
        .select([
            col("ID"),
            col("Age Range"),
            col("Gender"),
            col("ADHD Type"),
            col("Mental Health Condition")
        ]).collect()?;
    
    let result = serde_json::to_string(&df)?;

    Ok(result)
}