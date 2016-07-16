use access::Access;
use error::FromElementError;
use protection::Protection;
use types::*;
use utils::get_child_text;
use xmltree;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct RegisterPropertiesGroup {
    pub size: Option<ScaledNonNegativeInteger>,
    pub access: Option<Access>,
    pub protection: Option<Protection>,
    pub reset_value: Option<ScaledNonNegativeInteger>,
    pub reset_mask: Option<ScaledNonNegativeInteger>,
}

impl RegisterPropertiesGroup {
    pub fn from_element(element: &xmltree::Element)
                        -> Result<RegisterPropertiesGroup, FromElementError> {
        let size = match get_child_text(element, "size") {
            Some(s) => Some(try!(s.parse())),
            None => None,
        };
        let access = match get_child_text(element, "access") {
            Some(s) => Some(try!(s.parse())),
            None => None,
        };
        let protection = match get_child_text(element, "protection") {
            Some(s) => Some(try!(s.parse())),
            None => None,
        };
        let reset_value = match get_child_text(element, "resetValue") {
            Some(s) => Some(try!(s.parse())),
            None => None,
        };
        let reset_mask = match get_child_text(element, "resetMask") {
            Some(s) => Some(try!(s.parse())),
            None => None,
        };
        Ok(RegisterPropertiesGroup {
            size: size,
            access: access,
            protection: protection,
            reset_value: reset_value,
            reset_mask: reset_mask,
        })
    }

    pub fn merge(&self, other: &RegisterPropertiesGroup) -> RegisterPropertiesGroup {
        RegisterPropertiesGroup {
            size: self.size.or(other.size),
            access: self.access.or(other.access),
            protection: self.protection.or(other.protection),
            reset_value: self.reset_value.or(other.reset_value),
            reset_mask: self.reset_mask.or(other.reset_mask),
        }
    }
}
