use polars::prelude::*;
use std::path::{Path, PathBuf};

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