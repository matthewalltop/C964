use plotlars::{Plot, ScatterPlot};
use polars::prelude::{col, lit, when};
use frames::hyperaktiv::*;
use crate::frames;

pub fn comorbidity_of_anxiety_or_mood_disorder() {
    let dataset = load_patient_info(false)
                                .filter(
                                    col("BIPOLAR")
                                        .eq(1)
                                        .or(col("UNIPOLAR").eq(1))
                                        .or(col("ANXIETY").eq(1))
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