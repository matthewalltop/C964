use std::ops::Deref;
use polars::prelude::{col, lit, when, Expr, LazyFrame};
use crate::enums::{AdhdSubtype, Age, Gender, MentalHealthCondition};

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

    fn with_patient_takes_medication(&mut self) -> Self;

    fn with_patient_does_not_take_medication(&mut self) -> Self;
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
            Some(AdhdSubtype::All) => col("ADHD").eq(1).or(col("ADD").eq(1)),
            _ => col("ADHD").eq(0).and(col("ADD").eq(0))
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
        };

        let mhc = condition.unwrap();

        self.deref().clone()
            .filter(col(mhc).eq(1))
    }


    fn with_patient_takes_medication(&mut self) -> Self {
        self.deref().clone()
            .filter(col("MED").eq(1).or(
                col("MED_Antidepr").eq(1)).or(
                col("MED_Moodstab").eq(1)).or(
                col("MED_Antipsych").eq(1)).or(
                col("MED_Anxiety_Benzo").eq(1)).or(
                col("MED_Sleep").eq(1)).or(
                col("MED_Analgesics_Opioids").eq(1)).or(
                col("MED_Stimulants").eq(1)))
    }

    fn with_patient_does_not_take_medication(&mut self) -> Self {
        self.deref().clone()
            .filter(col("MED").eq(0).and(
                col("MED_Antidepr").is_null()).and(
                col("MED_Moodstab").is_null()).and(
                col("MED_Antipsych").is_null()).and(
                col("MED_Anxiety_Benzo").is_null()).and(
                col("MED_Sleep").is_null()).and(
                col("MED_Analgesics_Opioids").is_null()).and(
                col("MED_Stimulants").is_null()))
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

