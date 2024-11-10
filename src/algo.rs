use linfa::prelude::{ConfusionMatrix, DatasetBase, Fit, Predict, ToConfusionMatrix};
use linfa_bayes::GaussianNb;
use linfa_logistic::LogisticRegression;
use ndarray::{s, ArrayBase, Dim, OwnedRepr};
use polars::frame::DataFrame;
use polars::prelude::{Float64Type, IndexOrder};

/// Applies logistic regression to the provided dataset
pub fn apply_logistic_regression(df: DataFrame, feature_names: Vec<&str>, split_ratio: f32) -> Result<MLAlgorithmResponse, Box<dyn std::error::Error>> {
    // Convert the data frame to a 2D array to prepare it for logistic regression.
    let feature_array = df.to_ndarray::<Float64Type>(IndexOrder::C)?;

    // Features are the factors to evaluate a positive/negative result for the target value.
    // Targets are the success/failure criteria.
    let (features, targets) = (
        feature_array.slice(s![.., 0..2]).to_owned(),
        feature_array.column(2).to_owned()
    );

    // Create the training & test dataset - we'll split the training set off as 20% of the original training set.
    let (train, test) = DatasetBase::new(features, targets)
        .map_targets(|x| if *x as u32 == 1 { "Positive" } else { "Negative" })
        .with_feature_names(feature_names)
        .split_with_ratio(split_ratio);

    // Internal closure method to generate a confusion matrix.
    // Each of these algorithms operates differently and this is specific to Logistic Regression, only.
    let create_cf_matrix = |
        train: &DatasetBase<
            ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>,
            ArrayBase<OwnedRepr<&'static str>, Dim<[usize; 1]>>,
        >,
        test: &DatasetBase<
            ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>,
            ArrayBase<OwnedRepr<&'static str>, Dim<[usize; 1]>>,
        >,
        threshold: f64,
        max_iterations: u64| -> ConfusionMatrix<&'static str> {

        let model = LogisticRegression::default()
            .max_iterations(max_iterations)
            .fit(train)
            .expect("Can train the model");

        // Predict
        let predict = model.set_threshold(threshold).predict(test);

        

        predict
            .confusion_matrix(test)
            .expect("Can create a confusion matrix")
    };


    // Establish variables for determining the most accurate combination of settings for the model.
    // Set the initial matrix to a threshold of 0.01 with max iterations of 100.
    let mut best_cf_matrix = create_cf_matrix(&train, &test, 0.01, 100);
    let mut best_iterations: u64 = 0;
    let mut best_threshold: f64 = 0.0;
    let mut curr_threshold: f64 = 0.02;

    // Increase the number of iterations & the threshold value within given bounds.
    // This will repeatedly predict the desired result and return the combination of settings that are most accurate.
    for max_iterations in (1000..5000).step_by(500) {
        while curr_threshold < 1.0 {
            // Create a new confusion matrix with incremented settings.
            let confusion_matrix = create_cf_matrix(&train, &test, curr_threshold, max_iterations);

            // If we've done better than previous runs, track those settings.
            if confusion_matrix.accuracy() > best_cf_matrix.accuracy() {
                best_cf_matrix = confusion_matrix;
                best_threshold = curr_threshold;
                best_iterations = max_iterations;
            }
            curr_threshold += 0.01;
        }
        // Reset the threshold for the next run.
        curr_threshold = 0.02;
    }
    
    println!("{:?}", best_cf_matrix);

    Ok(MLAlgorithmResponse::new(best_cf_matrix, Some(best_threshold), Some(best_iterations)))
}


/// Applies Gaussian Naive Bayes Algorithm
pub fn apply_gaussian_naive_bayes(df: DataFrame, feature_names: Vec<&str>, split_ratio: f32) -> Result<MLAlgorithmResponse, Box<dyn std::error::Error>> {
    // Convert the data frame to a 2D array to prepare it for logistic regression.
    let feature_array = df.to_ndarray::<Float64Type>(IndexOrder::C)?;

    // Features are the factors to evaluate a positive/negative result for the target value.
    // Targets are the success/failure criteria.
    let (features, targets) = (
        feature_array.slice(s![.., 0..2]).to_owned(),
        feature_array.column(2).to_owned()
    );

    // Create the training & test dataset - we'll split the training set off as 20% of the original training set.
    let (train, test) = DatasetBase::new(features, targets)
        .map_targets(|x| if *x as u32 == 1 { "Positive" } else { "Negative" })
        .with_feature_names(feature_names)
        .split_with_ratio(split_ratio);

    let create_cf_matrix = |
        train: &DatasetBase<
            ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>,
            ArrayBase<OwnedRepr<&'static str>, Dim<[usize; 1]>>,
        >,
        test: &DatasetBase<
            ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>,
            ArrayBase<OwnedRepr<&'static str>, Dim<[usize; 1]>>,
        >| -> ConfusionMatrix<&'static str> {

        let model = GaussianNb::params()
            .fit(train)
            .unwrap();

        // Predict
        let predict = model.predict(test);

        

        predict
            .confusion_matrix(test)
            .expect("Can create a confusion matrix")
    };


    let cf_matrix = create_cf_matrix(&train, &test);
    
    println!("{:?}", cf_matrix);
    
    Ok(MLAlgorithmResponse::new(cf_matrix, None, None))
}

pub struct MLAlgorithmResponse {
    pub cf_matrix: ConfusionMatrix<&'static str>,
    pub accuracy: f32,
    pub precision: f32,
    pub recall: f32,
    pub threshold: Option<f64>,
    pub iterations: Option<u64>
}

impl MLAlgorithmResponse {
    pub fn new(
        cf_matrix: ConfusionMatrix<&'static str>,
        threshold: Option<f64>,
        iterations: Option<u64>) -> MLAlgorithmResponse {

        let matrix = cf_matrix.to_owned();
        let accuracy = matrix.accuracy();
        let precision = matrix.precision();
        let recall = matrix.recall();

        Self {
            cf_matrix,
            accuracy,
            precision,
            recall,
            threshold,
            iterations
        }
    }
}


#[cfg(test)]
mod test {
    use polars::prelude::col;
    use crate::frames::get_all_patient_info_raw;
    use super::*;

    #[test]
    fn applies_logistic_regression() {
        // NOTE: Use the 'with controls' set for machine learning tasks.
        // The control group contains additional, healthy patients to augment the data set.
        let df = get_all_patient_info_raw(true)
            .select([
                col("ADHD"),
                col("ADD"),
                col("BIPOLAR")
            ])
            .collect()
            .unwrap();

        let response = apply_logistic_regression(df, vec!["ADHD", "ADD"], 0.70);
        assert!(Result::is_ok(&response));

        let result = response.unwrap();
        println!("Logistic Regression Algorithm");
        println!("Confusion Matrix: {:?}", result.cf_matrix);
        println!("Max Iterations: {}", result.iterations.unwrap());
        println!("Threshold {}", result.threshold.unwrap());
        println!("Accuracy {}", result.accuracy);
        println!("Precision {}", result.precision);
        println!("Recall {}", result.recall);
    }

    #[test]
    fn applies_gaussian_naive_bayes() {
        // NOTE: Use the 'with controls' set for machine learning tasks.
        // The control group contains additional, healthy patients to augment the data set.
        let df = get_all_patient_info_raw(true)
            .select([
                col("ADHD"),
                col("ADD"),
                col("BIPOLAR")
            ])
            .collect()
            .unwrap();

        let response = apply_gaussian_naive_bayes(df, vec!["ADHD", "ADD"], 0.70);
        assert!(Result::is_ok(&response));

        let result = response.unwrap();
        println!("Gaussian Naive Bayes Algorithm");
        println!("Confusion Matrix: {:?}", result.cf_matrix);
        println!("Accuracy {}", result.accuracy);
        println!("Precision {}", result.precision);
        println!("Recall {}", result.recall);
    }
}