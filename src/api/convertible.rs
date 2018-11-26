//! Convertible API module

use crate::{device::DeviceIn, types::DeviceId};

/// Trait to convert an input object (from a device file) to an output object (to template).
pub trait Convertible {
    /// Type of the outputed object
    type Output;

    /// Convert to an outputable object.
    ///
    /// Use the `id` parameter to match against the current device identification number and the
    /// `device` parameter to match against the device's file input.
    fn to_output(&self, id: &DeviceId, device: &DeviceIn) -> Self::Output;
}
