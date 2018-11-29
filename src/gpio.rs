//! A gpio peripheral

use crate::{
    api::Convertible,
    device::DeviceIn,
    gpio_bank::{GpioBankIn, GpioBankOut},
    types::DeviceId,
};
use serde_derive::{Deserialize, Serialize};

/// A gpio peripheral (from device file).
#[derive(Debug, Deserialize)]
pub struct GpioIn {
    /// The gpio version
    pub version: u8,
    /// The gpio banks
    pub banks: Vec<GpioBankIn>,
}

/// A gpio peripheral (to template).
#[derive(Debug, Serialize)]
pub struct GpioOut {
    /// The gpio version
    pub version: u8,
    /// The gpio banks
    pub banks: Vec<GpioBankOut>,
}

impl Convertible for GpioIn {
    type Output = GpioOut;

    /// Convert to outputable gpio peripheral.
    fn to_output(&self, id: &DeviceId, device: &DeviceIn) -> GpioOut {
        GpioOut {
            version: self.version.clone(),
            banks: self
                .banks
                .iter()
                .map(|bank| bank.to_output(&id, &device))
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    fn gpio_under_test() -> GpioOut {
        GpioIn {
            version: 2,
            banks: vec![
                GpioBankIn {
                    name: "GPIOA".to_owned(),
                    pins: valid_gpio_pins(),
                },
                GpioBankIn {
                    name: "GPIOB".to_owned(),
                    pins: valid_gpio_pins(),
                },
            ],
        }
        .to_output(&valid_device_id(), &valid_device_in())
    }

    #[test]
    fn has_a_version_number() {
        assert_eq!(gpio_under_test().version, 2);
    }
    #[test]
    fn has_some_banks() {
        let banks = gpio_under_test().banks;
        let mut banks = banks.iter();

        assert_eq!(banks.next().unwrap().GPIO, "GPIOA");
        assert_eq!(banks.next().unwrap().GPIO, "GPIOB");
        assert!(banks.next().is_none());
    }
}