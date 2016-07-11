use access::Access;
use cpu::Cpu;
use error::{FromElementError, ParseError};
use peripheral::Peripheral;
use protection::Protection;
use std::io::Read;
use std::str::FromStr;
use types::*;
use xmltree;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Device {
    vendor: Option<String>,
    vendor_id: Option<IdentifierType>,
    name: IdentifierType,
    series: Option<String>,
    version: String,
    description: String,
    license_text: Option<String>,
    cpu: Option<Cpu>,
    header_system_filename: Option<IdentifierType>,
    header_definition_prefix: Option<IdentifierType>,
    address_unit_bits: ScaledNonNegativeInteger,
    width: ScaledNonNegativeInteger,
    size: Option<ScaledNonNegativeInteger>,
    access: Option<Access>,
    protection: Option<Protection>,
    reset_value: Option<ScaledNonNegativeInteger>,
    reset_mask: Option<ScaledNonNegativeInteger>,
    peripherals: Vec<Peripheral>,
}

fn get_child_text(element: &xmltree::Element, name: &str) -> Option<String> {
    element.get_child(name).map(|child| child.text.clone().unwrap_or_else(String::new))
}

impl Device {
    pub fn parse<R: Read>(r: R) -> Result<Device, ParseError> {
        let element = try!(xmltree::Element::parse(r));
        Device::from_element(&element).map_err(|e| e.into())
    }

    pub fn from_element(element: &xmltree::Element) -> Result<Device, FromElementError> {
        let vendor = get_child_text(element, "vendor");
        let vendor_id = get_child_text(element, "vendor_id");
        let name = get_child_text(element, "name");
        let series = get_child_text(element, "series");
        let version = get_child_text(element, "version");
        let description = get_child_text(element, "description");
        let license_text = get_child_text(element, "licenseText");
        let cpu = match element.get_child("cpu") {
            Some(element) => Some(try!(Cpu::from_element(element))),
            None => None,
        };
        let header_system_filename = get_child_text(element, "headerSystemFilename");
        let header_definition_prefix = get_child_text(element, "headerDefinitionsPrefix");
        let address_unit_bits = get_child_text(element, "addressUnitBits");
        let width = get_child_text(element, "width");
        let size = match get_child_text(element, "size") {
            Some(s) => Some(try!(ScaledNonNegativeInteger::from_str(&s))),
            None => None,
        };
        let access = match get_child_text(element, "access") {
            Some(s) => Some(try!(Access::from_str(&s))),
            None => None,
        };
        let protection = match get_child_text(element, "protection") {
            Some(s) => Some(try!(Protection::from_str(&s))),
            None => None,
        };
        let reset_value = match get_child_text(element, "resetValue") {
            Some(s) => Some(try!(ScaledNonNegativeInteger::from_str(&s))),
            None => None,
        };
        let reset_mask = match get_child_text(element, "resetMask") {
            Some(s) => Some(try!(ScaledNonNegativeInteger::from_str(&s))),
            None => None,
        };

        let peripherals = element.get_child("peripherals");

        if name.is_none() || version.is_none() || description.is_none() ||
           address_unit_bits.is_none() || width.is_none() || peripherals.is_none() {
            Err(FromElementError::MissingField)
        } else {
            let name = name.unwrap();
            let version = version.unwrap();
            let description = description.unwrap();
            let address_unit_bits =
                try!(ScaledNonNegativeInteger::from_str(&address_unit_bits.unwrap()));
            let width = try!(ScaledNonNegativeInteger::from_str(&width.unwrap()));
            let peripherals: Result<Vec<_>, FromElementError> =
                peripherals.unwrap().children.iter().map(Peripheral::from_element).collect();
            let peripherals = try!(peripherals);

            Ok(Device {
                vendor: vendor,
                vendor_id: vendor_id,
                name: name,
                series: series,
                version: version,
                description: description,
                license_text: license_text,
                cpu: cpu,
                header_system_filename: header_system_filename,
                header_definition_prefix: header_definition_prefix,
                address_unit_bits: address_unit_bits,
                width: width,
                size: size,
                access: access,
                protection: protection,
                reset_value: reset_value,
                reset_mask: reset_mask,
                peripherals: peripherals,
            })
        }
    }
}
