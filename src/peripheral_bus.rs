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
