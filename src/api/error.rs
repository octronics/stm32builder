//! Our Error definition module

use serde_yaml::Error as YamlError;
use std::{
    error::Error as StdError,
    fmt::{Display, Formatter, Result as FmtResult},
};

/// Our public errors
#[derive(Debug)]
pub enum Error {
    NoBrand,
    BadBrand,
    NoFamily,
    BadFamily,
    NoSubFamily,
    BadSubFamily,
    NoPinCount,
    BadPinCount,
    UnknownPinCount(String),
    NoFlashSize,
    BadFlashSize,
    UnknownFlashSize(String),
    NoPackageType,
    BadPackageType,
    UnknownPackageType(String),
    NoTemperatureRange,
    BadTemperatureRange,
    UnknownTemperatureRange(String),
    ParseError(YamlError),
    Gpio(GpioError),
    NoMatchFound,
}

#[derive(Debug)]
pub enum GpioError {
    NoPinNumber(String),
    InvalidPinNumber(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::Error::*;

        match *self {
            NoBrand => f.write_str("no brand found"),
            BadBrand => f.write_str("brand was not 'stm32'"),
            NoFamily => f.write_str("no family found"),
            BadFamily => f.write_str("family was not alphabetic"),
            NoSubFamily => f.write_str("no sub-family found"),
            BadSubFamily => f.write_str("sub-family was not digit"),
            NoPinCount => f.write_str("no pin count found"),
            BadPinCount => f.write_str("pin count was not alphabetic"),
            UnknownPinCount(ref code) => write!(f, "pin count code '{}' unknown", code),
            NoFlashSize => f.write_str("no flash size code"),
            BadFlashSize => f.write_str("flash size was not a digit"),
            UnknownFlashSize(ref code) => write!(f, "flash size code '{}' unknown", code),
            NoPackageType => f.write_str("no package type code found"),
            BadPackageType => f.write_str("package type was not a alphabetic"),
            UnknownPackageType(ref code) => write!(f, "package type code '{}' unknown", code),
            NoTemperatureRange => f.write_str("no temperature range code found"),
            BadTemperatureRange => f.write_str("temperature range was not a digit"),
            UnknownTemperatureRange(ref code) => {
                write!(f, "temperature range code '{}' unknown", code)
            }
            ParseError(ref yaml) => yaml.fmt(f),
            NoMatchFound => f.write_str("device id and device file does not match"),
            Gpio(ref error) => error.fmt(f),
        }
    }
}

impl StdError for Error {}

impl StdError for GpioError {}

impl Display for GpioError {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::GpioError::*;
        match *self {
            NoPinNumber(ref pin) => write!(f, "pin '{}' has no pin number", pin),
            InvalidPinNumber(ref pin) => write!(f, "pin number '{}' is not a number", pin),
        }
    }
}
