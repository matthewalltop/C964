use polars::frame::DataFrame;
use polars::prelude::col;
use crate::frames::{GenderAndADHDTypeFilter, SelectPatientInfoColumns};
use crate::frames::hyperaktiv::load_patient_info;

/// Returns all patients who indicated they are currently taking medication.
pub fn patient_info_patient_takes_medication() -> DataFrame {
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
    use super::*;
    
    #[test]
    fn loads_all_patients_taking_medication() {
        let df = patient_info_patient_takes_medication();
        assert!(!df.is_empty());
        println!("{}", df);
    }

    #[test]
    fn loads_all_patients_not_taking_medication() {
        let df = patient_info_patient_does_not_take_medication();
        assert!(!df.is_empty());
        println!("{}", df);
    }
}