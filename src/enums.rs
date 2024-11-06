use std::fmt;
use std::str::FromStr;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Gender {
    Female = 0,
    Male = 1,
    None = 2, // This is breaking convention because the data set cleanly aligns with '0' = Female & '1' = Male.
}
impl fmt::Display for Gender {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl FromStr for Gender {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "female" => Ok(Gender::Female),
            "male" => Ok(Gender::Male),
            _ => Ok(Gender::None)
        }
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum AdhdSubtype {
    None = 0,
    PrimaryHyperactive = 1,
    PrimaryInattentive = 2,
    All = 3,
}
impl fmt::Display for AdhdSubtype {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl FromStr for AdhdSubtype {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "adhd-ph" => Ok(AdhdSubtype::PrimaryHyperactive),
            "adhd-pi" => Ok(AdhdSubtype::PrimaryInattentive),
            "ph" => Ok(AdhdSubtype::PrimaryHyperactive),
            "pi" => Ok(AdhdSubtype::PrimaryInattentive),
            "all" => Ok(AdhdSubtype::All),
            _ => Ok(AdhdSubtype::None)
        }
    }
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum Age {
    None = 0,
    SeventeenToTwentyNine = 1,
    ThirtyToThirtyNine = 2,
    FortyToFortyNine = 3,
    FiftyToSixtySeven = 4,
}

impl fmt::Display for Age {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl FromStr for Age {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "seventeentotwentynine" => Ok(Age::SeventeenToTwentyNine),
            "thirtytothirtynine" => Ok(Age::ThirtyToThirtyNine),
            "fortytofortynine" => Ok(Age::FortyToFortyNine),
            "fiftytosixtyseven" => Ok(Age::FiftyToSixtySeven),
            _ => Ok(Age::None)
        }
    }
}


#[derive(Debug, Serialize, Deserialize, PartialEq)]
pub enum MentalHealthCondition {
    None = 0,
    BipolarDisorder = 1,
    UnipolarDepression = 2,
    AnxietyDisorder = 3,
    SubstanceAbuseDisorder = 4,
    Other = 5,
}

impl fmt::Display for MentalHealthCondition {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Debug::fmt(self, f)
    }
}

impl FromStr for MentalHealthCondition {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.to_lowercase().as_str() {
            "bipolardisorder" => Ok(MentalHealthCondition::BipolarDisorder),
            "unipolardepression" => Ok(MentalHealthCondition::UnipolarDepression),
            "anxietydisorder" => Ok(MentalHealthCondition::AnxietyDisorder),
            "substanceabusedisorder" => Ok(MentalHealthCondition::SubstanceAbuseDisorder),
            "other" => Ok(MentalHealthCondition::Other),
            _ => Ok(MentalHealthCondition::None)
        }
    }
}

#[cfg(test)]
mod test {
    use std::str::FromStr;
    use super::*;

    #[test]
    fn gender_parses_to_i32() {
        assert_eq!(Gender::Female as i32, 0);
        assert_eq!(Gender::Male as i32, 1);
    }

    #[test]
    fn gender_to_strings() {
        assert_eq!(Gender::Female.to_string(), "Female");
        assert_eq!(Gender::Male.to_string(), "Male");
    }

    #[test]
    fn gender_from_strings() {
        assert_eq!(Gender::from_str("This is clearly not one of the allowed values").unwrap(), Gender::None);

        assert_eq!(Gender::from_str("Female").unwrap(), Gender::Female);
        assert_eq!(Gender::from_str("Male").unwrap(), Gender::Male);
        assert_eq!(Gender::from_str("female").unwrap(), Gender::Female);
        assert_eq!(Gender::from_str("male").unwrap(), Gender::Male);
        assert_eq!(Gender::from_str("FeMAlE").unwrap(), Gender::Female);
        assert_eq!(Gender::from_str("mAlE").unwrap(), Gender::Male);
    }

    #[test]
    fn subtype_parses_to_i32() {
        assert_eq!(AdhdSubtype::PrimaryHyperactive as i32, 1);
        assert_eq!(AdhdSubtype::PrimaryInattentive as i32, 2);
    }

    #[test]
    fn subtype_to_strings() {
        assert_eq!(AdhdSubtype::PrimaryHyperactive.to_string(), "PrimaryHyperactive");
        assert_eq!(AdhdSubtype::PrimaryInattentive.to_string(), "PrimaryInattentive");
    }

    #[test]
    fn subtype_from_strings() {
        assert_eq!(AdhdSubtype::from_str("This is clearly not one of the allowed values").unwrap(), AdhdSubtype::None);

        assert_eq!(AdhdSubtype::from_str("adhd-ph").unwrap(), AdhdSubtype::PrimaryHyperactive);
        assert_eq!(AdhdSubtype::from_str("adhd-pi").unwrap(), AdhdSubtype::PrimaryInattentive);
        assert_eq!(AdhdSubtype::from_str("ph").unwrap(), AdhdSubtype::PrimaryHyperactive);
        assert_eq!(AdhdSubtype::from_str("pi").unwrap(), AdhdSubtype::PrimaryInattentive);
    }


    #[test]
    fn age_parses_to_i32() {
        assert_eq!(Age::SeventeenToTwentyNine as i32, 1);
        assert_eq!(Age::ThirtyToThirtyNine as i32, 2);
        assert_eq!(Age::FortyToFortyNine as i32, 3);
        assert_eq!(Age::FiftyToSixtySeven as i32, 4);
    }

    #[test]
    fn age_to_strings() {
        assert_eq!(Age::SeventeenToTwentyNine.to_string(), "SeventeenToTwentyNine");
        assert_eq!(Age::ThirtyToThirtyNine.to_string(), "ThirtyToThirtyNine");
        assert_eq!(Age::FortyToFortyNine.to_string(), "FortyToFortyNine");
        assert_eq!(Age::FiftyToSixtySeven.to_string(), "FiftyToSixtySeven");
    }

    #[test]
    fn age_from_strings() {
        assert_eq!(Age::from_str("This is clearly not one of the allowed values").unwrap(), Age::None);


        assert_eq!(Age::from_str("SeventeenToTwentyNine").unwrap(), Age::SeventeenToTwentyNine);
        assert_eq!(Age::from_str("ThirtyToThirtyNine").unwrap(), Age::ThirtyToThirtyNine);
        assert_eq!(Age::from_str("FortyToFortyNine").unwrap(), Age::FortyToFortyNine);
        assert_eq!(Age::from_str("FiftyToSixtySeven").unwrap(), Age::FiftyToSixtySeven);

        assert_eq!(Age::from_str("seventeentotwentynine").unwrap(), Age::SeventeenToTwentyNine);
        assert_eq!(Age::from_str("thirtytothirtynine").unwrap(), Age::ThirtyToThirtyNine);
        assert_eq!(Age::from_str("fortytofortynine").unwrap(), Age::FortyToFortyNine);
        assert_eq!(Age::from_str("fiftytosixtyseven").unwrap(), Age::FiftyToSixtySeven);

        assert_eq!(Age::from_str("SeventEenTOTwEnTyNiNe").unwrap(), Age::SeventeenToTwentyNine);
        assert_eq!(Age::from_str("ThIrtyToTHirTyNINe").unwrap(), Age::ThirtyToThirtyNine);
        assert_eq!(Age::from_str("FORTYTOFORTYNINE").unwrap(), Age::FortyToFortyNine);
        assert_eq!(Age::from_str("FiftyTosiXtyseVeN").unwrap(), Age::FiftyToSixtySeven);
    }

    #[test]
    fn mental_health_condition_parses_to_i32() {
        assert_eq!(MentalHealthCondition::BipolarDisorder as i32, 1);
        assert_eq!(MentalHealthCondition::UnipolarDepression as i32, 2);
        assert_eq!(MentalHealthCondition::AnxietyDisorder as i32, 3);
        assert_eq!(MentalHealthCondition::SubstanceAbuseDisorder as i32, 4);
        assert_eq!(MentalHealthCondition::Other as i32, 5);
    }

    #[test]
    fn mental_health_condition_to_strings() {
        assert_eq!(MentalHealthCondition::BipolarDisorder.to_string(), "BipolarDisorder");
        assert_eq!(MentalHealthCondition::UnipolarDepression.to_string(), "UnipolarDepression");
        assert_eq!(MentalHealthCondition::AnxietyDisorder.to_string(), "AnxietyDisorder");
        assert_eq!(MentalHealthCondition::SubstanceAbuseDisorder.to_string(), "SubstanceAbuseDisorder");
        assert_eq!(MentalHealthCondition::Other.to_string(), "Other");
    }

    #[test]
    fn mental_health_condition_from_strings() {
        assert_eq!(MentalHealthCondition::from_str("This is clearly not one of the allowed values").unwrap(), MentalHealthCondition::None);

        assert_eq!(MentalHealthCondition::from_str("BipolarDisorder").unwrap(), MentalHealthCondition::BipolarDisorder);
        assert_eq!(MentalHealthCondition::from_str("UnipolarDepression").unwrap(), MentalHealthCondition::UnipolarDepression);
        assert_eq!(MentalHealthCondition::from_str("AnxietyDisorder").unwrap(), MentalHealthCondition::AnxietyDisorder);
        assert_eq!(MentalHealthCondition::from_str("SubstanceAbuseDisorder").unwrap(), MentalHealthCondition::SubstanceAbuseDisorder);
        assert_eq!(MentalHealthCondition::from_str("Other").unwrap(), MentalHealthCondition::Other);
    }
}
