//! Our API

// NOTE: Re-export the types to allow internal refactoring later.
pub use self::convertible::Convertible;
pub use self::error::Error;
pub use self::error::GpioError;
pub use self::peripheral_on_bus::{PeripheralOnBus, PeripheralsOnBus};
#[cfg(feature = "render")]
pub use self::render::{render, Context};
pub use self::validatable::Validatable;

pub mod convertible;
pub mod error;
pub mod peripheral_on_bus;
#[cfg(feature = "render")]
pub mod render;
pub mod validatable;
