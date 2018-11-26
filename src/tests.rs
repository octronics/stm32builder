//! Data used on unit tests

// Import all our types
use crate::{device::*, device_info::*, types::*};

pub fn valid_device_id() -> DeviceId {
    DeviceId::from_str("stm32f051R8T6").unwrap()
}
pub fn another_valid_device_id() -> DeviceId {
    DeviceId::from_str("stm32f051C8T6").unwrap()
}

pub fn valid_device_in() -> DeviceIn {
    DeviceIn {
        name: "stm32f051".to_owned(),
        info: valid_device_info_in(),
        parts: vec![valid_device_part_in(), another_valid_device_part_in()],
    }
}
pub fn another_valid_device_in() -> DeviceIn {
    DeviceIn {
        name: "stm32f051".to_owned(),
        info: valid_device_info_in(),
        parts: vec![another_valid_device_part_in()],
    }
}

pub fn valid_device_info_in() -> DeviceInfoIn {
    DeviceInfoIn {
        datasheet: "https://somewhere.org/".to_owned(),
        reference: "https://somewhereelse.org/".to_owned(),
        svd: "stm32f0x1".to_owned(),
    }
}

pub fn valid_part() -> Part {
    Part("R8".to_owned())
}

pub fn valid_ram_size() -> RamSize {
    RamSize(8)
}

pub fn valid_package() -> Package {
    Package::LQFP64
}

pub fn valid_device_part_in() -> DevicePartIn {
    DevicePartIn {
        name: valid_part(),
        ram: valid_ram_size(),
        packages: vec![valid_package(), Package::UFBGA64],
    }
}

pub fn another_valid_device_part_in() -> DevicePartIn {
    DevicePartIn {
        name: Part("C8".to_owned()),
        ram: RamSize(8),
        packages: vec![Package::LQFP48, Package::UFQFPN48],
    }
}

pub fn expected_device_info_out() -> DeviceInfoOut {
    DeviceInfoOut {
        id: valid_device_id(),
        part: valid_part(),
        package: valid_device_id().package,
        flash_size: valid_device_id().flash_size,
        ram_size: valid_ram_size(),
        temperature: valid_device_id().temperature,
        datasheet: valid_device_info_in().datasheet,
        reference: valid_device_info_in().reference,
        svd: valid_device_info_in().svd,
    }
}
