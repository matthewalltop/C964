pub mod commands {
     
}

pub mod queries {
    use serde::{Deserialize, Serialize};
    use crate::frames::enums::{Age, Gender};
    
    #[derive(Debug, Serialize, Deserialize)]
    pub struct Demographics {
        gender: Gender,
        age: Age,
    }
}

