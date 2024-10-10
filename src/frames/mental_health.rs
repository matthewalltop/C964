use polars::frame::DataFrame;
use crate::frames::{GenderAndADHDTypeFilter, MentalHealthFilter, SelectPatientInfoColumns};
use crate::frames::enums::MentalHealthCondition;
use crate::frames::hyperaktiv::load_patient_info;

/// Returns all patients who reported a mental health or substance abuse condition.
pub fn patient_mental_health_conditions() -> DataFrame {
    load_patient_info(false)
        .with_presence_of_mental_health_condition()
        .translate_gender_and_adhd_type()
        .select_patient_info_columns()
        .collect()
        .unwrap()
}

/// Returns all patients who reported no additional mental health or substance abuse conditions.
pub fn patients_without_mental_health_conditions() -> DataFrame {
    load_patient_info(false)
        .with_absence_of_mental_health_condition()
        .translate_gender_and_adhd_type()
        .select_patient_info_columns()
        .collect()
        .unwrap()
}

/// Returns all patients who have Bipolar Disorder
/// Does not discriminate whether patients have another co-morbid mental health condition
pub fn patient_info_has_bipolar_disorder() -> DataFrame {
    load_patient_info(false)
        .with_presence_of_given_mental_health_condition(MentalHealthCondition::BipolarDisorder)
        .translate_gender_and_adhd_type()
        .select_patient_info_columns()
        .collect()
        .unwrap()
}

/// Returns all patients who have Unipolar Depression
/// /// Does not discriminate whether patients have another co-morbid mental health condition
pub fn patient_info_has_unipolar_depression() -> DataFrame {
    load_patient_info(false)
        .with_presence_of_given_mental_health_condition(MentalHealthCondition::UnipolarDepression)
        .translate_gender_and_adhd_type()
        .select_patient_info_columns()
        .collect()
        .unwrap()
}

/// Returns all patients who have Anxiety
/// Does not discriminate whether patients have another co-morbid mental health condition
pub fn patient_info_has_anxiety() -> DataFrame {
    load_patient_info(false)
        .with_presence_of_given_mental_health_condition(MentalHealthCondition::AnxietyDisorder)
        .translate_gender_and_adhd_type()
        .select_patient_info_columns()
        .collect()
        .unwrap()
}

/// Returns all patients who have a substance abuse disorder
/// Does not discriminate whether patients have another co-morbid mental health condition
pub fn patient_info_has_substance_abuse_disorder() -> DataFrame {
    load_patient_info(false)
        .with_presence_of_given_mental_health_condition(MentalHealthCondition::SubstanceAbuseDisorder)
        .translate_gender_and_adhd_type()
        .select_patient_info_columns()
        .collect()
        .unwrap()
}

/// Returns all patients who indicated 'Other' for presence of a mental health condition.
/// Does not discriminate whether patients have another comorbid mental health condition
pub fn patient_info_has_other_mental_health_condition() -> DataFrame {
    load_patient_info(false)
        .with_presence_of_given_mental_health_condition(MentalHealthCondition::Other)
        .translate_gender_and_adhd_type()
        .select_patient_info_columns()
        .collect()
        .unwrap()
}


#[cfg(test)]
mod test {
    use polars::export::arrow::legacy::utils::CustomIterTools;
    use polars::prelude::{IntoLazy, NamedFrom, Series};
    use super::*;

    #[test]
    fn loads_all_patients_with_a_mental_health_condition() {
        let df = patient_mental_health_conditions();
        assert!(!df.is_empty());

        let patients_without_mental_health_conditions = df.lazy()
            .with_absence_of_mental_health_condition()
            .collect()
            .unwrap();
        
        assert!(patients_without_mental_health_conditions.is_empty())        
    }

    #[test]
    fn loads_all_patients_without_a_mental_health_condition() {
        let df = patients_without_mental_health_conditions();
        assert!(!df.is_empty());

        let patients_with_mental_health_conditions = df.lazy()
            .with_presence_of_mental_health_condition()
            .collect()
            .unwrap();

        assert!(patients_with_mental_health_conditions.is_empty())
    }

    #[test]
    fn loads_all_patients_with_bipolar_disorder() {
        let df = patient_info_has_bipolar_disorder();
        assert!(!df.is_empty());
        assert_eq!(df.column("BIPOLAR").unwrap().tail(Some(1)), Series::new("BIPOLAR", &[1]));
        assert!(df.column("BIPOLAR").unwrap().iter().all_equal());
    }

    #[test]
    fn loads_all_patients_with_unipolar_depression() {
        let df = patient_info_has_unipolar_depression();
        assert!(!df.is_empty());
        assert_eq!(df.column("UNIPOLAR").unwrap().tail(Some(1)), Series::new("UNIPOLAR", &[1]));
        assert!(df.column("UNIPOLAR").unwrap().iter().all_equal());
    }

    #[test]
    fn loads_all_patients_with_anxiety() {
        let df = patient_info_has_anxiety();
        assert!(!df.is_empty());
        assert_eq!(df.column("ANXIETY").unwrap().tail(Some(1)), Series::new("ANXIETY", &[1]));
        assert!(df.column("ANXIETY").unwrap().iter().all_equal());
    }

    #[test]
    fn loads_all_patients_with_substance_abuse_disorder() {
        let df = patient_info_has_substance_abuse_disorder();
        assert!(!df.is_empty());
        assert_eq!(df.column("SUBSTANCE").unwrap().tail(Some(1)), Series::new("SUBSTANCE", &[1]));
        assert!(df.column("SUBSTANCE").unwrap().iter().all_equal());
    }

    #[test]
    fn loads_all_patients_with_other_mental_health_condition() {
        let df = patient_info_has_other_mental_health_condition();
        assert!(!df.is_empty());
        assert_eq!(df.column("OTHER").unwrap().tail(Some(1)), Series::new("OTHER", &[1]));
        assert!(df.column("OTHER").unwrap().iter().all_equal());
    }
    
}