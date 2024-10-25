use polars::frame::DataFrame;
use crate::frames::{ PatientInfoFilter, PatientInfoSelection, PatientInfoTranslation };
use crate::frames::enums::MentalHealthCondition;
use crate::frames::hyperaktiv::load_patient_info;

/// Returns all patients who reported a mental health or substance abuse condition.
pub fn patient_mental_health_conditions() -> DataFrame {
    load_patient_info(false)
        .with_presence_of_mental_health_condition()
        .with_age_range_translation()
        .with_adhd_type_translation()
        .with_gender_translation()
        .select_default_patient_info_columns()
        .collect()
        .unwrap()
}

/// Returns all patients who reported no additional mental health or substance abuse conditions.
pub fn patients_without_mental_health_conditions() -> DataFrame {
    load_patient_info(false)
        .with_absence_of_mental_health_condition()
        .with_age_range_translation()
        .with_adhd_type_translation()
        .with_gender_translation()
        .select_default_patient_info_columns()
        .collect()
        .unwrap()
}

/// Returns all patients who have Bipolar Disorder
/// Does not discriminate whether patients have another co-morbid mental health condition
pub fn patient_info_has_bipolar_disorder() -> DataFrame {
    load_patient_info(false)
        .with_presence_of_given_mental_health_condition(MentalHealthCondition::BipolarDisorder)
        .with_age_range_translation()
        .with_adhd_type_translation()
        .with_gender_translation()
        .select_default_patient_info_columns()
        .collect()
        .unwrap()
}

/// Returns all patients who have Unipolar Depression
/// /// Does not discriminate whether patients have another co-morbid mental health condition
pub fn patient_info_has_unipolar_depression() -> DataFrame {
    load_patient_info(false)
        .with_presence_of_given_mental_health_condition(MentalHealthCondition::UnipolarDepression)
        .with_age_range_translation()
        .with_adhd_type_translation()
        .with_gender_translation()
        .select_default_patient_info_columns()
        .collect()
        .unwrap()
}

/// Returns all patients who have Anxiety
/// Does not discriminate whether patients have another co-morbid mental health condition
pub fn patient_info_has_anxiety() -> DataFrame {
    load_patient_info(false)
        .with_presence_of_given_mental_health_condition(MentalHealthCondition::AnxietyDisorder)
        .with_age_range_translation()
        .with_adhd_type_translation()
        .with_gender_translation()
        .select_default_patient_info_columns()
        .collect()
        .unwrap()
}

/// Returns all patients who have a substance abuse disorder
/// Does not discriminate whether patients have another co-morbid mental health condition
pub fn patient_info_has_substance_abuse_disorder() -> DataFrame {
    load_patient_info(false)
        .with_presence_of_given_mental_health_condition(MentalHealthCondition::SubstanceAbuseDisorder)
        .with_age_range_translation()
        .with_adhd_type_translation()
        .with_gender_translation()
        .select_default_patient_info_columns()
        .collect()
        .unwrap()
}

/// Returns all patients who indicated 'Other' for presence of a mental health condition.
/// Does not discriminate whether patients have another comorbid mental health condition
pub fn patient_info_has_other_mental_health_condition() -> DataFrame {
    load_patient_info(false)
        .with_presence_of_given_mental_health_condition(MentalHealthCondition::Other)
        .with_age_range_translation()
        .with_adhd_type_translation()
        .with_gender_translation()
        .select_default_patient_info_columns()
        .collect()
        .unwrap()
}


#[cfg(test)]
mod test {
use super::*;
    use polars::prelude::*;

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
        assert_eq!(df.column("BIPOLAR").unwrap().tail(Some(1)), Series::new("BIPOLAR".into(), &[1]));
        assert!(df.column("BIPOLAR").unwrap().cast(&DataType::Boolean).unwrap().iter().all(|_| true));
    }

    #[test]
    fn loads_all_patients_with_unipolar_depression() {
        let df = patient_info_has_unipolar_depression();
        assert!(!df.is_empty());
        assert_eq!(df.column("UNIPOLAR").unwrap().tail(Some(1)), Series::new("UNIPOLAR".into(), &[1]));
        assert!(df.column("UNIPOLAR").unwrap().cast(&DataType::Boolean).unwrap().iter().all(|_| true));
    }

    #[test]
    fn loads_all_patients_with_anxiety() {
        let df = patient_info_has_anxiety();
        assert!(!df.is_empty());
        assert_eq!(df.column("ANXIETY").unwrap().tail(Some(1)), Series::new("ANXIETY".into(), &[1]));
        assert!(df.column("ANXIETY").unwrap().cast(&DataType::Boolean).unwrap().iter().all(|_| true));
    }

    #[test]
    fn loads_all_patients_with_substance_abuse_disorder() {
        let df = patient_info_has_substance_abuse_disorder();
        assert!(!df.is_empty());
        assert_eq!(df.column("SUBSTANCE").unwrap().tail(Some(1)), Series::new("SUBSTANCE".into(), &[1]));
        assert!(df.column("SUBSTANCE").unwrap().cast(&DataType::Boolean).unwrap().iter().all(|_| true));
    }

    #[test]
    fn loads_all_patients_with_other_mental_health_condition() {
        let df = patient_info_has_other_mental_health_condition();
        assert!(!df.is_empty());
        assert_eq!(df.column("OTHER").unwrap().tail(Some(1)), Series::new("OTHER".into(), &[1]));
        assert!(df.column("OTHER").unwrap().cast(&DataType::Boolean).unwrap().iter().all(|_| true));
    }
    
}