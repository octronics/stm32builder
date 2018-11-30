//! Data used on unit tests

// Import all our types
use crate::{
    device::*, device_info::*, gpio::*, gpio_bank::*, gpio_pin::*, peripheral_bus::*,
    peripherals::*, types::*,
};

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
        peripherals: valid_peripherals(),
    }
}
pub fn another_valid_device_in() -> DeviceIn {
    DeviceIn {
        name: "stm32f051".to_owned(),
        info: valid_device_info_in(),
        parts: vec![another_valid_device_part_in()],
        peripherals: valid_peripherals(),
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

pub fn valid_peripherals() -> PeripheralsIn {
    PeripheralsIn {
        gpio: Some(valid_gpio()),
    }
}

pub fn expected_peripherals() -> PeripheralsOut {
    PeripheralsOut {
        gpio: Some(expected_gpio()),
    }
}

pub fn valid_gpio() -> GpioIn {
    GpioIn {
        version: 2,
        banks: valid_gpio_banks(),
    }
}
pub fn expected_gpio() -> GpioOut {
    GpioOut {
        version: 2,
        banks: expected_gpio_banks(),
    }
}

pub fn valid_gpio_banks() -> Vec<GpioBankIn> {
    vec![
        valid_gpio_bank(),
        GpioBankIn {
            name: "GPIOB".to_owned(),
            bus: PeripheralBusIn {
                name: Bus::AHB,
                field: "IOPB".to_owned(),
                resetable: true,
            },
            pins: vec![
                GpioPinIn {
                    name: "PB0".to_owned(),
                    initial_mode: None,
                    valid: valid_for_other(),
                },
                GpioPinIn {
                    name: "PB1".to_owned(),
                    initial_mode: None,
                    valid: valid_for_all(),
                },
            ],
            valid: Valid::default(),
        },
    ]
}
pub fn expected_gpio_banks() -> Vec<GpioBankOut> {
    vec![
        expected_gpio_bank(),
        GpioBankOut {
            GPIO: "GPIOB".to_owned(),
            gpio: "gpiob".to_owned(),
            pins: vec![GpioPinOut {
                PIN: "PB1".to_owned(),
                pin: "pb1".to_owned(),
                n: 1,
                initial_mode: "Input<Floating>".to_owned(),
            }],
        },
    ]
}

pub fn valid_gpio_bank() -> GpioBankIn {
    GpioBankIn {
        name: "GPIOA".to_owned(),
        pins: valid_gpio_pins(),
        valid: Valid::default(),
        bus: PeripheralBusIn {
            name: Bus::AHB,
            field: "IOPA".to_owned(),
            resetable: true,
        },
    }
}
pub fn expected_gpio_bank() -> GpioBankOut {
    GpioBankOut {
        GPIO: "GPIOA".to_owned(),
        gpio: "gpioa".to_owned(),
        pins: expected_gpio_pins(),
    }
}

pub fn valid_gpio_pins() -> Vec<GpioPinIn> {
    vec![
        GpioPinIn {
            name: "PA0".to_owned(),
            initial_mode: None,
            valid: valid_for_all(),
        },
        GpioPinIn {
            name: "PA1".to_owned(),
            initial_mode: Some("Output<PushPull>".to_owned()),
            valid: valid_for_other(),
        },
    ]
}
pub fn expected_gpio_pins() -> Vec<GpioPinOut> {
    vec![GpioPinOut {
        PIN: "PA0".to_owned(),
        pin: "pa0".to_owned(),
        n: 0,
        initial_mode: "Input<Floating>".to_owned(),
    }]
}

pub fn valid_for_all() -> Valid {
    Valid {
        only_on_packages: None,
        not_on_packages: None,
        only_on_parts: None,
        not_on_parts: None,
    }
}
pub fn valid_for_other() -> Valid {
    Valid {
        only_on_packages: None,
        not_on_packages: None,
        only_on_parts: None,
        not_on_parts: Some(vec!["R8".to_owned()]),
    }
}
