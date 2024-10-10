use polars::frame::DataFrame;
use polars::prelude::{col};
use crate::frames::hyperaktiv::load_patient_info;
use crate::frames::{PatientInfoSelection, PatientInfoTranslation};

/// Returns ADHD subtypes, gender, and age ranges of patients.
pub fn adhd_subtypes_with_gender_and_age() -> DataFrame {
    load_patient_info(false)
        .translate_gender_and_adhd_type()
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
        .translate_gender_and_adhd_type()
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
        .translate_gender_and_adhd_type()
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
        .translate_gender_and_adhd_type()
        .select_patient_info_columns()
        .collect()
        .unwrap()
}

/// Returns all patients with inattentive symptoms present.
/// Note - the dataset indicates the "ADD" field is only representative of
/// inattentive traits being present. It does not account for PI or C, specifically. 
pub fn patient_info_has_adhd_combined() -> DataFrame {
    load_patient_info(false)
        .filter(
            col("ADHD").eq(1).and(col("ADD").eq(1))
        )
        .translate_gender_and_adhd_type()
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

    #[test]
    fn loads_all_patients_adhd_combined_type() {
        let df = patient_info_has_adhd_combined();
        assert!(!df.is_empty());
        assert_eq!(df.column("ADHD Type").unwrap().tail(Some(1)), Series::new("ADHD Type", &["ADHD-C"]));
        assert!(df.column("ADHD Type").unwrap().iter().all_equal());
    }

}