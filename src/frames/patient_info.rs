use crate::frames::hyperaktiv::{load_patient_info};
use polars::prelude::*;
use crate::traits::patient_info_ext::{GenderAndADHDTypeFilter, SelectPatientInfoColumns};



/// Returns all patients who have Bipolar Disorder
pub fn patient_info_has_bipolar_disorder() -> DataFrame {
    load_patient_info(false)
        .filter(
            col("BIPOLAR").eq(1)
        )
        .apply_gender_age_adhd_type_translation()
        .select_patient_info_columns()
        .collect()
        .unwrap()
}

/// Returns all patients who have Unipolar Depression
pub fn patient_info_has_unipolar_depression() -> DataFrame {
    load_patient_info(false)
        .filter(
            col("UNIPOLAR").eq(1)
        )
        .apply_gender_age_adhd_type_translation()
        .select_patient_info_columns()
        .collect()
        .unwrap()
}

/// Returns all patients who have Anxiety
/// Does not discriminate whether patients have another comorbid mental health condition
pub fn patient_info_has_anxiety() -> DataFrame {
    load_patient_info(false)
        .filter(
            col("ANXIETY").eq(1)
        )
        .apply_gender_age_adhd_type_translation()
        .select_patient_info_columns()
        .collect()
        .unwrap()
}

/// Returns all patients who have a subtance abuse disorder
/// Does not discriminate whether patients have another comorbid mental health condition
pub fn patient_info_has_substance_abuse_disorder() -> DataFrame {
    load_patient_info(false)
        .filter(
            col("SUBSTANCE").eq(1)
        )
        .apply_gender_age_adhd_type_translation()
        .select_patient_info_columns()
        .collect()
        .unwrap()
}

/// Returns all patients who indicated 'Other' for presence of a mental health condition.
/// Does not discriminate whether patients have another comorbid mental health condition
pub fn patient_info_has_other_mental_health_condition() -> DataFrame {
    load_patient_info(false)
        .filter(
            col("OTHER").eq(1)
        )
        .apply_gender_age_adhd_type_translation()
        .select_patient_info_columns()
        .collect()
        .unwrap()
}

/// Returns all patients who indicated they are currently taking medication.
pub  fn patient_info_patient_takes_medication() -> DataFrame {
    load_patient_info(false)
        .filter(
            col("MED").eq(1).or(
            col("MED_Antidepr").eq(1)).or(
            col("MED_Moodstab").eq(1)).or(
            col("MED_Antipsych").eq(1)).or(
            col("MED_Anxiety_Benzo").eq(1)).or(
            col("MED_Sleep").eq(1)).or(
            col("MED_Analgesics_Opioids").eq(1)).or(
            col("MED_Stimulants").eq(1))
        )
        .apply_gender_age_adhd_type_translation()
        .select_patient_info_columns()
        .collect()
        .unwrap()
}

/// Returns all patients who indicated they take no medication for any condition.
pub  fn patient_info_patient_does_not_take_medication() -> DataFrame {
    load_patient_info(false)
        .filter(
            col("MED").eq(0).and(
                col("MED_Antidepr").eq(0)).and(
                col("MED_Moodstab").eq(0)).and(
                col("MED_Antipsych").eq(0)).and(
                col("MED_Anxiety_Benzo").eq(0)).and(
                col("MED_Sleep").eq(0)).and(
                col("MED_Analgesics_Opioids").eq(0)).and(
                col("MED_Stimulants").eq(0))
        )
        .apply_gender_age_adhd_type_translation()
        .select_patient_info_columns()
        .collect()
        .unwrap()
}

#[cfg(test)]
mod test {

    
}