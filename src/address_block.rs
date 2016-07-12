use error::FromElementError;
use protection::Protection;
use std::str::FromStr;
use types::*;
use utils::get_child_text;
use xmltree;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Usage {
    Registers,
    Buffer,
    Reserved,
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct AddresBlock {
    offset: ScaledNonNegativeInteger,
    size: ScaledNonNegativeInteger,
    usage: Usage,
    protection: Option<Protection>,
}

impl FromStr for Usage {
    type Err = FromElementError;

    fn from_str(s: &str) -> Result<Usage, FromElementError> {
        match s {
            "registers" => Ok(Usage::Registers),
            "buffer" => Ok(Usage::Buffer),
            "reserved" => Ok(Usage::Reserved),
            _ => Err(FromElementError::InvalidFormat),
        }

    }
}

impl AddresBlock {
    pub fn from_element(element: &xmltree::Element) -> Result<AddresBlock, FromElementError> {
        let offset = get_child_text(element, "offset");
        let size = get_child_text(element, "size");
        let usage = get_child_text(element, "usage");
        let protection = match get_child_text(element, "protection") {
            Some(s) => Some(try!(s.parse())),
            None => None,
        };

        if offset.is_none() || size.is_none() || usage.is_none() {
            Err(FromElementError::MissingField)
        } else {
            let offset = try!(offset.unwrap().parse());
            let size = try!(size.unwrap().parse());
            let usage = try!(usage.unwrap().parse());

            Ok(AddresBlock {
                offset: offset,
                size: size,
                usage: usage,
                protection: protection,
            })
        }
    }
}
