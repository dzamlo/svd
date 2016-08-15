use address_block::AddresBlock;
use dim_element_group::DimElementGroup;
use error::FromElementError;
use interrupt::Interrupt;
use register_or_cluster::RegisterOrCluster;
use register_properties_group::RegisterPropertiesGroup;
use types::*;
use utils::{extract_prefix, get_child_text};
use xmltree;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Peripheral {
    pub derived_from: Option<IdentifierType>,
    pub dim_element: DimElementGroup,
    pub name: IdentifierType,
    pub version: Option<String>,
    pub description: Option<String>,
    pub alternate_peripheral: Option<IdentifierType>,
    pub group_name: Option<String>,
    pub prepend_to_name: Option<IdentifierType>,
    pub append_to_name: Option<IdentifierType>,
    pub header_struct_name: Option<IdentifierType>,
    pub disable_condition: Option<String>,
    pub base_address: ScaledNonNegativeInteger,
    pub register_properties: RegisterPropertiesGroup,
    pub address_blocks: Vec<AddresBlock>,
    pub interrupts: Vec<Interrupt>,
    pub registers: Option<Vec<RegisterOrCluster>>,
}

impl Peripheral {
    pub fn from_element(element: &xmltree::Element) -> Result<Peripheral, FromElementError> {
        let derived_from = element.attributes.get("derivedFrom").cloned();
        let dim_element = try!(DimElementGroup::from_element(element));
        let name = get_child_text(element, "name");
        let version = get_child_text(element, "version");
        let description = get_child_text(element, "description");
        let alternate_peripheral = get_child_text(element, "alternatePeripheral");
        let group_name = get_child_text(element, "groupName");
        let prepend_to_name = get_child_text(element, "prependToName");
        let append_to_name = get_child_text(element, "appendToName");
        let header_struct_name = get_child_text(element, "headerStructName");
        let disable_condition = get_child_text(element, "disableCondition");
        let base_address = get_child_text(element, "baseAddress");
        let register_properties = try!(RegisterPropertiesGroup::from_element(element));
        let address_blocks: Result<Vec<_>, FromElementError> = element.children
            .iter()
            .filter(|e| e.name == "addressBlock")
            .map(AddresBlock::from_element)
            .collect();
        let address_blocks = try!(address_blocks);
        let interrupts: Result<Vec<_>, FromElementError> = element.children
            .iter()
            .filter(|e| e.name == "interrupt")
            .map(Interrupt::from_element)
            .collect();
        let interrupts = try!(interrupts);
        let registers = match element.get_child("registers") {
            Some(registers) => {
                let registers: Result<Vec<_>, FromElementError> =
                    registers.children.iter().map(RegisterOrCluster::from_element).collect();
                let registers = try!(registers);
                Some(registers)
            }
            None => None,
        };


        if name.is_none() || base_address.is_none() {
            Err(FromElementError::MissingField)
        } else {
            let name = name.unwrap();
            let base_address = try!(base_address.unwrap().parse());

            Ok(Peripheral {
                derived_from: derived_from,
                dim_element: dim_element,
                name: name,
                version: version,
                description: description,
                alternate_peripheral: alternate_peripheral,
                group_name: group_name,
                prepend_to_name: prepend_to_name,
                append_to_name: append_to_name,
                header_struct_name: header_struct_name,
                disable_condition: disable_condition,
                base_address: base_address,
                register_properties: register_properties,
                address_blocks: address_blocks,
                interrupts: interrupts,
                registers: registers,
            })
        }
    }

    pub fn propagate_register_properties(&mut self,
                                         register_properties: &RegisterPropertiesGroup) {
        self.register_properties = self.register_properties.merge(register_properties);
        if let Some(ref mut registers) = self.registers {
            for r_or_c in registers {
                r_or_c.propagate_register_properties(&self.register_properties);
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct PeripheralsGroup {
    struct_name: IdentifierType,
    peripherals: Vec<Peripheral>,
}

impl PeripheralsGroup {
    /// Group similar peripherals together.
    pub fn from_peripherals<'a, I>(peripherals: I) -> (Vec<PeripheralsGroup>, Vec<Peripheral>)
        where I: IntoIterator<Item = &'a Peripheral>
    {
        let mut groups: Vec<Vec<Peripheral>> = vec![];
        for peripheral in peripherals {
            let mut group_found = false;
            for group in &mut groups {
                if should_group(group, peripheral) {
                    group.push(peripheral.clone());
                    group_found = true;
                    break;
                }
            }

            if !group_found {
                groups.push(vec![peripheral.clone()])
            }
        }

        let mut groups2 = vec![];
        let mut individuals = vec![];

        for mut group in groups {
            if group.len() == 1 {
                individuals.append(&mut group);
            } else {
                let struct_name = struct_name(&group);
                if struct_name.is_empty() {
                    individuals.append(&mut group);
                } else {
                    groups2.push(PeripheralsGroup {
                        struct_name: struct_name,
                        peripherals: group,
                    });
                }
            }
        }

        (groups2, individuals)
    }

    pub fn struct_name(&self) -> &str {
        &*self.struct_name
    }

    pub fn peripherals(&self) -> &[Peripheral] {
        &*self.peripherals
    }
}

fn should_group(group: &[Peripheral], peripheral: &Peripheral) -> bool {
    match peripheral.derived_from {
        Some(ref derived_from) if derived_from == &group[0].name => true,
        _ => false,
    }
}

fn struct_name(peripherals: &[Peripheral]) -> IdentifierType {
    for peripheral in peripherals {
        if let Some(ref name) = peripheral.header_struct_name {
            return name.clone();
        }
    }

    if peripherals[0].name.chars().last().map_or(false, |c| c.is_digit(10)) {
        return extract_prefix(&peripherals[0].name).0.into();
    }

    let mut names: Vec<_> = peripherals.iter().map(|p| p.name.chars()).collect();
    let mut struct_name = String::new();
    let min_len = peripherals.iter().map(|p| p.name.len()).min().unwrap_or(0);
    for _ in 0..min_len {
        let c = names[0].next();
        if c.is_some() && names[1..].iter_mut().all(|name| name.next() == c) {
            struct_name.push(c.unwrap());
        } else {
            break;
        }
    }

    struct_name
}
