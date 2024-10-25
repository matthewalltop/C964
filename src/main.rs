use std::error::Error;
use crate::plots::demographic::plot_by_adhd_type_with_gender_and_age;

// TODO: Remove this later.
// I'm building this in layers rather than as vertically sliced features so my methods are not being used right away.
#[allow(dead_code)]

mod frames;
mod experiments;
mod algo;
mod plots;

pub fn main() -> Result<(), Box<dyn Error>> {
    plot_by_adhd_type_with_gender_and_age().expect("Plots");
    Ok(())
}
/**

Design and develop your fully functional data product that addresses your identified business problem or organizational need from part A. Include each of the following attributes, as they are the minimum required elements for the product:

- [x]  collected or available datasets
- [x]  ability to support featurizing, parsing, cleaning, and wrangling datasets
- [x]  methods and algorithms supporting data exploration and preparation
- [x]   data visualization functionalities for data exploration and inspection
- [x]  implementation of machine-learning methods and algorithms
- [x]  functionalities to evaluate the accuracy of the data product
- []  one descriptive method and one non-descriptive (predictive or prescriptive) method
- []  decision support functionality
- []  implementation of interactive queries
- []  industry-appropriate security features
- []  tools to monitor and maintain the product
- []  a user-friendly, functional dashboard that includes three visualization types

*/



