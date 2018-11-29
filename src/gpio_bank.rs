//! Gpio bank input and output types

use crate::{
    api::Convertible,
    device::DeviceIn,
    gpio_pin::{GpioPinIn, GpioPinOut},
    types::DeviceId,
};
use serde::de::{Deserialize, Deserializer};
use serde_derive::{Deserialize, Serialize};

/// A gpio bank (from device file).
#[derive(Debug, Deserialize)]
pub struct GpioBankIn {
    /// The gpio bank name.
    pub name: String,
    /// The gpio bank pins.
    #[serde(deserialize_with = "seq_of_pins_as_string_or_struct")]
    pub pins: Vec<GpioPinIn>,
}

/// A gpio bank (to template).
#[allow(non_snake_case)]
#[derive(Debug, PartialEq, Serialize)]
pub struct GpioBankOut {
    /// The gpio bank type (aka GPIOA).
    pub GPIO: String,
    /// The gpio bank name (aka gpioa).
    pub gpio: String,
    /// The gpio bank pins.
    pub pins: Vec<GpioPinOut>,
}

impl Convertible for GpioBankIn {
    type Output = GpioBankOut;

    /// Convert to outputable gpio bank.
    fn to_output(&self, id: &DeviceId, device: &DeviceIn) -> GpioBankOut {
        GpioBankOut {
            GPIO: self.name.clone().to_uppercase(),
            gpio: self.name.clone().to_lowercase(),
            pins: self
                .pins
                .iter()
                .map(|pin| pin.to_output(&id, &device))
                .collect(),
        }
    }
}

fn seq_of_pins_as_string_or_struct<'de, D>(deserializer: D) -> Result<Vec<GpioPinIn>, D::Error>
where
    D: Deserializer<'de>,
{
    #[derive(Deserialize)]
    struct Wrapper(
        #[serde(deserialize_with = "crate::helpers::serde::string_or_struct")] GpioPinIn,
    );

    let v = Vec::deserialize(deserializer)?;
    Ok(v.into_iter().map(|Wrapper(a)| a).collect())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    fn bank_under_test() -> GpioBankOut {
        GpioBankIn {
            name: "gpioa".to_owned(),
            pins: vec![
                GpioPinIn {
                    name: "pa0".to_owned(),
                    initial_mode: None,
                },
                GpioPinIn {
                    name: "pa1".to_owned(),
                    initial_mode: None,
                },
            ],
        }
        .to_output(&valid_device_id(), &valid_device_in())
    }

    #[test]
    fn has_a_type() {
        assert_eq!(bank_under_test().GPIO, "GPIOA");
    }
    #[test]
    fn has_a_name() {
        assert_eq!(bank_under_test().gpio, "gpioa");
    }
    #[test]
    fn has_some_pins() {
        let pins = bank_under_test().pins;
        let mut pins = pins.iter();
        assert_eq!(pins.next().unwrap().PIN, "PA0");
        assert_eq!(pins.next().unwrap().PIN, "PA1");
        assert!(pins.next().is_none());
    }
}
