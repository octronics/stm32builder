//! Peripheral on rcc controlled bus.

use crate::{device::DeviceIn, peripheral_bus::PeripheralBusOut, types::DeviceId};

/// One peripheral with rcc controlled bus.
pub trait PeripheralOnBus {
    /// Represent the bus this peripheral is connected to.
    fn peripheral_bus(&self) -> PeripheralBusOut;
}

/// One peripheral with multiple rcc controlled buses.
pub trait PeripheralsOnBus {
    /// Returns all the rcc controlled buses this peripheral is connected to.
    fn peripheral_buses(&self, id: &DeviceId, device: &DeviceIn) -> Vec<PeripheralBusOut>;
}
