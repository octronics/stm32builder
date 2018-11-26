//! Device informations

use crate::{
    api::{Convertible, Validatable},
    device::DeviceIn,
    types::{DeviceId, FlashSize, Package, Part, RamSize, TemperatureRange},
};
use serde_derive::Deserialize;

/// One of the available parts under a device file.
#[derive(Debug, Deserialize)]
pub struct DevicePartIn {
    /// The name of this part.
    pub name: Part,
    /// The memory on this part.
    pub ram: RamSize,
    /// All the available packages for this part.
    pub packages: Vec<Package>,
}

/// Device information (from device file).
#[derive(Debug, Deserialize)]
pub struct DeviceInfoIn {
    /// The device datasheet url.
    pub datasheet: String,
    /// The device reference manual url.
    pub reference: String,
    /// The device svd name (without the trailing `.svd`).
    pub svd: String,
}

/// Device information (to template).
#[derive(Debug, PartialEq)]
pub struct DeviceInfoOut {
    /// The device identifiant.
    pub id: DeviceId,
    /// The device hardware package.
    pub package: Package,
    /// The device amount of flash size.
    pub flash_size: FlashSize,
    /// The device temperature range.
    pub temperature: TemperatureRange,
    /// The device datasheet url.
    pub datasheet: String,
    /// The device reference manual url.
    pub reference: String,
    /// The device svd name (without trailing `.svd`).
    pub svd: String,
}

impl Convertible for DeviceInfoIn {
    type Output = DeviceInfoOut;

    /// Convert to outputable device information.
    fn to_output(&self, id: &DeviceId, _device: &DeviceIn) -> DeviceInfoOut {
        DeviceInfoOut::from_info_and_id(self, &id)
    }
}

impl DeviceInfoOut {
    /// Get an outputable device information from its input and the device identifiant.
    fn from_info_and_id(info: &DeviceInfoIn, id: &DeviceId) -> Self {
        DeviceInfoOut {
            id: id.clone(),
            package: id.package.clone(),
            flash_size: id.flash_size.clone(),
            temperature: id.temperature.clone(),
            datasheet: info.datasheet.clone(),
            reference: info.reference.clone(),
            svd: info.svd.clone(),
        }
    }
}

impl Validatable for DevicePartIn {
    /// Valid if its name and its package match the ones encoded on the device id.
    fn is_valid_for(&self, id: &DeviceId, _device: &DeviceIn) -> bool {
        self.name == Part(id.part().to_owned()) && self.packages.contains(&id.package)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::tests::*;

    fn output() -> DeviceInfoOut {
        let id = DeviceId::from_str("stm32f051R8T6").unwrap();
        let info = DeviceInfoIn {
            datasheet: "https://somewhere.org/".to_owned(),
            reference: "https://somewhereelse.org/".to_owned(),
            svd: "stm32f0x1".to_owned(),
        };
        DeviceInfoOut::from_info_and_id(&info, &id)
    }
    #[test]
    fn output_a_package() {
        assert_eq!(output().package, crate::types::Package::LQFP64);
    }
    #[test]
    fn output_a_flash_size() {
        assert_eq!(output().flash_size, crate::types::FlashSize(64));
    }
    #[test]
    fn output_a_temperature_range() {
        assert_eq!(
            output().temperature,
            crate::types::TemperatureRange { min: -40, max: 85 }
        );
    }
    #[test]
    fn output_a_datasheet() {
        assert_eq!(output().datasheet, "https://somewhere.org/".to_owned());
    }
    #[test]
    fn output_a_reference() {
        assert_eq!(output().reference, "https://somewhereelse.org/".to_owned());
    }
    #[test]
    fn output_a_svd() {
        assert_eq!(output().svd, "stm32f0x1".to_owned());
    }

    #[test]
    fn part_is_valid_for_a_valid_device_id() {
        assert!(valid_device_part_in().is_valid_for(&valid_device_id(), &valid_device_in()));
        assert!(
            !valid_device_part_in().is_valid_for(&another_valid_device_id(), &valid_device_in())
        );
        assert!(valid_device_part_in().is_valid_for(&valid_device_id(), &another_valid_device_in()));
        assert!(!valid_device_part_in()
            .is_valid_for(&another_valid_device_id(), &another_valid_device_in()));
    }
}
