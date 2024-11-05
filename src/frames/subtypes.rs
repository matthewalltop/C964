use polars::prelude::{col, CategoricalOrdering, DataType, LazyFrame};
use crate::frames::hyperaktiv::load_patient_info;
use crate::frames::{PatientInfoFilter, PatientInfoSelection, PatientInfoTranslation};
use crate::frames::enums::AdhdSubtype;

/// Returns ADHD subtypes, gender, and age ranges of patients.
pub fn adhd_subtypes_with_gender_and_age() -> LazyFrame {
    load_patient_info(false)
        .with_age_range_translation()
        .with_adhd_type_translation()
        .with_gender_translation()
        .select([
            col("ID").cast(DataType::Int32),
            col("Gender").cast(DataType::Categorical(None, CategoricalOrdering::default())),
            col("ADHD Type").cast(DataType::Categorical(None, CategoricalOrdering::default())),
            col("Age Range").cast(DataType::Categorical(None, CategoricalOrdering::default()))
        ])
}

/// Returns ADHD subtypes, age, and gender for each female patient
pub fn adhd_subtypes_female() -> LazyFrame {
    load_patient_info(false)
        .filter(col("SEX").eq(0))
                .with_age_range_translation()
        .with_adhd_type_translation()
        .with_gender_translation()
        .select([
            col("Gender").cast(DataType::Categorical(None, CategoricalOrdering::default())),
            col("ADHD Type").cast(DataType::Categorical(None, CategoricalOrdering::default())),
            col("Age Range").cast(DataType::Categorical(None, CategoricalOrdering::default()))
        ])
}

/// Returns ADHD subtypes, age, and gender for each male patient
pub fn adhd_subtypes_male() -> LazyFrame {
    load_patient_info(false)
        .filter(col("SEX").eq(1))
                .with_age_range_translation()
        .with_adhd_type_translation()
        .with_gender_translation()
        .select([
            col("ID").cast(DataType::Int32),
            col("Gender").cast(DataType::Categorical(None, CategoricalOrdering::default())),
            col("ADHD Type").cast(DataType::Categorical(None, CategoricalOrdering::default())),
            col("Age Range").cast(DataType::Categorical(None, CategoricalOrdering::default()))
        ])
}

/// Returns all patients who have ADHD-PH
pub fn patient_info_has_adhd_hyperactive() -> LazyFrame {
    load_patient_info(false)
        .with_adhd(Some(AdhdSubtype::PrimaryHyperactive))
        .with_age_range_translation()
        .with_adhd_type_translation()
        .with_gender_translation()
        .select_default_patient_info_columns()
}

/// Returns all patients with inattentive symptoms present.
/// Note - the dataset indicates the "ADD" field is only representative of
/// inattentive traits being present. It does not account for PI or C, specifically. 
pub fn patient_info_has_adhd_inattentive() -> LazyFrame {
    load_patient_info(false)
        .with_adhd(Some(AdhdSubtype::PrimaryInattentive))
        .with_age_range_translation()
        .with_adhd_type_translation()
        .with_gender_translation()
        .select_default_patient_info_columns()
}

/// Returns all patients from the control group, without ADHD present.
pub fn patient_info_does_not_have_adhd() -> LazyFrame {
    load_patient_info(false)
        .filter(
            col("ADHD").eq(0).and(col("ADD").eq(0))
        )
        .with_age_range_translation()
        .with_adhd_type_translation()
        .with_gender_translation()
        .select_default_patient_info_columns()
}


#[cfg(test)]
mod test {
    use std::error::Error;
    use super::*;
    use polars::datatypes::DataType;
    use polars::prelude::NamedFrom;
    use polars::series::Series;
    use crate::frames::subtypes::{adhd_subtypes_female, adhd_subtypes_male, adhd_subtypes_with_gender_and_age};

    #[test]
    fn loads_adhd_subtypes_with_gender_and_age() -> Result<(), Box<dyn Error>> {
        let df = adhd_subtypes_with_gender_and_age().collect()?;
        assert!(!df.is_empty());
        Ok(())
    }

    #[test]
    fn loads_adhd_subtypes_female() -> Result<(), Box<dyn Error>> {
        let df = adhd_subtypes_female().collect()?;
        assert!(!df.is_empty());
        assert_eq!(df.column("Gender").unwrap().tail(Some(1)), Series::new("Gender".into(), &["Female"]));
        assert!(df.column("Gender").unwrap().cast(&DataType::String).unwrap().iter().all(|x| x == "Female".into()));
        Ok(())
    }

    #[test]
    fn loads_adhd_subtypes_male() -> Result<(), Box<dyn Error>> {
        let df = adhd_subtypes_male().collect()?;
        assert!(!df.is_empty());
        assert_eq!(df.column("Gender").unwrap().tail(Some(1)), Series::new("Gender".into(), &["Male"]));
        assert!(df.column("Gender").unwrap().cast(&DataType::String).unwrap().iter().all(|x| x == "Male".into()));
        Ok(())
    }

    #[test]
    fn loads_all_patients_adhd_predominantly_hyperactive() -> Result<(), Box<dyn Error>> {
        let df = patient_info_has_adhd_hyperactive().collect()?;
        assert!(!df.is_empty());
        assert_eq!(df.column("ADHD Type").unwrap().tail(Some(1)), Series::new("ADHD Type".into(), &["ADHD-PH"]));
        assert!(df.column("ADHD Type").unwrap().cast(&DataType::String).unwrap().iter().all(|x| x == "ADHD-PH".into()));
        Ok(())
    }

    #[test]
    fn loads_all_patients_adhd_predominantly_inattentive() -> Result<(), Box<dyn Error>> {
        let df = patient_info_has_adhd_inattentive().collect()?;
        assert!(!df.is_empty());
        assert_eq!(df.column("ADHD Type").unwrap().tail(Some(1)), Series::new("ADHD Type".into(), &["ADHD-PI"]));
        assert!(df.column("ADHD Type").unwrap().cast(&DataType::String).unwrap().iter().all(|x| x == "ADHD-PI".into()));
        Ok(())
    }

}