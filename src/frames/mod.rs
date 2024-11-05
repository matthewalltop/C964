use std::ops::Deref;
use std::str::FromStr;
use polars::prelude::{col, lit, when, Expr, LazyFrame};
use crate::frames::enums::{AdhdSubtype, Age, Gender, MentalHealthCondition};
use crate::frames::hyperaktiv::load_patient_info;

/// This module exposes the raw hyperaktiv dataset
mod hyperaktiv;

/// This module exposes queries that provide data on the ADHD subtypes of the patients contained within the dataset.
pub mod subtypes;

/// This module exposes queries that provide data on mental health conditions
pub mod mental_health;

/// This module exposes queries that provide data about patient medication.
pub mod medication;

/// Returns a translated and filtered version of the patient_info dataset from Hyperaktiv with default column selection.
pub fn get_all_patient_info(with_controls: bool) -> LazyFrame {
    load_patient_info(with_controls)
        .with_age_range_translation()
        .with_adhd_type_translation()
        .with_gender_translation()
        .select_default_patient_info_columns()
}

/// Returns full, untranslated, unfiltered Hyperaktiv patient info data 
pub fn get_all_patient_info_raw(with_controls: bool) -> LazyFrame {
    load_patient_info(with_controls)
}

pub mod enums {
    use std::fmt;
    use std::str::FromStr;
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub enum Gender {
        Female = 0,
        Male = 1,
        None = 2, // This is breaking convention because the data set cleanly aligns with '0' = Female & '1' = Male.
    }
    impl fmt::Display for Gender {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(self, f)
        }
    }

    impl FromStr for Gender {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_lowercase().as_str() {
                "female" => Ok(Gender::Female),
                "male" => Ok(Gender::Male),
                _ => Ok(Gender::None)
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub enum AdhdSubtype {
        None = 0,
        PrimaryHyperactive = 1,
        PrimaryInattentive = 2,
    }
    impl fmt::Display for AdhdSubtype {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(self, f)
        }
    }

    impl FromStr for AdhdSubtype {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_lowercase().as_str() {
                "adhd-ph" => Ok(AdhdSubtype::PrimaryHyperactive),
                "adhd-pi" => Ok(AdhdSubtype::PrimaryInattentive),
                "ph" => Ok(AdhdSubtype::PrimaryHyperactive),
                "pi" => Ok(AdhdSubtype::PrimaryInattentive),
                _ => Ok(AdhdSubtype::None)
            }
        }
    }


    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub enum Age {
        None = 0,
        SeventeenToTwentyNine = 1,
        ThirtyToThirtyNine = 2,
        FortyToFortyNine = 3,
        FiftyToSixtySeven = 4,
    }

    impl fmt::Display for Age {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(self, f)
        }
    }

    impl FromStr for Age {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_lowercase().as_str() {
                "seventeentotwentynine" => Ok(Age::SeventeenToTwentyNine),
                "thirtytothirtynine" => Ok(Age::ThirtyToThirtyNine),
                "fortytofortynine" => Ok(Age::FortyToFortyNine),
                "fiftytosixtyseven" => Ok(Age::FiftyToSixtySeven),
                _ => Ok(Age::None)
            }
        }
    }


    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub enum MentalHealthCondition {
        None = 0,
        BipolarDisorder = 1,
        UnipolarDepression = 2,
        AnxietyDisorder = 3,
        SubstanceAbuseDisorder = 4,
        Other = 5,
    }

    impl fmt::Display for MentalHealthCondition {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(self, f)
        }
    }

    impl FromStr for MentalHealthCondition {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_lowercase().as_str() {
                "bipolardisorder" => Ok(MentalHealthCondition::BipolarDisorder),
                "unipolardepression" => Ok(MentalHealthCondition::UnipolarDepression),
                "anxietydisorder" => Ok(MentalHealthCondition::AnxietyDisorder),
                "substanceabusedisorder" => Ok(MentalHealthCondition::SubstanceAbuseDisorder),
                "other" => Ok(MentalHealthCondition::Other),
                _ => Ok(MentalHealthCondition::None)
            }
        }
    }
}


pub trait PatientInfoTranslation {
    fn with_age_range_translation(&mut self) -> Self;
    fn with_adhd_type_translation(&mut self) -> Self;
    fn with_gender_translation(&mut self) -> Self;
    fn with_mental_health_translation(&mut self) -> Self;
}

pub trait PatientInfoFilter {
    fn with_adhd(&mut self, adhd_subtype: Option<AdhdSubtype>) -> Self;
    fn with_gender(&mut self, gender: Option<Gender>) -> Self;
    fn with_presence_of_mental_health_condition(&mut self) -> Self;
    fn with_absence_of_mental_health_condition(&mut self) -> Self;
    fn with_presence_of_given_mental_health_condition(&mut self, mental_health_condition: MentalHealthCondition) -> Self;
}

pub trait PatientInfoSelection {
    fn select_default_patient_info_columns(&mut self) -> Self;
    fn select_patient_info_columns(&mut self, fields: Vec<&str>) -> Self;
}

impl PatientInfoTranslation for LazyFrame {
    fn with_age_range_translation(&mut self) -> Self {
        self.deref().clone().with_column(
            when(
                col("AGE").eq(Age::SeventeenToTwentyNine as i32)
            )
                .then(lit("17-29"))
                .when(col("AGE").eq(Age::ThirtyToThirtyNine as i32))
                .then(lit("30-39"))
                .when(col("AGE").eq(Age::FortyToFortyNine as i32))
                .then(lit("40-49"))
                .otherwise(lit("50-67"))
                .alias("Age Range")
        )
    }
    fn with_adhd_type_translation(&mut self) -> Self {
        self.deref().clone().with_column(
            when(
                col("ADHD")
                    .eq(1)
                    .and(col("ADD").eq(1))
            ).then(
                lit("ADHD-PI")
            ).when(
                col("ADHD")
                    .eq(0)

                    .and(col("ADD").eq(0))
            ).then(
                lit("N/A")
            ).otherwise(lit("ADHD-PH"))
                .alias("ADHD Type")
        )
    }
    fn with_gender_translation(&mut self) -> LazyFrame {
        self.deref().clone().with_column(
            when(
                col("SEX").eq(Gender::Female as i32)
            ).then(
                lit("Female")
            ).otherwise(
                lit("Male")
            )
                .alias("Gender")
        )
    }

    fn with_mental_health_translation(&mut self) -> Self {
        self.deref().clone().with_column(
            when(
                col("BIPOLAR").eq(1)
            ).then(
                lit("Bipolar Disorder")
            ).when(
                col("UNIPOLAR").eq(1)
            ).then(
                lit("Unipolar Depression")
            ).when(
                col("ANXIETY").eq(1)
            ).then(
                lit("Anxiety Disorder")
            ).when(
                col("SUBSTANCE").eq(1)
            ).then(
                lit("Substance Abuse Disorder")
            ).when(
                col("OTHER").eq(1)
            ).then(
                lit("Other Condition")
            ).otherwise(lit("N/A"))
                .alias("Mental Health Condition")
        )
    }
}

impl PatientInfoFilter for LazyFrame {
    fn with_adhd(&mut self, adhd_subtype: Option<AdhdSubtype>) -> Self {
        let fltr: Expr = match adhd_subtype {
            Some(AdhdSubtype::PrimaryHyperactive) => col("ADHD").eq(1).and(col("ADD").eq(0)),
            Some(AdhdSubtype::PrimaryInattentive) => col("ADHD").eq(1).and(col("ADD").eq(1)),
            _ => col("ADHD").eq(1).or(col("ADD").eq(1))
        };

        self.deref().clone().filter(fltr)
    }

    fn with_gender(&mut self, gender: Option<Gender>) -> Self {
        let fltr: Expr = match gender {
            Some(Gender::Female) => col("SEX").eq(0),
            Some(Gender::Male) => col("SEX").eq(1),
            _ => col("ADHD").eq(1).or(col("ADD").eq(1))
        };

        self.deref().clone().filter(fltr)
    }

    fn with_presence_of_mental_health_condition(&mut self) -> Self {
        self.deref().clone().filter(
            col("BIPOLAR")
                .eq(1)
                .or(col("UNIPOLAR").eq(1))
                .or(col("ANXIETY").eq(1))
                .or(col("SUBSTANCE").eq(1))
                .or(col("OTHER").eq(1))
        )
    }

    fn with_absence_of_mental_health_condition(&mut self) -> Self {
        self.deref().clone().filter(
            col("BIPOLAR")
                .eq(0)
                .and(col("UNIPOLAR").eq(0))
                .and(col("ANXIETY").eq(0))
                .and(col("SUBSTANCE").eq(0))
                .and(col("OTHER").eq(0))
        )
    }

    fn with_presence_of_given_mental_health_condition(&mut self, mental_health_condition: MentalHealthCondition) -> Self {
        let condition = match mental_health_condition {
            MentalHealthCondition::None => Some(""),
            MentalHealthCondition::BipolarDisorder => Some("BIPOLAR"),
            MentalHealthCondition::UnipolarDepression => Some("UNIPOLAR"),
            MentalHealthCondition::SubstanceAbuseDisorder => Some("SUBSTANCE"),
            MentalHealthCondition::AnxietyDisorder => Some("ANXIETY"),
            MentalHealthCondition::Other => Some("OTHER"),
            _ => Some("")
        };

        let mhc = condition.unwrap();

        self.deref().clone()
            .filter(col(mhc).eq(1))
    }
}

impl PatientInfoSelection for LazyFrame {
    fn select_default_patient_info_columns(&mut self) -> Self {
        self.deref().clone().select(
            [
                col("ID"),
                col("Gender"),
                col("Age Range"),
                col("ADHD Type"),
                col("BIPOLAR"),
                col("UNIPOLAR"),
                col("ANXIETY"),
                col("SUBSTANCE"),
                col("OTHER"),
                col("MED"),
                col("MED_Antidepr"),
                col("MED_Moodstab"),
                col("MED_Antipsych"),
                col("MED_Anxiety_Benzo"),
                col("MED_Sleep"),
                col("MED_Analgesics_Opioids"),
                col("MED_Stimulants")
            ]
        )
    }

    fn select_patient_info_columns(&mut self, fields: Vec<&str>) -> Self {
        let mut _col_selection: Vec<Expr> = Vec::new();
        for c in fields {
            _col_selection.push(col(c));
        }

        self.deref().clone().select(_col_selection)
    }
}


#[cfg(test)]
mod test {
    use std::str::FromStr;
    use crate::frames::enums::AdhdSubtype;
    use super::*;

    #[test]
    fn gender_parses_to_i32() {
        assert_eq!(Gender::Female as i32, 0);
        assert_eq!(Gender::Male as i32, 1);
    }

    #[test]
    fn gender_to_strings() {
        assert_eq!(Gender::Female.to_string(), "Female");
        assert_eq!(Gender::Male.to_string(), "Male");
    }

    #[test]
    fn gender_from_strings() {
        assert_eq!(Gender::from_str("This is clearly not one of the allowed values").unwrap(), Gender::None);

        assert_eq!(Gender::from_str("Female").unwrap(), Gender::Female);
        assert_eq!(Gender::from_str("Male").unwrap(), Gender::Male);
        assert_eq!(Gender::from_str("female").unwrap(), Gender::Female);
        assert_eq!(Gender::from_str("male").unwrap(), Gender::Male);
        assert_eq!(Gender::from_str("FeMAlE").unwrap(), Gender::Female);
        assert_eq!(Gender::from_str("mAlE").unwrap(), Gender::Male);
    }

    #[test]
    fn subtype_parses_to_i32() {
        assert_eq!(AdhdSubtype::PrimaryHyperactive as i32, 1);
        assert_eq!(AdhdSubtype::PrimaryInattentive as i32, 2);
    }

    #[test]
    fn subtype_to_strings() {
        assert_eq!(AdhdSubtype::PrimaryHyperactive.to_string(), "PrimaryHyperactive");
        assert_eq!(AdhdSubtype::PrimaryInattentive.to_string(), "PrimaryInattentive");
    }

    #[test]
    fn subtype_from_strings() {
        assert_eq!(AdhdSubtype::from_str("This is clearly not one of the allowed values").unwrap(), AdhdSubtype::None);

        assert_eq!(AdhdSubtype::from_str("adhd-ph").unwrap(), AdhdSubtype::PrimaryHyperactive);
        assert_eq!(AdhdSubtype::from_str("adhd-pi").unwrap(), AdhdSubtype::PrimaryInattentive);
        assert_eq!(AdhdSubtype::from_str("ph").unwrap(), AdhdSubtype::PrimaryHyperactive);
        assert_eq!(AdhdSubtype::from_str("pi").unwrap(), AdhdSubtype::PrimaryInattentive);
    }


    #[test]
    fn age_parses_to_i32() {
        assert_eq!(Age::SeventeenToTwentyNine as i32, 1);
        assert_eq!(Age::ThirtyToThirtyNine as i32, 2);
        assert_eq!(Age::FortyToFortyNine as i32, 3);
        assert_eq!(Age::FiftyToSixtySeven as i32, 4);
    }

    #[test]
    fn age_to_strings() {
        assert_eq!(Age::SeventeenToTwentyNine.to_string(), "SeventeenToTwentyNine");
        assert_eq!(Age::ThirtyToThirtyNine.to_string(), "ThirtyToThirtyNine");
        assert_eq!(Age::FortyToFortyNine.to_string(), "FortyToFortyNine");
        assert_eq!(Age::FiftyToSixtySeven.to_string(), "FiftyToSixtySeven");
    }

    #[test]
    fn age_from_strings() {
        assert_eq!(Age::from_str("This is clearly not one of the allowed values").unwrap(), Age::None);


        assert_eq!(Age::from_str("SeventeenToTwentyNine").unwrap(), Age::SeventeenToTwentyNine);
        assert_eq!(Age::from_str("ThirtyToThirtyNine").unwrap(), Age::ThirtyToThirtyNine);
        assert_eq!(Age::from_str("FortyToFortyNine").unwrap(), Age::FortyToFortyNine);
        assert_eq!(Age::from_str("FiftyToSixtySeven").unwrap(), Age::FiftyToSixtySeven);

        assert_eq!(Age::from_str("seventeentotwentynine").unwrap(), Age::SeventeenToTwentyNine);
        assert_eq!(Age::from_str("thirtytothirtynine").unwrap(), Age::ThirtyToThirtyNine);
        assert_eq!(Age::from_str("fortytofortynine").unwrap(), Age::FortyToFortyNine);
        assert_eq!(Age::from_str("fiftytosixtyseven").unwrap(), Age::FiftyToSixtySeven);

        assert_eq!(Age::from_str("SeventEenTOTwEnTyNiNe").unwrap(), Age::SeventeenToTwentyNine);
        assert_eq!(Age::from_str("ThIrtyToTHirTyNINe").unwrap(), Age::ThirtyToThirtyNine);
        assert_eq!(Age::from_str("FORTYTOFORTYNINE").unwrap(), Age::FortyToFortyNine);
        assert_eq!(Age::from_str("FiftyTosiXtyseVeN").unwrap(), Age::FiftyToSixtySeven);
    }

    #[test]
    fn mental_health_condition_parses_to_i32() {
        assert_eq!(MentalHealthCondition::BipolarDisorder as i32, 1);
        assert_eq!(MentalHealthCondition::UnipolarDepression as i32, 2);
        assert_eq!(MentalHealthCondition::AnxietyDisorder as i32, 3);
        assert_eq!(MentalHealthCondition::SubstanceAbuseDisorder as i32, 4);
        assert_eq!(MentalHealthCondition::Other as i32, 5);
    }

    #[test]
    fn mental_health_condition_to_strings() {
        assert_eq!(MentalHealthCondition::BipolarDisorder.to_string(), "BipolarDisorder");
        assert_eq!(MentalHealthCondition::UnipolarDepression.to_string(), "UnipolarDepression");
        assert_eq!(MentalHealthCondition::AnxietyDisorder.to_string(), "AnxietyDisorder");
        assert_eq!(MentalHealthCondition::SubstanceAbuseDisorder.to_string(), "SubstanceAbuseDisorder");
        assert_eq!(MentalHealthCondition::Other.to_string(), "Other");
    }

    #[test]
    fn mental_health_condition_from_strings() {
        assert_eq!(MentalHealthCondition::from_str("This is clearly not one of the allowed values").unwrap(), MentalHealthCondition::None);

        assert_eq!(MentalHealthCondition::from_str("BipolarDisorder").unwrap(), MentalHealthCondition::BipolarDisorder);
        assert_eq!(MentalHealthCondition::from_str("UnipolarDepression").unwrap(), MentalHealthCondition::UnipolarDepression);
        assert_eq!(MentalHealthCondition::from_str("AnxietyDisorder").unwrap(), MentalHealthCondition::AnxietyDisorder);
        assert_eq!(MentalHealthCondition::from_str("SubstanceAbuseDisorder").unwrap(), MentalHealthCondition::SubstanceAbuseDisorder);
        assert_eq!(MentalHealthCondition::from_str("Other").unwrap(), MentalHealthCondition::Other);
    }
}
