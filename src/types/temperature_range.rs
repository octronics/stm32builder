//! Temperature type module

use crate::api::Error;

/// Temperature range allowed
/// Encoded on the 12th character of the device identification number.
#[derive(Debug, Clone, PartialEq)]
pub struct TemperatureRange {
    pub min: i16,
    pub max: i16,
}

impl TemperatureRange {
    /// Return a new temperature range from its encoding character or `UnknownTemperatureRange` on
    /// failure.
    pub fn from_temperature_range_str(code: &str) -> Result<Self, Error> {
        // NOTE: Please add a test when decoding a new code
        match code {
            "6" => Ok(Self { min: -40, max: 85 }),
            "7" => Ok(Self { min: -40, max: 105 }),
            code => Err(Error::UnknownTemperatureRange(code.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! decode { (
            $name:ident: $code:tt as $sign:tt$min:tt to $max:tt
        ) => {
            #[test]
            fn $name() {
                let temperature = TemperatureRange::from_temperature_range_str($code).unwrap();
                assert_eq!(temperature.min, $sign$min);
                assert_eq!(temperature.max, $max);
            }
        };
    }

    decode!(decode_6_as_m40_to_85: "6" as -40 to 85 );
    decode!(decode_7_as_m40_to_105: "7" as -40 to 105 );

    #[test]
    fn error_on_unknown_code() {
        assert!(TemperatureRange::from_temperature_range_str("_").is_err());
    }
}
