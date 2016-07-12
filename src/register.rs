use error::FromElementError;
use xmltree;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Register {

}

impl Register {
    pub fn from_element(_element: &xmltree::Element) -> Result<Register, FromElementError> {
        Ok(Register {})
    }
}
