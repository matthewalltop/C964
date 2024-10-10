use polars::frame::DataFrame;
use polars::prelude::col;
use crate::frames::{GenderAndADHDTypeFilter, SelectPatientInfoColumns};
use crate::frames::hyperaktiv::load_patient_info;

/// Returns all patients who indicated they are currently taking medication.
/// Note: All the patients in this dataset are taking medication - there is no inverse condition for this one.
pub fn patient_takes_medication() -> DataFrame {
    load_patient_info(false)
        .filter(col("MED").eq(1).or(
                col("MED_Antidepr").eq(1)).or(
                col("MED_Moodstab").eq(1)).or(
                col("MED_Antipsych").eq(1)).or(
                col("MED_Anxiety_Benzo").eq(1)).or(
                col("MED_Sleep").eq(1)).or(
                col("MED_Analgesics_Opioids").eq(1)).or(
                col("MED_Stimulants").eq(1)))
        .translate_gender_and_adhd_type()
        .select_patient_info_columns()
        .collect()
        .unwrap()
}

/// Returns all patients who indicated they are currently taking medication.
pub fn patient_does_not_take_medication() -> DataFrame {
    load_patient_info(false)
        // The additional filtering here is redundant
        .filter(col("MED").eq(0).and(
                col("MED_Antidepr").is_null()).and(
                col("MED_Moodstab").is_null()).and(
                col("MED_Antipsych").is_null()).and(
                col("MED_Anxiety_Benzo").is_null()).and(
                col("MED_Sleep").is_null()).and(
                col("MED_Analgesics_Opioids").is_null()).and(
                col("MED_Stimulants").is_null()))
        .translate_gender_and_adhd_type()
        .select_patient_info_columns()
        .collect()
        .unwrap()
}

// TODO: If it provides value, add more methods to query for specific medications.


#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn loads_all_patients_taking_medication() {
        let df = patient_takes_medication();
        assert!(!df.is_empty());
        println!("{}", df);
    }

    #[test]
    fn loads_all_patients_not_taking_medication() {
        let df = patient_does_not_take_medication();
        assert!(!df.is_empty());
        println!("{}", df);
    }
}