//! Rcc peripheral

use crate::{
    api::{Convertible, PeripheralsOnBus},
    device::DeviceIn,
    peripheral_bus::PeripheralBusOut,
    types::DeviceId,
};
use serde_derive::{Deserialize, Serialize};

/// The RCC peripheral (from device file).
#[derive(Debug, Deserialize)]
pub struct RccIn {}

/// The RCC peripheral (to template).
#[derive(Debug, PartialEq, Serialize)]
pub struct RccOut {
    pub buses: Vec<PeripheralBusOut>,
}

impl Convertible for RccIn {
    type Output = RccOut;

    /// Convert to outputable rcc peripheral.
    fn to_output(&self, id: &DeviceId, device: &DeviceIn) -> RccOut {
        let mut buses = Vec::new();
        device.peripherals.gpio.as_ref().map(|gpio| {
            gpio.peripheral_buses(&id, &device)
                .into_iter()
                .for_each(|bus| buses.push(bus))
        });
        RccOut { buses: buses }
    }
}
