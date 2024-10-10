use linfa::prelude::*;
use linfa_linear::{LinearRegression, TweedieRegressor};
use polars::prelude::{Float64Type, IndexOrder};
use crate::frames::get_all_patient_info;


pub fn comorbidity_of_mental_health_condition() {
    // Start by grabbing all the patients
    let df = get_all_patient_info();
    
    // Initially, let's just ignore any sort of granularity and focus on all of the patients, in general.
    let features = df.to_ndarray::<Float64Type>(IndexOrder::C);
    let dataset = Dataset::from(features.unwrap());
    
    // Train the model using linear regression.
    // This removes regularization & establishes this as a normal Linear Regression.
    // See example here: https://github.com/rust-ml/linfa/blob/master/algorithms/linfa-linear/examples/glm.rs
    let lin_reg = TweedieRegressor::params().power(0.).alpha(0.);
    let model = lin_reg.fit(&dataset).unwrap();

    // We print the Mean Absolute Error (MAE) on the training data
    //
    // Some(43.27739632065444)
    let prediction = model.predict(&dataset);
    let loss = (dataset.targets() - &prediction.insert_axis(ndarray::Axis(1)))
        .mapv(|x| x.abs())
        .mean();

    println!("{:?}", loss);
    
    println!("{}", model.intercept);
    println!("{}", model.coef);
}

pub fn comorbidity_of_bipolar_disorder() {
    
}