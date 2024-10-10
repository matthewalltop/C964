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

pub fn patients_without_mental_health_conditions() -> DataFrame {
    load_patient_info(false)
        .filter(
            col("BIPOLAR")
                .eq(0)
                .and(col("UNIPOLAR").eq(0))
                .and(col("ANXIETY").eq(0))
                .and(col("SUBSTANCE").eq(0))
                .and(col("OTHER").eq(0))
        )
        .apply_gender_age_adhd_type_translation()
        .select([
            col("Gender"),
            col("AGE"),
            col("ADHD Type"),
            // TODO: Aggregate these such that they display the condition.
            col("BIPOLAR"),
            col("UNIPOLAR"),
            col("ANXIETY"),
            col("SUBSTANCE"),
            col("OTHER")
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
            .filter(col("BIPOLAR").eq(0).and(col("UNIPOLAR").eq(0)).and(col("ANXIETY").eq(0)))
            .collect()
            .unwrap();
        
        assert!(patients_without_mental_health_conditions.is_empty())        
    }

    #[test]
    fn loads_all_patients_without_a_mental_health_condition() {
        let df = patients_without_mental_health_conditions();
        assert!(!df.is_empty());

        let patients_with_mental_health_conditions = df.lazy()
            .filter(col("BIPOLAR").eq(1).or(col("UNIPOLAR").eq(1)).or(col("ANXIETY").eq(1)).or(col("SUBSTANCE").eq(1)).or(col("OTHER").eq(1)))
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