//! Flash size type module

use crate::api::Error;
use serde_derive::Serialize;

/// Flash size a device can have
/// Encoded on the 11th character on the device identification number.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct FlashSize(pub u32);

impl FlashSize {
    /// Return flash size or `UnknownFlashSize` error.
    pub fn from_flash_size_str(code: &str) -> Result<Self, Error> {
        match code {
            "4" => Ok(FlashSize(16)),
            "6" => Ok(FlashSize(32)),
            "8" => Ok(FlashSize(64)),
            code => Err(Error::UnknownFlashSize(code.to_string())),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! decode {
        ( $name:tt, $code:tt, $size:tt) => {
            #[test]
            fn $name() {
                assert_eq!(
                    FlashSize::from_flash_size_str($code).unwrap(),
                    FlashSize($size)
                );
            }
        };
    }
    decode!(decode_4_as_16_kb, "4", 16);
    decode!(decode_6_as_32_kb, "6", 32);
    decode!(decode_8_as_64_kb, "8", 64);

    #[test]
    fn error_on_unknown_str() {
        assert!(FlashSize::from_flash_size_str("_").is_err());
    }
}
