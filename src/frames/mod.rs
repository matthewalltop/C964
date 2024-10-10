use std::ops::Deref;
use polars::prelude::{col, lit, when, LazyFrame};
use crate::frames::enums::{Age, Gender, MentalHealthCondition};

/// This module exposes the raw hyperaktiv dataset
mod hyperaktiv;

pub mod subtypes;
pub mod mental_health;
pub mod medication;

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

// TODO: I consolidated several traits into this one large one - it may just be better to keep them all entirely separate.
// They all do technically apply to the same "domain", as it were, each of the methods adds a translation, filter, or selection to the patient_info dataset once loaded into a Polars DataFrame

trait PatientInfoTranslation {
    fn translate_gender_and_adhd_type(&mut self) -> Self;
}

trait PatientInfoFilter {
    fn with_presence_of_mental_health_condition(&mut self) -> Self;
    fn with_absence_of_mental_health_condition(&mut self) -> Self;
    fn with_presence_of_given_mental_health_condition(&mut self, mental_health_condition: MentalHealthCondition) -> Self;
}

trait PatientInfoSelection {
    fn select_patient_info_columns(&mut self) -> Self;
}

impl PatientInfoTranslation for LazyFrame {
    // TODO: Not super happy about cloning here, see the actual implementation for ideas on a potentially better way to do this.
    // Note that a lot of the mechanisms that would make this easier are internal to the LazyFrame - so this may end up being the best path anyway.
    // https://github.com/pola-rs/polars/blob/main/crates/polars-lazy/src/frame/mod.rs
    fn translate_gender_and_adhd_type(&mut self) -> LazyFrame {
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
            .with_column(
                when(
                    col("ADHD")
                        .eq(1)
                        .and(col("ADD").eq(1))
                ).then(
                    lit("ADHD-C")
                )
                    .when(col("ADHD").eq(1))
                    .then(lit("ADHD-PH"))
                    // This condition doesn't actually exist in the data set, explicitly - but inattentive symptoms are implied by a '1' in the ADD column.
                    // Patients with this flag could be considered of the ADHD-Combined type or ADHD-Primary Inattentive type.
                    // If we did have the presence of a 1 in the ADD column, but a 0 in the ADHD column - this would strongly imply ADHD-PI.
                    .otherwise(lit("ADHD-PI"))
                    .alias("ADHD Type")
            )
            .with_column(
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
    fn select_patient_info_columns(&mut self) -> Self {
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
}