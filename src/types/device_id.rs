//! Device Identifiant

use crate::{
    api::Error,
    types::{FlashSize, Package, TemperatureRange},
};

/// The device identifiant number.
///
/// ```
/// use stm32builder::types::{DeviceId, Package};
/// let id = DeviceId::from_str("stm32f051R8T6").unwrap();
///
/// assert_eq!(id.id, "stm32f051R8T6");
/// assert_eq!(id.name(), "stm32f051");
/// assert_eq!(id.part(), "R8");
/// assert_eq!(id.package, Package::LQFP64);
/// ```
#[derive(Debug, Clone, PartialEq)]
pub struct DeviceId {
    pub id: String,
    pub package: Package,
    pub flash_size: FlashSize,
    pub temperature: TemperatureRange,
}

// NOTE: We need the `$function(id)` for our own use and `$method(&self)` for use by the user.
// We use `get()` on function to prevent panicking not present characters.
// We `unwrap()` on methods as has already validate the id, so we know it can't fail.
macro_rules! decode {
    ( $method:ident (on $pos:tt position) as $function:ident ) =>  {
        decode!($method (from $pos to $pos characters) as $function);
    };
    ( $method:ident (from $start:tt to $end:tt characters) as  $function:ident ) => {
        pub fn $method(&self) -> &str {
            Self::$function(&self.id).unwrap()
        }
        fn $function(id: &str) -> Option<&str> {
            id.get($start..=$end)
        }
    };
}

impl DeviceId {
    const BRAND: &'static str = "stm32";

    // NOTE: Remenber indice starts at zero.
    decode!( header (from 0 to 4 characters) as header_from_str );
    decode!( family (on 5 position) as family_from_str );
    decode!( sub_family (from 6 to 8 characters) as sub_family_from_str );
    decode!( name (from 0 to 8 characters) as name_from_str );
    decode!( part (from 9 to 10 characters) as part_from_str );
    decode!( pin_count (on 9 position) as pin_count_from_str );
    decode!( flash_size (on 10 position) as flash_size_from_str );
    decode!( package_type (on 11 position) as package_type_from_str );
    decode!( temperature_range (on 12 position) as temperature_range_from_str );

    /// Decode the device idendification number.
    ///
    /// ## Returns
    ///
    /// One of `NoBrand`, `BadBrand`, `NoFamily`, `BadFamily`, `NoSubFamily`, `BadSubFamily`,
    /// `NoPinCount`, `BadPinCount`, `UnknownPinCount`, `NoFlashSize`, `BadFlashSize`,
    /// `UnknownFlashSize`, `NoPackageType`, `BadPackageType`, `UnknownPackageType`,
    /// `NoTemperatureRange`, `BadTemperatureRange`, `UnknownTemperatureRange` if case of error.
    ///
    pub fn from_str(id: &str) -> Result<Self, Error> {
        let is_ascii_digit = |c: u8| c.is_ascii_digit();
        let is_ascii_alphabetic = |c: u8| c.is_ascii_alphabetic();

        if Self::header_from_str(id).ok_or(Error::NoBrand)? != Self::BRAND {
            return Err(Error::BadBrand);
        }
        if !Self::family_from_str(id)
            .ok_or(Error::NoFamily)?
            .bytes()
            .all(is_ascii_alphabetic)
        {
            return Err(Error::BadFamily);
        }
        if !Self::sub_family_from_str(id)
            .ok_or(Error::NoSubFamily)?
            .bytes()
            .all(is_ascii_digit)
        {
            return Err(Error::BadSubFamily);
        }
        let pin_count = Self::pin_count_from_str(id).ok_or(Error::NoPinCount)?;
        if !pin_count.bytes().all(is_ascii_alphabetic) {
            return Err(Error::BadPinCount);
        }
        let flash_size = Self::flash_size_from_str(id).ok_or(Error::NoFlashSize)?;
        if !flash_size.bytes().all(is_ascii_digit) {
            return Err(Error::BadFlashSize);
        }
        let package_type = Self::package_type_from_str(id).ok_or(Error::NoPackageType)?;
        if !package_type.bytes().all(is_ascii_alphabetic) {
            return Err(Error::BadPackageType);
        }
        let temperature_range =
            Self::temperature_range_from_str(id).ok_or(Error::NoTemperatureRange)?;
        if !temperature_range.bytes().all(is_ascii_digit) {
            return Err(Error::BadTemperatureRange);
        }
        Ok(DeviceId {
            id: id.to_string(),
            package: Package::from_pin_and_package_str(pin_count, package_type)?,
            flash_size: FlashSize::from_flash_size_str(flash_size)?,
            temperature: TemperatureRange::from_temperature_range_str(temperature_range)?,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(non_snake_case)]
    fn decoding_stm32f051R8T6() {
        let id = DeviceId::from_str("stm32f051R8T6").unwrap();

        assert_eq!(id.header(), "stm32");
        assert_eq!(id.name(), "stm32f051");
        assert_eq!(id.family(), "f");
        assert_eq!(id.sub_family(), "051");
        assert_eq!(id.part(), "R8");
        assert_eq!(id.pin_count(), "R");
        assert_eq!(id.flash_size(), "8");
        assert_eq!(id.package_type(), "T");
        assert_eq!(id.temperature_range(), "6");
        assert_eq!(id.package, Package::LQFP64);
        assert_eq!(id.flash_size, FlashSize(64));
        assert_eq!(id.temperature.min, -40);
        assert_eq!(id.temperature.max, 85);
    }

    // NOTE: The match against error is a hack around the fact that Error must implement PartialEq
    // for assert to work on but as we want to embed serde_yaml errors on it and this one doesn't
    // implement PartialEq, we can't write `assert_eq!(..., Error::$error);` assertion.
    macro_rules! decoding_fail {
        ( $str:tt $error:ident $name:ident) => {
            #[test]
            pub fn $name() {
                match DeviceId::from_str($str).err().unwrap() {
                    Error::$error => {}
                    _ => assert!(false, "expected {} error", stringify!($error)),
                }
            }
        };
        ( $str:tt $error:ident($with:tt) $name:ident ) => {
            #[test]
            pub fn $name() {
                match DeviceId::from_str($str).err().unwrap() {
                    Error::$error(value) => assert_eq!(value, $with.to_string()),
                    _ => assert!(false, "expected {} error", stringify!($error)),
                }
            }
        };
    }
    decoding_fail!( ""              NoBrand                        error_on_empty_str );
    decoding_fail!( "stm"           NoBrand                        error_on_incomplet_brand ); // Should be BadBrand
    decoding_fail!( "st_32f051R8T6" BadBrand                       error_on_invalid_brand );
    decoding_fail!( "stm32"         NoFamily                       error_without_family );
    decoding_fail!( "stm32_051R8T6" BadFamily                      error_on_digit_family );
    decoding_fail!( "stm32f"        NoSubFamily                    error_without_sub_family );
    decoding_fail!( "stm32f_51R8T6" BadSubFamily                   error_on_alphabetical_sub_family );
    decoding_fail!( "stm32f051"     NoPinCount                     error_without_pin_count );
    decoding_fail!( "stm32f05108T6" BadPinCount                    error_on_digit_pin_count );
    decoding_fail!( "stm32f051O8T6" UnknownPinCount("O")           error_on_unknown_pin_count );
    decoding_fail!( "stm32f051R"    NoFlashSize                    error_without_flash_size );
    decoding_fail!( "stm32f051RAT6" BadFlashSize                   error_on_alphabetical_flash_size );
    decoding_fail!( "stm32f051R0T6" UnknownFlashSize("0")          error_on_unknown_flash_size );
    decoding_fail!( "stm32f051R8"   NoPackageType                  error_without_package_type );
    decoding_fail!( "stm32f051R886" BadPackageType                 error_on_digit_package_type );
    decoding_fail!( "stm32f051R8O6" UnknownPackageType("O")        error_on_unknown_package_type );
    decoding_fail!( "stm32f051R8T"  NoTemperatureRange             error_without_temperature_range );
    decoding_fail!( "stm32f051R8TT" BadTemperatureRange            error_on_alphabetic_temperature_range );
    decoding_fail!( "stm32f051R8T0" UnknownTemperatureRange("0")   error_on_unknown_temperature_range );

    #[test]
    fn can_be_compared_against() {
        assert_eq!(
            DeviceId::from_str("stm32f051R8T6").unwrap(),
            DeviceId::from_str("stm32f051R8T6").unwrap()
        );
    }
}
