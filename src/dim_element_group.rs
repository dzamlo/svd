use error::FromElementError;
use types::*;
use utils::get_child_text;
use xmltree;

pub type DimIndexType = String;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct DimElementGroup {
    pub dim: Option<ScaledNonNegativeInteger>,
    pub dim_increment: Option<ScaledNonNegativeInteger>,
    pub dim_index: Option<DimIndexType>,
}

impl DimElementGroup {
    pub fn from_element(element: &xmltree::Element) -> Result<DimElementGroup, FromElementError> {
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
