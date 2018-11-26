//! Device module

use crate::{
    api::{Convertible, Error},
    types::DeviceId,
};
use serde_derive::Deserialize;
use std::fs::File;

/// The input device.
#[derive(Debug, Deserialize)]
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

/// An stm32 device.
// NOTE: It is our user facing entry point.
pub struct Device;
impl Device {
    /// Get a device from its device file (if the id match the `name` key on the device).
    pub fn from_id_and_file(id: &DeviceId, device: &File) -> Result<DeviceOut, Error> {
        // First deserialize the device file.
        let device: DeviceIn = serde_yaml::from_reader(device).map_err(|e| Error::ParseError(e))?;

        // Then check if the two match
        if id.name() != device.name {
            return Err(Error::NoMatchFound);
        }

        // Finaly convert to output device.
        let device = device.to_output(&id, &device);

        Ok(device)
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

    #[test]
    fn created_from_id_and_file() {
        let device = File::open("tests/stm32f051.yaml").unwrap();
        let id = DeviceId::from_str("stm32f051R8T6").unwrap();

        let device = Device::from_id_and_file(&id, &device).unwrap();

        assert_eq!(device.id, id);
        assert_eq!(device.name, "stm32f051");
    }
}
