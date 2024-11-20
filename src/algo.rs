use std::error::Error;
use linfa::Dataset;
use linfa::prelude::{ConfusionMatrix, DatasetBase, Fit, Predict, ToConfusionMatrix};
use linfa_bayes::GaussianNb;
use linfa_logistic::LogisticRegression;
use linfa_trees::{DecisionTree, SplitQuality};
use ndarray::{s, ArrayBase, Dim, Ix1, OwnedRepr};
use polars::frame::DataFrame;
use polars::prelude::{Float64Type, IndexOrder};
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, NoneAsEmptyString};
use crate::predict::MLResponse;

type MLDataset = DatasetBase<
    ArrayBase<OwnedRepr<f64>, Dim<[usize; 2]>>,
    ArrayBase<OwnedRepr<&'static str>, Dim<[usize; 1]>>
>;

#[serde_as]
#[derive(Debug, Serialize, Deserialize)]
pub struct MLAlgorithmResponse {
    pub cf_matrix: String,
    pub accuracy: f32,
    pub precision: f32,
    pub recall: f32,
    pub threshold: Option<f64>,
    pub iterations: Option<u64>,
    #[serde_as(as = "NoneAsEmptyString")]
    pub tikz: Option<String>
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
            cf_matrix: format!("{:?}", matrix),
            accuracy,
            precision,
            recall,
            threshold,
            iterations,
            tikz: None
        }
    }
    
    pub fn with_tikz(&mut self, tikz: &str) -> Self {
        Self {
            cf_matrix: self.cf_matrix.to_owned(),
            accuracy: self.accuracy,
            precision: self.precision,
            recall: self.recall,
            threshold: self.threshold,
            iterations: self.iterations,
            tikz: Some(tikz.to_string())
        }
    }
}

/// Applies logistic regression to the provided dataset
pub fn apply_logistic_regression(df: DataFrame, feature_names: Vec<&'static str>, split_ratio: f32) -> MLResponse {

    let (train, test) = create_datasets(df, feature_names, split_ratio)?;

    // Internal closure method to generate a confusion matrix.
    // Each of these algorithms operates differently and this is specific to Logistic Regression, only.
    let create_cf_matrix = |train: &MLDataset,
                            test: &MLDataset,
                            threshold: f64,
                            max_iterations: u64| -> ConfusionMatrix<&'static str> {

        // Fit the logistic regression model to the training dataset.
        let model = LogisticRegression::default()
            .max_iterations(max_iterations)
            .fit(train)
            .expect("Can train the model");

        // Predict
        let predict = model.set_threshold(threshold).predict(test);

        // Validate
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


    Ok(MLAlgorithmResponse::new(best_cf_matrix, Some(best_threshold), Some(best_iterations)))
}


/// Applies Gaussian Naive Bayes Algorithm
pub fn apply_gaussian_naive_bayes(df: DataFrame, feature_names: Vec<&'static str>, split_ratio: f32) -> MLResponse {

    let (train, test) = create_datasets(df, feature_names, split_ratio)?;
    
    let create_cf_matrix = |train: &MLDataset, test: &MLDataset| -> ConfusionMatrix<&'static str> {
        // Fit the GNB model to the training dataset.
        let model = GaussianNb::params()
            .fit(train)
            .unwrap();

        // Predict
        let predict = model.predict(test);

        // Validate
        predict
            .confusion_matrix(test)
            .expect("Can create a confusion matrix")
    };


    let cf_matrix = create_cf_matrix(&train, &test);

    Ok(MLAlgorithmResponse::new(cf_matrix, None, None))
}

type DecisionTreeConfusionMatrix<'a> = (DecisionTree<f64, &'a str>, ConfusionMatrix<&'static str>);

pub fn apply_decision_tree(df: DataFrame, feature_names: Vec<&'static str>, split_ratio: f32) -> MLResponse {

    let (train, test) = create_datasets(df, feature_names, split_ratio)?;
    
    // Internal closure method to generate a confusion matrix.
    // Each of these algorithms operates differently and this is specific to Logistic Regression, only.
    let create_cf_matrix = |train: &MLDataset,
                            test: &MLDataset,
                            max_depth: usize,
                            min_weight_split: f32,
                            min_weight_leaf: f32| ->  DecisionTreeConfusionMatrix {

        // Initialize the decision tree & fit the data.
        let model = DecisionTree::params()
            .split_quality(SplitQuality::Gini)
            .max_depth(Some(max_depth))
            .min_weight_split(min_weight_split)
            .min_weight_leaf(min_weight_leaf)
            .fit(train)
            .expect("Can train the model");

        // Predict
        let predict = model.predict(test);

        // Validate
        let cf_matrix = predict
            .confusion_matrix(test)
            .expect("Can create a confusion matrix");

        (model, cf_matrix)
    };

    // Establish variables for determining the most accurate combination of settings for the model.
    // Set the initial matrix to a threshold of 0.01 with max iterations of 100.
    let mut best_cf_matrix = create_cf_matrix(&train, &test, 100, 1.0, 1.0);
    let mut best_depth: usize = 0;
    let mut best_threshold: f64 = 0.0;
    let mut curr_threshold: f64 = 0.02;

    // Increase the number of iterations & the threshold value within given bounds.
    // This will repeatedly predict the desired result and return the combination of settings that are most accurate.
    for max_depth in (150..2000).step_by(150) {
        while curr_threshold < 1.0 {
            // Create a new confusion matrix with incremented settings.
            let confusion_matrix = create_cf_matrix(&train, &test, max_depth, 2.0, 1.0);

            // If we've done better than previous runs, track those settings.
            if confusion_matrix.1.accuracy() > best_cf_matrix.1.accuracy() {
                best_cf_matrix = confusion_matrix;
                best_threshold = curr_threshold;
                best_depth = max_depth;
            }
            curr_threshold += 0.01;
        }
        // Reset the threshold for the next run.
        curr_threshold = 0.02;
    }

    
    let response = MLAlgorithmResponse::new(best_cf_matrix.1, Some(best_threshold), Some(best_depth as u64))
        .with_tikz(&best_cf_matrix.0
            .export_to_tikz()
            .with_legend()
            .complete(true)
            .to_string());
    
    Ok(response)
}

type PredictionDatasets<'a> = Result<(Dataset<f64, &'a str, Ix1>, Dataset<f64, &'a str, Ix1>), Box<dyn Error>>;

fn create_datasets(data_frame: DataFrame, feature_names: Vec<&str>, split_ratio: f32) -> PredictionDatasets {
    let feature_len = data_frame.columns(feature_names.to_owned()) ?.len();
    // Convert the data frame to a 2D array to prepare it for logistic regression.
    let feature_array = data_frame.to_ndarray::< Float64Type > (IndexOrder::C) ?;
    
    // Features are the factors to evaluate a positive/negative result for the target value.
    // Targets are the success/failure criteria.
    let (features, targets) = (
    feature_array.slice(s ![.., 0..feature_len]).to_owned(),
    feature_array.column(feature_len).to_owned()
    );
    
    // Create the training & test dataset - we'll split the training set off as 20% of the original training set.
    let (train, test) = DatasetBase::new(features, targets)
    .map_targets( | x | if *x as u32 == 1 { "Positive" } else { "Negative" })
    .with_feature_names(feature_names)
    .split_with_ratio(split_ratio);
    
    Ok((train, test))
}

#[cfg(test)]
mod test {
    use polars::prelude::{col, lit, when};
    use crate::enums::AdhdSubtype;
    use crate::frames::get_all_patient_info_raw;
    use crate::traits::PatientInfoFilter;
    use super::*;

    #[test]
    fn applies_logistic_regression() {
        let df = get_all_patient_info_raw(true)
            .with_column(
                when(
                    col("BIPOLAR").eq(1).or(
                        col("UNIPOLAR").eq(1).or(
                            col("ANXIETY").eq(1)).or(
                            col("SUBSTANCE").eq(1).or(
                                col("OTHER").eq(1))
                        ))).then(lit(1)).otherwise(lit(0)).alias("MENTAL_ILLNESS")
            )
            .select([
                col("ADHD"),
                col("ADD"),
                col("MENTAL_ILLNESS")
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
        let df = get_all_patient_info_raw(true)
            .select([
                col("SEX"),
                col("AGE"),
                col("ADHD"),
                col("ADD"),
                col("BIPOLAR")
            ])
            .collect()
            .unwrap();

        let response = apply_gaussian_naive_bayes(df, vec!["SEX", "AGE", "ADHD", "ADD"], 0.70);
        assert!(Result::is_ok(&response));

        let result = response.unwrap();
        
        println!("Gaussian Naive Bayes Algorithm");
        println!("Confusion Matrix: {:?}", result.cf_matrix);
        println!("Accuracy {}", result.accuracy);
        println!("Precision {}", result.precision);
        println!("Recall {}", result.recall);
    }

    #[test]
    fn applies_decision_tree() {
        let df = get_all_patient_info_raw(true)
            .with_adhd(Some(AdhdSubtype::All))
            .with_column(
                when(
                    col("BIPOLAR").eq(1).or(
                        col("UNIPOLAR").eq(1).or(
                            col("ANXIETY").eq(1)).or(
                            col("SUBSTANCE").eq(1).or(
                                col("OTHER").eq(1))
                        ))).then(lit(1)).otherwise(lit(0)).alias("MENTAL_ILLNESS")
            )
            .select([
                col("SEX"),
                col("AGE"),
                col("ADHD"),
                col("MENTAL_ILLNESS")
            ])
            .collect()
            .unwrap();

        let response = apply_decision_tree(df, vec!["SEX", "AGE", "ADHD", "ADD"], 0.70);
        assert!(Result::is_ok(&response));

        let result = response.unwrap();
        
        println!("Decision Tree Algorithm");
        println!("Confusion Matrix: {:?}", result.cf_matrix);
        println!("Accuracy {}", result.accuracy);
        println!("Precision {}", result.precision);
        println!("Recall {}", result.recall);
    }
}