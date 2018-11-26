//! Validatable API

use crate::{device::DeviceIn, types::DeviceId};

/// Trait to validate an object around a given device
pub trait Validatable {
    /// Is this object valid for the given device?
    fn is_valid_for(&self, id: &DeviceId, device: &DeviceIn) -> bool;
}
