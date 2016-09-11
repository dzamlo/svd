use errors::*;
use std::str::FromStr;
use types::*;
use utils::get_child_text;
use xmltree;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DimElementGroup {
    pub dim: Option<ScaledNonNegativeInteger>,
    pub dim_increment: Option<ScaledNonNegativeInteger>,
    pub dim_index: Option<DimIndexType>,
}

impl DimElementGroup {
    pub fn from_element(element: &xmltree::Element) -> Result<DimElementGroup> {
        let dim = match get_child_text(element, "dim") {
            Some(s) => Some(try!(s.parse())),
            None => None,
        };

        let dim_increment = match get_child_text(element, "dimIncrement") {
            Some(s) => Some(try!(s.parse())),
            None => None,
        };

        let dim_index = match get_child_text(element, "dimIndex") {
            Some(s) => Some(try!(s.parse())),
            None => None,
        };

        Ok(DimElementGroup {
            dim: dim,
            dim_increment: dim_increment,
            dim_index: dim_index,
        })
    }

    pub fn merge_derived_from(&mut self, derived_from: &DimElementGroup) {
        merge_option_field!(self.dim, derived_from.dim);
        merge_option_field!(self.dim_increment, derived_from.dim_increment);
        merge_option_field!(self.dim_index, derived_from.dim_index);
    }
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum DimIndexType {
    // [_0-9a-zA-Z]+(,\s*[_0-9a-zA-Z]+)+
    List(Vec<String>),
    // [A-Z]-[A-Z]
    CharRange { start: char, end: char },
    // [0-9]+\-[0-9]+|
    DecimalRange { start: u64, end: u64 },
}

fn is_dim_index_char_valid(c: char) -> bool {
    match c {
        '_' | '0'...'9' | 'a'...'z' | 'A'...'Z' => true,
        _ => false,
    }
}

fn is_dim_index_str_valid(s: &str) -> bool {
    s.chars().all(is_dim_index_char_valid)
}

impl FromStr for DimIndexType {
    type Err = Error;

    fn from_str(s: &str) -> Result<DimIndexType> {
        if s.contains('-') {
            let mut splitted = s.split('-');
            let left = splitted.next().unwrap();
            let right = splitted.next().unwrap();
            if left.len() == 1 && right.len() == 1 {
                let left_char = left.chars().next().unwrap();
                let right_char = right.chars().next().unwrap();
                if let ('A'...'Z', 'A'...'Z') = (left_char, right_char) {
                    return Ok(DimIndexType::CharRange {
                        start: left_char,
                        end: right_char,
                    });
                }
            }

            if let (Ok(start), Ok(end)) = (left.parse(), right.parse()) {
                Ok(DimIndexType::DecimalRange {
                    start: start,
                    end: end,
                })
            } else {
                Err(ErrorKind::UnexpectedValue("a value valid for dimIndex", s.to_string()).into())
            }
        } else {
            let list: Vec<_> = s.split(',').map(|s| s.trim().to_string()).collect();
            if !list.is_empty() && list.iter().all(|s| is_dim_index_str_valid(&*s)) {
                Ok(DimIndexType::List(list))
            } else {
                Err(ErrorKind::UnexpectedValue("a value valid for dimIndex", s.to_string()).into())
            }
        }
    }
}
