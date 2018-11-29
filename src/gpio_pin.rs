//! Gpio peripherals

use crate::{
    api::Convertible,
    api::{Error, GpioError},
    device::DeviceIn,
    types::DeviceId,
};
use serde_derive::{Deserialize, Serialize};

/// A gpio pin (from device file).
#[derive(Debug, Deserialize)]
pub struct GpioPinIn {
    /// The gpio pin name (aka PA0).
    pub name: String,
    /// The gpio pin initial mode.
    pub initial_mode: Option<String>,
}

/// A gpio pin (to template).
#[allow(non_snake_case)]
#[derive(Debug, Serialize)]
pub struct GpioPinOut {
    /// The gpio pin type (aka PA0).
    pub PIN: String,
    /// The gpio pin name (aka pa0).
    pub pin: String,
    /// The gpio pin number (aka 0).
    pub n: u8,
    /// The gpio initial mode (default to Input<Floating>)
    pub initial_mode: String,
}

impl Convertible for GpioPinIn {
    type Output = GpioPinOut;

    /// Convert to outputable gpio pin.
    // FIXME add Result to to_output api to express invalid pin number.
    // FIXME Take the initial mode from default defined in GpioIn or GpioBankI
    fn to_output(&self, _id: &DeviceId, _device: &DeviceIn) -> GpioPinOut {
        GpioPinOut {
            PIN: self.name.clone().to_uppercase(),
            pin: self.name.clone().to_lowercase(),
            n: self.pin_number().unwrap(),
            initial_mode: self
                .initial_mode
                .clone()
                .unwrap_or("Input<Floating>".to_string()),
        }
    }
}

impl GpioPinIn {
    fn pin_number(&self) -> Result<u8, Error> {
        let pin = self
            .name
            .get(2..=2)
            .ok_or_else(|| Error::Gpio(GpioError::NoPinNumber(self.name.to_owned())))?;
        pin.parse::<u8>()
            .map_err(|_| Error::Gpio(GpioError::InvalidPinNumber(pin.to_owned())))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    fn pin_under_test() -> GpioPinOut {
        GpioPinIn {
            name: "pa0".to_owned(),
            initial_mode: None,
        }
        .to_output(&valid_device_id(), &valid_device_in())
    }

    #[test]
    fn warn_without_pin_number() {
        let pin = GpioPinIn {
            name: "pa".to_owned(),
            initial_mode: None,
        };
        assert!(pin.pin_number().is_err());
    }
    #[test]
    fn warn_on_invalid_number() {
        let pin = GpioPinIn {
            name: "paa".to_owned(),
            initial_mode: None,
        };
        assert!(pin.pin_number().is_err());
    }
    #[test]
    fn get_pin_number() {
        let pin = GpioPinIn {
            name: "pa0".to_owned(),
            initial_mode: None,
        };
        assert_eq!(pin.pin_number().unwrap(), 0);
    }
    #[test]
    fn has_a_pin_type() {
        assert_eq!(pin_under_test().PIN, "PA0".to_owned());
    }
    #[test]
    fn has_a_pin_name() {
        assert_eq!(pin_under_test().pin, "pa0".to_owned());
    }
    #[test]
    fn has_a_pin_number() {
        assert_eq!(pin_under_test().n, 0);
    }
    #[test]
    fn has_a_default_initial_mode() {
        assert_eq!(pin_under_test().initial_mode, "Input<Floating>".to_owned());
    }
    #[test]
    fn has_a_initial_mode() {
        let pin = GpioPinIn {
            name: "PA3".to_owned(),
            initial_mode: Some("MyMode".to_owned()),
        }
        .to_output(&valid_device_id(), &valid_device_in());

        assert_eq!(pin.initial_mode, "MyMode");
    }
}
