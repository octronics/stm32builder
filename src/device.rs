//! Device module

use crate::{
    api::{Convertible, Error},
    device_info::{DeviceInfoIn, DeviceInfoOut},
    types::DeviceId,
};
use serde_derive::Deserialize;
use std::fs::File;

/// The input device.
#[derive(Debug, Deserialize)]
pub struct DeviceIn {
    /// The device name this device file is for.
    pub name: String,
    /// The device common information.
    pub info: DeviceInfoIn,
}

/// The output device.
#[derive(Debug)]
pub struct DeviceOut {
    /// This device information.
    pub info: DeviceInfoOut,
}

impl Convertible for DeviceIn {
    type Output = DeviceOut;

    /// Convert to outputable device.
    fn to_output(&self, id: &DeviceId, device: &DeviceIn) -> DeviceOut {
        DeviceOut {
            info: self.info.to_output(&id, &device),
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
    use crate::tests::*;

    fn device_under_test() -> DeviceOut {
        valid_device_in().to_output(&valid_device_id(), &valid_device_in())
    }

    #[test]
    fn output_device_informations() {
        assert_eq!(device_under_test().info, expected_device_info_out())
    }

    #[test]
    fn created_from_id_and_file() {
        let device = File::open("tests/stm32f051.yaml").unwrap();
        let id = DeviceId::from_str("stm32f051R8T6").unwrap();

        let device = Device::from_id_and_file(&id, &device).unwrap();

        assert_eq!(device.info, expected_device_info_out());
    }
}
