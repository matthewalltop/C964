use std::fmt;
use std::str::FromStr;
use serde::{Deserialize, Serialize};
use serde_with::{serde_as, NoneAsEmptyString};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum DisplayType {
    Plot = 0,
    Table = 1,
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
pub struct DemographicParams {
    #[serde_as(as = "NoneAsEmptyString")]
    pub(crate) display: Option<String>,
    pub(crate) sub_category: Option<String>,
    pub(crate) gender: Option<String>,
    pub(crate) with_controls: Option<bool>,
}

impl fmt::Display for DemographicParams {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}

#[serde_as]
#[derive(Debug, Deserialize)]
pub struct MentalHealthParams {
    #[serde_as(as = "NoneAsEmptyString")]
    pub(crate) display: Option<String>,
    pub(crate) category: Option<String>,
    pub(crate) with_controls: Option<bool>,
}

#[derive(Deserialize)]
pub struct PredictParams {
    pub(crate) condition: Option<String>,
    pub(crate) gender: Option<String>,
    pub(crate) age_ranges: Option<Vec<String>>,
    pub(crate) adhd_type: Option<String>,
    pub(crate) with_controls: Option<bool>,
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum DemographicCategory {
    None = 0,
    ADHDSubtypesByGender = 1,
    ADHDSubtypesByAgeGroup = 2,
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
    HasOther = 6,
    All = 7
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
            "all" => Ok(MentalHealthCategory::All),
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

#[cfg(test)]
mod test {
    use super::*;
    
    #[test]
    fn display_type_to_strings() {
        assert_eq!(DisplayType::Plot.to_string(), "Plot");
        assert_eq!(DisplayType::Table.to_string(), "Table");
    }

    #[test]
    fn display_type_from_strings() {
        assert_eq!(DisplayType::from_str(&"Plot").unwrap(), DisplayType::Plot);
        assert_eq!(DisplayType::from_str(&"Table").unwrap(), DisplayType::Table);
    }
}