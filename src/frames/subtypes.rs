use polars::frame::DataFrame;
use polars::prelude::{col};
use crate::frames::hyperaktiv::load_patient_info;
use crate::traits::patient_info_ext::GenderAndADHDTypeFilter;


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


#[cfg(test)]
mod test {
    use super::*;
    use polars::export::arrow::legacy::utils::CustomIterTools;
    use polars::prelude::NamedFrom;
    use polars::series::Series;
    

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
}