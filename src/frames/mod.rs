use polars::prelude::{LazyFrame};
use crate::frames::hyperaktiv::load_patient_info;
use crate::traits::{PatientInfoFilter, PatientInfoTranslation};

/// This module exposes the raw hyperaktiv dataset
mod hyperaktiv;

/// Contains frames to display for demographic info
pub mod demographics;

/// Contains frames to display for mental health info.
pub  mod mental_health;

pub fn get_all_patient_info_clean(with_controls: bool) -> LazyFrame {
    load_patient_info(with_controls)
        .with_adhd_type_translation()
        .with_gender_translation()
        .with_age_range_translation()
}

/// Returns full, untranslated, unfiltered Hyperaktiv patient info data 
pub fn get_all_patient_info_raw(with_controls: bool) -> LazyFrame {
    load_patient_info(with_controls)
}