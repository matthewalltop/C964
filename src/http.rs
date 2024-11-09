pub mod requests {
    use std::fmt;
    use std::str::FromStr;
    use serde::{Deserialize, Deserializer, Serialize};
    use serde_with::{serde_as, NoneAsEmptyString};

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub enum DisplayType {
        Plot = 0,
        Table = 1
    }

    impl fmt::Display for DisplayType {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(self, f)
        }
    }

    impl FromStr for DisplayType {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_lowercase().as_str() {
                "plot" => Ok(DisplayType::Plot),
                "table" => Ok(DisplayType::Table),
                _ => Ok(DisplayType::Plot)
            }
        }
    }
    #[serde_as]
    #[derive(Debug, Deserialize)]
    pub struct DemographicParams{
        #[serde_as(as = "NoneAsEmptyString")]
        pub(crate) display: Option<String>,
        pub(crate) sub_category: Option<String>,
        pub(crate) gender: Option<String>,
        pub(crate) with_controls: Option<bool>,
    }

    impl DemographicParams {
        pub fn default() ->  DemographicParams {
            DemographicParams {
                display: None,
                sub_category: None,
                gender: None,
                with_controls: None,
            }
        }
    }

    impl fmt::Display for DemographicParams {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            write!(f, "{:?}", self)
        }
    }

    #[serde_as]
    #[derive(Debug, Deserialize)]
    pub struct MentalHealthParams{
        #[serde_as(as = "NoneAsEmptyString")]
        pub(crate) display: Option<String>,
        pub(crate) category: Option<String>,
        pub(crate) with_controls: Option<bool>,
    }

    impl MentalHealthParams {
        pub fn default() -> MentalHealthParams {
            MentalHealthParams {
                display: None,
                category: None,
                with_controls: None,
            }
        }
    }

    #[derive(Debug, Deserialize)]
    pub struct PredictParams {
        pub(crate) gender: Option<String>
    }

    impl PredictParams {
        pub fn default() -> PredictParams {
            PredictParams {
                gender: None
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub enum DemographicCategory {
        None = 0,
        ADHDSubtypesByGender = 1,
        ADHDSubtypesByAgeGroup = 2
    }

    impl fmt::Display for DemographicCategory {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(self, f)
        }
    }

    impl FromStr for DemographicCategory {
        type Err = ();

        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_lowercase().as_str() {
                "adhdsubtypesbygender" => Ok(DemographicCategory::ADHDSubtypesByGender),
                "adhdsubtypesbyagegroup" => Ok(DemographicCategory::ADHDSubtypesByAgeGroup),
                _ => Ok(DemographicCategory::None)
            }
        }
    }

    #[derive(Debug, Serialize, Deserialize, PartialEq)]
    pub enum MentalHealthCategory {
        None = 0,
        HasCoMorbidMentalHealthCondition = 1,
        HasBipolarDisorder = 2,
        HasUnipolarDepression = 3,
        HasAnxiety = 4,
        HasSubstanceAbuseDisorder = 5,
        HasOther = 6
    }

    impl fmt::Display for MentalHealthCategory {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(self, f)
        }
    }

    impl FromStr for MentalHealthCategory {
        type Err = ();
    
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            match s.to_lowercase().as_str() {
                "hascomorbidmentalhealthcondition" => Ok(MentalHealthCategory::HasCoMorbidMentalHealthCondition),
                "hasbipolardisorder" => Ok(MentalHealthCategory::HasBipolarDisorder),
                "hasunipolardepression" => Ok(MentalHealthCategory::HasUnipolarDepression),
                "hasanxiety" => Ok(MentalHealthCategory::HasAnxiety),
                "hassubstanceabusedisorder" => Ok(MentalHealthCategory::HasSubstanceAbuseDisorder),
                "hasother" => Ok(MentalHealthCategory::HasOther),
                _ => Ok(MentalHealthCategory::None)
            }
        }
    }
}

pub mod responses {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PlotlyPlot {
        pub(crate) layout: String,
        pub(crate) traces: Vec<String>
    }
}


#[cfg(test)]
mod test {
    use crate::http::requests::DisplayType;
    use super::*;
    
    #[test]
    fn display_type_to_strings() {
        assert_eq!(DisplayType::Plot.to_string(), "Plot");
        assert_eq!(DisplayType::Table.to_string(), "Table");
    }

    #[test]
    fn display_type_from_strings() {
        
    }
    
    
}