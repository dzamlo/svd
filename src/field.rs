use error::FromElementError;
use xmltree;

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Field {

}

impl Field {
    pub fn from_element(_element: &xmltree::Element) -> Result<Field, FromElementError> {
        Ok(Field {})
    }
}
