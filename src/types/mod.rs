//! Common types module

// NOTE: Re-export our types for convenience.
pub use self::device_id::DeviceId;
pub use self::flash_size::FlashSize;
pub use self::others::Part;
pub use self::others::RamSize;
pub use self::package::Package;
pub use self::temperature_range::TemperatureRange;

pub mod device_id;
pub mod flash_size;
pub mod others;
pub mod package;
pub mod temperature_range;
