use polars::prelude::{col};
use frames::hyperaktiv::*;
use crate::frames;

pub fn comorbidity_of_anxiety_or_mood_disorder() {
    // TODO: This belongs in frames - the graphical element can be exposed here, however.
    
    let dataset = load_patient_info(false)
        .filter(
            col("BIPOLAR")
                .eq(1)
                .or(col("UNIPOLAR").eq(1))
                .or(col("ANXIETY").eq(1))
        )
        .select([
            col("SEX"),
            col("AGE"),
            col("ADHD Type"),
            // TODO: Aggregate these such that they display the condition.
            col("BIPOLAR"),
            col("UNIPOLAR"),
            col("ANXIETY")
        ])
        .collect()
        .unwrap();

    println!("{}", dataset);
    // ScatterPlot::builder()
    //     .data(&dataset)
    //     .x("ADHD Type")
    //     .y("BIPOLAR")
    //     .build()
    //     .plot()
}