use access::Access;
use bit_range::BitRange;
use enumerated_values::EnumeratedValues;
use error::FromElementError;
use modified_write_values::ModifiedWriteValues;
use read_action::ReadAction;
use std::collections::HashMap;
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

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct FieldsGroup {
    prefix: String,
    lsb: u32,
    width: u32,
    count: usize,
    lsb_increment: u32,
    access: Option<Access>,
}

impl FieldsGroup {
    /// Group similar fields together. Returns the groups found and the fields that can't be grouped
    pub fn from_fields<'a, I: IntoIterator<Item = &'a Field>>(fields: I)
                                                              -> (Vec<FieldsGroup>, Vec<Field>) {
        let mut prefix_map = HashMap::new();
        for field in fields {
            let (prefix, suffix) = extract_prefix(&field.name);
            prefix_map.entry(prefix.to_owned())
                .or_insert_with(|| vec![])
                .push((field.clone(), suffix));
        }

        let mut groups = vec![];
        let mut individual = vec![];
        for (prefix, mut fields) in prefix_map {
            if !prefix.is_empty() && should_group(&mut fields) {
                let first = &fields[0].0;
                let second = &fields[1].0;
                groups.push(FieldsGroup {
                    prefix: prefix,
                    lsb: first.bit_range.lsb,
                    width: first.bit_range.width(),
                    count: fields.len(),
                    lsb_increment: second.bit_range.lsb - first.bit_range.lsb,
                    access: first.access,
                });
            } else {
                for (field, _) in fields {
                    individual.push(field);
                }
            }
        }

        (groups, individual)
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

    pub fn prefix(&self) -> &str {
        &*self.prefix
    }
    pub fn lsb(&self) -> u32 {
        self.lsb
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn count(&self) -> usize {
        self.count
    }
    pub fn lsb_increment(&self) -> u32 {
        self.lsb_increment
    }
    pub fn access(&self) -> Option<Access> {
        self.access
    }
}

fn extract_prefix(name: &str) -> (&str, Option<usize>) {
    let prefix_end = name.rfind(|c: char| !c.is_digit(10));
    match prefix_end {
        Some(prefix_end) => (&name[..prefix_end + 1], name[prefix_end + 1..].parse().ok()),
        None => ("", name.parse().ok()),
    }

}

fn should_group(fields: &mut [(Field, Option<usize>)]) -> bool {
    if fields.len() > 1 && fields.iter().all(|&(_, suffix)| suffix.is_some()) {
        fields.sort_by_key(|&(_, suffix)| suffix);
        let suffix_correct =
            fields.iter().enumerate().all(|(idx, &(_, suffix))| suffix == Some(idx));
        let width = fields[0].0.bit_range.width();
        let same_width = fields.iter().all(|&(ref field, _)| field.bit_range.width() == width);
        let lsb_increment = fields[1].0.bit_range.lsb - fields[0].0.bit_range.lsb;
        let same_lsb_increment = fields.windows(2)
            .all(|pair| pair[1].0.bit_range.lsb - pair[0].0.bit_range.lsb == lsb_increment);
        let access = fields[0].0.access;
        let same_access = fields.iter().all(|&(ref field, _)| field.access == access);

        suffix_correct && same_width && same_lsb_increment && same_access
    } else {
        false
    }
    // false
}

#[cfg(test)]
mod tests {
    use super::extract_prefix;

    #[test]
    fn test_extract_prefix() {
        assert_eq!(("", None), extract_prefix(""));
        assert_eq!(("Foo", None), extract_prefix("Foo"));
        assert_eq!(("Foo", Some(123)), extract_prefix("Foo123"));
        assert_eq!(("Foo123Bar", Some(456)), extract_prefix("Foo123Bar456"));
        assert_eq!(("", Some(456)), extract_prefix("456"));
    }
}
