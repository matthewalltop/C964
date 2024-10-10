use linfa::prelude::*;
use linfa_linear::{LinearRegression};
use polars::prelude::{DataType, Float64Type, IndexOrder};
use crate::frames::get_all_patient_info;



pub fn comorbidity_of_mental_health_condition() {
    unimplemented!()
}

/// Experiment to determine comorbidity of bipolar disorder among patient population.
pub fn comorbidity_of_bipolar_disorder() -> Result<(), Box<dyn std::error::Error>> {
    // Start by grabbing all the patients
    let df = get_all_patient_info();

    // Initially, let's just ignore any sort of granularity and focus on all of the patients, in general.
    let bipolar_series = df.column("BIPOLAR")?.cast(&DataType::Float64)?;
    let target = bipolar_series.f64()?;


    let mut features = df.drop("BIPOLAR")?;
    for col in features.get_column_names_owned() {
        let cast = df.column(&col)?
                               .cast(&DataType::Float64)
                               .expect("Failed to cast");
        
        features.with_column(cast)?;
    }


    let features_ndarray = features.to_owned().to_ndarray::<Float64Type>(IndexOrder::C)?;
    let target_ndarray = target.to_ndarray()?.to_owned();
    let (dataset_training, dataset_validation) = Dataset::new(features_ndarray, target_ndarray).split_with_ratio(0.80);


    // Train the model
    let model = LinearRegression::default().fit(&dataset_training)?;

    // Predictions
    let pred = model.predict(&dataset_validation);

    // Evaluate the model
    let r2 = pred.r2(&dataset_validation)?;
    println!("r2 from prediction: {}", r2);
    
    Ok(())
}

pub fn comorbidity_of_unipolar_depression() {
    unimplemented!()
}

pub fn comborbidity_of_anxiety_disorder() {
    unimplemented!()
}

pub fn comorbidity_of_substance_abuse_disorder() {
    unimplemented!()
}

