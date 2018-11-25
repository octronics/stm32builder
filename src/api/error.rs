//! Our Error definition module

use std::{error::Error as StdError, fmt::{Display, Formatter, Result as FmtResult}};

/// Our public errors
#[derive(Debug)]
pub enum Error {
    UnknownPinCount(String),
    UnknownFlashSize(String),
    UnknownPackageType(String),
    UnknownTemperatureRange(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::Error::*;

        match *self {
            UnknownPinCount(ref code) => write!(f, "pin count code '{}' unknown", code),
            UnknownFlashSize(ref code) => write!(f, "flash size code '{}' unknown", code),
            UnknownPackageType(ref code) => write!(f, "package type code '{}' unknown", code),
            UnknownTemperatureRange(ref code) => write!(f, "temperature range code '{}' unknown", code),
        }
    }
}

impl StdError for Error { }
