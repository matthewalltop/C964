use std::ops::Deref;
use polars::prelude::{col, lit, when, Expr, LazyFrame};
use crate::frames::enums::{Age, Gender, MentalHealthCondition};
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
    
    #[derive(Debug)]
    pub enum Gender {
        Female = 0,
        Male = 1
    }

    impl fmt::Display for Gender {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(self, f)
        }
    }

    #[derive(Debug)]
    pub enum Age {
        SeventeenToTwentyNine = 1,
        ThirtyToThirtyNine = 2,
        FortyToFortyNine = 3,
        FiftyToSixtySeven = 4
    }

    impl fmt::Display for Age {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(self, f)
        }
    }
    
    #[derive(Debug)]
    pub enum MentalHealthCondition {
        BipolarDisorder = 1,
        UnipolarDepression = 2,
        AnxietyDisorder = 3,
        SubstanceAbuseDisorder = 4,
        Other = 5
    }

    impl fmt::Display for MentalHealthCondition {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(self, f)
        }
    }

    #[cfg(test)]
    mod test {
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
    }

} 

pub trait PatientInfoTranslation {
    fn with_age_range_translation(&mut self) -> Self;
    fn with_adhd_type_translation(&mut self) -> Self;
    fn with_gender_translation(&mut self) -> Self;
}

pub trait PatientInfoFilter {
    fn with_presence_of_mental_health_condition(&mut self) -> Self;
    fn with_absence_of_mental_health_condition(&mut self) -> Self;
    fn with_presence_of_given_mental_health_condition(&mut self, mental_health_condition: MentalHealthCondition) -> Self;
}

pub trait PatientInfoSelection {
    fn select_default_patient_info_columns(&mut self) -> Self;
    fn select_patient_info_columns(&mut self, fields: Vec<&str>) -> Self;
}

impl PatientInfoTranslation for LazyFrame {
    // TODO: Not super happy about cloning here, see the actual implementation for ideas on a potentially better way to do this.
    // Note that a lot of the mechanisms that would make this easier are internal to the LazyFrame - so this may end up being the best path anyway.
    // https://github.com/pola-rs/polars/blob/main/crates/polars-lazy/src/frame/mod.rs
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
}

impl PatientInfoFilter for LazyFrame {
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
        let condition = match  mental_health_condition {
            MentalHealthCondition::BipolarDisorder => Some("BIPOLAR"),
            MentalHealthCondition::UnipolarDepression => Some("UNIPOLAR"),
            MentalHealthCondition::SubstanceAbuseDisorder => Some("SUBSTANCE"),
            MentalHealthCondition::AnxietyDisorder => Some("ANXIETY"),
            MentalHealthCondition::Other => Some("OTHER")
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