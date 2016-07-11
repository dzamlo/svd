use error::FromElementError;
use xmltree;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Cpu {

}

impl Cpu {
    pub fn from_element(_element: &xmltree::Element) -> Result<Cpu, FromElementError> {
        Ok(Cpu {})
    }
}
