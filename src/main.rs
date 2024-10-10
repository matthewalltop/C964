use std::error::Error;
use crate::experiments::comorbidity::comorbidity_of_mental_health_condition;

// TODO: Remove this.
// Linting can be a bit annoying while development is still ongoing.
#[allow(dead_code)]

mod frames;
mod experiments;

pub fn main() -> Result<(), Box<dyn Error>> {
    let _experiment = comorbidity_of_mental_health_condition();
    Ok(())
}
