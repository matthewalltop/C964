use std::ops::Deref;
use polars::prelude::{col, lit, when, Expr, LazyFrame};
use crate::enums::{AdhdSubtype, Age, Gender };

pub trait PatientInfoTranslation {
    fn with_age_range_translation(&mut self) -> Self;
    fn with_adhd_type_translation(&mut self) -> Self;
    fn with_gender_translation(&mut self) -> Self;
    fn with_mental_health_translation(&mut self) -> Self;
}

pub trait PatientInfoFilter {
    fn with_adhd(&mut self, adhd_subtype: Option<AdhdSubtype>) -> Self;
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
                lit(AdhdSubtype::None.to_string())
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
            Some(AdhdSubtype::All) => col("ADHD").eq(1).or(col("ADD").eq(1)),
            _ => col("ADHD").eq(0).and(col("ADD").eq(0))
        };

        self.deref().clone().filter(fltr)
    }
}