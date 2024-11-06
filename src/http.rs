pub mod requests {
    use std::fmt;
    use std::str::FromStr;
    use serde::{Deserialize, Serialize};
    use crate::enums::Gender;

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
    #[derive(Deserialize)]
    pub struct DemographicParams{
        pub(crate) display: Option<DisplayType>,
        pub(crate) category: Option<DemographicCategory>,
        pub(crate) gender: Option<Gender>,
        pub(crate) with_controls: Option<bool>,
    }

    impl DemographicParams {
        pub fn default() ->  DemographicParams {
            DemographicParams {
                display: None,
                category: None,
                gender: None,
                with_controls: None,
            }
        }
    }

    #[derive(Deserialize)]
    pub struct MentalHealthParams{
        pub(crate) display: Option<DisplayType>,
        pub(crate) category: Option<MentalHealthCategory>,
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

    #[derive(Deserialize)]
    pub struct PredictParams {
        pub(crate) demographic: Option<String>
    }

    impl PredictParams {
        pub fn default() -> PredictParams {
            PredictParams {
                demographic: None
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
                // TODO
                _ => Ok(MentalHealthCategory::None)
            }
        }
    }
}

pub mod responses {
    use serde::{Deserialize, Serialize};

    #[derive(Debug, Serialize, Deserialize)]
    pub struct PlotlyGraph {
        layout: String,
        data: Vec<String>
    }

    impl PlotlyGraph {
        pub(crate) fn new(layout: String, traces: Vec<String>) -> PlotlyGraph {
            Self {
                layout,
                data: traces
            }
        }
    }
}



#[cfg(test)]
mod test {


}
