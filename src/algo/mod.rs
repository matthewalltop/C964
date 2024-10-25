use linfa::{DatasetBase};
use linfa::prelude::{Fit, Predict}; // ToConfusionMatrix
use linfa_logistic::{LogisticRegression};
use polars::frame::DataFrame;
use polars::prelude::{DataType, Float64Type, IndexOrder};

/// Applies binomial logistic regression to the provided data and dependent variable.
pub fn apply_binomial_logistic_regression(df: DataFrame, independent: &str) -> Result<f32, Box<dyn std::error::Error>> {
    // Extract the independent variable from the dataset & cast it to a float64 value.
    let series = df.column(independent)?.cast(&DataType::Float64)?;
    let target = series.f64()?;

    let mut features = df.drop(independent)?;
    // Ensure 
    for col in features.get_column_names_owned() {
        let cast = df.column(&col)?
            .cast(&DataType::Float64)
            .expect("Failed to cast");

        features.with_column(cast)?;
    }

    let features_ndarray = features.to_owned().to_ndarray::<Float64Type>(IndexOrder::C)?;
    let target_ndarray = target.to_ndarray()?.to_owned();
    let (dataset_training, dataset_validation) = DatasetBase::new(features_ndarray, target_ndarray)
        .map_targets(|x| *x == 1.0)
        .split_with_ratio(0.80);

    
    // Train the model
    let model = LogisticRegression::default()
        .max_iterations(150)
        .fit(&dataset_training)?;

    // Predictions
    let predict = model.predict(&dataset_validation);
   // let confusion_matrix = predict.confusion_matrix(&dataset_training)?; 
    
    println!("{}", predict);
   // println!("{}", confusion_matrix.accuracy());

    Ok(1.0)
}

/// Applies multinomial logistic regression to the provided data and dependent variable.
// pub fn apply_multinomial_logistic_regression(df: DataFrame, independent: &str) -> Result<f32, Box<dyn std::error::Error>> {
//     unimplemented!()
// }


#[cfg(test)]
mod test {
    use polars::prelude::col;
    use crate::frames::get_all_patient_info_raw;
    use super::*;
    
    #[test]
    fn applies_binomial_logistic_regression() {
        let df = get_all_patient_info_raw(false)
            .select([
                col("ADHD"),
                col("BIPOLAR")
            ])
            .collect()
            .unwrap();
        
        let result = apply_binomial_logistic_regression(df, "BIPOLAR");
        assert!(Result::is_ok(&result))
    }
}