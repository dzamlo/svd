use dim_element_group::DimElementGroup;
use data_type::DataType;
use error::FromElementError;
use field::Field;
use modified_write_values::ModifiedWriteValues;
use read_action::ReadAction;
use register_properties_group::RegisterPropertiesGroup;
use types::*;
use utils::get_child_text;
use xmltree;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Register {
    pub derived_from: Option<IdentifierType>,
    pub dim_element: DimElementGroup,
    pub name: IdentifierType,
    pub display_name: Option<String>,
    pub description: Option<String>,
    pub alternate_group: Option<IdentifierType>,
    pub alternate_register: Option<IdentifierType>,
    pub address_offset: ScaledNonNegativeInteger,
    pub register_properties: RegisterPropertiesGroup,
    pub data_type: Option<DataType>,
    pub modified_write_values: Option<ModifiedWriteValues>,
    pub read_action: Option<ReadAction>,
    pub fields: Option<Vec<Field>>,
}

impl Register {
    pub fn from_element(element: &xmltree::Element) -> Result<Register, FromElementError> {
        let derived_from = element.attributes.get("derivedFrom").cloned();
        let dim_element = try!(DimElementGroup::from_element(element));
        let name = get_child_text(element, "name");
        let display_name = get_child_text(element, "displayName");
        let description = get_child_text(element, "description");
        let alternate_group = get_child_text(element, "alternateGroup");
        let alternate_register = get_child_text(element, "alternateRegister");
        let address_offset = get_child_text(element, "addressOffset");
        let register_properties = try!(RegisterPropertiesGroup::from_element(element));
        let data_type = match get_child_text(element, "dataType") {
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

        let fields = match element.get_child("fields") {
            Some(fields) => {
                let fields: Result<Vec<_>, FromElementError> =
                    fields.children.iter().map(Field::from_element).collect();
                let fields = try!(fields);
                Some(fields)
            }
            None => None,
        };

        if name.is_none() || address_offset.is_none() {
            Err(FromElementError::MissingField)
        } else {
            let name = name.unwrap();
            let address_offset = try!(address_offset.unwrap().parse());
            Ok(Register {
                derived_from: derived_from,
                dim_element: dim_element,
                name: name,
                display_name: display_name,
                description: description,
                alternate_group: alternate_group,
                alternate_register: alternate_register,
                address_offset: address_offset,
                register_properties: register_properties,
                data_type: data_type,
                modified_write_values: modified_write_values,
                read_action: read_action,
                fields: fields,
            })

        }
    }

    pub fn is_read(&self) -> bool {
        match self.register_properties.access {
            Some(ref access) => access.is_read(),
            None => true,
        }
    }

    pub fn is_write(&self) -> bool {
        match self.register_properties.access {
            Some(ref access) => access.is_write(),
            None => true,
        }
    }

    pub fn size(&self) -> u64 {
        self.register_properties.size.map_or(32, |s| s.0)
    }
}
