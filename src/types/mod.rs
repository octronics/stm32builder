//! Common types module

// NOTE: Re-export our types for convenience.
pub use self::package::Package;
pub use self::flash_size::FlashSize;
pub use self::temperature_range::TemperatureRange;

pub mod package;
pub mod flash_size;
pub mod temperature_range;
