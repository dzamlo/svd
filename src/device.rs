use cpu::Cpu;
use error::{FromElementError, ParseError};
use peripheral::Peripheral;
use register_properties_group::RegisterPropertiesGroup;
use std::collections::HashMap;
use std::io::Read;
use types::*;
use utils::get_child_text;
use xmltree;

pub type PeripheralsMap<'a, 'b> = HashMap<&'a str, &'b Peripheral>;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Device {
    pub vendor: Option<String>,
    pub vendor_id: Option<IdentifierType>,
    pub name: IdentifierType,
    pub series: Option<String>,
    pub version: String,
    pub description: String,
    pub license_text: Option<String>,
    pub cpu: Option<Cpu>,
    pub header_system_filename: Option<IdentifierType>,
    pub header_definition_prefix: Option<IdentifierType>,
    pub address_unit_bits: ScaledNonNegativeInteger,
    pub width: ScaledNonNegativeInteger,
    pub register_properties: RegisterPropertiesGroup,
    pub peripherals: Vec<Peripheral>,
}

impl Device {
    pub fn from_reader<R: Read>(r: R) -> Result<Device, ParseError> {
        let element = try!(xmltree::Element::parse(r));
        let mut d = try!(Device::from_element(&element));
        d.propagate_register_properties();
        Ok(d)
    }

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
        let register_properties = try!(RegisterPropertiesGroup::from_element(element));
        let peripherals = element.get_child("peripherals");

        if name.is_none() || version.is_none() || description.is_none() ||
           address_unit_bits.is_none() || width.is_none() || peripherals.is_none() {
            Err(FromElementError::MissingField)
        } else {
            let name = name.unwrap();
            let version = version.unwrap();
            let description = description.unwrap();
            let address_unit_bits = try!(address_unit_bits.unwrap().parse());
            let width = try!(width.unwrap().parse());
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
                register_properties: register_properties,
                peripherals: peripherals,
            })
        }
    }

    pub fn peripherals_map(&self) -> PeripheralsMap {
        self.peripherals.iter().map(|p| (&*p.name, p)).collect()
    }

    pub fn propagate_register_properties(&mut self) {
        for peripheral in &mut self.peripherals {
            peripheral.propagate_register_properties(&self.register_properties);
        }
    }
}
