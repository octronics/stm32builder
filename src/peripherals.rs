//! Device peripherals

use crate::{
    api::Convertible,
    device::DeviceIn,
    gpio::{GpioIn, GpioOut},
    rcc::{RccIn, RccOut},
    types::DeviceId,
};
use serde_derive::{Deserialize, Serialize};

/// The device peripherals (from device file).
#[derive(Debug, Deserialize)]
pub struct PeripheralsIn {
    /// The rcc peripheral.
    pub rcc: Option<RccIn>,
    /// The gpio peripheral.
    pub gpio: Option<GpioIn>,
}

/// The device peripherals (to template).
// NOTE The stm32hal build.rs script expect the optional API to conditionnaly render templates.
#[derive(Debug, PartialEq, Serialize)]
pub struct PeripheralsOut {
    /// The rcc peripheral.
    pub rcc: Option<RccOut>,
    /// The gpio peripheral.
    pub gpio: Option<GpioOut>,
}

impl Convertible for PeripheralsIn {
    type Output = PeripheralsOut;

    /// Convert to outputable device peripherals
    fn to_output(&self, id: &DeviceId, device: &DeviceIn) -> PeripheralsOut {
        PeripheralsOut {
            rcc: self.rcc.as_ref().map(|rcc| rcc.to_output(&id, &device)),
            gpio: self.gpio.as_ref().map(|gpio| gpio.to_output(&id, &device)),
        }
    }
}
