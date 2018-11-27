//! Device informations

use crate::{
    api::{Convertible, Validatable},
    device::DeviceIn,
    types::{DeviceId, FlashSize, Package, Part, RamSize, TemperatureRange},
};
use serde_derive::{Deserialize, Serialize};

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
#[derive(Debug, PartialEq, Serialize)]
pub struct DeviceInfoOut {
    /// The device identifiant.
    pub id: DeviceId,
    /// The device part code.
    pub part: Part,
    /// The device hardware package.
    pub package: Package,
    /// The device amount of flash size.
    pub flash_size: FlashSize,
    /// The device amount of memory.
    pub ram_size: RamSize,
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
    // FIXME Add errors propagation on `to_output()` api.
    fn to_output(&self, id: &DeviceId, device: &DeviceIn) -> DeviceInfoOut {
        let part = find_part_matching_id(&id, &device).unwrap();
        DeviceInfoOut::from_info_and_part_and_id(self, part, &id)
    }
}

fn find_part_matching_id<'a>(id: &DeviceId, device: &'a DeviceIn) -> Option<&'a DevicePartIn> {
    device
        .parts
        .iter()
        .filter(|part| part.is_valid_for(&id, &device))
        .next()
}

impl DeviceInfoOut {
    /// Get an outputable device information from its input, the part and the device identifiant number.
    fn from_info_and_part_and_id(info: &DeviceInfoIn, part: &DevicePartIn, id: &DeviceId) -> Self {
        DeviceInfoOut {
            id: id.clone(),
            part: part.name.clone(),
            package: id.package.clone(),
            flash_size: id.flash_size.clone(),
            ram_size: part.ram.clone(),
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

    fn device_info_under_test() -> DeviceInfoOut {
        DeviceInfoOut::from_info_and_part_and_id(
            &valid_device_info_in(),
            &valid_device_part_in(),
            &valid_device_id(),
        )
    }

    #[test]
    fn device_info_out_has_correct_part() {
        assert_eq!(device_info_under_test().part, valid_part());
    }
    #[test]
    fn device_info_out_has_correct_ram_size() {
        assert_eq!(device_info_under_test().ram_size, valid_ram_size());
    }
    #[test]
    fn device_info_out_has_correct_package() {
        assert_eq!(
            device_info_under_test().package,
            crate::types::Package::LQFP64
        );
    }
    #[test]
    fn device_info_out_has_correct_flash_size() {
        assert_eq!(
            device_info_under_test().flash_size,
            crate::types::FlashSize(64)
        );
    }
    #[test]
    fn device_info_out_has_correct_temperature() {
        assert_eq!(
            device_info_under_test().temperature,
            crate::types::TemperatureRange { min: -40, max: 85 }
        );
    }
    #[test]
    fn device_info_out_has_correct_datasheet() {
        assert_eq!(
            device_info_under_test().datasheet,
            "https://somewhere.org/".to_owned()
        );
    }
    #[test]
    fn device_info_out_has_correct_reference() {
        assert_eq!(
            device_info_under_test().reference,
            "https://somewhereelse.org/".to_owned()
        );
    }
    #[test]
    fn device_info_out_has_correct_svd() {
        assert_eq!(device_info_under_test().svd, "stm32f0x1".to_owned());
    }

    #[test]
    fn device_part_is_valid_for_valid_device_id() {
        assert!(valid_device_part_in().is_valid_for(&valid_device_id(), &valid_device_in()));
        assert!(
            !valid_device_part_in().is_valid_for(&another_valid_device_id(), &valid_device_in())
        );
        assert!(valid_device_part_in().is_valid_for(&valid_device_id(), &another_valid_device_in()));
        assert!(!valid_device_part_in()
            .is_valid_for(&another_valid_device_id(), &another_valid_device_in()));
    }
}
