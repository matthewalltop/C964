pub mod queries {
    use serde::{Deserialize};

    #[derive(Deserialize)]
    pub struct ApiParameters<T> {
        pub(crate) category: Option<String>, // Requested Data - subtypes, demographics, mental-health, medication
        pub(crate) display: Option<String>, // Grid or Graph/Plot.
        pub(crate) sub_query: Option<T> // Subquery of one of the below types.
    }

    impl<T> ApiParameters<T> {
        pub fn default() ->  ApiParameters<T> {
            ApiParameters {
                category: None,
                display: None,
                sub_query: None
            }
        }
        
        pub fn new(category: Option<String>, display: Option<String>, sub_query: Option<T>) -> ApiParameters<T> {
            ApiParameters {
                category,
                display,
                sub_query
            }
        }
    }
    
    #[derive(Deserialize)]
    pub struct SubtypeParams {
        pub(crate) adhd_subtype: Option<String>,
        pub(crate) gender: Option<String>,
    }

    impl SubtypeParams {
        pub fn default() -> SubtypeParams {
            SubtypeParams {
                adhd_subtype: None,
                gender: None
            }
        }
    }
    
    #[derive(Deserialize)]
    pub struct DemographicsParams {
        pub(crate) gender: Option<String>,
        pub(crate) age_range: Option<String>,
    }

    impl DemographicsParams {
        pub fn default() -> DemographicsParams {
            DemographicsParams {
                gender: None,
                age_range: None
            }
        }
    }

    #[derive(Deserialize)]
    pub struct MentalHealthParams {
        pub(crate) conditions: Option<Vec<String>>
    }

    impl MentalHealthParams {
        pub fn default() -> MentalHealthParams {
            MentalHealthParams {
                conditions: None
            }
        }
    }

    #[derive(Deserialize)]
    pub struct MedicationParams {
        pub(crate) medications: Option<Vec<String>>
    }

    impl MedicationParams {
        pub fn default() -> MedicationParams {
            MedicationParams {
                medications: None
            }
        }
    }
}

