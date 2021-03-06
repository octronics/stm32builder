//! This crate provides tools to help building stm32 micro-controller hardware abstraction layer.

// NOTE: Re-export our main API entry point for convenience.
#[cfg(feature = "render")]
pub use crate::api::render::{render, Context};
pub use crate::device::Device;
pub use crate::types::DeviceId;

// Input and output
pub mod device;
pub mod device_info;
pub mod gpio;
pub mod gpio_bank;
pub mod gpio_pin;
pub mod peripheral_bus;
pub mod peripherals;
pub mod rcc;

// Internal use
pub mod api;
mod helpers;
pub mod types;

// Testing data
#[cfg(test)]
mod tests;
