use errors::*;
use types::*;
use utils::get_child_text;
use xmltree;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct BitRange {
    pub lsb: u32,
    pub msb: u32,
}

impl BitRange {
    pub fn from_element(element: &xmltree::Element) -> Result<BitRange> {
        let offset = get_child_text(element, "bitOffset");
        let width = get_child_text(element, "bitWidth");
        let lsb = get_child_text(element, "lsb");
        let msb = get_child_text(element, "msb");
        let bit_range = get_child_text(element, "bitRange");
        if let (Some(offset), Some(width)) = (offset, width) {
            let offset: ScaledNonNegativeInteger = try!(offset.parse());
            let offset = offset.0 as u32;
            let width: ScaledNonNegativeInteger = try!(width.parse());
            let width = width.0 as u32;

            Ok(BitRange {
                lsb: offset,
                msb: offset + width - 1,
            })
        } else if let (Some(lsb), Some(msb)) = (lsb, msb) {
            let lsb: ScaledNonNegativeInteger = try!(lsb.parse());
            let lsb = lsb.0 as u32;
            let msb: ScaledNonNegativeInteger = try!(msb.parse());
            let msb = msb.0 as u32;

            Ok(BitRange {
                lsb: lsb,
                msb: msb,
            })
        } else if let Some(bit_range) = bit_range {
            let colon_pos = bit_range.find(':');
            if !bit_range.starts_with('[') || !bit_range.ends_with(']') || colon_pos.is_none() {
                Err(ErrorKind::UnexpectedValue(" a value of the form \\[[0-9]+:[0-9]+\\]",
                                               bit_range.to_string())
                    .into())
            } else {
                let colon_pos = colon_pos.unwrap();
                let msb: u32 = try!(bit_range[1..colon_pos].parse());
                let lsb: u32 = try!(bit_range[colon_pos + 1..bit_range.len() - 1].parse());
                Ok(BitRange {
                    lsb: lsb,
                    msb: msb,
                })
            }
        } else {
            Err(ErrorKind::MissingField("field",
                                        "bitOffset and bitWidth, lsb and msb, or bitRange")
                .into())
        }
    }

    pub fn width(&self) -> u32 {
        self.msb - self.lsb + 1
    }
}
