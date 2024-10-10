use polars::frame::DataFrame;
use polars::prelude::col;
use crate::frames::{GenderAndADHDTypeFilter, SelectPatientInfoColumns};
use crate::frames::hyperaktiv::load_patient_info;


pub fn patient_mental_health_conditions() -> DataFrame {
    load_patient_info(false)
        .filter(
            col("BIPOLAR")
                .eq(1)
                .or(col("UNIPOLAR").eq(1))
                .or(col("ANXIETY").eq(1))
        )
        .apply_gender_age_adhd_type_translation()
        .select([
            col("Gender"),
            col("AGE"),
            col("ADHD Type"),
            // TODO: Aggregate these such that they display the condition.
            col("BIPOLAR"),
            col("UNIPOLAR"),
            col("ANXIETY")
        ])
        .collect()
        .unwrap()
}

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
/// Does not discriminate whether patients have another co-morbid mental health condition
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