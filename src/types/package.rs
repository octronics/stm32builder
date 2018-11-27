//! Device Package module

use crate::api::Error;
use serde_derive::{Deserialize, Serialize};

macro_rules! define_package {
    ( $($package:tt as $type:tt => { $( $pin:tt => $PACKAGE:tt, )+ }, )+ ) => {

        /// The possible device packaging
        /// Encoded as the pair of the 10th caracter (for the pin count) and the 11th character
        /// (for the package type) of the device identification number.
        #[derive(Debug, Clone, PartialEq, Deserialize, Serialize)]
        pub enum Package {
            $($( $PACKAGE, )+)+
        }

        impl Package {
            /// Decode the package from the pin count code and the package type code. Returns
            /// `UnknownPinCount` or `UnknownPackageType` error in case of failure to parse the codes.
            pub fn from_pin_and_package_str(pin: &str, package: &str) -> Result<Package, Error> {
                match package {
                    $( $package => match pin {
                        $( $pin => Ok(Package::$PACKAGE), )+
                        code => Err(Error::UnknownPinCount(code.to_string())),
                    } )+
                    code => Err(Error::UnknownPackageType(code.to_string())),
                }
            }
        }

        #[cfg(test)]
        #[allow(non_snake_case)]
        mod packages {
            use super::*;
            $($( #[test]
            fn $PACKAGE() { assert_eq!(Package::from_pin_and_package_str($pin, $package).unwrap(), Package::$PACKAGE); }
            )+)+
        }
    };
}

// NOTE: We define the pair package / pin count as plain enum to catch the unavailable pair.
// Also when serialized, they will be represented as a more readable string.
define_package!(
    "P" as TSSOP =>  {
        "F" => TSSOP20,
    },
    "T" as LQFP => {
        "K" => LQFP32,
        "C" => LQFP48,
        "R" => LQFP64,
    },
    "H" as UFBGA => {
        "R" => UFBGA64,
    },
    "U" as UFQFPN => {
        "K" => UFQFPN32,
        "C" => UFQFPN48,
    },
    "Y" as WLCSP => {
        "T" => WLCSP36,
    },
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn error_on_unknown_package() {
        assert!(Package::from_pin_and_package_str("F", "_").is_err());
    }
    #[test]
    fn error_on_unknown_pin_count() {
        assert!(Package::from_pin_and_package_str("_", "T").is_err());
    }
}
