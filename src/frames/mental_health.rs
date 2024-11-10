 use polars::prelude::{col, lit, LazyFrame};
use crate::frames::{PatientInfoFilter, PatientInfoSelection, PatientInfoTranslation};
use crate::frames::hyperaktiv::load_patient_info;
use crate::JsonResponse;

/// Returns all patients who reported a mental health or substance abuse condition.
pub fn patients_with_comorbid_mental_health_conditions(with_controls: bool) -> JsonResponse {
    let df = mental_health_base_dataset(with_controls).collect()?;
    let result = serde_json::to_string(&df)?;

    Ok(result)
}

/// Returns all patients who reported no additional mental health or substance abuse conditions.
pub fn patients_without_comorbid_mental_health_conditions() -> JsonResponse {
    let df = load_patient_info(false)
        .with_absence_of_mental_health_condition()
        .with_age_range_translation()
        .with_adhd_type_translation()
        .with_gender_translation()
        .select_default_patient_info_columns()
        .collect()?;

    let result = serde_json::to_string(&df)?;

    Ok(result)
}

/// Returns all patients who have Bipolar Disorder
/// Does not discriminate whether patients have another co-morbid mental health condition
pub fn patient_info_has_bipolar_disorder(with_controls: bool) -> JsonResponse {
    let df = mental_health_base_dataset(with_controls).filter(
        col("Mental Health Condition").eq(lit("Bipolar Disorder"))
    ).collect()?;
    let result = serde_json::to_string(&df)?;

    Ok(result)
}

/// Returns all patients who have Unipolar Depression
/// /// Does not discriminate whether patients have another co-morbid mental health condition
pub fn patient_info_has_unipolar_depression(with_controls: bool) -> JsonResponse {
    let df = mental_health_base_dataset(with_controls).filter(
        col("Mental Health Condition").eq(lit("Unipolar Depression"))
    ).collect()?;
    
    let result = serde_json::to_string(&df)?;

    Ok(result)
}

/// Returns all patients who have Anxiety
/// Does not discriminate whether patients have another co-morbid mental health condition
pub fn patient_info_has_anxiety(with_controls: bool) -> JsonResponse {
    let df = mental_health_base_dataset(with_controls).filter(
        col("Mental Health Condition").eq(lit("Anxiety Disorder"))
    ).collect()?;

    let result = serde_json::to_string(&df)?;

    Ok(result)
}

/// Returns all patients who have a substance abuse disorder
/// Does not discriminate whether patients have another co-morbid mental health condition
pub fn patient_info_has_substance_abuse_disorder(with_controls: bool) -> JsonResponse {
    let df = mental_health_base_dataset(with_controls).filter(
        col("Mental Health Condition").eq(lit("Substance Abuse Disorder"))
    ).collect()?;

    let result = serde_json::to_string(&df)?;

    Ok(result)
}

/// Returns all patients who indicated 'Other' for presence of a mental health condition.
/// Does not discriminate whether patients have another comorbid mental health condition
pub fn patient_info_has_other_mental_health_condition(with_controls: bool) -> JsonResponse {
    let df = mental_health_base_dataset(with_controls).filter(
        col("Mental Health Condition").eq(lit("Other"))
    ).collect()?;

    let result = serde_json::to_string(&df)?;

    Ok(result)
}

fn mental_health_base_dataset(with_controls: bool) -> LazyFrame {
    load_patient_info(with_controls)
        .with_presence_of_mental_health_condition()
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
        ])
}