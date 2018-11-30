//! Peripheral bus

use crate::types::Bus;
use serde_derive::{Deserialize, Serialize};

/// A peripheral bus (from device file).
#[derive(Debug, Deserialize)]
pub struct PeripheralBusIn {
    /// Peripheral bus name (aka AHB).
    pub name: Bus,
    /// Peripheral field on the bus (aka IOPA).
    pub field: String,
    /// Is this peripheral resetable on the bus? (default to true)
    #[serde(default = "crate::helpers::serde::default_true")]
    pub resetable: bool,
}

/// A peripheral on a bus (to template)
/// NOTE: Only to be used on the RCC peripheral.
#[derive(Debug, PartialEq, Serialize)]
pub struct PeripheralBusOut {
    /// Peripheral name (aka gpioa).
    pub peripheral: String,
    /// Peripheral bus name (aka AHB).
    pub bus: Bus,
    /// Peripheral field on the bus (aka IOPA).
    pub field: String,
    /// Is this peripheral resetable on the bus?
    pub resetable: bool,
}

// NOTE: Convertible trait is not implemented for PeripheralBusIn as this struct is placed on
// sub-peripheral struct and collected by the rcc using the `PeripheralsOnBus` and the
// `PeripheralOnBus` api traits.

#[cfg(test)]
mod tests {
    use super::*;
    use crate::api::PeripheralOnBus;
    use crate::tests::*;
    use crate::types::Bus;

    fn peripheral_bus_under_test() -> PeripheralBusOut {
        valid_gpio_bank().peripheral_bus()
    }

    #[test]
    fn has_a_peripheral() {
        assert_eq!(peripheral_bus_under_test().peripheral, "gpioa");
    }
    #[test]
    fn has_a_bus() {
        assert_eq!(peripheral_bus_under_test().bus, Bus::AHB);
    }
    #[test]
    fn has_a_field() {
        assert_eq!(peripheral_bus_under_test().field, "IOPA");
    }
    #[test]
    fn can_be_resetable() {
        assert_eq!(peripheral_bus_under_test().resetable, true);
    }
}
