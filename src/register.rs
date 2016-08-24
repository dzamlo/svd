use data_type::DataType;
use dim_element_group::DimElementGroup;
use error::FromElementError;
use field::Field;
use is_similar::{IsSimilar, IsSimilarOptions};
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

    pub fn merge_derived_from(&mut self, derived_from: &Register) {
        self.derived_from = derived_from.derived_from.clone();
        self.dim_element.merge_derived_from(&derived_from.dim_element);
        merge_option_field!(self.display_name, derived_from.display_name);
        merge_option_field!(self.description, derived_from.description);
        merge_option_field!(self.alternate_group, derived_from.alternate_group);
        merge_option_field!(self.alternate_register, derived_from.alternate_register);
        self.register_properties = self.register_properties
            .merge(&derived_from.register_properties);
        merge_option_field!(self.data_type, derived_from.data_type);
        merge_option_field!(self.modified_write_values,
                            derived_from.modified_write_values);
        merge_option_field!(self.read_action, derived_from.read_action);
        merge_option_field!(self.fields, derived_from.fields);
    }

    #[allow(unknown_lints)]
    #[allow(needless_range_loop)]
    pub fn propagate_derived_from(&mut self) {
        if let Some(ref mut fields) = self.fields {
            for i in 0..fields.len() {
                while fields[i].derived_from.is_some() {
                    let mut field_derived_from = None;
                    if let Some(ref derived_from) = fields[i].derived_from {
                        for field in &*fields {
                            if field.name == *derived_from {
                                field_derived_from = Some(field.clone());
                                break;
                            }
                        }
                    }

                    if let Some(field_derived_from) = field_derived_from {
                        fields[i].merge_derived_from(&field_derived_from);
                    }
                }
            }
        }
    }
}

impl<'a, 'b> IsSimilar<&'a Register> for &'b Register {
    fn is_similar(self, other: &Register, options: &IsSimilarOptions) -> bool {
        self.name == other.name && self.address_offset == other.address_offset &&
        self.register_properties.is_similar(&other.register_properties, options) &&
        self.modified_write_values == other.modified_write_values &&
        self.read_action == other.read_action &&
        (options.ignore_fields() || self.fields.is_similar(&other.fields, options))
    }
}
