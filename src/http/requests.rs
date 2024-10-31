pub mod commands {
     
}

pub mod queries {
    use serde::{Deserialize};
    
    #[derive(Deserialize)]
    pub struct SubtypeParams {
        pub(crate) gender: Option<String>,
        pub(crate) adhd_subtype: Option<String>,
    }
    
    #[derive(Debug, Deserialize)]
    pub struct DemographicsParams {
        pub(crate) gender: Option<String>,
        pub(crate) age: Option<String>,
    }
}

