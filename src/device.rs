//! Device module

use crate::{api::Convertible, types::DeviceId};

/// The input device.
#[derive(Debug)]
pub struct DeviceIn {
    /// The device name this device file is for.
    pub name: String,
}

/// The output device.
#[derive(Debug)]
pub struct DeviceOut {
    /// The device identifiant
    pub id: DeviceId,
    /// The device name (temporary).
    pub name: String,
}

impl Convertible for DeviceIn {
    type Output = DeviceOut;

    /// Convert to outputable device.
    fn to_output(&self, id: &DeviceId, _device: &DeviceIn) -> DeviceOut {
        DeviceOut {
            id: id.clone(),
            name: self.name.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn convert_to_output() {
        let device = DeviceIn {
            name: "stm32f051".to_string(),
        };
        let id = DeviceId::from_str("stm32f051R8T6").unwrap();

        let device = device.to_output(&id, &device);

        assert_eq!(device.id, id);
        assert_eq!(device.name, "stm32f051");
    }
}
