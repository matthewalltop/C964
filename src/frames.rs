use std::path::{Path, PathBuf};
use polars::prelude::{col, LazyCsvReader, LazyFileListReader, LazyFrame, PolarsResult};
use crate::enums::AdhdSubtype;
use crate::JsonResponse;
use crate::traits::{PatientInfoFilter, PatientInfoTranslation};

/// Returns full, untranslated, unfiltered Hyperaktiv patient info data 
pub fn get_all_patient_info_raw(with_controls: bool) -> LazyFrame {
    load_patient_info(with_controls)
}

/// Returns filtered data frame containing filtered ADHD Subtype info.
pub fn adhd_subtype_info(with_controls: bool) -> JsonResponse {
    let df = get_all_patient_info_raw(with_controls)
        .with_adhd(Some(AdhdSubtype::All))
        .with_gender_translation()
        .with_adhd_type_translation()
        .with_age_range_translation()
        .select([
            col("ID"),
            col("Gender"),
            col("ADHD Type"),
            col("Age Range")
        ]).collect()?;

    let result = serde_json::to_string(&df).unwrap();

    Ok(result)
}


/// Loads patient_info.csv data containing 
/// the demographic, physical, and psychological details for each patient.
pub fn load_patient_info(with_controls: bool) -> LazyFrame {
    let root_path = get_path(with_controls);
    load_frame_from_csv(root_path.join("patient_info.csv")).unwrap()
}

/// Opens the given CSV file into a LazyFrame
fn load_frame_from_csv(path_buf: PathBuf) -> PolarsResult<LazyFrame> {
    // Convert the delimiter to u8
    let dlm = ";".as_bytes().first().unwrap().to_ascii_lowercase();
    LazyCsvReader::new(path_buf).with_has_header(true).with_separator(dlm).finish()
}

/// Retrieves the correct path for data files based 
/// on whether the consumer wishes for controls or not.
fn get_path<'a>(with_controls: bool) -> &'a Path {
    if with_controls {
        Path::new("data/hyperaktiv/hyperaktiv_with_controls")
    } else {
        Path::new("data/hyperaktiv")
    }
}

/// Returns all patients who reported a mental health or substance abuse condition.
pub fn patients_with_comorbid_mental_health_conditions(with_controls: bool) -> JsonResponse {
    let df =  load_patient_info(with_controls)
        .with_age_range_translation()
        .with_adhd_type_translation()
        .with_gender_translation()
        .with_mental_health_translation()
        .select([
            col("ID"),
            col("Age Range"),
            col("Gender"),
            col("ADHD Type"),
            col("Mental Health Condition")
        ]).collect()?;

    let result = serde_json::to_string(&df)?;

    Ok(result)
}

#[cfg(test)]
mod test {
    /// These could be considered integration tests & probably should be externalized
    use super::*;

    #[test]
    fn loads_patient_info() {
        let result = load_patient_info(false).collect().unwrap();
        assert!(!result.is_empty());
        println!("{}", result)
    }
}



