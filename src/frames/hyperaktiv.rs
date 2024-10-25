use polars::prelude::*;
use std::path::{Path, PathBuf};

/// Loads patient_info.csv data containing 
/// the demographic, physical, and psychological details for each patient.
pub fn load_patient_info(with_controls: bool) -> LazyFrame {
    let root_path = get_path(with_controls);
    load_frame_from_csv(root_path.join("patient_info.csv")).unwrap()
}

/// Loads Conner's Continuous Performance Test (CPT II) Data
/// Not every patient has test results, patients with test results can be identified
/// by the presence of a '1' value in the CPT_II column of the 'patient_info' data 
pub fn load_cpt_ii_data(with_controls: bool) -> LazyFrame {
    let root_path = get_path(with_controls);
    load_frame_from_csv(root_path.join("CPT_II_ConnersContinuousPerformanceTest.csv")).unwrap()
}

/// Loads features.csv data
pub fn load_features(with_controls: bool) -> LazyFrame {
    let root_path = get_path(with_controls);
    load_frame_from_csv(root_path.join("features.csv")).unwrap()
}

/// Loads heart-rate monitoring data for the given patient.
/// Note that this data is not present for every patient in the data
/// Patient's with this data available are denoted in the 'patient_info.csv' file
/// by the presence of a '1' in the HRV column.
pub fn load_patient_hrv_data(patient_id: i32, with_controls: bool) -> LazyFrame {
    // Note: There is not one of these for every patient - but that is established by the patient_info dataset
    let root_path = get_path(with_controls);
    let data_path = root_path.join("hrv_data").join(format!("patient_hr_{}.csv", patient_id));
    load_frame_from_csv(data_path).unwrap()
}

/// Loads patient activity data for the given patient id.
/// Note that this data is not present for every patient in the data
/// Patient's with this data available are denoted in the 'patient_info.csv' file
/// by the presence of a '1' in the ACC column.
pub fn load_patient_activity_data(patient_id: i32, with_controls: bool) -> LazyFrame {
    // Note: There is not one of these for every patient - but that is established by the patient_info dataset
    let root_path = get_path(with_controls);
    let data_path = root_path.join("activity_data").join(format!("patient_activity_{}.csv", patient_id));
    load_frame_from_csv(data_path).unwrap()
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


#[cfg(test)]
mod test {
    /// These could be considered integration tests & probably should be externalized
    use super::*;
    #[test]
    fn loads_cpt_ii_test_data() {
        let result = load_cpt_ii_data(false).collect().unwrap();
        assert!(!result.is_empty());
        println!("{}", result)
    }

    #[test]
    fn loads_features() {
        let result = load_features(false).collect().unwrap();
        assert!(!result.is_empty());
        println!("{}", result)
    }

    #[test]
    fn loads_patient_info() {
        let result = load_patient_info(false).collect().unwrap();
        assert!(!result.is_empty());
        println!("{}", result)
    }

    #[test]
    fn loads_patient_activity_data() {
        let existing_file_ids = vec![1, 2, 3, 5, 7, 8, 9];
        for patient_id in existing_file_ids {
            let result = load_patient_activity_data(patient_id, false).collect().unwrap();
            assert!(!result.is_empty());
            println!("{}", result)
        }
    }

    #[test]
    fn loads_patient_hrv_data() {
        let existing_file_ids = vec![1, 3, 4, 5, 7, 9];
        for patient_id in existing_file_ids {
            let result = load_patient_hrv_data(patient_id, false).collect().unwrap();
            assert!(!result.is_empty());
            println!("{}", result)
        }
    }
}