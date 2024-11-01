pub mod queries {
    use serde::{Deserialize};
    
    #[derive(Deserialize)]
    pub struct ApiParameters<T> {
        pub(crate) category: Option<String>, // Requested Data - subtypes, demographics, mental-health, medication
        pub(crate) display: Option<String>, // Grid or Graph/Plot.
        pub(crate) sub_query: Option<T> // Subquery of one of the below types.
    }
    
    #[derive(Deserialize)]
    pub struct SubtypeParams {
        pub(crate) adhd_subtype: Option<String>,
        pub(crate) gender: Option<String>,
    }
    
    #[derive(Deserialize)]
    pub struct DemographicsParams {
        pub(crate) gender: Option<String>,
        pub(crate) age_range: Option<String>,
    }

    #[derive(Deserialize)]
    pub struct MentalHealthParams {
        pub(crate) conditions: Option<Vec<String>>
    }

    #[derive(Deserialize)]
    pub struct MedicationParams {
        pub(crate) medications: Option<Vec<String>>
    }
}

