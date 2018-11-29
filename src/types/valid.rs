//! Validatable API

use crate::{
    device::DeviceIn,
    types::{DeviceId, Package},
};
use serde_derive::Deserialize;

/// Validity matrix for an object
#[derive(Debug, Default, Deserialize)]
pub struct Valid {
    pub not_on_parts: Option<Vec<String>>,
    pub only_on_parts: Option<Vec<String>>,
    pub not_on_packages: Option<Vec<Package>>,
    pub only_on_packages: Option<Vec<Package>>,
}

impl Valid {
    /// Is an object valid on the given devices?
    // NOTE It use the same API the Validatable trait provides but we not implements it as the
    // Valid type is not useful by itself. Instead we call `is_valid_for()` on it parent type.
    pub fn is_valid_for(&self, id: &DeviceId, _device: &DeviceIn) -> bool {
        self.validate(&id.part().to_owned(), &id.package)
    }

    // NOTE Implemented as an helper to be able to test the logic without needing a device
    // identifiant nor a device.
    fn validate(&self, part: &String, package: &Package) -> bool {
        let validate_part = |parts: &Vec<String>| parts.contains(&part.to_string());
        let validate_package = |packages: &Vec<Package>| packages.contains(&package);

        !(!self.only_on_parts.as_ref().map_or(true, validate_part)
            || self.not_on_parts.as_ref().map_or(false, validate_part)
            || !self
                .only_on_packages
                .as_ref()
                .map_or(true, validate_package)
            || self
                .not_on_packages
                .as_ref()
                .map_or(false, validate_package))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::Package::{LQFP32, TSSOP20};

    macro_rules! test_api {
        ( @helper None ) => { None };
        ( @helper P1 ) => { Some(vec![LQFP32]) };
        ( @helper allPackages) => { Some(vec![LQFP32,TSSOP20]) };
        ( @helper D1) => { Some(vec!["dev1".to_string()]) };
        ( @helper allParts) => { Some(vec!["dev1".to_string(), "dev2".to_string()]) };
        ( $name:ident: { $only_on_packages:tt, $not_on_packages:tt, $only_on_parts:tt, $not_on_parts:tt }
          => ( $result1:tt, $result2:tt, $result3:tt, $result4:tt )
        ) => {
            #[test]
            fn $name() {
                let test = Valid {
                    only_on_packages: test_api!( @helper $only_on_packages ),
                    not_on_packages: test_api!( @helper $not_on_packages ),
                    only_on_parts: test_api!( @helper $only_on_parts ),
                    not_on_parts: test_api!( @helper $not_on_parts ),
                };

                assert_eq!(test.validate(&"dev1".to_string(), &LQFP32), $result1);
                assert_eq!(test.validate(&"dev2".to_string(), &LQFP32), $result2);
                assert_eq!(test.validate(&"dev1".to_string(), &TSSOP20), $result3);
                assert_eq!(test.validate(&"dev2".to_string(), &TSSOP20), $result4);
            }
        };
    }

    test_api!( no_only_or_not: { None, None, None, None } => (true, true, true, true));
    test_api!( only_on_packages: { P1, None, None, None } => (true, true, false, false));
    test_api!( only_on_all_packages: { allPackages, None, None, None } => (true, true, true, true));
    test_api!( not_on_packages:  { None, P1, None, None } => (false, false, true, true));
    test_api!( not_on_any_packages:  { None, allPackages, None, None } => (false, false, false, false));
    test_api!( only_and_not_on_same_package: { P1, P1, None, None } => (false, false, false, false));

    test_api!( only_on_part: { None, None, D1, None } => (true, false, true, false));
    test_api!( only_on_all_parts: { None, None, allParts, None } => (true, true, true, true));
    test_api!( not_on_part: { None, None, None, D1 } => (false, true, false, true));
    test_api!( not_on_any_part: { None, None, None, allParts } => (false, false, false, false));
    test_api!( only_and_not_on_same_part: { None, None, D1, D1 } => (false, false, false, false));

    test_api!( only_on_package_and_only_on_part: { P1, None, D1, None } => (true, false, false, false));
    test_api!( not_on_package_and_not_on_part: { None, P1, None, D1 } => (false, false, false, true));
    test_api!( not_on_package_and_only_on_part: { None, P1, D1, None } => (false, false, true, false));
    test_api!( only_on_package_and_not_on_part: { P1, None, None, D1 } => (false, true, false, false));
}
