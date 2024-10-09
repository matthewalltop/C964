use polars::frame::DataFrame;
use polars::prelude::{col};
use crate::frames::hyperaktiv::load_patient_info;
use crate::traits::patient_info_ext::{GenderAndADHDTypeFilter, SelectPatientInfoColumns};


/// Returns ADHD subtypes, gender, and age ranges of patients.
pub fn adhd_subtypes_with_gender_and_age() -> DataFrame {
    load_patient_info(false)
        .apply_gender_age_adhd_type_translation()
        .select([
            col("Gender"),
            col("ADHD Type"),
            col("Age Range"),
        ])
        .collect()
        .unwrap()
}

/// Returns ADHD subtypes, age, and gender for each female patient
pub fn adhd_subtypes_female() -> DataFrame {
    load_patient_info(false)
        .filter(col("SEX").eq(0))
        .apply_gender_age_adhd_type_translation()
        .select([
            col("Gender"),
            col("ADHD Type"),
            col("Age Range"),
        ])
        .collect()
        .unwrap()
}

/// Returns ADHD subtypes, age, and gender for each male patient
pub fn adhd_subtypes_male() -> DataFrame {
    load_patient_info(false)
        .filter(col("SEX").eq(1))
        .apply_gender_age_adhd_type_translation()
        .select([
            col("Gender"),
            col("ADHD Type"),
            col("Age Range"),
        ])
        .collect()
        .unwrap()
}

/// Returns all patients who have ADHD-PH
pub fn patient_info_has_adhd_hyperactive() -> DataFrame {
    load_patient_info(false)
        .filter(
            col("ADHD").eq(1).and(col("ADD").eq(0))
        )
        .apply_gender_age_adhd_type_translation()
        .select_patient_info_columns()
        .collect()
        .unwrap()
}

/// Returns all patients who have ADHD-PI
/// Note - there are not actually any patients who are explicitly PI types reported in the dataset.
#[deprecated]
pub fn patient_info_has_adhd_inattentive() -> DataFrame {
    load_patient_info(false)
        .filter(
            col("ADD").eq(1).and(col("ADHD").eq(0))
        )
        .apply_gender_age_adhd_type_translation()
        .select_patient_info_columns()
        .collect()
        .unwrap()
}

/// Returns all patients who have ADHD-C
pub fn patient_info_has_adhd_combined() -> DataFrame {
    load_patient_info(false)
        .filter(
            col("ADHD").eq(1).and(col("ADD").eq(1))
        )
        .apply_gender_age_adhd_type_translation()
        .select_patient_info_columns()
        .collect()
        .unwrap()
}


#[cfg(test)]
mod test {
    use super::*;
    use polars::export::arrow::legacy::utils::CustomIterTools;
    use polars::prelude::NamedFrom;
    use polars::series::Series;
    use crate::frames::subtypes::{adhd_subtypes_female, adhd_subtypes_male, adhd_subtypes_with_gender_and_age};

    #[test]
    fn loads_adhd_subtypes_with_gender_and_age() {
        let df = adhd_subtypes_with_gender_and_age();
        assert!(!df.is_empty())
    }

    #[test]
    fn loads_adhd_subtypes_female() {
        let df = adhd_subtypes_female();
        assert!(!df.is_empty());
        assert_eq!(df.column("Gender").unwrap().tail(Some(1)), Series::new("Gender", &["Female"]));
        assert!(df.column("Gender").unwrap().iter().all_equal());
    }

    #[test]
    fn loads_adhd_subtypes_male() {
        let df = adhd_subtypes_male();
        assert!(!df.is_empty());
        assert_eq!(df.column("Gender").unwrap().tail(Some(1)), Series::new("Gender", &["Male"]));
        assert!(df.column("Gender").unwrap().iter().all_equal());
    }

    #[test]
    fn loads_all_patients_adhd_primary_hyperactive() {
        let df = patient_info_has_adhd_hyperactive();
        assert!(!df.is_empty());
        assert_eq!(df.column("ADHD Type").unwrap().tail(Some(1)), Series::new("ADHD Type", &["ADHD-PH"]));
        assert!(df.column("ADHD Type").unwrap().iter().all_equal());
    }

    #[deprecated]
    /// This test always fails because there are no patients matching this criteria.
    /// Leaving this here more for informational purposes than anything.
    fn loads_all_patients_adhd_primary_inattentive() {
        let df = patient_info_has_adhd_inattentive();
        assert!(!df.is_empty());
        assert_eq!(df.column("ADHD Type").unwrap().tail(Some(1)), Series::new("ADHD Type", &["ADHD-PI"]));
        assert!(df.column("ADHD Type").unwrap().iter().all_equal());
    }

    #[test]
    fn loads_all_patients_adhd_combined_type() {
        let df = patient_info_has_adhd_combined();
        assert!(!df.is_empty());
        assert_eq!(df.column("ADHD Type").unwrap().tail(Some(1)), Series::new("ADHD Type", &["ADHD-C"]));
        assert!(df.column("ADHD Type").unwrap().iter().all_equal());
    }

}