//! Device peripherals

use crate::{
    api::Convertible,
    device::DeviceIn,
    gpio::{GpioIn, GpioOut},
    types::DeviceId,
};
use serde_derive::{Deserialize, Serialize};

/// The device peripherals (from device file).
#[derive(Debug, Deserialize)]
pub struct PeripheralsIn {
    /// The gpio peripheral.
    pub gpio: Option<GpioIn>,
}

/// The device peripherals (to template).
#[derive(Debug, PartialEq, Serialize)]
pub struct PeripheralsOut {
    /// The gpio peripheral.
    pub gpio: Option<GpioOut>,
}

impl Convertible for PeripheralsIn {
    type Output = PeripheralsOut;

    /// Convert to outputable device peripherals
    fn to_output(&self, id: &DeviceId, device: &DeviceIn) -> PeripheralsOut {
        PeripheralsOut {
            gpio: self.gpio.as_ref().map(|gpio| gpio.to_output(&id, &device)),
        }
    }
}
