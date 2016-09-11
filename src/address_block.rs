use errors::*;
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
    pub offset: ScaledNonNegativeInteger,
    pub size: ScaledNonNegativeInteger,
    pub usage: Usage,
    pub protection: Option<Protection>,
}

impl FromStr for Usage {
    type Err = Error;

    fn from_str(s: &str) -> Result<Usage> {
        match s {
            "registers" => Ok(Usage::Registers),
            "buffer" => Ok(Usage::Buffer),
            "reserved" => Ok(Usage::Reserved),
            _ => {
                Err(ErrorKind::UnexpectedValue("one of registers, buffer or reserved",
                                               s.to_string())
                    .into())
            }

        }

    }
}

impl AddresBlock {
    pub fn from_element(element: &xmltree::Element) -> Result<AddresBlock> {
        let offset = get_mandatory_child_text!(element, "addressBlock", "offset");
        let size = get_mandatory_child_text!(element, "addressBlock", "size");
        let usage = get_mandatory_child_text!(element, "addressBlock", "usage");
        let protection = match get_child_text(element, "protection") {
            Some(s) => Some(try!(s.parse())),
            None => None,
        };

        let offset = try!(offset.parse());
        let size = try!(size.parse());
        let usage = try!(usage.parse());

        Ok(AddresBlock {
            offset: offset,
            size: size,
            usage: usage,
            protection: protection,
        })

    }
}
