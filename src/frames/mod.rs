pub mod hyperaktiv;
pub  mod patient_info;
mod subtypes;

pub mod enums {
    use std::fmt;
    
    #[derive(Debug)]
    pub enum Gender {
        Female = 0,
        Male = 1
    }

    impl fmt::Display for Gender {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(self, f)
        }
    }

    #[derive(Debug)]
    pub enum Age {
        SeventeenToTwentyNine = 1,
        ThirtyToThirtyNine = 2,
        FortyToFortyNine = 3,
        FiftyToSixtySeven = 4
    }

    impl fmt::Display for Age {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            fmt::Debug::fmt(self, f)
        }
    }

    #[cfg(test)]
    mod test {
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
    }

}