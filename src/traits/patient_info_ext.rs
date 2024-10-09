use std::ops::Deref;
use polars::prelude::*;
use crate::frames::enums::{Age, Gender};

pub trait GenderAndADHDTypeFilter {
    fn apply_gender_age_adhd_type_translation(&mut self) -> Self;
}

pub trait SelectPatientInfoColumns {
    fn select_patient_info_columns(&mut self) -> Self;
}


impl GenderAndADHDTypeFilter for LazyFrame {
    // TODO: Not super happy about cloning here, see the actual implementation for ideas on a potentially better way to do this.
    // Note that a lot of the mechanisms that would make this easier are internal to the LazyFrame - so this may end up being the best path anyway.
    // https://github.com/pola-rs/polars/blob/main/crates/polars-lazy/src/frame/mod.rs
    fn apply_gender_age_adhd_type_translation(&mut self) -> LazyFrame {
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


impl SelectPatientInfoColumns for LazyFrame {

    fn select_patient_info_columns(&mut self) -> Self {
        self.deref().clone().select(
            [
                col("ID").alias("Patient ID"),
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