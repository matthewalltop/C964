mod frames;
mod experiments;

use polars::prelude::*;
use plotlars::{BarPlot, Plot, ScatterPlot};
use crate::experiments::comorbidity::comorbidity_of_anxiety_or_mood_disorder;
use crate::frames::hyperaktiv::*;

pub fn main() {
    comorbidity_of_anxiety_or_mood_disorder();
    
    
    
    // let data_set: DataFrame = load_patient_info(false).collect().unwrap();
    // 
    // BarPlot::builder()
    //     .data(&data_set)
    //     .labels("SEX")
    //     .values("AGE")
    //     .plot_title("Age and Sex of Participants")
    //     .x_title("Sex")
    //     .y_title("Age")
    //     .build()
    //     .plot()
}
