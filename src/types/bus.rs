//! Peripheral bus type

use serde_derive::{Deserialize, Serialize};

/// Peripheral bus
#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Bus {
    AHB,
    APB1,
    APB2,
}
