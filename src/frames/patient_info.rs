use crate::frames::hyperaktiv::{load_patient_info, Age, Gender};
use polars::prelude::*;

pub fn patient_info_has_adhd_hyperactive() -> DataFrame {
    load_patient_info(false)
        .filter(
            col("ADHD").eq(1)
        )
        .filter( col("ADD").eq(0))
        .with_column(
            when(
                col("ADHD").eq(1)
            ).then(
                lit("ADHD-PH")
            )
                .otherwise("ADHD")
                .alias("ADHD Type")
        )
        .with_column(
            when(
                col("SEX").eq(Gender::Female as i32)
            )
                .then(lit("Female"))
                .otherwise(lit("Male"))
                .alias("Gender")
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
        .select(
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
        .collect()
        .unwrap()
}

pub fn patient_info_has_adhd_innattentive() -> DataFrame {
    load_patient_info(false)
        .filter(
            col("ADHD").eq(0)
        )
        .filter(col("ADD").eq(1))
        .collect()
        .unwrap()
}

pub fn patient_info_has_adhd_combined() -> DataFrame {
    load_patient_info(false)
        .filter(
            col("ADHD").eq(1)
        )
        .filter( col("ADD").eq(1))
        .collect()
        .unwrap()
}


