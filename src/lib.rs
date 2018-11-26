//! This crate provides tools to help building stm32 micro-controller hardware abstraction layer.

// NOTE: Re-export our main API entry point for convenience.
pub use crate::types::DeviceId;

pub mod api;
pub mod types;
