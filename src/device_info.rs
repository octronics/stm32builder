//! Device informations

use crate::{
    api::Convertible,
    device::DeviceIn,
    types::{DeviceId, FlashSize, Package, TemperatureRange},
};
use serde_derive::Deserialize;

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

#[cfg(test)]
mod tests {
    use super::*;

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
}
