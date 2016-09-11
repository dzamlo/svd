use errors::*;
use std::str::FromStr;
use types::*;
use utils::get_child_text;
use xmltree;

pub type EnumerationName = String;


str_enum!{EnumUsage,
    "read" => Read,
    "write" => Write,
    "read-write" => ReadWrite,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum EnumeratedValueData {
    IsDefault(bool),
    Value { value: u64, do_not_care: u64 },
}

impl EnumeratedValueData {
    pub fn from_value_str(s: &str) -> Result<EnumeratedValueData> {
        let s = if s.starts_with('x') { &s[1..] } else { s };

        if s.starts_with('#') && (s.contains('x') || s.contains('X')) {
            let mut do_not_care = 0;
            let mut value = 0;
            for c in s.chars().skip(1) {
                do_not_care <<= 1;
                value <<= 1;
                if c == 'x' || c == 'X' {
                    do_not_care |= 1;
                } else if c == '1' {
                    value |= 1;
                } else if c != '0' {
                    return Err(ErrorKind::UnexpectedValue("one of x, X, 0 or 1", c.to_string())
                        .into());
                }
            }
            Ok(EnumeratedValueData::Value {
                value: value,
                do_not_care: do_not_care,
            })
        } else {
            let v: ScaledNonNegativeInteger = try!(s.parse());
            Ok(EnumeratedValueData::Value {
                value: v.0,
                do_not_care: 0,
            })
        }
    }

    pub fn from_element(element: &xmltree::Element) -> Result<EnumeratedValueData> {
        if let Some(is_default) = get_child_text(element, "isDefault") {
            match &*is_default {
                "true" => Ok(EnumeratedValueData::IsDefault(true)),
                "false" => Ok(EnumeratedValueData::IsDefault(false)),
                _ => {
                    Err(ErrorKind::UnexpectedValue("one of true or false", is_default.to_string())
                        .into())
                }
            }
        } else if let Some(value) = get_child_text(element, "value") {
            EnumeratedValueData::from_value_str(&value)
        } else {
            Err(ErrorKind::MissingField("enumeratedValue", "isDefault or value").into())
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EnumeratedValue {
    pub name: EnumerationName,
    pub description: Option<String>,
    pub value: EnumeratedValueData,
}

impl EnumeratedValue {
    pub fn from_element(element: &xmltree::Element) -> Result<EnumeratedValue> {
        let name = get_mandatory_child_text!(element, "enumeratedValue", "name");
        let description = get_child_text(element, "description");
        let value = try!(EnumeratedValueData::from_element(element));
        Ok(EnumeratedValue {
            name: name,
            description: description,
            value: value,
        })
    }
}


#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct EnumeratedValues {
    derived_from: Option<IdentifierType>,
    name: Option<EnumerationName>,
    usage: Option<EnumUsage>,
    enumerated_values: Vec<EnumeratedValue>,
}

impl EnumeratedValues {
    pub fn from_element(element: &xmltree::Element) -> Result<EnumeratedValues> {
        let derived_from = element.attributes.get("derivedFrom").cloned();
        let name = get_child_text(element, "name");
        let usage = match get_child_text(element, "usage") {
            Some(s) => Some(try!(s.parse())),
            None => None,
        };

        let enumerated_values: Result<Vec<_>> = element.children
            .iter()
            .filter(|e| e.name == "enumeratedValue")
            .map(EnumeratedValue::from_element)
            .collect();
        let enumerated_values = try!(enumerated_values);
        if enumerated_values.is_empty() {
            Err(ErrorKind::MissingField("enumeratedValues", "enumeratedValue").into())
        } else {
            Ok(EnumeratedValues {
                derived_from: derived_from,
                name: name,
                usage: usage,
                enumerated_values: enumerated_values,
            })
        }
    }
}
