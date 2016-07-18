use access::Access;
use bit_range::BitRange;
use enumerated_values::EnumeratedValues;
use error::FromElementError;
use modified_write_values::ModifiedWriteValues;
use read_action::ReadAction;
use types::*;
use utils::get_child_text;
use xmltree;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Field {
    pub derived_from: Option<IdentifierType>,
    pub name: IdentifierType,
    pub description: Option<String>,
    pub bit_range: BitRange,
    pub access: Option<Access>,
    pub modified_write_values: Option<ModifiedWriteValues>,
    pub read_action: Option<ReadAction>,
    pub enumerated_values: Vec<EnumeratedValues>,
}

impl Field {
    pub fn from_element(element: &xmltree::Element) -> Result<Field, FromElementError> {
        let derived_from = element.attributes.get("derivedFrom").cloned();
        let name = get_child_text(element, "name");
        let description = get_child_text(element, "description");
        let bit_range = try!(BitRange::from_element(element));

        let access = match get_child_text(element, "access") {
            Some(s) => Some(try!(s.parse())),
            None => None,
        };

        let modified_write_values = match get_child_text(element, "modifiedWriteValues") {
            Some(s) => Some(try!(s.parse())),
            None => None,
        };

        let read_action = match get_child_text(element, "readAction") {
            Some(s) => Some(try!(s.parse())),
            None => None,
        };

        let enumerated_values: Result<Vec<_>, FromElementError> = element.children
            .iter()
            .filter(|e| e.name == "enumeratedValues")
            .map(EnumeratedValues::from_element)
            .collect();
        let enumerated_values = try!(enumerated_values);

        if name.is_none() {
            Err(FromElementError::MissingField)
        } else {
            let name = name.unwrap();
            Ok(Field {
                derived_from: derived_from,
                name: name,
                description: description,
                bit_range: bit_range,
                access: access,
                modified_write_values: modified_write_values,
                read_action: read_action,
                enumerated_values: enumerated_values,
            })
        }

    }

    pub fn is_read(&self) -> bool {
        match self.access {
            Some(ref access) => access.is_read(),
            None => true,
        }
    }

    pub fn is_write(&self) -> bool {
        match self.access {
            Some(ref access) => access.is_write(),
            None => true,
        }
    }
}
