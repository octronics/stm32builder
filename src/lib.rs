//! This crate provides tools to help building stm32 micro-controller hardware abstraction layer.

// NOTE: Re-export our main API entry point for convenience.
pub use crate::device::Device;
pub use crate::types::DeviceId;

// Input and output
pub mod device;
pub mod device_info;

// Internal use
pub mod api;
pub mod types;
