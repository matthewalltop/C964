use std::sync::Arc;
use polars::datatypes::DataType;
use polars::frame::DataFrame;
use polars::prelude::{col, lit, when, RevMapping};
use crate::frames::enums::{Age, Gender};
use crate::frames::hyperaktiv::load_patient_info;


/// Returns ADHD subtypes, gender, and age ranges of patients.
pub fn adhd_subtypes_with_gender_and_age() -> DataFrame {
    load_patient_info(false)
        .with_column(
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
        .with_column( 
            col("SEX").map(|_| -> Gender::Male.to_string(), Default::default())
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
        .with_column(
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
    use polars::export::arrow::legacy::utils::CustomIterTools;
    use polars::prelude::NamedFrom;
    use polars::series::Series;
    use super::*;
    
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