use error::FromElementError;
use xmltree;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Peripheral {

}

impl Peripheral {
    pub fn from_element(_element: &xmltree::Element) -> Result<Peripheral, FromElementError> {
        Ok(Peripheral {})
    }
}
