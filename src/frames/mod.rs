use polars::prelude::*;
use std::path::{Path, PathBuf};

pub fn load_conners_test(with_controls: bool) -> DataFrame {
    let root_path = get_path(with_controls);
    load_frame_from_csv(root_path.join("CPT_II_ConnersContinuousPerformanceTest.csv")).unwrap()
}

pub fn load_features(with_controls: bool) -> DataFrame {
    let root_path = get_path(with_controls);
    load_frame_from_csv(root_path.join("features.csv")).unwrap()
}

pub fn load_patient_info(with_controls: bool) -> DataFrame {
    let root_path = get_path(with_controls);
    load_frame_from_csv(root_path.join("patient_info.csv")).unwrap()
}

/**
TODO: Both the HRV & PA methods should be using Option<DataFrame>
 We do not have one of these files for each patient - so will need to ensure error handling is present when they are being retrieved.
 If possible, it would be good to store these in memory or push them into a database for more rapid retrieval.
*/
pub fn load_patient_hrv_data(patient_id: i32, with_controls: bool) -> DataFrame {
    let root_path = get_path(with_controls);
    let data_path = root_path.join("hrv_data").join(format!("patient_hr_{}.csv", patient_id));
    load_frame_from_csv(data_path).unwrap()
}

pub fn load_patient_activity_data(patient_id: i32, with_controls: bool) -> DataFrame {
    let root_path = get_path(with_controls);
    let data_path = root_path.join("activity_data").join(format!("patient_activity_{}.csv", patient_id));
    load_frame_from_csv(data_path).unwrap()
}


fn load_frame_from_csv(path_buf: PathBuf) -> PolarsResult<DataFrame> {
    // Convert the delimiter to u8
    let dlm = ";".as_bytes().first().unwrap().to_ascii_lowercase();

    CsvReadOptions::default()
        .with_has_header(true)
        .with_parse_options(CsvParseOptions::default().with_separator(dlm))
        .try_into_reader_with_file_path(Some(path_buf))?
        .finish()
}

fn get_path<'a>(with_controls: bool) -> &'a Path {
    if with_controls {
        Path::new("hyperaktiv/hyperaktiv_with_controls")
    } else {
        Path::new("hyperaktiv")
    }
}


#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn loads_conners_test_data() {
        let result = load_conners_test(false);
        assert!(!result.is_empty());
        println!("{}", result)
    }

    #[test]
    fn loads_features() {
        let result = load_features(false);
        assert!(!result.is_empty());
        println!("{}", result)
    }

    #[test]
    fn loads_patient_info() {
        let result = load_patient_info(false);
        assert!(!result.is_empty());
        println!("{}", result)
    }

    #[test]
    fn loads_patient_activity_data() {
        let existing_file_ids = vec![1, 2, 3, 5, 7, 8, 9];
        for patient_id in existing_file_ids {
            let result = load_patient_activity_data(patient_id, false);
            assert!(!result.is_empty());
            println!("{}", result)
        }
    }

    #[test]
    fn loads_patient_hrv_data() {
        let existing_file_ids = vec![1, 3, 4, 5, 7, 9];
        for patient_id in existing_file_ids {
            let result = load_patient_hrv_data(patient_id, false);
            assert!(!result.is_empty());
            println!("{}", result)
        }
    }
}