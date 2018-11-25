//! Our Error definition module

use std::{error::Error as StdError, fmt::{Display, Formatter, Result as FmtResult}};

/// Our public errors
#[derive(Debug)]
pub enum Error {
    UnknownFlashSize(String),
    UnknownTemperatureRange(String),
}

impl Display for Error {
    fn fmt(&self, f: &mut Formatter) -> FmtResult {
        use self::Error::*;

        match *self {
            UnknownFlashSize(ref code) => write!(f, "flash size code '{}' unknown", code),
            UnknownTemperatureRange(ref code) => write!(f, "temperature range code '{}' unknown", code),
        }
    }
}

impl StdError for Error { }
